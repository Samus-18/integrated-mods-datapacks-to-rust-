//! M25: ten more ORIGINAL per-ID Charged Standard spell VFX (rank 9/20).
//! Spatial silhouettes derive from the canonical registry's exact mesh and
//! gameplay mechanic texts, never just school hue. Draft, not final art.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,orb,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
const IDS:[u16;10]=[9,29,49,69,89,109,129,149,169,189];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let t=for_spell(id)?;
 let clock=age/104.*t.total_ticks() as f64;
 let mut previous=0.0;
 for stage in t.phases{
  let finish=previous+stage.duration_ticks as f64;
  if clock<finish{return Some((stage.kind,((clock-previous)/(finish-previous)).clamp(0.,1.)))}
  previous=finish;
 }
 None
}
fn shape(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
 let time=age*0.024;
 match id {
 9=>{ // COALSTORM: physically uneven glowing char-rocks, rolling hot haze.
   for i in 0..12 {
     let a=TAU*i as f64/12.+time*0.65;
     let radius=(1.35+0.27*(i%5) as f64)*g;
     let center=p(o,radius*a.cos(),(0.8+0.28*(i%7) as f64)*g,
                       radius*a.sin());
     let k=(0.30+0.10*(i%4) as f64)*g;
     let base=p(center,-0.22*g,0.,0.);
     let tip=p(center,0.32*k,(0.55+0.13*(i%3) as f64)*g,
                       0.23*g);
     crystal(s,base,tip,k,a,tint(c,0.53+0.05*(i%4) as f64,0.95));
     crystal(s,p(center,0.,-0.24*g,0.),
         p(center,-0.22*a.sin()*g,0.43*g,0.32*a.cos()*g),
         0.26*g,a+0.3,tint(c,0.90,0.88));
   }
   for sheet in 0..2{
     curtain(s,o,(1.4+0.41*sheet as f64)*g,3.8*g,age,
         sheet as f64*PI/2.+time*0.20,tint(c,0.76,0.19));
   }
  },
 29=>{ // GLACIER LANCE: massive prismatic piercing ice spear with fins.
   let root=p(o,0.,1.1*g,-3.2*g);
   let mid=p(o,0.,1.42*g,0.65*g);
   let tip=p(o,0.,2.12*g,5.3*g);
   crystal(s,root,mid,0.72*g,time,tint(c,0.62,0.91));
   crystal(s,mid,tip,0.49*g,time+0.42,tint(c,0.97,0.92));
   for i in 0..8{
      let a=TAU*i as f64/8.+time*0.08;
      let center=p(o,0.,1.45*g,(i as f64-3.)*0.75*g);
      wing(s,center,a,(0.48+0.11*(i%3) as f64)*g,
        (0.60+0.10*(i%4) as f64)*g,tint(c,0.82,0.79));
   }
   for i in 0..6{
     let a=TAU*i as f64/6.;
     let base=p(o,1.23*g*a.cos(),-0.37*g,1.23*g*a.sin()-2.05*g);
     crystal(s,base,p(base,0.,1.12*g,0.),0.33*g,a,
        tint(c,0.74,0.76));
   }
  },
 49=>{ // AERIAL MANTLE: a true double-wing 3D cape wrapped around torso.
   body(s,p(o,0.,1.54*g,0.),[0.42*g,0.88*g,0.27*g],tint(c,0.64,0.60));
   for side in [-1.,1.]{
    for feather in 0..9{
      let u=feather as f64/8.;
      let a=if side<0. {PI-0.43*u}else{0.43*u};
      let spread=(1.62+0.28*feather as f64)*g;
      let height=(2.0+0.32*(feather%4) as f64)*g;
      wing(s,p(o,side*0.34*g,1.65*g,-0.21*g),
            a,spread,height,tint(c,0.67+0.025*(feather%5) as f64,0.40));
    }
   }
   crystal(s,p(o,0.,0.4*g,0.),p(o,0.,3.13*g,0.),0.30*g,
       time,tint(c,0.87,0.80));
  },
 69=>{ // PEBBLE BARRAGE: twisting stream of FACETED shooting stones.
   // The first picture was a string of identical cuboid chunks.
   for i in 0..20{
     let t=i as f64/19.;
     let a=t*TAU*2.6+time;
     let radius=(0.38+1.05*(1.0-t))*g;
     let point=p(o,radius*a.cos(),(1.05+0.60*a.sin())*g,
                   (t-0.2)*8.8*g);
     let width=(0.22+0.06*(i%4) as f64)*g;
     crystal(s,p(point,-0.21*a.sin()*g,-0.19*g,-0.26*g),
        p(point,0.28*a.cos()*g,0.29*g,0.47*g),
        width,a,tint(c,0.61+0.035*(i%4) as f64,0.92));
   }
   for i in 0..4 {
     let a=TAU*i as f64/4.+time*0.20;
     let root=p(o,0.,1.15*g,-1.3*g);
     let tip=p(o,1.25*g*a.cos(),1.3*g+0.41*a.sin(),4.5*g);
     let flank=p(o,0.70*g*a.cos(),1.95*g,1.28*g);
     s.membrane(root,flank,tip,p(o,0.62*g*a.cos(),0.80*g,1.1*g),
       tint(c,0.84,0.34));
   }
  },
 89=>{ // ABYSS JET: coherent thick fluid lance with cascading water skin.
   let base=p(o,0.,1.42*g,-0.8*g);
   let end=p(o,0.,1.74*g,6.1*g);
   crystal(s,base,end,0.80*g,time,tint(c,0.70,0.65));
   for i in 0..8{
      let a=TAU*i as f64/8.+time*0.37;
      let radius=0.48*g;
      let inner=p(base,radius*a.cos(),0.2*g*a.sin(),0.);
      let outer=p(end,0.43*g*a.cos(),0.36*g*a.sin(),0.16*g);
      let crest=p(o,1.06*g*a.cos(),2.1*g+0.30*a.sin(),3.7*g);
      s.membrane(inner,crest,outer,p(o,0.85*g*a.cos(),0.82*g,3.3*g),
          tint(c,0.75+0.025*(i%3) as f64,0.44));
   }
   for i in 0..10{
      let a=TAU*i as f64/10.+time;
      let tip=p(o,1.4*g*a.cos(),(1.2+0.32*(i%3) as f64)*g,
                5.6*g+0.54*g*a.sin());
      crystal(s,p(tip,0.,-0.14*g,0.),p(tip,0.18*g*a.cos(),0.33*g,0.),
          0.15*g,a,tint(c,0.97,0.78));
   }
  },
 109=>{ // BLOOMSTEP: a progressive pathway of differentiated large flowers.
   // Eight complete flowers rather than nine near-budget flowers leave
   // room for impact particles without saturating the 500-face scene cap.
   for bloom in 0..8{
      let t=bloom as f64/7.;
      let x=(if bloom%2==0{-0.72}else{0.72})*g;
      let base=p(o,x,-0.41*g,(t-0.5)*8.6*g);
      let seed=p(base,0.,0.40*g,0.);
      crystal(s,base,seed,0.27*g,t*PI,tint(c,0.56,0.90));
      for petal in 0..5{
         let a=TAU*petal as f64/5.+time*0.07;
         wing(s,seed,a,(0.73+0.10*(petal%3) as f64)*g,
           (0.56+0.10*(bloom%4) as f64)*g,
           tint(c,0.77+0.025*petal as f64,0.81));
      }
      crystal(s,p(seed,0.,0.01*g,0.),p(seed,0.,0.31*g,0.),
             0.23*g,t,tint(c,0.96,0.91));
   }
  },
 129=>{ // SURGE PALM: readable electric *hand* before branching blast.
   body(s,p(o,0.,0.48*g,0.),[0.39*g,0.56*g,0.29*g],
      tint(c,0.56,0.94));
   body(s,p(o,0.,1.67*g,0.),[0.90*g,0.74*g,0.39*g],
      tint(c,0.65,0.88));
   for finger in 0..5{
     let u=(finger as f64-2.)*0.41;
     let knuckle=p(o,u*g,2.17*g,0.14*g);
     let tip=p(knuckle,0.09*u*g,
               (0.93+0.16*(finger%3) as f64)*g,0.33*g);
     crystal(s,knuckle,tip,0.23*g,finger as f64*0.25,
           tint(c,0.84,0.91));
   }
   // Only four substantial storm ribbons: the old seven spikes hid hand.
   for wave in 0..4 {
     let a=TAU*wave as f64/4.+time*0.32;
     let root=p(o,0.,1.54*g,-0.28*g);
     let middle=p(o,0.78*g*a.cos(),1.91*g,0.46*g*a.sin()-0.30*g);
     let tip=p(o,1.96*g*a.cos(),
            2.45*g+0.23*g*a.sin(),0.92*g*a.sin()-0.27*g);
     s.membrane(root,middle,tip,p(o,0.9*g*a.cos(),0.76*g,0.74*g*a.sin()),
         tint(c,0.94,0.47));
   }
   crystal(s,p(o,0.,1.32*g,0.45*g),
     p(o,0.,1.98*g,0.53*g),0.26*g,time,tint(c,0.99,0.92));
  },
 149=>{ // NIGHTGLASS MIRROR: physically thick dark glass with broken rim.
    let center=p(o,0.,2.05*g,0.);
    s.membrane(p(center,-1.5*g,-1.72*g,-0.23*g),
      p(center,1.5*g,-1.72*g,-0.23*g),
      p(center,1.62*g,1.62*g,0.26*g),
      p(center,-1.62*g,1.62*g,0.26*g),
      tint(c,0.37,0.71));
    for side in [-1.,1.]{
      crystal(s,p(center,side*1.51*g,-1.73*g,0.),
         p(center,side*1.61*g,1.72*g,0.),0.30*g,
         side*0.19,tint(c,0.61,0.93));
    }
    for i in 0..8{
       let a=TAU*i as f64/8.+time*0.12;
       let x=1.38*g*a.cos();
       let y=1.50*g*a.sin();
       let base=p(center,x,y,0.10*g);
       crystal(s,base,p(base,0.14*x,0.22*y,0.28*g),
         0.27*g,a,tint(c,0.84,0.81));
    }
    crystal(s,p(center,0.,-0.30*g,-0.34*g),
              p(center,0.,0.45*g,-0.34*g),
              0.29*g,time,tint(c,0.95,0.65));
  },
 169=>{ // CLOCKWORK WARD: open deep gear-shaped PHYSICAL shield.
   // Former picture resembled a solid brown cube with poorly visible teeth.
   body(s,p(o,0.,1.87*g,0.),[0.64*g,0.70*g,0.39*g],
       tint(c,0.52,0.91));
   for tooth in 0..12{
     let a=TAU*tooth as f64/12.+time*0.12;
     let root=p(o,0.90*g*a.cos(),1.87*g+0.90*g*a.sin(),0.04*g);
     let tip=p(o,1.77*g*a.cos(),1.87*g+1.77*g*a.sin(),0.06*g);
     crystal(s,root,tip,0.40*g,a,tint(c,0.68+0.027*(tooth%5) as f64,0.94));
     if tooth%3==0{
       body(s,p(o,1.6*g*a.cos(),1.87*g+1.6*g*a.sin(),0.10*g),
          [0.30*g,0.34*g,0.38*g],
          tint(c,0.82,0.91));
     }
   }
   for hand in 0..3{
     let a=TAU*hand as f64/3.+time*0.48;
     crystal(s,p(o,0.,1.87*g,0.56*g),
         p(o,1.09*g*a.cos(),1.87*g+1.09*g*a.sin(),0.66*g),
         (0.18+0.04*hand as f64)*g,a,tint(c,0.89,0.94));
   }
   crystal(s,p(o,0.,1.43*g,0.72*g),
         p(o,0.,2.38*g,0.73*g),0.37*g,time,
         tint(c,0.97,0.93));
  },
 189=>{ // EPITAPH MARK: deep arched TOMBSTONE with engraved spirit eye.
   // A big blank slab did not communicate a magical gravemark.
   body(s,p(o,0.,0.94*g,0.),[0.80*g,0.93*g,0.49*g],
        tint(c,0.49,0.94));
   for i in 0..9 {
     let a=PI*i as f64/8.;
     let x=1.03*g*a.cos();
     let y=1.63*g+1.07*g*a.sin();
     let center=p(o,x,y,0.);
     body(s,center,[0.25*g,0.32*g,0.45*g],
          tint(c,0.56+0.03*(i%4) as f64,0.93));
   }
   for side in [-1.,1.]{
     crystal(s,p(o,side*0.97*g,-0.40*g,0.),
       p(o,side*1.01*g,1.66*g,0.),0.32*g,side*0.18,
       tint(c,0.62,0.92));
   }
   for i in 0..6{
     let a=TAU*i as f64/6.+time*0.13;
     let start=p(o,0.65*g*a.cos(),1.25*g+0.48*g*a.sin(),0.59*g);
     let tip=p(o,0.23*g*a.cos(),1.21*g+0.18*g*a.sin(),0.67*g);
     crystal(s,start,tip,0.15*g,a,tint(c,0.96,0.85));
   }
   for fragment in 0..9{
     let a=TAU*fragment as f64/9.;
     let base=p(o,(1.03+0.11*(fragment%3) as f64)*g*a.cos(),
       -0.49*g,(1.03+0.11*(fragment%3) as f64)*g*a.sin());
     crystal(s,base,p(base,0.15*g*a.cos(),0.65*g,0.15*g*a.sin()),
       0.25*g,a,tint(c,0.71,0.82));
   }
   curtain(s,p(o,0.,-0.2*g,-0.36*g),
      1.10*g,3.9*g,age,0.1,tint(c,0.86,0.14));
  },
  _=>unreachable!("only ten M25 Charged Standard IDs"),
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((stage,t))=phase(id,age)else{return;};
 let g=match stage{
  Stage::Telegraph=>0.13+0.21*t,
  Stage::Cast=>0.34+0.66*t,
  Stage::Impact=>1.06+0.11*t,
  Stage::Afterglow=>(1.0-t).max(0.08)
 };
 let before=s.facets.len();
 shape(s,id,cast.center,age,g,catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
 if stage==Stage::Afterglow{
  for (_,col) in &mut s.facets[before..]{
    let alpha=((*col>>24)&255) as f64/255.;
    *col=(*col&0x00FF_FFFF)|
      (((alpha*(1.0-t).powf(0.81)*255.).round() as u32)<<24);
  }
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn m25_charged_spells_are_source_specific_finite_and_not_clipped(){
  use std::collections::BTreeSet;
  let mut signatures=BTreeSet::new();
  for id in IDS {
   let tick=(0..104).find(|n|matches!(
      phase(id,*n as f64),Some((Stage::Cast,p)) if p>0.6
   )).unwrap() as f64;
   let mut s=Scene::new(500);
   render(Cast{kind:(id-1) as usize,variant:0,start:0,
       center:[0.,64.,0.],direction:[0.,0.,1.]},tick,&mut s);
   assert!((20..500).contains(&s.facets.len()),
       "clipped or empty spell {id} {} quads",s.facets.len());
   assert!(s.facets.iter().all(|(q,_)|q.iter().flatten().all(|x|x.is_finite())));
   let fingerprint=s.facets.iter().flat_map(|(q,_)|
       q.iter().flat_map(|v|v.iter().map(|x|(*x*100.).round() as i64))
   ).collect::<Vec<_>>();
   assert!(signatures.insert(fingerprint),"same shape clone {id}");
  }
  assert_eq!(signatures.len(),10);
 }
}
