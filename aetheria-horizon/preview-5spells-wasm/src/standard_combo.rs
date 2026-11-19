//! M25: original explicit Combo-class Standard compositions (rank 8/20).
//! All ten are first-pass geometry, NOT approved beautiful cinematic VFX
//! nor actual gameplay world mechanics. No empty central ring shortcuts.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,orb,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
const IDS:[u16;10]=[8,28,48,68,88,108,128,148,168,188];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)> {
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let t=for_spell(id)?;
 let now=age/104.*t.total_ticks() as f64;
 let mut previous=0.;
 for part in t.phases{
  let end=previous+part.duration_ticks as f64;
  if now<end{return Some((part.kind,((now-previous)/(end-previous)).clamp(0.,1.)))}
  previous=end;
 }
 None
}
fn shape(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
 let time=age*0.028;
 match id{
 8=>{ // LAVA BLOOM: erupted volcanic flower that opens OUTWARD.
    // A real molten central bud, visible eight broad curled petals.
    body(s,p(o,0.,0.12*g,0.),[0.80*g,0.37*g,0.81*g],
        tint(c,0.49,0.95));
    crystal(s,p(o,0.,-0.22*g,0.),p(o,0.,2.85*g,0.),
        0.73*g,time,tint(c,0.90,0.93));
    for petal in 0..10 {
      let a=TAU*petal as f64/10.+time*0.1;
      let spread=(1.6+0.15*(petal%4) as f64)*g;
      let root=p(o,0.48*g*a.cos(),0.46*g,0.48*g*a.sin());
      let mid=p(o,spread*0.62*a.cos(),(2.6+0.11*(petal%3) as f64)*g,
               spread*0.62*a.sin());
      let tip=p(o,spread*a.cos(),(2.18+0.23*(petal%3) as f64)*g,
               spread*a.sin());
      let side=[-a.sin()*0.44*g,0.08*g,a.cos()*0.44*g];
      s.membrane(root,add(mid,side),tip,sub(mid,side),
            tint(c,0.60+0.03*(petal%5) as f64,0.77));
      crystal(s,p(mid,0.,-0.41*g,0.),tip,0.13*g,a,
           tint(c,0.91,0.83));
    }
  },
 28=>{ // SLEET MIRROR: three actual solid *oblique ice mirrors* on feet.
    for i in 0..3{
      let center_x=(i as f64-1.)*1.72*g;
      let angle=(i as f64-1.)*0.33+0.17*(time*0.3).sin();
      let h=(3.4+0.3*(i%2) as f64)*g;
      let width=(0.88+0.13*i as f64)*g;
      let center=p(o,center_x,h*0.5,0.38*g*(i%2) as f64);
      let right=[width*angle.cos(),0.,width*angle.sin()];
      let diag=[-0.2*g,0.16*g,0.34*g];
      let a=sub(center,add(right,[0.,h*0.5,0.]));
      let b=add(center,sub(right,[0.,h*0.5,0.]));
      let c0=add(center,add(right,[0.,h*0.5,0.]));
      let d=sub(add(center,[0.,h*0.5,0.]),right);
      s.membrane(a,b,add(c0,diag),add(d,diag),
           tint(c,0.67+0.06*i as f64,0.61));
      crystal(s,p(center,0.,-h*0.6,0.),p(center,0.,-h*0.15,0.),
           0.28*g,angle,tint(c,0.93,0.87));
      for side in [-1.,1.]{
        crystal(s,p(center,side*width,-h*0.44,0.),
          p(center,side*width,h*0.47,0.30*g),
          0.22*g,side*angle,tint(c,0.82,0.87));
      }
    }
  },
 48=>{ // TWISTER SEED: substantial spiraling tapered funnel SURFACE.
    for layer in 0..9 {
       let t=layer as f64/9.;
       let u=(layer+1) as f64/9.;
       let radius=0.38*g+2.08*g*t.powf(0.92);
       let upper=0.38*g+2.08*g*u.powf(0.92);
       for face in 0..8 {
         let a=TAU*face as f64/8.+t*TAU*1.1+time;
         let b=TAU*(face+1) as f64/8.+t*TAU*1.1+time;
         let aa=TAU*face as f64/8.+u*TAU*1.1+time;
         let bb=TAU*(face+1) as f64/8.+u*TAU*1.1+time;
         let bottom=p(o,radius*a.cos(),t*4.1*g,radius*a.sin());
         let bottom2=p(o,radius*b.cos(),t*4.1*g,radius*b.sin());
         let top=p(o,upper*aa.cos(),u*4.1*g,upper*aa.sin());
         let top2=p(o,upper*bb.cos(),u*4.1*g,upper*bb.sin());
         s.quad([bottom,bottom2,top2,top],
                tint(c,0.53+0.018*(face%5) as f64,0.34));
       }
    }
    for i in 0..6 {
      let a=TAU*i as f64/6.+time*1.2;
      let root=p(o,0.7*g*a.cos(),-0.15*g,0.7*g*a.sin());
      crystal(s,root,p(root,0.42*g*a.cos(),0.85*g,0.42*g*a.sin()),
          0.25*g,a,tint(c,0.86,0.72));
    }
  },
 68=>{ // EARTHBIND: four physical locked stone ankle cuffs and ground cracks.
    for side in [-1.,1.] {
     for depth in [-1.,1.] {
      let origin=p(o,side*1.14*g,0.,depth*0.68*g);
      for slab in 0..4 {
       let a=(slab as f64-1.5)*0.63;
       let x=0.40*g*a.cos();
       let z=0.4*g*a.sin();
       body(s,p(origin,x,0.22*g,z),
            [0.24*g,0.37*g,0.40*g],
            tint(c,0.59+0.055*slab as f64,0.94));
      }
      crystal(s,p(origin,0.,-0.45*g,0.),p(origin,0.,1.13*g,0.),
         0.32*g,depth*0.2,tint(c,0.80,0.88));
     }
    }
    for crack in 0..7{
      let a=TAU*crack as f64/7.;
      let root=p(o,0.28*g*a.cos(),-0.41*g,0.28*g*a.sin());
      crystal(s,root,p(o,2.11*g*a.cos(),0.08*g,1.76*g*a.sin()),
          0.24*g,a,tint(c,0.52,0.93));
    }
  },
 88=>{ // FOAM CASCADE: three separately advancing foaming breakers.
    for wave in 0..3 {
      let z=(wave as f64-1.)*1.55*g;
      let width=(2.9-0.33*wave as f64)*g;
      let h=(2.6+0.42*wave as f64)*g;
      for section in 0..6{
        let u=section as f64/6.*2.-1.;
        let v=(section+1) as f64/6.*2.-1.;
        let base=p(o,width*u,-0.4*g,z);
        let next=p(o,width*v,-0.4*g,z);
        let high=p(o,width*v*0.92,h*(0.85+0.06*(section%3) as f64),
                    z+0.7*g);
        let high2=p(o,width*u*0.92,h*(0.87+0.04*(section%4) as f64),
                    z+0.7*g);
        s.membrane(base,next,high,high2,
            tint(c,0.61+0.04*wave as f64,0.51));
      }
      for bubble in 0..8{
         let a=TAU*bubble as f64/8.+time;
         let center=p(o,width*0.62*a.cos(),h*(0.88+0.06*a.sin()),
              z+0.56*g+0.26*g*a.sin());
         crystal(s,p(center,0.,-0.22*g,0.),
             p(center,0.12*g*a.cos(),0.25*g,0.12*g*a.sin()),
             0.20*g,a,tint(c,0.97,0.79));
      }
    }
  },
 108=>{ // BARKSKIN: actual articulated ribbed tree-bark armor, not a cube.
    body(s,p(o,0.,1.45*g,0.),[0.67*g,0.97*g,0.48*g],
        tint(c,0.53,0.91));
    for side in [-1.,1.]{
      for panel in 0..6{
        let h=(0.4+panel as f64*0.33)*g;
        let a=panel as f64*0.21+time*0.02;
        let origin=p(o,side*(0.53+0.13*(panel%3) as f64)*g,h,
                         (0.25+0.14*(panel%2) as f64)*g);
        crystal(s,p(origin,0.,-0.35*g,0.),
             p(origin,side*(0.14+0.10*a.sin())*g,
                           0.38*g,0.14*g),
             (0.36+0.04*(panel%3) as f64)*g,a,
             tint(c,0.63+0.025*panel as f64,0.90));
        if panel%2==0{
          wing(s,p(origin,0.,0.05*g,0.),PI*0.38*side,
             0.75*g,0.75*g,tint(c,0.83,0.84));
        }
      }
    }
  },
 128=>{ // CHAIN ARC: physically branching thunder TREE, braided discharge.
    let base=p(o,0.,-0.45*g,0.);
    let trunk=p(o,0.,3.0*g,0.);
    crystal(s,base,trunk,0.54*g,time,tint(c,0.88,0.92));
    for branch in 0..9{
      let a=TAU*branch as f64/9.+time*0.18;
      let root=p(o,0.13*g*a.cos(),1.1*g+(branch%3) as f64*0.55*g,
                        0.13*g*a.sin());
      let mid=p(o,1.0*g*a.cos(),2.4*g,1.0*g*a.sin());
      let end=p(o,(2.0+0.15*(branch%3) as f64)*g*a.cos(),
                 (3.2+0.18*(branch%4) as f64)*g,
                 (2.0+0.15*(branch%3) as f64)*g*a.sin());
      crystal(s,root,mid,0.23*g,a,tint(c,0.75,0.87));
      crystal(s,mid,end,0.18*g,a+0.5,tint(c,0.96,0.92));
      wing(s,mid,a,0.72*g,0.55*g,tint(c,0.78,0.55));
    }
  },
 148=>{ // WEIGHTLESS FIELD: broad folded levitation membranes and feathers.
    for plane in 0..6{
      let a=TAU*plane as f64/6.+time*0.2;
      curtain(s,p(o,0.,-0.25*g,0.),(1.5+0.15*plane as f64)*g,
           (3.1+0.18*(plane%3) as f64)*g,age,a,
           tint(c,0.56+0.02*plane as f64,0.25));
    }
    for feather in 0..12{
      let a=TAU*feather as f64/12.+time*0.43;
      let radius=(0.5+0.19*(feather%6) as f64)*g;
      let point=p(o,radius*a.cos(),(0.8+0.25*(feather%7) as f64)*g,
                   radius*a.sin());
      wing(s,point,a,0.48*g,0.56*g,tint(c,0.92,0.60));
    }
  },
 168=>{ // STOLEN BREATH: time fractured into floating clock-plates.
    for i in 0..12{
      let a=TAU*i as f64/12.;
      let radius=(1.2+0.18*(i%3) as f64)*g;
      let center=p(o,radius*a.cos(),1.75*g+radius*a.sin(),
                  (0.19*(i%3) as f64)*g);
      body(s,center,[0.22*g,0.37*g,0.30*g],
           tint(c,0.61+0.02*(i%4) as f64,0.91));
    }
    for i in 0..5{
       let a=TAU*i as f64/5.+time*0.24;
       let from=p(o,0.20*g*a.cos(),1.75*g,0.12*g*a.sin());
       let to=p(o,(1.0+0.08*i as f64)*g*a.cos(),
                (1.75+0.34*(i%3) as f64)*g,
                0.33*g*a.sin());
       s.membrane(from,p(to,-0.19*g,0.22*g,0.),to,
            p(to,0.18*g,-0.24*g,0.),tint(c,0.80,0.57));
    }
  },
 188=>{ // MARROW SHARD: closed bone sickle dagger and shattered fragments.
    let root=p(o,-1.12*g,0.21*g,0.);
    let mid=p(o,-0.50*g,1.31*g,0.08*g);
    let tip=p(o,1.64*g,4.21*g,-0.29*g);
    crystal(s,root,mid,0.58*g,time,tint(c,0.56,0.95));
    crystal(s,mid,tip,0.52*g,time+0.21,tint(c,0.94,0.94));
    for i in 0..11 {
      let a=TAU*i as f64/11.+time*0.25;
      let center=p(o,(0.45+0.19*(i%5) as f64)*g*a.cos(),
              (0.62+0.24*(i%6) as f64)*g,
              0.30*g*a.sin());
      crystal(s,p(center,0.,-0.18*g,0.),
           p(center,0.32*g*a.cos(),0.38*g,0.32*g*a.sin()),
           (0.14+0.04*(i%3) as f64)*g,a,tint(c,0.73,0.87));
    }
  },
  _=>unreachable!("only M25 Combo Standard spell IDs"),
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((stage,t))=phase(id,age)else{return;};
 let g=match stage{
  Stage::Telegraph=>0.11+0.26*t,
  Stage::Cast=>0.33+0.67*t,
  Stage::Impact=>1.02+0.14*t,
  Stage::Afterglow=>(1.0-t).max(0.08)
 };
 let begin=s.facets.len();
 shape(s,id,cast.center,age,g,catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
 if stage==Stage::Afterglow{
  for (_,col) in &mut s.facets[begin..]{
   let a=((*col>>24)&255) as f64/255.;
   *col=(*col&0x00FF_FFFF)|
      (((a*(1.0-t).powf(0.81)*255.).round() as u32)<<24);
  }
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test] fn m25_combo_spells_have_unique_finite_broad_volume_without_clipping(){
  use std::collections::BTreeSet;
  let mut signatures=BTreeSet::new();
  for id in IDS {
   let tick=(0..104).find(|n|matches!(
    phase(id,*n as f64),Some((Stage::Cast,t)) if t>0.60
   )).expect("M25 stage missing") as f64;
   let mut s=Scene::new(500);
   render(Cast{kind:(id-1) as usize,variant:0,start:0,
      center:[0.,64.,0.],direction:[0.,0.,1.]},tick,&mut s);
   assert!((20..500).contains(&s.facets.len()),
      "M25 Combo empty/clipped for {id}: {}",s.facets.len());
   assert!(s.facets.iter().all(|(q,_)|
      q.iter().flatten().all(|v|v.is_finite())));
   let shape=s.facets.iter().flat_map(|(q,_)|
      q.iter().flat_map(|pt|pt.iter().map(|x|(*x*100.).round() as i64))
   ).collect::<Vec<_>>();
   assert!(signatures.insert(shape),"geometry clone in M25 Combo {id}");
  }
  assert_eq!(signatures.len(),10);
 }
}
