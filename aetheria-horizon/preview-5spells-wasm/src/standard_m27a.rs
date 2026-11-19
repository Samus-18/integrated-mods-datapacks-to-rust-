//! M27A: last uncurated Standard positions 12–14 in the first five schools.
//! 15 manually selected 3D spell identities, all sourced by registry ID/mesh.
//! Client-side VFX drafts only; live world mechanics are DesignIntent.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
pub(super) const IDS:[u16;15]=[12,13,14,32,33,34,52,53,54,72,73,74,92,93,94];
pub(super) fn dedicated(id:u16)->bool {IDS.contains(&id)}
fn stage(id:u16,age:f64)->Option<(Stage,f64)> {
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let timeline=for_spell(id)?;
 let tick=age/104.0*timeline.total_ticks() as f64;
 let mut base=0.;
 for part in timeline.phases {
  let end=base+part.duration_ticks as f64;
  if tick<end {return Some((part.kind,((tick-base)/part.duration_ticks as f64).clamp(0.,1.)))}
  base=end;
 }
 None
}
fn draw(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32) {
 let time=age*0.022;
 match id {
 12=>{ // Basalt Burst — rupturing radial volcanic fan of hefty rock teeth.
  floor_slab(s,o,2.6*g,0.36*g,time*0.035,tint(c,0.37,0.95));
  for i in 0..10{
   let a=TAU*i as f64/10.+0.13*(i%3) as f64;
   let r=(0.55+0.20*(i%5) as f64)*g;
   let at=p(o,r*a.cos(),-0.43*g,r*a.sin());
   crystal(s,at,p(at,(1.5+0.16*(i%3) as f64)*g*a.cos(),
      (1.5+0.38*(i%4) as f64)*g,
      (1.5+0.16*(i%3) as f64)*g*a.sin()),
      (0.36+0.10*(i%3) as f64)*g,a,tint(c,0.52+0.037*(i%4) as f64,0.94));
   if i%2==0 {wing(s,p(at,0.,0.32*g,0.),a,0.82*g,1.05*g,tint(c,0.95,0.74));}
  }
 },
 13=>{ // Heat Mirage — displaced broad thermal glass with material refraction hint.
  for sheet in 0..3 {
    let a=sheet as f64*0.77+time*0.24;
    curtain(s,p(o,(sheet as f64-1.)*0.65*g,0.,0.),
      (2.0+sheet as f64*0.3)*g,3.9*g,age,a,
      tint(c,0.64+0.08*sheet as f64,0.30));
  }
  for i in 0..6{
   let a=TAU*i as f64/6.+time*0.31;
   let at=p(o,1.3*g*a.cos(),-0.28*g,1.3*g*a.sin());
   body(s,at,[0.45*g,0.14*g,0.42*g],tint(c,0.59,0.86));
  }
 },
 14=>{ // Infernal Anvil — forged raised anvil body with side-horns and impact.
  body(s,p(o,0.,1.40*g,0.),[1.34*g,0.29*g,0.82*g],tint(c,0.56,0.98));
  body(s,p(o,0.,0.84*g,0.),[0.60*g,0.72*g,0.56*g],tint(c,0.40,0.97));
  body(s,p(o,0.,0.18*g,0.),[1.05*g,0.27*g,0.88*g],tint(c,0.52,0.94));
  for side in [-1.0,1.0]{
   let at=p(o,side*1.05*g,1.45*g,0.);
   crystal(s,at,p(at,side*1.02*g,0.43*g,0.),0.51*g,
      time,tint(c,0.78,0.95));
  }
  for i in 0..8{
   let a=TAU*i as f64/8.;
   crystal(s,p(o,0.,0.45*g,0.),
     p(o,1.9*g*a.cos(),(0.5+0.40*(i%3) as f64)*g,1.9*g*a.sin()),
     0.12*g,a,tint(c,0.96,0.78));
  }
 },
 32=>{ // Icicle Rain — hanging chandelier clusters, not naked falling sticks.
  body(s,p(o,0.,4.4*g,0.),[1.62*g,0.30*g,1.25*g],tint(c,0.69,0.80));
  for i in 0..12{
   let a=TAU*i as f64/12.;
   let at=p(o,(0.8+0.47*(i%3) as f64)*g*a.cos(),
        (4.2+0.18*(i%3) as f64)*g,
        (0.6+0.41*(i%4) as f64)*g*a.sin());
   let low=p(at,0.,-(1.3+0.28*(i%4) as f64)*g,0.);
   crystal(s,at,low,(0.35+0.065*(i%3) as f64)*g,a,
      tint(c,0.74+0.03*(i%4) as f64,0.92));
   if i%3==0 {wing(s,at,a,0.83*g,-0.36*g,tint(c,0.96,0.72));}
  }
 },
 33=>{ // Crystal Fault — jagged ice RIDGE rising along a broken seam.
  for i in 0..12 {
   let u=i as f64/11.;
   let x=(u-0.5)*7.5*g;
   let z=(u*TAU*1.7+time*0.24).sin()*0.50*g;
   let at=p(o,x,-0.50*g,z);
   body(s,p(at,0.,0.17*g,0.),[0.34*g,0.19*g,0.64*g],
       tint(c,0.57+0.04*(i%4) as f64,0.91));
   let top=p(at,(if i%2==0{-0.4}else{0.4})*g,
        (1.55+0.37*(i%5) as f64)*g,0.28*g);
   crystal(s,at,top,(0.31+0.07*(i%3) as f64)*g,
      time+u,tint(c,0.71,0.94));
  }
 },
 34=>{ // Winter Breath — widening crystal fog cone made from filled sheets.
  for i in 0..7 {
    let a=TAU*i as f64/7.;
    let start=p(o,0.,1.4*g,-0.4*g);
    let tip=p(o,2.6*g*a.cos(),(1.2+1.2*(i%3) as f64/2.)*g,
       (3.0+0.3*(i%3) as f64)*g);
    let middle=add(start,scale(sub(tip,start),0.62));
    let flank=[-0.38*g*a.sin(),0.22*g,0.45*g*a.cos()];
    s.membrane(start,add(middle,flank),tip,sub(middle,flank),
       tint(c,0.74+0.026*(i%4) as f64,0.40));
    crystal(s,p(tip,0.,-0.20*g,0.),p(tip,0.,0.63*g,0.),
       0.19*g,a,tint(c,0.94,0.82));
  }
 },
 52=>{ // Skyward Hook — a substantial curving air-stream grappling cloth.
  let mut last=p(o,-2.25*g,0.3*g,0.);
  for i in 0..10 {
    let t=i as f64/9.;
    let a=t*PI*1.18+time*0.18;
    let next=p(o,(-2.15+4.2*t)*g,
      (0.7+3.3*t+0.27*a.sin())*g,(0.62*a.cos())*g);
    let mid=add(last,scale(sub(next,last),0.5));
    s.membrane(last,p(mid,0.,0.22*g,0.36*g),next,
        p(mid,0.,-0.22*g,-0.36*g),
        tint(c,0.71+0.017*i as f64,0.55));
    last=next;
  }
  crystal(s,last,p(last,-0.85*g,0.58*g,0.),0.46*g,time,tint(c,0.87,0.87));
 },
 53=>{ // Soundbreak — filled wedge shockfront with embossed pressure waves.
  for layer in 0..3{
    let z=(1.0+0.62*layer as f64)*g;
    let width=(1.0+0.63*layer as f64)*g;
    for side in [-1.,1.] {
      let base=p(o,side*0.31*g,0.6*g,0.);
      let low=p(o,side*width,0.25*g,z);
      let high=p(o,side*width*0.84,3.0*g,z);
      let inner=p(o,0.,1.75*g,z*0.85);
      s.membrane(base,low,high,inner,
        tint(c,0.64+0.08*layer as f64,0.42));
      body(s,p(o,side*width*0.55,1.55*g,z),
        [0.15*g,0.16*g,0.28*g],tint(c,0.81,0.79));
    }
  }
 },
 54=>{ // Windwall Gallery — curved gallery of solid spatial wind pillars.
  for i in 0..5 {
    let z=(i as f64-2.)*1.05*g;
    let rise=(2.2+0.15*(i%3) as f64)*g;
    for side in [-1.,1.] {
      let x=side*(1.55+0.11*(i%2) as f64)*g;
      crystal(s,p(o,x,-0.40*g,z),p(o,x*0.75,rise,z),
        0.36*g,i as f64,tint(c,0.75,0.69));
      let top=p(o,0.,(rise+0.53*g),z);
      s.membrane(p(o,x*0.78,rise,z),p(o,x*0.38,rise+0.45*g,z),
        top,p(o,0.,rise+0.12*g,z),
        tint(c,0.87,0.47));
    }
  }
 },
 72=>{ // Quartz Garden — living branching mineral grove with broad facets.
  for i in 0..9 {
    let a=TAU*i as f64/9.;
    let r=(1.4+0.32*(i%3) as f64)*g;
    let root=p(o,r*a.cos(),-0.51*g,r*a.sin());
    let trunk=p(root,0.,(2.0+0.31*(i%4) as f64)*g,0.);
    crystal(s,root,trunk,0.43*g,a,tint(c,0.70,0.95));
    for j in 0..2 {
      let spread=a+(if j==0{-0.48}else{0.48});
      crystal(s,p(root,0.,0.79*g,0.),
        p(trunk,0.90*g*spread.cos(),0.64*g,0.90*g*spread.sin()),
        0.28*g,spread,tint(c,0.87,0.90));
    }
  }
 },
 73=>{ // Mountain Shoulder — heavy overlapping shifting cliff buttresses.
  for row in 0..3 {
    for col in 0..4 {
      let x=(col as f64-1.5)*1.12*g;
      let z=(row as f64-1.)*1.0*g;
      let height=(1.0+0.38*((row+col)%3) as f64)*g;
      body(s,p(o,x,height*0.5-0.47*g,z),
        [0.60*g,height*0.5,0.58*g],
        tint(c,0.50+0.047*((row+col)%5) as f64,0.96));
    }
  }
  for i in 0..5 {
    let x=(i as f64-2.)*0.91*g;
    crystal(s,p(o,x,0.16*g,-1.0*g),
      p(o,x+0.17*g,(2.6+0.16*(i%3) as f64)*g,-0.75*g),
      0.42*g,i as f64,tint(c,0.74,0.90));
  }
 },
 74=>{ // Sandglass Trap — unstable sinking trap made of filled sand funnels.
  floor_slab(s,o,2.5*g,0.30*g,time*0.04,tint(c,0.53,0.90));
  for i in 0..12 {
    let a=TAU*i as f64/12.;
    let b=TAU*(i+1) as f64/12.;
    let outer=p(o,2.30*g*a.cos(),-0.14*g,2.30*g*a.sin());
    let outer2=p(o,2.30*g*b.cos(),-0.14*g,2.30*g*b.sin());
    let inner=p(o,0.53*g*a.cos(),-1.68*g,0.53*g*a.sin());
    let inner2=p(o,0.53*g*b.cos(),-1.68*g,0.53*g*b.sin());
    s.quad([outer,outer2,inner2,inner],
      tint(c,0.50+0.033*(i%5) as f64,0.74));
    if i%3==0{
      crystal(s,outer,p(outer,0.,0.61*g,0.),0.27*g,a,
        tint(c,0.83,0.77));
    }
  }
 },
 92=>{ // River Serpent — wide serpentine mass and distinct upright head/fin.
  for i in 0..11 {
    let u=i as f64/10.;
    let x=(u-0.5)*6.2*g;
    let z=0.9*g*(u*TAU*1.45+time).sin();
    let y=(0.78+0.31*(u*TAU*1.7).cos())*g;
    let from=p(o,x,y,z);
    let to=p(o,x+0.57*g,y+0.24*g*(u*TAU).sin(),z+0.25*g);
    crystal(s,from,to,(0.35+0.19*(1.-u))*g,u+time,tint(c,0.66,0.79));
    if i%2==0 {
      wing(s,from,u*TAU*1.5,0.72*g,0.79*g,tint(c,0.91,0.62));
    }
  }
  body(s,p(o,3.0*g,1.0*g,0.2*g),[0.63*g,0.38*g,0.58*g],tint(c,0.80,0.87));
 },
 93=>{ // Tidal Mirror — three curved tall translucent reflection screens.
  for panel in 0..3{
    let a=(panel as f64-1.)*0.52;
    let z=panel as f64*0.68*g;
    for i in 0..8{
      let u=i as f64/8.;
      let v=(i+1) as f64/8.;
      let pos=|q:f64,height:f64|{
        p(o,(q*4.0-2.0)*g,height*g,
          z+0.53*g*((q*TAU+a+time).sin())+0.28*g*height);
      };
      s.quad([pos(u,0.12),pos(v,0.12),pos(v,3.75),pos(u,3.75)],
        tint(c,0.67+0.06*panel as f64+0.017*i as f64,0.49));
      s.quad([pos(u,0.12),pos(v,0.12),pos(v,0.32),pos(u,0.32)],
        tint(c,0.92,0.76));
    }
  }
 },
 94=>{ // Undertow Hand — full articulated gripping water palm/fingers.
  body(s,p(o,0.,0.58*g,0.),[0.97*g,0.50*g,0.46*g],tint(c,0.66,0.69));
  for finger in 0..5 {
    let x=(finger as f64-2.)*0.42*g;
    let root=p(o,x,0.78*g,0.);
    let joint=p(root,0.11*g*((finger%2) as f64*2.-1.),1.25*g,0.2*g);
    let end=p(joint,0.16*g*((finger%3) as f64-1.),
      (0.65+0.19*(finger%3) as f64)*g,0.48*g);
    crystal(s,root,joint,0.22*g,finger as f64,tint(c,0.78,0.72));
    crystal(s,joint,end,0.17*g,finger as f64+0.3,tint(c,0.91,0.69));
  }
  curtain(s,p(o,0.,-0.47*g,0.),1.1*g,1.3*g,age,0.1,tint(c,0.71,0.18));
 },
 _=>unreachable!("M27A matched only fifteen source-specific spell IDs")
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((kind,progress))=stage(id,age)else{return;};
 let g=match kind{
   Stage::Telegraph=>0.14+progress*0.26,
   Stage::Cast=>0.38+progress*0.62,
   Stage::Impact=>1.03+progress*0.06,
   Stage::Afterglow=>(1.-progress).max(0.07)
 };
 let prev=s.facets.len();
 draw(s,id,cast.center,age,g,
      catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
 if kind==Stage::Afterglow{
  for (_,col) in &mut s.facets[prev..]{
    let a=((*col>>24)&255) as f64/255.;
    *col=(*col&0x00FFFFFF) |
       (((a*(1.-progress).powf(0.72)*255.).round() as u32)<<24);
  }
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn source_specific_fifteen_finite_distinct_spells_no_clipping(){
  let mut geometries=std::collections::BTreeSet::new();
  for (i,id) in IDS.iter().copied().enumerate(){
    assert_eq!((id-1)/20,i as u16/3);
    let slot=(id-1) as usize;
    let age=(0..104).find(|tick|matches!(
      stage(id,*tick as f64),Some((Stage::Cast,p)) if p>0.56
    )).unwrap() as f64;
    let mut s=Scene::new(500);
    render(Cast{kind:slot,variant:4,start:0,center:[0.,64.,0.],
      direction:[0.,0.,1.]},age,&mut s);
    assert!(s.facets.len()>=14,"too few visible faces for {id}");
    assert!(s.facets.len()<500,"cut off at 500 budget for {id}");
    assert!(s.facets.iter().all(|(vertices,_)|
       vertices.iter().flatten().all(|f|f.is_finite())),"bad geometry: {id}");
    let signature=s.facets.iter().flat_map(|(quad,_)|
      quad.iter().flat_map(|v|v.iter().map(|n|(*n*100.).round() as i64))
    ).collect::<Vec<_>>();
    assert!(geometries.insert(signature),"identical geometry (ignoring color) {id}");
  }
  assert_eq!(geometries.len(),15);
 }
}
