//! Aetheria Horizon first native visual preview. Five 3D fantasy spell shapes.
//! These are LOCAL client effects only; no server damage, block edits or claimed FPS.
use mod_api::{
    self, Hooks, Registrar, RegistrationContext, KeyRegistration, KeyConflictContext,
    ClientHost, ClientTickPostEvent, ClientSetupEvent, ClientResourcesReloadedEvent,
    SubmitCustomGeometryEvent, GuiRenderPostEvent, RegisterRenderPipelinesEvent,
    RenderPipeline, RenderPipelineRef, PipelineBlend, PipelineQuad, WorldDepth,
    WorldQuad, Identifier,
};
use std::{f64::consts::{PI, TAU}, sync::Mutex};

const MOD_ID: &str = "aetheria_preview";
const IDS: [&str;5] = ["phoenix_garden", "glacier_cathedral", "worldroot", "thunder_wyrm", "astral_sunder"];
const KEYS: [&str;5] = ["key.keyboard.z", "key.keyboard.x", "key.keyboard.c", "key.keyboard.v", "key.keyboard.b"];
const NAMES: [&str;5] = ["Phoenix Garden", "Glacier Cathedral", "Living Worldroot", "Thunder Wyrm", "Astral Sunder"];

#[derive(Clone, Copy)]
struct Cast { kind: usize, start: u64, center: [f64;3], direction: [f64;3] }
#[derive(Default)]
struct State { tick: u64, effects: Vec<Cast>, pipeline: Option<RenderPipelineRef>, last_cast: Option<usize> }
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
            PipelineBlend::Additive, WorldDepth::Tested).optional());
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
                    state.effects.push(Cast {kind, start: tick, center, direction: dir});
                    state.last_cast = Some(kind);
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
        let mut scene = Scene::new(capacity.min(1_600) as usize);
        for cast in &effects {
            let age = (now.saturating_sub(cast.start)) as f64 + time;
            if age >= 105.0 || distance2(camera, cast.center) > 115.0 * 115.0 { continue; }
            render_spell(*cast, age, &mut scene);
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
            event.batch.text(NAMES[last], [8.0, 22.0], 0xFFE8C9FF, true);
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
    match cast.kind {
        0=>phoenix_garden(cast.center,age,grow,scene),
        1=>glacier_cathedral(cast.center,age,grow,scene),
        2=>worldroot(cast.center,age,grow,scene),
        3=>thunder_wyrm(cast.center,cast.direction,age,grow,scene),
        _=>astral_sunder(cast.center,age,t,grow,scene),
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
