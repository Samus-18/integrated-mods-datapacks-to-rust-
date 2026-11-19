//! Aetheria Horizon first native/wasm visual preview. Five 3D fantasy spell shapes.
//! These are LOCAL client effects only; no server damage, block edits or claimed FPS.
use mod_api::{
    self, Hooks, Registrar, RegistrationContext, KeyRegistration, KeyConflictContext,
    ClientHost, ClientTickPostEvent, ClientSetupEvent, ClientResourcesReloadedEvent,
    SubmitCustomGeometryEvent, GuiRenderPostEvent, RegisterRenderPipelinesEvent,
    RenderPipeline, RenderPipelineRef, PipelineBlend, PipelineQuad, WorldDepth,
    WorldQuad, Identifier,
};
use std::{f64::consts::TAU, sync::Mutex};

const MOD_ID: &str = "aetheria_preview";
const IDS: [&str;5] = ["phoenix_garden", "glacier_cathedral", "worldroot", "thunder_wyrm", "astral_sunder"];
const KEYS: [&str;5] = ["key.keyboard.z", "key.keyboard.x", "key.keyboard.c", "key.keyboard.v", "key.keyboard.b"];
const NAMES: [&str;5] = ["Phoenix Garden", "Glacier Cathedral", "Living Worldroot", "Thunder Wyrm", "Astral Sunder"];
const VARIANT_NAMES: [[&str;3];5] = [
    ["Phoenix Garden", "Phoenix Wings", "Ember Cyclone"],
    ["Glacier Cathedral", "Glacier Arches", "Frost Surge"],
    ["Living Worldroot", "Living Cage", "Verdant Canopy"],
    ["Thunder Wyrm", "Storm Antlers", "Storm Manta"],
    ["Astral Sunder", "Void Blossom", "Fractured Stair"],
];

#[derive(Clone, Copy)]
struct Cast { kind: usize, variant: u8, start: u64, center: [f64;3], direction: [f64;3] }
#[derive(Default)]
struct State { tick: u64, effects: Vec<Cast>, pipeline: Option<RenderPipelineRef>, last_cast: Option<usize>, last_variant: u8, variant_cycle: [u8; 5] }
#[derive(Default)]
struct Mod { state: Mutex<State> }

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        for i in 0..5 {
            registrar.register_key_mapping(
                KeyRegistration::new(IDS[i], KEYS[i])
                    .context(KeyConflictContext::InGame)
                    .label(NAMES[i])
                    .category("Aetheria magic")
            );
        }
    }

    fn register_render_pipelines(
        &self, _: &RegistrationContext, event: &mut RegisterRenderPipelinesEvent<'_>
    ) {
        let id = Identifier::parse("aetheria_preview:ethereal").unwrap();
        // Optional shader: if unsupported on a local build, plain geometry still draws.
        event.pipelines.push(RenderPipeline::new(id.clone(), id,
            PipelineBlend::Translucent, WorldDepth::Tested).optional());
    }

    fn client_setup(&self, host: &mut dyn ClientHost, _: &mut ClientSetupEvent) {
        self.resolve_pipeline(host);
    }
    fn client_resources_reloaded(&self, host: &mut dyn ClientHost,
        _: &mut ClientResourcesReloadedEvent) {
        self.resolve_pipeline(host);
    }
    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        let playing = host.playing();
        let mut pressed = Vec::new();
        if playing {
            for (i, id) in IDS.iter().enumerate() {
                if host.consume_click(MOD_ID, id) { pressed.push(i); }
            }
        }
        let camera = if pressed.is_empty() { None } else { host.camera() };
        let mut state = self.state.lock().unwrap();
        if !playing {
            state.effects.clear();
            return;
        }
        state.tick = state.tick.saturating_add(1);
        let now = state.tick;
        state.effects.retain(|c| now.saturating_sub(c.start) <= 104);
        if let Some(cam) = camera {
            let yaw = f64::from(cam.yaw).to_radians();
            let pitch = f64::from(cam.pitch).to_radians();
            let dir = [-yaw.sin()*pitch.cos(), -pitch.sin(), yaw.cos()*pitch.cos()];
            if cam.position.iter().all(|v| v.is_finite()) && dir.iter().all(|v| v.is_finite()) {
                for kind in pressed {
                    let center = add(cam.position, scale(dir, 7.0));
                    if state.effects.len() >= 8 { state.effects.remove(0); }
                    let tick = state.tick;
                    // Each repeated cast cycles through three *different geometries*.
                    let variant = state.variant_cycle[kind];
                    state.variant_cycle[kind] = (variant + 1) % 3;
                    state.effects.push(Cast {kind, variant, start: tick, center, direction: dir});
                    state.last_cast = Some(kind);
                    state.last_variant = variant;
                }
            }
        }
    }

    fn submit_custom_geometry(&self, _: &dyn ClientHost, event: &mut SubmitCustomGeometryEvent<'_>) {
        let (effects, now, pipeline) = {
            let state = self.state.lock().unwrap();
            (state.effects.clone(), state.tick, state.pipeline)
        };
        if effects.is_empty() { return; }
        let time = f64::from(event.view.partial_tick.clamp(0.0, 1.0));
        let camera = event.view.camera;
        let capacity = event.draw.usage().quad_capacity.saturating_sub(event.draw.usage().quads);
        // Fair allocation: none of the active spells can consume the entire draw budget.
        let live: Vec<(Cast,f64)> = effects.iter().filter_map(|cast| {
            let age = (now.saturating_sub(cast.start)) as f64 + time;
            (age < 105.0 && distance2(camera, cast.center) <= 115.0 * 115.0)
                .then_some((*cast, age))
        }).collect();
        if live.is_empty() { return; }
        let total_budget = capacity.min(900) as usize;
        let per_spell_budget = total_budget / live.len();
        let mut scene = Scene::new(total_budget);
        for (cast, age) in live {
            let mut spell = Scene::new(per_spell_budget);
            render_spell(cast, age, &mut spell);
            scene.facets.extend(spell.facets);
        }
        if let Some(pipeline) = pipeline {
            let quads: Vec<PipelineQuad> = scene.facets.iter().map(|(corners, color)|
                PipelineQuad::spanning(*corners, *color)).collect();
            if event.draw.pipeline_quads(pipeline, &quads).is_ok() { return; }
        }
        // Fallback on plain geometry if optional shader could not compile or resolve.
        let quads: Vec<WorldQuad> = scene.facets.iter().map(|(corners,color)|
            WorldQuad::new(*corners, *color, WorldDepth::Tested)).collect();
        let _ = event.draw.quads(&quads);
    }

    fn gui_render_post(&self, _: &dyn ClientHost, event: &mut GuiRenderPostEvent<'_>) {
        if event.gui_hidden || !event.focused { return; }
        event.batch.text("AETHERIA  |  Z Phoenix  X Glacier  C Worldroot  V Wyrm  B Rift",
            [8.0, 9.0], 0xFFC9D5E9, true);
        let state = self.state.lock().unwrap();
        if let Some(last) = state.last_cast {
            event.batch.text(VARIANT_NAMES[last][state.last_variant as usize], [8.0, 22.0], 0xFFE8C9FF, true);
        }
    }
}
impl Mod {
    fn resolve_pipeline(&self, host: &dyn ClientHost) {
        let id = Identifier::parse("aetheria_preview:ethereal").unwrap();
        let shader = host.render_assets().render_pipeline(&id).ok();
        self.state.lock().unwrap().pipeline = shader;
    }
}

fn add(a:[f64;3],b:[f64;3])->[f64;3]{[a[0]+b[0],a[1]+b[1],a[2]+b[2]]}
fn sub(a:[f64;3],b:[f64;3])->[f64;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn scale(a:[f64;3],s:f64)->[f64;3]{[a[0]*s,a[1]*s,a[2]*s]}
fn length(a:[f64;3])->f64{(a[0]*a[0]+a[1]*a[1]+a[2]*a[2]).sqrt()}
fn norm(a:[f64;3])->[f64;3]{let s=length(a).max(0.000001);scale(a,1.0/s)}
fn cross(a:[f64;3],b:[f64;3])->[f64;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn distance2(a:[f64;3],b:[f64;3])->f64{let d=sub(a,b);d[0]*d[0]+d[1]*d[1]+d[2]*d[2]}
fn c(a:[f64;3],b:[f64;3],d:[f64;3])->[f64;3]{add(add(a,b),d)}
fn alpha(color:u32, age:f64, end:f64)->u32 {
    let opacity = ((end - age)/18.0).clamp(0.0,1.0);
    let a = (((color >> 24)&255) as f64 * opacity).round() as u32;
    (color & 0x00FF_FFFF) | (a << 24)
}

struct Scene { facets:Vec<([[f64;3];4],u32)>, budget:usize }
impl Scene {
    fn new(budget:usize)->Self { Self{facets:Vec::new(),budget} }
    fn quad(&mut self,points:[[f64;3];4],color:u32) {
        if self.facets.len() < self.budget && points.iter().flatten().all(|v|v.is_finite()) {
            self.facets.push((points,color));
        }
    }
    fn shard(&mut self, from:[f64;3], to:[f64;3], width:f64, twist:f64, color:u32){
        let axis=norm(sub(to,from));
        let basis=if axis[1].abs()<0.8 {[0.0,1.0,0.0]}else{[1.0,0.0,0.0]};
        let tangent=norm(cross(axis,basis));
        let normal=norm(cross(axis,tangent));
        let side=add(scale(tangent,twist.cos()*width),scale(normal,twist.sin()*width));
        let quarter=add(scale(tangent,-twist.sin()*width*0.8),scale(normal,twist.cos()*width*0.8));
        let middle=add(from,scale(sub(to,from),0.62));
        self.quad([from,add(middle,side),to,sub(middle,side)],color);
        self.quad([from,add(middle,quarter),to,sub(middle,quarter)],color);
    }
    fn leaf(&mut self,at:[f64;3],dir:[f64;3],size:f64,color:u32){
        let tip=add(at,scale(dir,size));
        self.shard(at,tip,size*0.28,(at[0]+at[2]).sin(),color);
    }
    fn membrane(&mut self, a:[f64;3],b:[f64;3],c0:[f64;3],d:[f64;3],color:u32){
        self.quad([a,b,c0,d],color)
    }
}

fn render_spell(cast:Cast, age:f64, scene:&mut Scene) {
    let t=(age/60.0).clamp(0.0,1.5);
    // Gradually reveal shape; fade begins late in its distinct lifetime.
    let grow=(age/12.0).clamp(0.0,1.0).powf(0.66);
    match (cast.kind, cast.variant) {
        (0,0)=>phoenix_garden(cast.center,age,grow,scene),
        (0,1)=>phoenix_wings(cast.center,age,grow,scene),
        (0,_)=>ember_cyclone(cast.center,age,grow,scene),
        (1,0)=>glacier_cathedral(cast.center,age,grow,scene),
        (1,1)=>glacier_arches(cast.center,age,grow,scene),
        (1,_)=>frost_surge(cast.center,age,grow,scene),
        (2,0)=>worldroot(cast.center,age,grow,scene),
        (2,1)=>living_cage(cast.center,age,grow,scene),
        (2,_)=>verdant_canopy(cast.center,age,grow,scene),
        (3,0)=>thunder_wyrm(cast.center,cast.direction,age,grow,scene),
        (3,1)=>storm_antlers(cast.center,age,grow,scene),
        (3,_)=>storm_manta(cast.center,cast.direction,age,grow,scene),
        (4,0)=>astral_sunder(cast.center,age,t,grow,scene),
        (4,1)=>void_blossom(cast.center,age,grow,scene),
        (4,_)=>fractured_stair(cast.center,age,grow,scene),
        _ => {}
    }
}

/// 1: Opening, asymmetrical fiery blossom with featherlike 3D petals and a rising phoenix tail.
fn phoenix_garden(o:[f64;3], age:f64, grow:f64, scene:&mut Scene) {
    let n=56;
    for i in 0..n {
        let u=i as f64/n as f64;
        let spiral=TAU*u*4.0+age*0.045;
        let radius=(0.18+4.0*u.powf(0.68))*grow;
        let stem=c(o,[spiral.cos()*radius, 0.2+4.1*u*grow,spiral.sin()*radius],[0.0;3]);
        let length=0.9+2.6*(1.0-u);
        let tip=add(stem,[(spiral+0.65).cos()*length,0.7+length*0.33,(spiral+0.65).sin()*length]);
        let color=if i%3==0 {0xDFFFF1A0} else if i%3==1 {0xDFFF6826} else {0xBFC92A52};
        scene.shard(stem,tip,0.32+(1.0-u)*0.55,spiral,alpha(color,age,96.0));
        let feather_tip=add(tip,[0.3*spiral.cos(),0.5,0.3*spiral.sin()]);
        scene.shard(tip,feather_tip,0.13,spiral+0.2,alpha(0xAFFFF6D2,age,96.0));
    }
    for j in 0..16 {
        let ang=TAU*j as f64/16.0+age*0.032;
        let tip=add(o,[3.0*ang.cos(),4.3+0.3*ang.sin(),3.0*ang.sin()]);
        scene.shard(add(o,[0.0,1.4,0.0]),tip,0.48,ang,alpha(0xAFFFAB29,age,96.0));
    }
}

/// 2: Growth of interlocking glacial crystalline vaults, no flat circles or sticks.
fn glacier_cathedral(o:[f64;3],age:f64,grow:f64,scene:&mut Scene){
    for side in 0..6 {
        let theta=TAU*side as f64/6.0;
        let dir=[theta.cos(),0.0,theta.sin()];
        let tangent=[-theta.sin(),0.0,theta.cos()];
        for rung in 0..7 {
            let k=rung as f64;
            let offset=scale(dir,(0.6+k*0.6)*grow);
            let base=add(o,add(offset,[0.0,-1.2,0.0]));
            let height=(3.8+k*0.8)*(0.85+0.15*(age*0.05+k).sin())*grow;
            let tip=add(base,c(scale(dir,0.9+0.25*k),[0.0,height,0.0],scale(tangent,(k*2.3).sin()*0.5)));
            let width=(0.36+0.1*k)*grow;
            scene.shard(base,tip,width,theta+0.35*k,alpha(if (side+rung)%3==0 {0xBEE6FFFF}else{0xB66ABDFF},age,104.0));
            let subtip=add(base,c(scale(dir,0.8),[0.0,height*0.65,0.0],scale(tangent,0.6)));
            scene.shard(add(base,[0.0,0.35,0.0]),subtip,width*0.35,theta+1.1,alpha(0xBFCDEFFF,age,104.0));
        }
    }
    // floating splinter storm
    for i in 0..32 {
        let a=i as f64*2.399963+age*0.017;
        let r=2.2+0.5*(age*0.08+i as f64).sin();
        let base=add(o,[a.cos()*r, 2.0+(i%5) as f64*0.48,a.sin()*r]);
        scene.shard(base,add(base,[a.sin()*0.7,0.9,a.cos()*0.7]),0.12,a,alpha(0xB4E2FAFF,age,104.0));
    }
}

/// 3: Branching organic roots, unfolding leaf clusters and bioluminescent blossoms.
fn worldroot(o:[f64;3],age:f64,grow:f64,scene:&mut Scene){
    for branch in 0..12 {
        let a=TAU*branch as f64/12.0;
        let mut p=add(o,[a.cos()*0.18, -0.8, a.sin()*0.18]);
        let sweep=3.5+0.6*(branch as f64*1.9).sin();
        for step in 0..11 {
            let v=step as f64/11.0;
            let theta=a+v*0.8*(branch as f64*2.1).sin();
            let next=add(o,[theta.cos()*sweep*v*grow,
                -0.8+v*(5.0+(branch%4) as f64*0.55)*grow,
                theta.sin()*sweep*v*grow]);
            scene.shard(p,next,(0.25*(1.0-v)+0.05)*grow,a+v,alpha(0xD66A9D45,age,103.0));
            if step>2 && step%2==0 {
                let leaf0=add(next,[0.25*theta.cos(),0.1,0.25*theta.sin()]);
                scene.leaf(leaf0,[theta.sin(),0.6,theta.cos()],1.0+v,alpha(0xB350F7A2,age,103.0));
                scene.leaf(leaf0,[-theta.sin(),0.8,-theta.cos()],0.8+v,alpha(0xB79BE865,age,103.0));
            }
            p=next;
        }
    }
    for i in 0..28 {
        let a=i as f64*2.399963;
        let r=(i%7) as f64*0.37+0.8;
        let at=add(o,[a.cos()*r, 1.3+(i%9) as f64*0.5+0.25*(age*0.1+i as f64).sin(),a.sin()*r]);
        scene.shard(at,add(at,[0.05,0.35,0.05]),0.10,a,alpha(0xD6E9FF9A,age,103.0));
    }
}

/// 4: A coiling thunder-serpent *body* with layered armour fins and bright moving sparks.
fn thunder_wyrm(o:[f64;3],forward:[f64;3],age:f64,grow:f64,scene:&mut Scene){
    let f=norm(forward);let side=norm(cross(f,[0.0,1.0,0.0]));
    let up=norm(cross(side,f));
    let head=add(o,add(scale(up,1.0),scale(f,3.0)));
    let mut last=head;
    for i in 1..66 {
        let u=i as f64/65.0;
        let wiggle=(u*TAU*3.4-age*0.15).sin();
        let s=scale(side,wiggle*(0.5+2.2*u));
        let v=scale(up,(u*TAU*2.2+age*0.10).cos()*1.3+2.0*u);
        let path=add(o,add(add(s,v),scale(f,(3.0-12.0*u)*grow)));
        let thickness=(0.30+0.62*(1.0-u))*grow;
        scene.shard(last,path,thickness,i as f64*0.19,alpha(if i%3==0 {0xD7FCF3FF}else{0xAE7D69F7},age,99.0));
        if i%3==0 {
            let fin=add(path,add(scale(side,((i%2) as f64*2.0-1.0)*(1.0+u)),scale(up,1.2)));
            scene.shard(path,fin,0.2*(1.0-u),i as f64,alpha(0xA9E7DBFF,age,99.0));
        }
        if i%2==0 {
            let spark=add(path,scale(side,1.0+(i%5) as f64*0.25));
            scene.shard(path,spark,0.055,i as f64*2.0,alpha(0xEFD6FFFF,age,99.0));
        }
        last=path;
    }
    // head crown: asymmetric triangular crests
    for k in 0..5 {
        let a=TAU*k as f64/5.0;
        scene.shard(head,add(head,add(scale(side,a.cos()*1.3),scale(up,a.sin()*1.3))),0.27,a,alpha(0xD5FFFAFF,age,99.0));
    }
}

/// 5: Splintered void *tear*, irregular folding membranes and sharp orbiting debris.
fn astral_sunder(o:[f64;3],age:f64,t:f64,grow:f64,scene:&mut Scene){
    let wobble=age*0.035;
    for i in 0..45 {
        let a=TAU*i as f64/45.0;
        let b=TAU*(i+1) as f64/45.0;
        let r1=(1.65+(a*3.0).sin()*0.55+(a*7.0).cos()*0.35)*grow;
        let r2=(1.65+(b*3.0).sin()*0.55+(b*7.0).cos()*0.35)*grow;
        let f1=add(o,[r1*a.cos(),r1*a.sin()*1.5,(a*5.0+wobble).sin()*0.7]);
        let f2=add(o,[r2*b.cos(),r2*b.sin()*1.5,(b*5.0+wobble).sin()*0.7]);
        let edge1=add(o,[(r1+1.25)*a.cos(),(r1+1.4)*a.sin()*1.5,(a*4.0).sin()*1.4]);
        let edge2=add(o,[(r2+1.25)*b.cos(),(r2+1.4)*b.sin()*1.5,(b*4.0).sin()*1.4]);
        let col=if i%4==0 {0xB2F2C7FF} else if i%4==1 {0xC45F38BD} else {0xA86939C9};
        scene.membrane(f1,edge1,edge2,f2,alpha(col,age,101.0));
    }
    for i in 0..84 {
        let a=i as f64*2.399963+wobble;
        let r=(1.7+(i%11) as f64*0.36)*(1.0+0.14*(t*8.0+i as f64).sin())*grow;
        let z=(i%7) as f64*0.45-1.5;
        let p=add(o,[a.cos()*r,a.sin()*r*1.3,z]);
        let tip=add(p,[a.sin()*0.95,a.cos()*0.5,0.75]);
        scene.shard(p,tip,0.18+(i%4) as f64*0.08,a,alpha(if i%2==0{0xC7D9A9FF}else{0xAC8B52F3},age,101.0));
    }
}



// V0.3.0 / VFX gallery: 10 additional true geometric variants.
// The rotation is per spell key, so casting Z three times cycles
// Phoenix Garden -> Phoenix Wings -> Ember Cyclone; X/C/V/B behave likewise.
// No external models, no destructive world interactions and no paid services.

/// Phoenix alternate A: a pair of wide feather membranes growing outward like a living bird.
fn phoenix_wings(o:[f64;3], age:f64, grow:f64, scene:&mut Scene) {
    let unfurl = (age / 22.0).clamp(0.0,1.0) * grow;
    for wing in [-1.0,1.0] {
        let mut prev_inner = add(o,[0.0,1.2,0.0]);
        let mut prev_outer = add(o,[0.0,2.0,0.0]);
        for i in 1..=23 {
            let u=i as f64 / 23.0;
            let sweep = 6.3*u*unfurl;
            let lift = 1.2 + 3.9*(u * std::f64::consts::PI).sin().max(0.0);
            let inner=add(o,[wing*sweep, lift, 0.45*(u*9.0+age*0.03).sin()]);
            let outer=add(inner,[wing*(0.25+1.1*u),0.8+0.4*(u*8.0).sin(),-0.9-1.0*u]);
            let col=if i%4==0 {0xB7FFF0AD} else if i%2==0 {0xA5F86B2B} else {0x9ED5394E};
            scene.membrane(prev_inner,inner,outer,prev_outer,alpha(col,age,102.0));
            if i%2==0 {
                scene.shard(outer,add(outer,[wing*0.4,-1.0-u*1.7,-0.5]),0.12,u*8.0,
                    alpha(0xA9FFCE7A,age,102.0));
            }
            prev_inner=inner;
            prev_outer=outer;
        }
    }
    // Distinct falling tail plume, not a third wing or straight beam.
    for i in 0..27 {
        let u=i as f64/27.0;
        let a=age*0.035+i as f64*2.4;
        let p=add(o,[a.sin()*0.45*u,1.4-4.0*u*grow,a.cos()*0.8*u]);
        scene.shard(p,add(p,[0.25*a.sin(),-0.4,a.cos()*0.18]),0.10,a,
            alpha(0x95FBB36C,age,102.0));
    }
}

/// Phoenix alternate B: four braided rivers of sparks, opening and dissipating organically.
fn ember_cyclone(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    for plume in 0..4 {
        let twist=TAU*plume as f64/4.0;
        let mut previous=add(o,[0.0,0.1,0.0]);
        for i in 1..=27 {
            let u=i as f64/27.0;
            let theta=twist+u*TAU*2.2+age*0.09*(1.0+0.12*plume as f64);
            let radius=(0.25+3.1*u.powf(0.8))*grow;
            let p=add(o,[theta.cos()*radius,6.2*u*grow,theta.sin()*radius]);
            let col=if i%3==0 {0xAFFFDD7A} else {0xA7D94A28};
            scene.shard(previous,p,0.12+0.15*(1.0-u),theta,alpha(col,age,100.0));
            if i%4==0 {
                scene.leaf(p,[theta.sin(),0.45,theta.cos()],0.7+u,
                    alpha(0x9DFFBE78,age,100.0));
            }
            previous=p;
        }
    }
}

/// Glacier alternate A: four segmented pointed vaulted arches with hanging ice tiles.
fn glacier_arches(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    for arch in 0..4 {
        let angle=TAU*arch as f64/4.0;
        let dir=[angle.cos(),0.0,angle.sin()];
        let side=[-angle.sin(),0.0,angle.cos()];
        for face in [-1.0,1.0] {
            let mut prev=add(o,add(scale(dir,face*4.8*grow),[0.0,-0.7,0.0]));
            for i in 1..=14 {
                let u=i as f64/14.0;
                let inward = 4.8*(1.0-u);
                let height = 7.0 * (u*(2.0-u)).sqrt() * grow;
                let p=add(o,add(scale(dir,face*inward*grow),[0.0,height-0.7,0.0]));
                let skew=scale(side,0.65*(u*5.0+arch as f64).sin());
                scene.shard(prev,add(p,skew),0.38*(1.0-0.65*u)*grow,
                    angle+u*2.0,alpha(if arch%2==0 {0xA978CFE7}else{0xB1ACDAFC},age,104.0));
                if i%3==0 {
                    let drop=add(p,[0.0,-(0.6+0.7*u)*grow,0.0]);
                    scene.shard(p,drop,0.17,angle+u,alpha(0x9DB3F5FF,age,104.0));
                }
                prev=p;
            }
        }
    }
}

/// Glacier alternate B: a forward-breaking glacial wave made from staggered broad facets.
fn frost_surge(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let advance=(age/32.0).clamp(0.0,1.0);
    for column in 0..8 {
        for rank in 0..8 {
            let u=rank as f64/8.0;
            let v=column as f64/8.0;
            let l=add(o,[(-3.8+column as f64*1.05)*grow,-0.75,
                         (rank as f64*1.2-3.5)*advance*grow]);
            let height=(1.3+4.4*(1.0-v)*(0.5+u))*grow;
            let tip=add(l,[0.25*(rank as f64*2.1).sin(),height,0.5+0.8*u]);
            scene.shard(l,tip,0.3+0.17*(1.0-u),v*TAU+u,
                alpha(if (column+rank)%3==0 {0xB1D5F8FF}else{0xA46EAFDD},age,104.0));
        }
    }
}

/// Druid alternate A: living woody tendrils bend inward to form an articulated prison.
fn living_cage(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let curl=(age/33.0).clamp(0.0,1.0);
    for vine in 0..10 {
        let a=TAU*vine as f64/10.0;
        let mut prev=add(o,[a.cos()*3.6*grow,-0.6,a.sin()*3.6*grow]);
        for i in 1..=13 {
            let u=i as f64/13.0;
            let radius=3.6*(1.0-u*u*curl);
            let p=add(o,[a.cos()*radius*grow,
                (-0.6+u*6.6)*grow+0.18*(age*0.035+a).sin(),
                a.sin()*radius*grow]);
            scene.shard(prev,p,0.25*(1.0-u)+0.09,a+u,alpha(0xB17DB347,age,102.0));
            if i==4 || i==8 || i==12 {
                let leafdir=[a.cos()*0.7,0.35,a.sin()*0.7];
                scene.leaf(p,leafdir,1.4,alpha(0xA36CD389,age,102.0));
                scene.leaf(p,[-leafdir[2],0.65,leafdir[0]],1.2,
                    alpha(0xB63FAF73,age,102.0));
            }
            prev=p;
        }
    }
}

/// Druid alternate B: a broad unfolding canopy above curved roots, with hanging seedpods.
fn verdant_canopy(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let pulse=0.84+0.16*(age*0.065).sin();
    for stem in 0..7 {
        let a=TAU*stem as f64/7.0;
        let base=add(o,[0.5*a.cos(),-0.6,0.5*a.sin()]);
        let crown=add(o,[a.cos()*2.1*grow,4.1*grow,a.sin()*2.1*grow]);
        scene.shard(base,crown,0.28,a,alpha(0xB777A044,age,103.0));
        for leaf in 0..10 {
            let u=leaf as f64/10.0;
            let bend=a+(u-0.5)*1.6;
            let p=add(crown,[bend.cos()*(1.4+2.5*u)*grow,
                (0.3-1.5*u*u)*grow,
                bend.sin()*(1.4+2.5*u)*grow]);
            let wing=add(p,[-bend.sin()*1.1,0.15,bend.cos()*1.1]);
            scene.membrane(crown,p,add(wing,[0.0,0.4*pulse,0.0]),
                add(crown,[0.0,0.3,0.0]),
                alpha(if leaf%3==0 {0xA3B5E971}else{0x9F3EB978},age,103.0));
            if leaf%3==0 {
                scene.shard(p,add(p,[0.06,-0.9*grow,0.0]),0.12,a,
                    alpha(0xB7DAED83,age,103.0));
            }
        }
    }
}

/// Tempest alternate A: asymmetrical branched thunder-antlers, not a straight ray.
fn storm_antlers(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    for root in 0..6 {
        let a=TAU*root as f64/6.0;
        let dir=[a.cos(),0.0,a.sin()];
        let mut prev=add(o,[0.0,0.5,0.0]);
        for i in 1..=13 {
            let u=i as f64/13.0;
            let jitter=0.5*(u*23.0+a*9.0+age*0.22).sin();
            let p=add(o,[
                dir[0]*5.7*u*grow - dir[2]*jitter,
                (1.1+4.0*u*u)*grow + 0.35*(age*0.22+i as f64).cos(),
                dir[2]*5.7*u*grow + dir[0]*jitter
            ]);
            scene.shard(prev,p,0.14*(1.0-u)+0.05,a+u,
                alpha(if i%2==0 {0xBA8CE7F5}else{0xAFE1D3FB},age,99.0));
            if i>=5 && i%3==0 {
                let fork=add(p,[
                    dir[0]*1.2+dir[2]*0.9,1.0+0.8*u,
                    dir[2]*1.2-dir[0]*0.9
                ]);
                scene.shard(p,fork,0.09,a+u,alpha(0xAFA8D2FF,age,99.0));
            }
            prev=p;
        }
    }
}

/// Tempest alternate B: a living electric manta, wing membranes and a trailing tail.
fn storm_manta(o:[f64;3],forward:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let dir=norm(forward);
    let lateral=norm(cross(dir,[0.0,1.0,0.0]));
    let raise=0.6*(age*0.16).sin();
    for wing in [-1.0,1.0] {
        let mut prev=add(o,[0.0,1.0,0.0]);
        let mut previous_edge=add(o,[0.0,1.0,0.0]);
        for i in 1..=22 {
            let u=i as f64/22.0;
            let core=add(o,add(scale(lateral,wing*6.0*u*grow),
                     scale(dir,2.0*u*grow)));
            let inner=add(core,[0.0,(1.0+raise+3.0*(1.0-u)*u)*grow,0.0]);
            let edge=add(inner,add(scale(dir,-2.2*grow),[0.0,-0.5-1.0*u,0.0]));
            scene.membrane(prev,inner,edge,previous_edge,
                alpha(if i%4==0 {0xA88DDDF7}else{0x9B7D96D9},age,100.0));
            if i%3==0 {
                scene.shard(edge,add(edge,scale(dir,-0.5)),0.08,age*0.06+u,
                    alpha(0xA6D9E6FF,age,100.0));
            }
            prev=inner;previous_edge=edge;
        }
    }
    let tail0=add(o,scale(dir,-1.6*grow));
    scene.shard(tail0,add(tail0,scale(dir,-7.0*grow)),0.12,age*0.03,
        alpha(0xA0A0DDFF,age,100.0));
}

/// Astral alternate A: twelve folded membrane petals peel away into the otherworld.
fn void_blossom(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let open=(age/28.0).clamp(0.0,1.0)*grow;
    for petal in 0..12 {
        let a=TAU*petal as f64/12.0 + age*0.011;
        let a2=a+TAU/12.0;
        let root=add(o,[a.cos()*0.4,a.sin()*0.4,0.0]);
        let root2=add(o,[a2.cos()*0.4,a2.sin()*0.4,0.0]);
        let outer=add(o,[a.cos()*4.8*open,a.sin()*5.5*open,
                         2.2*(petal as f64*1.3+age*0.06).sin()]);
        let outer2=add(o,[a2.cos()*4.5*open,a2.sin()*5.1*open,
                          2.2*(petal as f64*1.3+age*0.06+1.0).sin()]);
        scene.membrane(root,outer,outer2,root2,alpha(
            if petal%2==0 {0x9E9958BA} else {0xA46F81D5},age,101.0));
        scene.shard(outer,add(outer,[0.3*a.cos(),0.8*a.sin(),-0.8]),0.13,a,
            alpha(0xBAC6B8FF,age,101.0));
    }
}

/// Astral alternate B: a crooked broken staircase of hovering dimensional monoliths.
fn fractured_stair(o:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    for step in 0..13 {
        let u=step as f64/13.0;
        let a=u*TAU*1.7+age*0.02;
        let r=(1.0+3.5*u)*grow;
        let p=add(o,[a.cos()*r,(-1.0+6.7*u)*grow,a.sin()*r]);
        let tilt=0.25*(age*0.09+step as f64).sin();
        let span=1.1+(step%4) as f64*0.24;
        let a0=add(p,[-span,0.0,-span*0.35]);
        let b0=add(p,[span,tilt,-span*0.35]);
        let c0=add(p,[span*0.7,0.5*tilt,span*0.45]);
        let d0=add(p,[-span*0.6,-tilt,span*0.45]);
        scene.membrane(a0,b0,c0,d0,
            alpha(if step%2==0 {0xB06E51A0} else {0xAE956FC8},age,101.0));
        for fragment in 0..4 {
            let h=fragment as f64/4.0;
            let tip=add(p,[h*(step%3) as f64,0.7+h,a.sin()*0.7]);
            scene.shard(p,tip,0.11,h+age*0.02,alpha(0xA0D9B8FF,age,101.0));
        }
    }
}
