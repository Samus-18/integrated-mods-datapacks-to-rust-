//! M20: semantic *first-draft* 3D forms for the 170 extended Aetheria
//! spells. Source of shape meaning is EVERY spell's original registry.mesh
//! authored phrase, not a color-swapped school default. No ring/rod-only FX.
//! NOTE: These are visual placeholders awaiting dedicated art-direction;
//! they are NOT 170 verified cinematic quality VFX or live world mechanics.
use super::{Cast,Scene,add,sub,scale,shade_argb};
use crate::catalog200;
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
type P=[f64;3];
fn pos(o:P,x:f64,y:f64,z:f64)->P{add(o,[x,y,z])}
fn between(a:P,b:P,t:f64)->P{add(a,scale(sub(b,a),t))}
fn color(base:u32,a:f64,brightness:f64)->u32 {
 let v=shade_argb(base,brightness);
 (v&0x00FFFFFF)|(((a*255.0).clamp(0.,255.).round() as u32)<<24)
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Form{Crystal,Wave,Canopy,Beast,Armor,Architecture,Blade,
             SkyRift,Avatar,Time,Cloud,Flora,Boulder,Phoenix,Artifact,
             Glyph,Ribbon,Relic}
/// Primary shape is selected from an authored mesh description, NOT school
/// palette or a random hash. The ordering resolves overlapping descriptors.
pub fn semantic_form(words:&str)->Form{
 let w=words.to_ascii_lowercase();
 let has=|tokens:&[&str]|tokens.iter().any(|t|w.contains(t));
 if has(&["dragon","serpent","wyrm","leviathan","manta","beast","skeleton guardian","knight","goat"]) {return Form::Beast}
 if has(&["feather","wing","phoenix","moth","bird"]){return Form::Phoenix}
 if has(&["cord","thread","strand","tether","braid","chain","ribbon","whip","trail","twist"]){return Form::Ribbon}
 if has(&["rune","sigil","glyph","tablet","gravestone","inscription","etch","mark"]){return Form::Glyph}
 // Specific phrases in the actual 200-entry authored mesh catalog.
 // Never silently classify an unknown source silhouette as the same canopy!
 if has(&["ash cloak","rolling avalanche","refractive veil","wind torus",
         "air plate helix","sonic cone","vibration helix"]){return Form::Cloud}
 if has(&["engraved fire seal","constellation chart"]){return Form::Glyph}
 if has(&["ember hooks","conductor wand"]){return Form::Blade}
 if has(&["volcanic fan"]){return Form::Phoenix}
 if has(&["caldera","hail asteroids","fault ridge","cliff mantle"]){return Form::Boulder}
 if has(&["floating air plate","wind arcs","foaming breakers","liquid toroids"]){return Form::Wave}
 if has(&["black monolith","earth column","carved monolith","throne"]){return Form::Architecture}
 if has(&["gem-plated mantle","bark plating","gravity halos"]){return Form::Armor}
 if has(&["fluid palm","offset silhouette"]){return Form::Avatar}
 if has(&["insect veil"]){return Form::Phoenix}
 if has(&["arc torus"]){return Form::Cloud}
 if has(&["ground filaments","footstep echoes"]){return Form::Ribbon}
 if has(&["silver phase sheets"]){return Form::SkyRift}
 if has(&["sandglass"]){return Form::Time}
 if has(&["clock","time","hour","dial","pendulum","chron","sands","stopped"]){return Form::Time}
 if has(&["rift","portal","fracture","fractured sky","space","void","shadow tear","mirror"]) {return Form::SkyRift}
 if has(&["skull","ghost","spirit","humanoid","giant","hand","fist","gauntlet","claw"]){return Form::Avatar}
 if has(&["bell","lantern","totem","idol","reliquary","altar","vessel","urn"]){return Form::Relic}
 if has(&["flower","bloom","root","tree","vine","canopy","petal","lily","garden","leaf","branch"]){return Form::Flora}
 if has(&["water","wave","tide","river","current","whirl","surge","crest","flow","splash","fountain"]){return Form::Wave}
 if has(&["cloud","smoke","mist","fog","hurricane","storm","vortex","tornado","funnel","cyclone","pressure"]){return Form::Cloud}
 if has(&["bridge","cathedral","temple","gate","arch","stair","pillar","wall","tower","cage","barricade","fortress","palisade","mausoleum","pavement"]){return Form::Architecture}
 if has(&["armor","shield","aegis","shell","ward","dome","vault","sanctuary","lens","barrier"]){return Form::Armor}
 if has(&["blade","sword","dagger","scythe","lance","javelin","knife","spear","needle","dart","crescent"]){return Form::Blade}
 if has(&["rock","boulder","stone","basalt","gravel","ore","mountain","anvil","hammer","geode","island"]){return Form::Boulder}
 if has(&["orb","sun","moon","sphere","lantern","star","planet","eye","crucible","ball"]){return Form::Artifact}
 if has(&["spike","crystal","ice","shard","icicle","frost","prism","diamond"]){return Form::Crystal}
 Form::Canopy
}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
 if !age.is_finite() || !(0.0..104.0).contains(&age){return None;}
 let t=for_spell(id)?;
 let total=t.total_ticks() as f64;
 let current=age/104.0*total;
 let mut previous=0.0;
 for part in t.phases {
   let end=previous+part.duration_ticks as f64;
   if current<end{return Some((part.kind,((current-previous)/(end-previous)).clamp(0.,1.)))}
   previous=end;
 }
 None
}
/// Closed faceted boulder in arbitrary axes, not a double-crossed sprite.
fn solid(scene:&mut Scene,o:P,size:P,spin:f64,c:u32) {
 let Some(previous)=scene.begin_component(6) else{return;};
 let cs=spin.cos();let sn=spin.sin();
 let corner=|i:usize|->P{
   let k=[[-1.,-1.,-1.],[1.,-1.,-1.],[1.,1.,-1.],[-1.,1.,-1.],
     [-1.,-1.,1.],[1.,-1.,1.],[1.,1.,1.],[-1.,1.,1.]][i];
   pos(o,(k[0]*size[0])*cs-(k[2]*size[2])*sn,
       k[1]*size[1],(k[0]*size[0])*sn+(k[2]*size[2])*cs)
 };
 for (n,idx) in [[0,1,2,3],[4,7,6,5],[0,4,5,1],
                 [3,2,6,7],[1,5,6,2],[0,3,7,4]].into_iter().enumerate(){
    scene.quad(idx.map(corner),shade_argb(c,0.62+0.07*(n%4) as f64));
 }
 scene.end_component(previous);
}
/// A broad faceted curved leaf with FRONT, BACK and edge walls.
fn petal(scene:&mut Scene,o:P,a:f64,reach:f64,height:f64,width:f64,c:u32){
 let base=pos(o,a.cos()*0.3,0.0,a.sin()*0.3);
 let mid=pos(o,a.cos()*reach*0.54,height*0.75,a.sin()*reach*0.54);
 let tip=pos(o,a.cos()*reach,height,a.sin()*reach);
 let lateral=[-a.sin(),0.,a.cos()];
 let l=add(mid,scale(lateral,width));
 let r=add(mid,scale(lateral,-width));
 scene.membrane(base,l,tip,r,c);
}
fn central_core(scene:&mut Scene,o:P,id:u16,scale_factor:f64,c:u32){
 let seed=id as f64;
 let tilt=seed*0.217;
 scene.solid_crystal(pos(o,0.,-0.40,0.),
   pos(o,(tilt*0.41).sin()*scale_factor,
         (2.15+((id%7) as f64*0.19))*scale_factor,
         (tilt*0.29).cos()*0.42*scale_factor),
  0.48*scale_factor.max(0.2),tilt,c,6);
}
fn signature(scene:&mut Scene,form:Form,o:P,id:u16,age:f64,g:f64,c:u32){
 let local=((id-1)%20) as usize;
 let layer=((id-1)/20) as usize;
 let f=id as f64;
 let spread=0.72+((id*11)%13) as f64*0.062;
 let extent=(2.4+((id*7)%11) as f64*0.29)*g;
 let height=(2.5+((id*13)%9) as f64*0.33)*g;
 let petals=4+(id as usize%5);
 match form {
 Form::Glyph=>{
   // Carved stone/metal RUNIC PLINTH: large solid body, engraved inset plates
   // with readable negative-space relief rather than hollow glyph outlines.
   solid(scene,pos(o,0.,height*0.45,0.),[extent*0.64,height*0.53,0.40*g],
         f*0.013,color(c,0.92,0.77));
   for i in 0..6{
     let a=TAU*i as f64/6.0;
     let r=extent*0.32;
     solid(scene,pos(o,a.cos()*r,height*(0.51+a.sin()*0.22),
                       0.42*g),
           [0.10*g,height*0.13,0.08*g],a+f*0.013,color(c,0.80,0.98));
   }
 }
 Form::Ribbon=>{
   // Three BROAD woven magical bands with real membrane thickness and
   // changing curvature; no laser rods or a single school-wide ring.
   for band in 0..3 {
     let twist=TAU*band as f64/3.+f*0.018;
     let mut prev=pos(o,0.,0.2,0.);
     for i in 0..10{
       let t=(i+1) as f64/10.;
       let angle=twist+t*TAU*1.1+age*0.025;
       let r=extent*(0.16+t*0.65);
       let nxt=pos(o,r*angle.cos(),height*t,r*angle.sin());
       let side=[angle.sin(),0.,-angle.cos()];
       let left=add(between(prev,nxt,0.66),scale(side,0.54*g));
       let right=add(between(prev,nxt,0.66),scale(side,-0.54*g));
       scene.membrane(prev,left,nxt,right,color(c,0.52,0.74));
       prev=nxt;
     }
   }
 }
 Form::Relic=>{
   // Suspended artifact with a closed solid bell/cup and heavy floating
   // relief panels, not an orbital wire frame.
   solid(scene,pos(o,0.,height*0.53,0.),[0.64*g,height*0.41,0.69*g],
         f*0.017,color(c,0.82,0.72));
   solid(scene,pos(o,0.,height*0.90,0.),[0.29*g,0.29*g,0.32*g],
         f*0.017,color(c,0.96,0.92));
   for i in 0..petals{
     let a=TAU*i as f64/petals as f64+f*0.02;
     let r=1.35*g;
     solid(scene,pos(o,r*a.cos(),height*0.52,r*a.sin()),
        [0.25*g,0.41*g,0.27*g],a,color(c,0.65,0.87));
   }
 }
 Form::Crystal=>{
    central_core(scene,o,id,g,color(c,0.91,0.95));
    for i in 0..petals{
      let a=TAU*i as f64/petals as f64+f*0.061;
      let foot=pos(o,a.cos()*extent*0.54,-0.65,a.sin()*extent*0.54);
      let tip=pos(foot,a.sin()*0.32,
                height*(0.48+(i%4) as f64*0.19).max(0.16),a.cos()*0.32);
      scene.solid_crystal(foot,tip,(0.3+0.11*(i%3) as f64)*g.max(0.23),
         a,color(c,0.75,0.73+(i%3) as f64*0.1),6);
    }
 }
 Form::Flora|Form::Canopy=>{
   let trunk=pos(o,0.,-0.5,0.);
   solid(scene,pos(o,0.,height*0.25,0.),[0.5*g,height*0.28,0.48*g],
        f*0.036,color(c,0.79,0.57));
   for i in 0..petals+3{
      let a=TAU*i as f64/(petals+3) as f64+age*0.008;
      petal(scene,pos(trunk,0.,height*0.36,0.),a,
           extent*(0.7+(i%3) as f64*0.16),
           height*(0.72+(i%4) as f64*0.11),
           (0.38+0.09*(i%3) as f64)*g,
           color(c,0.69+0.02*(i%3) as f64,0.73));
   }
 }
 Form::Phoenix=>{
   let base=pos(o,0.,-0.25,0.);
   central_core(scene,o,id,g,color(c,0.94,0.9));
   for side in [-1.,1.]{
     for i in 0..4+(id%3) as usize {
       let a=if side>0. {0.5}else{PI-0.5}
             +(i as f64*0.18)*(if side>0.{1.}else{-1.});
       petal(scene,pos(base,0.,height*0.35,0.),a,extent*1.1,
              height*(1.0+i as f64*0.12),0.55*g,
              color(c,0.76,0.75+i as f64*0.034));
     }
   }
 }
 Form::Armor=>{
    // Three CLOSED shielding shells with physical depth; no empty rings.
    for i in 0..3 {
      let az=f*0.37+i as f64*0.56;
      solid(scene,pos(o,(i as f64-1.)*0.6*g,1.6*g,0.1*i as f64),
       [(1.35-i as f64*0.13)*g,(1.53-i as f64*0.15)*g,
        (0.42+0.12*i as f64)*g],az,color(c,0.42+i as f64*0.13,0.8));
    }
 }
 Form::Architecture=>{
   // Actual thick solid buttresses/tiles; avoid outline-only cathedral.
   let pieces=4+local%5;
   for i in 0..pieces{
      let a=TAU*i as f64/pieces as f64;
      let step=extent*0.53;
      let x=a.cos()*step;let z=a.sin()*step;
      let h=(height*0.45+(i%4) as f64*height*0.17).max(0.17);
      solid(scene,pos(o,x,-0.5+h*0.5,z),
         [0.38*g,h*0.5,0.46*g],a+f*0.01,
         color(c,0.89,0.68+i as f64/pieces as f64*0.18));
   }
   solid(scene,pos(o,0.,height*0.57,0.),[extent*0.59,0.22*g,extent*0.51],
        f*0.02,color(c,0.6,0.88));
 }
 Form::Boulder=>{
   for i in 0..petals+2{
     let a=TAU*i as f64/(petals+2) as f64+f*0.04;
     let r=extent*(0.3+i as f64/(petals+2) as f64*0.5);
     let h=height*(0.33+((i*11+local)%5) as f64*0.14);
     solid(scene,pos(o,a.cos()*r,-0.5+h*0.42,a.sin()*r),
       [(0.4+0.12*(i%3) as f64)*g,h*0.5,
         (0.42+0.1*(i%4) as f64)*g],
       a*0.65,color(c,0.88,0.54+(i%4) as f64*0.08));
   }
 }
 Form::Wave|Form::Cloud=>{
   // Thick 3D draped curtains; multi-angle legible, never line strokes.
   for sheet in 0..3+(id%3) as usize {
     let a=TAU*sheet as f64/(3+(id%3) as usize) as f64+f*0.031;
     let mut prev=pos(o,0.,0.,0.);
     for step in 0..8{
       let t=(step+1) as f64/8.0;
       let bend=0.38*(t*TAU*1.6+age*0.04+a).sin();
       let next=pos(o,(a+bend).cos()*extent*t,height*t,
                    (a+bend).sin()*extent*t);
       let left=pos(next,-a.sin()*spread*g,-0.11,a.cos()*spread*g);
       let right=pos(next,a.sin()*spread*g,0.13,-a.cos()*spread*g);
       scene.membrane(prev,left,next,right,
          color(c,0.22+0.06*(step%3) as f64,0.75));
       prev=next;
     }
   }
 }
 Form::SkyRift=>{
    // Two coherent warped overlapping crystal curtains separated in depth.
    for sheet in 0..2 {
      let Some(previous)=scene.begin_component(49) else{continue;};
      let z=(sheet as f64-0.5)*0.7;
      let columns=7;
      for row in 0..7 {
        let t=row as f64/7.;
        for column in 0..columns{
          let u=column as f64/columns as f64*2.-1.;
          let v=(column+1) as f64/columns as f64*2.-1.;
          let point=|x:f64,y:f64|pos(o,
            extent*x*(0.8+0.2*y)*(1.0+0.09*(x*3.+age*0.03).sin()),
            height*y+0.22*(x*5.+age*0.04).sin(),z+
              (x*3.0+y*TAU+age*0.025).sin()*0.47);
          scene.quad([point(u,t),point(v,t),point(v,t+1./7.),
                      point(u,t+1./7.)],
              color(c,0.45+0.13*sheet as f64,0.7+0.15*t));
        }
      }
      scene.end_component(previous);
    }
 }
 Form::Time=>{
   // Hourglass-shaped faceted body with actual broad faces.
   let sides=10;
   let Some(previous)=scene.begin_component(70) else{return;};
   for seg in 0..7{
     let v=seg as f64/7.;
     let w=(seg+1) as f64/7.;
     for i in 0..sides {
       let a=TAU*i as f64/sides as f64+age*0.005;
       let b=TAU*(i+1) as f64/sides as f64+age*0.005;
       let pt=|level:f64,ang:f64|{
          let r=(0.27+1.24*(level*2.-1.).abs())*g;
          pos(o,r*ang.cos(),0.2+level*height,r*ang.sin())
       };
       scene.quad([pt(v,a),pt(v,b),pt(w,b),pt(w,a)],
           color(c,0.7,0.63+0.09*(i%3) as f64));
     }
   }
   scene.end_component(previous);
 }
 Form::Blade=>{
    let apex=pos(o,0.,height*1.15,extent*0.25);
    let root=pos(o,0.,-0.3,-extent*0.36);
    let mid=between(root,apex,0.54);
    for side in [-1.,1.]{
      let span=pos(mid,side*0.56*extent,0.,0.3*side);
      scene.membrane(root,span,apex,pos(mid,0.,0.2*g,-0.42*g),
        color(c,0.88,0.73));
      scene.solid_crystal(pos(o,side*0.38*g,0.05,-0.12),
       pos(o,side*0.2*g,height*0.9,extent*0.22),
       0.38*g,f*0.07,color(c,0.82,0.85),6);
    }
 }
 Form::Avatar=>{
   // Head + broad torso + limbs are solid boxes and faceted armor, not rods.
   solid(scene,pos(o,0.,height*0.50,0.),[0.9*g,height*0.33,0.55*g],
       f*0.023,color(c,0.79,0.64));
   solid(scene,pos(o,0.,height*1.02,0.),[0.53*g,0.56*g,0.49*g],
       f*0.06,color(c,0.83,0.84));
   for side in [-1.,1.]{
     solid(scene,pos(o,side*1.13*g,height*0.54,0.),
       [0.35*g,height*0.28,0.36*g],f*0.014,
       color(c,0.82,0.73));
   }
 }
 Form::Beast=>{
   // Undulating actual physical body with attached angular shoulder masses.
   for seg in 0..8 {
      let t=seg as f64/8.;
      let x=(t-0.50)*extent*2.0;
      let yy=height*(0.45+0.17*(t*TAU*2.+age*0.07).sin());
      let z=0.49*(t*TAU*1.7+f*0.02).cos();
      solid(scene,pos(o,x,yy,z),[0.34*g,0.51*g*(1.2-t*0.6),
         0.44*g*(1.2-t*0.55)],f*0.06+t*0.5,
         color(c,0.74,0.58+t*0.25));
   }
   for wing in [-1.,1.]{
      petal(scene,pos(o,0.,height*0.65,0.),if wing>0.{0.9}else{2.25},
       extent,height,0.57*g,color(c,0.62,0.77));
   }
 }
 Form::Artifact=>{
   let rings=8;let sides=10;
   let Some(previous)=scene.begin_component(rings*sides) else{return;};
   for j in 0..rings {
      let aa=PI*j as f64/rings as f64;
      let bb=PI*(j+1) as f64/rings as f64;
      for i in 0..sides {
        let a=TAU*i as f64/sides as f64;
        let b=TAU*(i+1) as f64/sides as f64;
        let sphere=|polar:f64,az:f64|pos(o,
           extent*0.55*polar.sin()*az.cos(),
           height*0.57+height*0.56*polar.cos(),
           extent*0.55*polar.sin()*az.sin());
        scene.quad([sphere(aa,a),sphere(aa,b),
           sphere(bb,b),sphere(bb,a)],color(c,0.58,0.66+0.04*(i%5) as f64));
      }
   }
   scene.end_component(previous);
 }
}
}
/// Only first-draft geometry for registered 4–20 spells in each school.
/// Actual role/cast type/scene lighting must receive dedicated polish before
/// user-facing "complete 200-spell mod" release.
pub fn render_expansion(cast:Cast,age:f64,scene:&mut Scene){
 if cast.kind>=200 || catalog200::rank(cast.kind)<3{return;}
 let spec=catalog200::spec(cast.kind);
 let Some((stage,progress))=phase(spec.id,age)else{return;};
 let p=progress.clamp(0.,1.);
 let g=match stage {
  Stage::Telegraph=>0.14+0.27*p,
  Stage::Cast=>0.36+0.64*p,
  Stage::Impact=>1.0+0.08*(p*TAU).sin(),
  Stage::Afterglow=>1.0-0.70*p
 };
 let fading=if stage==Stage::Afterglow {(1.0-p).powf(1.35)}else{1.0};
 let school=catalog200::school(cast.kind);
 let c=color(catalog200::WORLD_COLORS[school],0.82*fading,0.95);
 let mut origin=cast.center;
 if school==1||school==3||school==5||school==9 {
     // Terra/ice/root spells grow around a surface target, not eye height.
     origin[1]-=0.18;
 }
 signature(scene,semantic_form(spec.mesh),origin,spec.id,age,g,c);
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn all_170_new_spells_have_distinct_bounded_solid_geometry_and_lifetime(){
     let mut fingerprints=std::collections::BTreeSet::new();
     for slot in 0..200{
         if catalog200::rank(slot)<3{continue;}
         let age=53.;
         let mut scene=Scene::new(500);
         render_expansion(Cast{kind:slot,variant:0,start:0,
           center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut scene);
         assert!(!scene.facets.is_empty(),"empty cast geometry slot {slot}");
         assert!(scene.facets.len()<=500);
         assert!(scene.facets.iter().all(|(q,_)|
             q.iter().flatten().all(|v|v.is_finite())));
         let verts=scene.facets.iter().take(28).flat_map(|(q,_)|
            q.iter().flat_map(|v|v.iter().map(|x|
                (x*100.) as i32))).collect::<Vec<_>>();
         assert!(fingerprints.insert(verts),"same first 28 panels for slot {slot}");
         let mut ended=Scene::new(500);
         render_expansion(Cast{kind:slot,variant:0,start:0,
           center:[0.,64.,0.],direction:[0.,0.,1.]},104.,&mut ended);
         assert!(ended.facets.is_empty());
     }
     assert_eq!(fingerprints.len(),170);
 }
 #[test]fn semantics_reject_color_clone_identity(){
     assert_eq!(semantic_form("floating skull lantern"),Form::Avatar);
     assert_eq!(semantic_form("towering ice cathedral ribs"),Form::Architecture);
     assert_eq!(semantic_form("interlocking suspended ice plates"),Form::Crystal);
     assert_eq!(semantic_form("unfolding flaming phoenix"),Form::Phoenix);
     assert_eq!(semantic_form("carved gravestone glyph"),Form::Glyph);
     assert_eq!(semantic_form("braided crimson cord"),Form::Ribbon);
     assert_eq!(semantic_form("vibrating spectral bell"),Form::Relic);
     // No unclassified mesh from the original 170-design registry may
     // silently inherit a generic placeholder canopy.
     for source in &aetheria_horizon::magic::registry::SPELLS{
         if (source.id-1)%20>=3 {
             assert_ne!(semantic_form(source.mesh),Form::Canopy,
                        "unclassified authored mesh for spell {}: {}",source.id,source.mesh);
         }
     }
 }
}
