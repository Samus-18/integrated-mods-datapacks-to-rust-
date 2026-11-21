//! Aetheria Horizon first native/wasm visual preview. Five 3D fantasy spell shapes.
//! These are LOCAL client effects only; no server damage, block edits or claimed FPS.
mod compact_ui;
mod fair_geometry;
mod catalog30;
mod cinematic_beams;
mod hero_vfx;
mod volumetric_forms;
mod sharp_projectile;
mod rime_bridge;
mod procedural_fx;
mod authored_100_data;
mod authored_100_fx;
mod signature_forms;
mod catalog200;
mod mass_forms;
mod ultimate_forms;
mod awakening_forms;
mod standard_forms;
mod standard_channelled;
mod standard_instant;
mod standard_combo;
mod standard_charged2;
mod standard_m26;
mod standard_m27a;
mod standard_m27b;
mod charged_standard_forms;
#[cfg(test)]
mod visual_audit;
#[cfg(test)]
mod explicit_coverage;
#[cfg(test)]
mod shader_validation;
use aetheria_horizon::ui::minimal_hud::QuickSelector;
use catalog200::SPELL_COUNT;
use aetheria_horizon::core::player_data::PlayerMagic;
use aetheria_horizon::cast as cast_spell;
use mod_api::{
    self, Hooks, Registrar, RegistrationContext, KeyRegistration, KeyConflictContext,
    ClientHost, ClientTickPostEvent, ClientSetupEvent, ClientResourcesReloadedEvent,
    SubmitCustomGeometryEvent, GuiRenderPostEvent, RegisterRenderPipelinesEvent,
    RenderPipeline, RenderPipelineRef, PipelineBlend, PipelineQuad, WorldDepth,
    MouseScrollingEvent, Flow,
    WorldQuad, Identifier, CrosshairTarget, SoundPlay, SoundSource,
};
use std::{f64::consts::TAU, sync::Mutex};
use aetheria_horizon::vfx::arcane_forms::{sculpt_school,Material as ArcaneMaterial};

const MOD_ID: &str = "aetheria_preview";

#[derive(Clone, Copy)]
struct Cast { kind: usize, variant: u8, start: u64, center: [f64;3], direction: [f64;3] }
struct State {
    tick:u64,effects:Vec<Cast>,pipeline:Option<RenderPipelineRef>,
    soft_particle_pipeline:Option<RenderPipelineRef>,
    last_cast:Option<usize>,last_variant:u8,
    quick:QuickSelector,picker_was_down:bool,picker_scrolled:bool,
    // Client-only training instance; NOT multiplayer player data or world save.
    demo_magic:Option<PlayerMagic>,
    local_mana:f32,local_cooldowns:[u64;SPELL_COUNT],
}
impl Default for State {
    fn default()->Self {
        Self{tick:0,effects:Vec::new(),pipeline:None,
            soft_particle_pipeline:None,
            last_cast:None,last_variant:0,
            quick:QuickSelector::default(),picker_was_down:false,picker_scrolled:false,
            demo_magic:None,local_mana:100.0,
            local_cooldowns:[0;SPELL_COUNT]}
    }
}
#[derive(Default)]
struct Mod { state: Mutex<State> }

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        // Two controls: tap R next school; hold R and scroll one of its 20
        // spells; Z casts. Old debug shortcuts are not registered.
        for (id,key,label) in [
            ("quick_picker","key.keyboard.r","Tap: next school | Hold + wheel: spells"),
            ("cast","key.keyboard.z","Cast selected magic spell"),
        ] {
            registrar.register_key_mapping(
                KeyRegistration::new(id,key)
                    .context(KeyConflictContext::InGame)
                    .label(label)
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
        // Separate optional pipeline: no alpha-tag collisions with legacy
        // solid surfaces, and never any plain-world raw particle rectangles.
        let mask=Identifier::parse("aetheria_preview:soft_particle").unwrap();
        event.pipelines.push(RenderPipeline::new(mask.clone(),mask,
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
        let playing = host.playing() && host.local_player().is_some();
        // Keys must not fire while inventory, pause or settings is open.
        // We still keep the world effects alive while those screens are shown.
        let active_gameplay = playing && host.observed_screen_id().is_none();
        let mut pressed = Vec::new();
        let cast_selected=active_gameplay && host.consume_click(MOD_ID,"cast");
        let picker_down=active_gameplay && host.is_down(MOD_ID,"quick_picker");
        let camera = if pressed.is_empty() && !cast_selected {None}else{host.camera()};
        // Client-side surface target: makes the ice and roots grow out of a
        // real selected block rather than floating at the player's eye level.
        let ground_target = if pressed.is_empty() && !cast_selected { None } else {
            match host.crosshair_target() {
                Some(CrosshairTarget::Block { hit, .. }) if hit.face[1] > 0
                    && hit.point.iter().all(|v| v.is_finite()) => Some(hit.point),
                _ => None,
            }
        };
        let mut state = self.state.lock().unwrap();
        let mut queued_sounds: Vec<(usize,u8,[f64;3])> = Vec::new();
        if !playing {
            state.effects.clear();
            state.picker_was_down=false;
            state.picker_scrolled=false;
            state.local_mana=100.0;
            state.local_cooldowns=[0;SPELL_COUNT];
            state.demo_magic=None;
            state.quick=QuickSelector::default();
            state.tick=0;
            return;
        }
        state.tick=state.tick.saturating_add(1);
        let now=state.tick;
        if state.demo_magic.is_none() {
            // All 200 authored spells are deliberately enabled for the visual
            // trial. Genuine discovery/unlocks remain server-owned future work.
            let mut demo=PlayerMagic::new(0);
            for slot in 0..SPELL_COUNT {
                demo.unlocked_nodes.insert(catalog200::id(slot));
            }
            state.demo_magic=Some(demo);
        }
        if let Some(demo)=state.demo_magic.as_mut() {
            demo.regenerate(1);
            state.local_mana=demo.mana;
        }
        // R short tap cycles schools, holding R+wheel browses within a
        // class. A scroll marks the gesture so release does NOT change school.
        if !picker_down && state.picker_was_down && !state.picker_scrolled {
            let school=(catalog200::school(state.quick.selected)+1)%10;
            let rank=catalog200::rank(state.quick.selected);
            state.quick.selected=school*20+rank;
            state.quick.visible_until_tick=now.saturating_add(36);
        }
        if picker_down && !state.picker_was_down {state.picker_scrolled=false;}
        if !picker_down {state.picker_scrolled=false;}
        state.picker_was_down=picker_down;
        if cast_selected {pressed.push(state.quick.selected.min(SPELL_COUNT-1));}
        state.effects.retain(|c| now.saturating_sub(c.start) <= 104);
        if let Some(cam) = camera {
            let yaw = f64::from(cam.yaw).to_radians();
            let pitch = f64::from(cam.pitch).to_radians();
            let dir = [-yaw.sin()*pitch.cos(), -pitch.sin(), yaw.cos()*pitch.cos()];
            if cam.position.iter().all(|v| v.is_finite()) && dir.iter().all(|v| v.is_finite()) {
                for kind in pressed {
                    // Preview mana is client-local: NOT the future multiplayer
                    // server authority, and never unlocks real world content.
                    let spec=catalog200::spec(kind);
                    // One source of truth for mana, cooldown and 200 distinct
                    // action intents: the shared, tested Rust casting core.
                    let result={
                        let Some(magic)=state.demo_magic.as_mut() else {continue;};
                        magic.selected_class=spec.class_id;
                        cast_spell(magic,spec.id,now,0.6)
                    };
                    let Ok(result)=result else {continue;};
                    if let Some(magic)=state.demo_magic.as_ref() {
                        state.local_mana=magic.mana;
                    }
                    state.local_cooldowns[kind]=result.next_allowed_tick;
                    state.quick.selected=kind;
                    let center = if matches!(catalog200::school(kind),1|3|5|9) {
                        ground_target.unwrap_or_else(|| {
                            let mut p = add(cam.position, scale(dir, 6.0));
                            p[1] -= 1.6;
                            p
                        })
                    } else {
                        add(cam.position, scale(dir, 7.0))
                    };
                    if state.effects.len() >= 8 { state.effects.remove(0); }
                    let tick = state.tick;
                    // 200 selectable registry identities; 30 bespoke VFX + 170 original-form drafts.
                    let variant = catalog200::rank(kind) as u8;
                    state.effects.push(Cast {kind, variant, start: tick, center, direction: dir});
                    state.last_cast = Some(kind);
                    state.last_variant = variant;
                    queued_sounds.push((kind, variant, center));
                }
            }
        }
        drop(state);
        // Original, locally synthesized spatial audio: charge -> cast -> impact.
        // Never emits vanilla beam sounds; only this client's player hears it.
        for (kind,variant,origin) in queued_sounds {
            // Charge / cast / impact sound timestamps must match each spell's
            // distinct four-stage VFX timing, never a fixed global 9/18 ticks.
            let offsets=if let Some(timeline)=
                aetheria_horizon::vfx::timeline::for_spell(catalog200::id(kind))
            {
                let scale=104.0/timeline.total_ticks() as f64;
                let charge=(f64::from(timeline.phases[0].duration_ticks)*scale)
                    .round().max(1.0) as u32;
                let impact=((f64::from(timeline.phases[0].duration_ticks)
                    +f64::from(timeline.phases[1].duration_ticks))*scale)
                    .round().max(f64::from(charge+1)) as u32;
                [0,charge,impact]
            } else {
                [0,9,18] // Defensive fallback for corrupt/absent registry.
            };
            for (i,offset) in offsets.into_iter().enumerate() {
                let play=SoundPlay::at(catalog200::SCHOOL_SFX[catalog200::school(kind)][i], SoundSource::Blocks, origin)
                    .with_volume(if i==2 {0.85} else {0.62})
                    .with_pitch(0.94+(variant%5) as f32*0.045);
                let _=host.play_local_sound(&play,offset);
            }
        }
    }

    fn mouse_scrolling(&self, host: &mut dyn ClientHost, event: &mut MouseScrollingEvent) -> Flow {
        // Real Iron-like alternative: hold R, turn the mouse wheel to select
        // a spell, cancel the corresponding vanilla hotbar scroll only then.
        if !host.playing() || host.observed_screen_id().is_some()
            || !host.is_down(MOD_ID, "quick_picker") {
            return Flow::Continue;
        }
        let steps=event.accumulated[1];
        if steps==0 {return Flow::Continue;}
        let mut state=self.state.lock().unwrap();
        let now=state.tick;
        for _ in 0..steps.unsigned_abs().min(8) {
            let school=catalog200::school(state.quick.selected);
            let rank=catalog200::rank(state.quick.selected);
            state.quick.selected=school*20+
                if steps>0 {(rank+19)%20}else{(rank+1)%20};
        }
        state.picker_scrolled=true;
        state.quick.visible_until_tick=now.saturating_add(36);
        Flow::Cancel
    }

    fn submit_custom_geometry(&self, _: &dyn ClientHost, event: &mut SubmitCustomGeometryEvent<'_>) {
        let (effects, now, pipeline, soft_particle_pipeline) = {
            let state = self.state.lock().unwrap();
            (state.effects.clone(), state.tick, state.pipeline,
             state.soft_particle_pipeline)
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
        if total_budget==0 {return;}
        // Give the 3D body priority, while holding a small separate budget
        // for genuinely soft GPU particles. No group of authored source mesh
        // faces will be chopped apart merely to make room for sparkles.
        let particle_reserve=if soft_particle_pipeline.is_some() && total_budget>120 {
            total_budget.min(144)
        }else{0};
        let body_budget=total_budget-particle_reserve;
        let source_budget=if live.len()==1 {900} else {500};
        let reduced_detail=live.len()>=4 || body_budget/live.len()<180;
        let mut drafts=Vec::with_capacity(live.len());
        for (cast,age) in live.iter().copied() {
            let mut spell=Scene::with_lod(source_budget,reduced_detail);
            render_spell(cast,age,&mut spell);
            drafts.push(spell);
        }
        let solid=fair_geometry::sample_fair(&drafts,body_budget);
        let mut drawn=false;
        if let Some(pipeline) = pipeline {
            let quads: Vec<PipelineQuad> = solid.iter().map(|(corners, color)|
                PipelineQuad::spanning(*corners, *color)).collect();
            drawn=event.draw.pipeline_quads(pipeline, &quads).is_ok();
        }
        if !drawn {
            // Existing body fallback always remains available if the
            // original material shader is unsupported by the Nightly build.
            let quads: Vec<WorldQuad> = solid.iter().map(|(corners,color)|
                WorldQuad::new(*corners, *color, WorldDepth::Tested)).collect();
            let _ = event.draw.quads(&quads);
        }
        if let Some(mask_pipeline)=soft_particle_pipeline {
            if particle_reserve==0{return;}
            let room=(event.draw.usage().quad_capacity
                .saturating_sub(event.draw.usage().quads) as usize)
                .min(particle_reserve);
            if room==0{return;}
            let mut accents=Vec::with_capacity(live.len());
            for (cast,age) in live {
                let mut fx=Scene::with_lod(80,reduced_detail);
                // First ten named spells in every school have a distinctly
                // authored choreography; remaining slots use M35 background.
                if authored_100_fx::render(cast,age,camera,&mut fx).is_none(){
                    procedural_fx::render(cast,age,camera,&mut fx);
                }
                accents.push(fx);
            }
            let accents=fair_geometry::sample_fair(&accents,room);
            if !accents.is_empty() {
                let quads: Vec<PipelineQuad> = accents.iter().map(|(corners,color)|
                    PipelineQuad::spanning(*corners,*color)).collect();
                // Never fallback to plain quads here. Without the dedicated
                // alpha-mask shader they would appear as flat opaque cards.
                let _=event.draw.pipeline_quads(mask_pipeline,&quads);
            }
        }
    }

    fn gui_render_post(&self, host: &dyn ClientHost, event: &mut GuiRenderPostEvent<'_>) {
        use aetheria_horizon::ui::minimal_hud::should_render_gameplay_hud;
        if !should_render_gameplay_hud(
            host.playing(), host.local_player().is_some(),
            host.observed_screen_id().is_some(), event.gui_hidden, event.focused
        ) {return;}
        let state=self.state.lock().unwrap();
        let chosen=state.quick.selected.min(SPELL_COUNT-1);
        let open=host.is_down(MOD_ID,"quick_picker")
            ||state.quick.visible(state.tick,false);
        compact_ui::draw(host,event,compact_ui::UiView{
            tick:state.tick,
            chosen,
            quick_open:open,
            mana:state.local_mana,
            cooldowns:state.local_cooldowns,
        });
    }
}
impl Mod {
    fn resolve_pipeline(&self, host: &dyn ClientHost) {
        let id = Identifier::parse("aetheria_preview:ethereal").unwrap();
        let shader=host.render_assets().render_pipeline(&id).ok();
        let soft=Identifier::parse("aetheria_preview:soft_particle").unwrap();
        let soft_shader=host.render_assets().render_pipeline(&soft).ok();
        let mut state=self.state.lock().unwrap();
        state.pipeline=shader;
        state.soft_particle_pipeline=soft_shader;
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

/// Recolors faces without changing alpha: avoids white-out in large formations.
fn shade_argb(argb:u32, brightness:f64)->u32 {
    let mul=|shift:u32|->u32 {
        (((argb>>shift)&255) as f64 * brightness.clamp(0.0,1.0)).round() as u32
    };
    (argb&0xFF00_0000) | (mul(16)<<16) | (mul(8)<<8) | mul(0)
}
struct Scene {
    facets:Vec<([[f64;3];4],u32)>,
    // M29: adjacent faces belonging to one solid body must never be sampled
    // separately. Each face belongs to a unique sequential atomic group.
    group_ids:Vec<usize>,active_group:Option<usize>,next_group:usize,
    budget:usize,reduced_detail:bool,
}
impl Scene {
    fn new(budget:usize)->Self {Self::with_lod(budget,false)}
    fn with_lod(budget:usize,reduced_detail:bool)->Self {
        Self{facets:Vec::new(),group_ids:Vec::new(),active_group:None,
            next_group:0,budget,reduced_detail}
    }
    /// Begin a contiguous, ALL-OR-NOTHING geometry component. Nested body
    /// helpers inherit the current component ID instead of fragmenting it.
    fn begin_component(&mut self,faces:usize)->Option<Option<usize>> {
        if self.facets.len().saturating_add(faces)>self.budget{return None;}
        let prev=self.active_group;
        if prev.is_none() {
            self.active_group=Some(self.next_group);
            self.next_group+=1;
        }
        Some(prev)
    }
    fn end_component(&mut self,previous:Option<usize>){
        self.active_group=previous;
    }
    fn quad(&mut self,points:[[f64;3];4],color:u32) {
        if self.facets.len() < self.budget && points.iter().flatten().all(|v|v.is_finite()) {
            let id=match self.active_group{
                Some(id)=>id,
                None=>{
                    let id=self.next_group;
                    self.next_group+=1;
                    id
                }
            };
            self.facets.push((points,color));
            self.group_ids.push(id);
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
        let Some(previous)=self.begin_component(2) else{return;};
        self.quad([from,add(middle,side),to,sub(middle,side)],color);
        self.quad([from,add(middle,quarter),to,sub(middle,quarter)],color);
        self.end_component(previous);
    }
    fn leaf(&mut self,at:[f64;3],dir:[f64;3],size:f64,color:u32){
        let end=add(at,scale(dir,size));
        let axis=norm(sub(end,at));
        let guide=if axis[1].abs()<0.9 {[0.0,1.0,0.0]}else{[1.0,0.0,0.0]};
        let tangent=norm(cross(axis,guide));
        let middle=add(at,scale(dir,size*0.53));
        let width=size*0.22;
        self.membrane(at,add(middle,scale(tangent,width)),end,
                      sub(middle,scale(tangent,width)),color);
    }
    // Six-sided thin solid sheet (front, back, four edge walls), used for
    // phoenix feathers, leaf canopies, storm wings and folded astral tears.
    // One whole membrane fits or none; never leave half the thickness.
    fn membrane(&mut self, a:[f64;3],b:[f64;3],c0:[f64;3],d:[f64;3],color:u32){
        if self.facets.len().saturating_add(6)>self.budget {return;}
        let normal=norm(cross(sub(b,a),sub(d,a)));
        if length(normal)<0.1 {return;}
        let offset=scale(normal,0.07);
        let Some(previous)=self.begin_component(6) else{return;};
        let top=[add(a,offset),add(b,offset),add(c0,offset),add(d,offset)];
        let bottom=[sub(a,offset),sub(b,offset),sub(c0,offset),sub(d,offset)];
        self.quad(top,color);
        self.quad([bottom[3],bottom[2],bottom[1],bottom[0]],shade_argb(color,0.67));
        for i in 0..4 {
            let j=(i+1)%4;
            self.quad([top[i],top[j],bottom[j],bottom[i]],shade_argb(color,0.75));
        }
        self.end_component(previous);
    }

    // Cylindrical world-space body, not billboards: used for roots, living
    // creepers and coiling serpent segments. Vary radius along growth.
    fn tube(&mut self, from:[f64;3],to:[f64;3],r0:f64,r1:f64,
            twist:f64,color:u32,sides:usize) {
        if !(5..=8).contains(&sides) || !r0.is_finite() || !r1.is_finite()
            || r0<=0.004 || r1<=0.004 || length(sub(to,from))<0.03
            || self.facets.len().saturating_add(sides)>self.budget {return;}
        let axis=norm(sub(to,from));
        let base=if axis[1].abs()<0.91 {[0.0,1.0,0.0]}else{[1.0,0.0,0.0]};
        let tangent=norm(cross(axis,base));
        let normal=norm(cross(axis,tangent));
        let Some(previous)=self.begin_component(sides) else{return;};
        for i in 0..sides {
            let a=TAU*i as f64/sides as f64+twist;
            let b=TAU*(i+1) as f64/sides as f64+twist;
            let radial=|ang:f64,r:f64|add(scale(tangent,ang.cos()*r),scale(normal,ang.sin()*r));
            let p0=add(from,radial(a,r0));
            let p1=add(from,radial(b,r0));
            let p2=add(to,radial(b,r1));
            let p3=add(to,radial(a,r1));
            self.quad([p0,p1,p2,p3],
                      shade_argb(color,match i%4{0=>0.86,1=>0.71,2=>1.0,_=>0.78}));
        }
        self.end_component(previous);
    }

    /// Closed-sided 3D crystal grown along its axis. Every side has two
    /// differently shaded *surfaces* (not two crossed transparent billboards).
    /// The base is buried beneath terrain; no underside is rendered.
    fn solid_crystal(
        &mut self, from:[f64;3], to:[f64;3], radius:f64,
        twist:f64, color:u32, sides:usize
    ) {
        if !(5..=8).contains(&sides) || !radius.is_finite() || radius <= 0.01
            || !from.iter().chain(to.iter()).all(|v| v.is_finite())
            || length(sub(to,from)) < 0.1 { return; }
        let faces = sides * 2;
        let Some(previous)=self.begin_component(faces) else{return;};
        let axis=norm(sub(to,from));
        let reference=if axis[1].abs() < 0.9 {[0.0,1.0,0.0]} else {[1.0,0.0,0.0]};
        let tangent=norm(cross(axis,reference));
        let normal=norm(cross(tangent,axis));
        let mid=add(from,scale(sub(to,from),0.59));
        for face in 0..sides {
            let a=TAU*face as f64/sides as f64+twist;
            let b=TAU*(face+1) as f64/sides as f64+twist;
            let ring=|center:[f64;3],r:f64,t:f64| -> [f64;3] {
                add(center,add(scale(tangent,t.cos()*r),scale(normal,t.sin()*r)))
            };
            let a0=ring(from,radius,a);
            let b0=ring(from,radius,b);
            let a1=ring(mid,radius*0.68,a+0.1);
            let b1=ring(mid,radius*0.68,b+0.1);
            let a2=ring(to,0.015,a+0.15);
            let b2=ring(to,0.015,b+0.15);
            let shade=match face%5 {0=>0.66,1=>0.82,2=>1.0,3=>0.76,_=>0.9};
            let shaded=shade_argb(color,shade);
            self.quad([a0,b0,b1,a1],shaded);
            self.quad([a1,b1,b2,a2],shade_argb(color,(shade*1.12).min(1.0)));
        }
        self.end_component(previous);
    }
}


/// Convert a bounded world-space 3D mesh into actual Horizon's supported
/// quad submissions. This is NOT GPU instancing: dense late-stage ultimates
/// will need a verified, separate instanced renderer integration.
fn render_portable_sculpt(cast:Cast,age:f64,scene:&mut Scene)->bool {
    let school=catalog30::school(cast.kind) as u8;
    let Some(form)=sculpt_school(school,(age.max(0.).min(119.)) as u32,1) else {
        return false;
    };
    // Reserve room for a spell-specific 3D silhouette, not just school meshes.
    let available=scene.budget.saturating_sub(scene.facets.len())*55/100;
    if available==0 {return true;}
    let triangles=form.triangle_count();
    if triangles==0{return false;}
    // Keep silhouettes from every part visible, even if a large ritual
    // has more triangles than the selected engine's per-frame quad limit.
    let stride=(triangles+available-1)/available;
    let mut index=0usize;
    let sun=norm([0.45,0.85,0.30]);
    for part in form.parts {
        let rgb=match part.material {
            ArcaneMaterial::Ember=>0x00FA7335,
            ArcaneMaterial::Ice=>0x0075BEDB,
            ArcaneMaterial::Wind=>0x00AEDADD,
            ArcaneMaterial::Stone=>0x008A7D82,
            ArcaneMaterial::Water=>0x00469ECF,
            ArcaneMaterial::Wood=>0x00766E45,
            ArcaneMaterial::Leaf=>0x0066BF79,
            ArcaneMaterial::Lightning=>0x0091C9F5,
            ArcaneMaterial::Rift=>0x006D5AAF,
            ArcaneMaterial::Starlight=>0x00B6A2E7,
            ArcaneMaterial::Brass=>0x00C2A260,
            ArcaneMaterial::Bone=>0x00D5CBAA,
            ArcaneMaterial::Soul=>0x00B9C3EB,
        };
        let alpha=((part.opacity * 195.0).clamp(0.0,195.0)).round() as u32;
        let base=(alpha<<24)|rgb;
        for tri in part.mesh.indices.chunks_exact(3) {
            let selected=index%stride==0;
            index+=1;
            if !selected {continue;}
            if scene.facets.len()>=scene.budget {return true;}
            let p=|v:u32|->Option<[f64;3]>{
                let x=part.mesh.vertices.get(v as usize)?.position;
                let tier=catalog30::rank(cast.kind);
                let (horizontal,vertical,depth)=match tier {
                    0=>(0.76,0.88,0.82),
                    1=>(1.24,0.83,1.17),
                    _=>(0.91,1.22,0.84),
                };
                Some(add(cast.center,[
                    x[0] as f64*horizontal,
                    x[1] as f64*vertical,
                    x[2] as f64*depth
                ]))
            };
            let (Some(a),Some(b),Some(c))=(p(tri[0]),p(tri[1]),p(tri[2])) else{
                continue;
            };
            let normal=cross(sub(b,a),sub(c,a));
            let nlen=length(normal);
            if nlen<=0.000001 {continue;}
            let diffuse=(normal[0]*sun[0]+normal[1]*sun[1]+normal[2]*sun[2]).abs()/nlen;
            let shaded=shade_argb(base,0.47+0.48*diffuse);
            // The duplicate last point produces a single triangular facet.
            scene.quad([a,b,c,c],shaded);
        }
    }
    true
}

/// Three distinct 3D attack silhouettes per school: rooted lance, unfolding
/// fan and braided lash. The ten independently sculpted school models live
/// underneath this signature; all coordinates are world-space.
fn render_spell_signature(cast:Cast,age:f64,scene:&mut Scene) {
    let school=catalog30::school(cast.kind);
    let rank=catalog30::rank(cast.kind);
    let tint=alpha(catalog30::WORLD_COLORS[school],age,104.0);
    let grow=(age/13.0).clamp(0.,1.).powf(0.68);
    let o=cast.center;
    let forward=norm([cast.direction[0],0.03,cast.direction[2]]);
    let right=norm(cross(forward,[0.,1.,0.]));
    let phase=age*0.035+school as f64*0.91;
    match rank {
        0 => { // Concentrated piercing crown of faceted solid lances
            for j in 0..7 {
                let a=TAU*j as f64/7.0+phase;
                let near=add(o,[a.cos()*0.56*grow,a.sin()*0.46*grow,a.sin()*0.36*grow]);
                let tip=add(o,add(scale(forward,(3.4+j as f64*0.19)*grow),
                    [a.cos()*1.35*grow,(0.75+a.sin()*0.85)*grow,a.sin()*0.95*grow]));
                scene.solid_crystal(near,tip,0.24+(j%3) as f64*0.05,a,tint,6);
            }
        }
        1 => { // Divergent three-dimensional volley, not a flat fan
            for wing in [-1.0,1.0] {
                for j in 0..5 {
                    let t=(j as f64+1.0)/5.0;
                    let spin=phase+wing*t*1.6;
                    let root=add(o,add(scale(right,wing*t*0.6),[0.,0.25,0.]));
                    let end=add(root,add(scale(forward,(3.0+3.1*t)*grow),
                        add(scale(right,wing*t*3.2*grow),
                        [0.,(0.7+(t*9.0+phase).sin()*1.45)*grow,0.])));
                    scene.solid_crystal(root,end,(0.17+0.16*(1.-t))*grow,spin,tint,5);
                }
            }
        }
        _ => { // Braided rising tendrils and a crown of hard crystal thorns
            for strand in 0..3 {
                let shift=TAU*strand as f64/3.0+phase;
                let mut prev=o;
                for j in 1..=12 {
                    let t=j as f64/12.0;
                    let a=shift+t*TAU*1.65;
                    let r=(0.42+2.5*t)*grow;
                    let point=add(o,[a.cos()*r,(t*5.6-0.2)*grow,a.sin()*r]);
                    scene.tube(prev,point,0.12*(1.-t)+0.035,0.09*(1.-t)+0.025,
                        a,tint,5);
                    if j%4==0 {
                        scene.solid_crystal(point,
                            add(point,[a.cos()*0.8,0.7*grow,a.sin()*0.8]),
                            0.15,a,tint,5);
                    }
                    prev=point;
                }
            }
        }
    }
}

// M32: all 200 local preview slots use one new visual rendering architecture.
// Old beam/rod/ring generators remain in source ONLY for reference and their
// separate legacy tests; no old path runs during the live user spell cast.
fn render_spell(cast:Cast, age:f64, scene:&mut Scene) {
    // M33: use the ACTUAL 56-quad authored Blender Frost Needle model for
    // spell ID 021; all other 199 draft visuals remain unchanged until their
    // own genuine model/material quality gate passes.
    if sharp_projectile::render(cast,age,scene) { return; }
    // M34: real independent sculpted Rime Bridge, registry #022.
    if rime_bridge::render(cast,age,scene) { return; }
    // M37: actual name-true connected 3D surfaces, replacing rejected
    // generic solid forms for eleven high-priority signature spells.
    if signature_forms::render(cast,age,scene) { return; }
    volumetric_forms::render(cast,age,scene);
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
fn glacier_cathedral(o:[f64;3],forward:[f64;3],age:f64,grow:f64,scene:&mut Scene){
    // Broad, stepped "Ice Awakening" ground eruption, similar in silhouette
    // to the user's 3D screenshot: real five/six-sided volume, rising in waves.
    let horizontal=norm([forward[0],0.0,forward[2]]);
    let side=[-horizontal[2],0.0,horizontal[0]];
    for row in 0..6 {
        let row_age=age-row as f64*3.6;
        if row_age <= 0.0 { continue; }
        let rise=(row_age/12.0).clamp(0.0,1.0).powf(0.72)*grow;
        for j in 0..5 {
            let across=j as f64-2.0;
            let seed=row as f64*2.31 + j as f64*0.91;
            let stagger=0.25*(seed*2.7).sin();
            let origin=add(o,add(
                scale(horizontal,(row as f64*1.75+stagger)-2.8),
                scale(side,across*1.12+0.2*seed.sin())
            ));
            let base=add(origin,[0.0,-0.35,0.0]);
            let height=(2.0+1.05*row as f64+1.15*(seed*1.2).sin().abs())
                       *rise;
            let lean=add(scale(horizontal,0.3+0.8*seed.cos()),
                         scale(side,0.5*(seed*2.0).sin()));
            let tip=add(base,add([0.0,height,0.0],scale(lean,rise)));
            let radius=(0.48+0.28*(seed*0.7).sin().abs())*rise.max(0.12);
            let ice=if (row+j)%4==0 {0xD36BA6DB}
                    else if j%3==0 {0xC24E93CE}
                    else {0xC46FAFD8};
            scene.solid_crystal(base,tip,radius,seed,alpha(ice,age,103.0),6);
            // Smaller secondary splinters split from the base and remain solid.
            if (row+j)%3==0 {
                let flank=add(origin,scale(side,0.4+0.3*seed.cos()));
                scene.solid_crystal(add(flank,[0.0,-0.24,0.0]),
                    add(flank,[0.38*seed.cos(),height*0.53,0.22*seed.sin()]),
                    radius*0.42,seed+1.2,alpha(0xB56BA9DB,age,103.0),5);
            }
        }
    }
}

/// 3: Branching organic roots, unfolding leaf clusters and bioluminescent blossoms.
fn worldroot(o:[f64;3],age:f64,grow:f64,scene:&mut Scene){
    for branch in 0..9 {
        let a=TAU*branch as f64/9.0;
        let mut p=add(o,[a.cos()*0.18, -0.8, a.sin()*0.18]);
        let sweep=3.5+0.6*(branch as f64*1.9).sin();
        for step in 0..9 {
            let v=step as f64/9.0;
            let theta=a+v*0.8*(branch as f64*2.1).sin();
            let next=add(o,[theta.cos()*sweep*v*grow,
                -0.8+v*(5.0+(branch%4) as f64*0.55)*grow,
                theta.sin()*sweep*v*grow]);
            scene.tube(p,next,(0.25*(1.0-v)+0.05)*grow,
               (0.25*(1.0-v*1.05)+0.05)*grow,a+v,alpha(0xD66A9D45,age,103.0),6);
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
        scene.tube(last,path,thickness,thickness*0.94,i as f64*0.19,
                alpha(if i%3==0 {0xD7FCF3FF}else{0xAE7D69F7},age,99.0),6);
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
fn glacier_arches(o:[f64;3],forward:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let dir=norm([forward[0],0.0,forward[2]]);
    let side=[-dir[2],0.0,dir[0]];
    for arch in 0..5 {
        let open=((age-arch as f64*4.0)/15.0).clamp(0.0,1.0)*grow;
        if open<=0.0 {continue;}
        for wing in [-1.0,1.0] {
            let base=add(o,add(scale(dir,arch as f64*1.7-2.6),
                       scale(side,wing*(2.2+arch as f64*0.35))));
            let top=add(base,add([0.0,(4.6+arch as f64*0.7)*open,0.0],
                    scale(side,-wing*(2.0+arch as f64*0.22)*open)));
            scene.solid_crystal(add(base,[0.0,-0.32,0.0]),top,
                0.64*open,arch as f64+wing,alpha(0xC464A3D7,age,103.0),7);
            let small=add(base,scale(dir,0.74));
            scene.solid_crystal(add(small,[0.0,-0.3,0.0]),
                add(small,add([0.0,(2.4+arch as f64*0.3)*open,0.0],
                    scale(side,-wing*0.44))),0.32*open,arch as f64,
                alpha(0xBD7EC7E8,age,103.0),5);
        }
    }
}

/// Glacier alternate B: a forward-breaking glacial wave made from staggered broad facets.
fn frost_surge(o:[f64;3],forward:[f64;3],age:f64,grow:f64,scene:&mut Scene) {
    let dir=norm([forward[0],0.0,forward[2]]);
    let side=[-dir[2],0.0,dir[0]];
    // Fan-shaped glacial wave; each ridge grows only when its pulse arrives.
    for row in 0..9 {
        let row_age=age-row as f64*3.1;
        if row_age<=0.0 {continue;}
        let rise=(row_age/10.0).clamp(0.0,1.0)*grow;
        for col in 0..6 {
            let u=col as f64-2.5;
            let seed=row as f64*1.71+col as f64*2.33;
            let base=add(o,add(
                scale(dir,row as f64*1.45-2.0),
                scale(side,u*(0.65+row as f64*0.12))
            ));
            let height=(1.2+row as f64*0.55+1.5*(seed*0.4).sin().abs())*rise;
            scene.solid_crystal(add(base,[0.0,-0.33,0.0]),
                add(base,[0.35*seed.sin(),height,0.4*seed.cos()]),
                (0.35+0.22*(seed*0.87).sin().abs())*rise.max(0.1),
                seed,alpha(if col%3==0 {0xC4488FC8}else{0xBF6BAAD1},age,103.0),5);
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
            scene.tube(prev,p,0.25*(1.0-u)+0.09,0.22*(1.0-u)+0.08,
                a+u,alpha(0xB17DB347,age,102.0),5);
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
