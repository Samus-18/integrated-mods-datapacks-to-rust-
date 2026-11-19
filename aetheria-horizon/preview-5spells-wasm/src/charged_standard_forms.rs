//! M23B: ten MORE explicitly composed Standard spell visual drafts,
//! one Charged spell per class. No shared school default silhouettes.
//! The server DesignIntent intentionally still cannot cast real world effects.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,orb,curtain,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
const IDS:[u16;10]=[5,25,45,65,85,105,125,145,165,185];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let timeline=for_spell(id)?;
 let at=age/104.0*timeline.total_ticks() as f64;
 let mut elapsed=0.;
 for part in timeline.phases {
  let n=part.duration_ticks as f64;
  if at<elapsed+n{return Some((part.kind,((at-elapsed)/n).clamp(0.,1.)))}
  elapsed+=n;
 }
 None
}
fn shape(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
 let t=age*0.024;let r=2.7*g;
 match id{
 5=>{ // Ashen Veil: irregular charged draped smoke-cloak, not one gray cube.
   for j in 0..7{
    let a=j as f64*TAU/7.0+t*0.22;
    let x=a.cos()*1.32*g;
    let z=a.sin()*1.32*g;
    curtain(s,p(o,x,0.,z),r*0.68,3.9*g,age,a,
         tint(c,0.34+0.055*(j%5) as f64,0.19+0.025*(j%3) as f64));
   }
   for j in 0..9 {
    let a=(j as f64)*TAU/9.+t;
    let base=p(o,a.cos()*r*0.62,0.,a.sin()*r*0.62);
    let tip=p(base,0.2*a.sin(),(1.0+0.35*(j%4) as f64)*g,0.);
    crystal(s,base,tip,0.15*g,a,tint(c,0.28,0.59));
   }
  },
 25=>{ // Hail Carousel: actual lopsided rotating hail ASTEROIDS.
   for i in 0..12{
    let a=TAU*i as f64/12.+t*0.75;
    let radial=(1.0+0.33*(i%3) as f64)*g;
    let center=p(o,radial*a.cos(),(1.2+(i%4) as f64*0.53)*g,
                        radial*a.sin());
    let k=(0.28+0.12*(i%4) as f64)*g;
    orb(s,center,[k*1.15,k,k*0.83],age+i as f64,
        tint(c,0.79+0.02*(i%4) as f64,0.67),0.72);
   }
  },
 45=>{ // Vacuum Pin: convergent solid funnels with denser focal body.
   for i in 0..5{
    let a=TAU*i as f64/5.+t;
    curtain(s,p(o,0.,0.,0.),(1.3+0.19*i as f64)*g,3.6*g,
      age,a,tint(c,0.58+0.025*i as f64,0.27));
   }
   orb(s,p(o,0.,1.6*g,0.),[0.57*g,0.91*g,0.57*g],age,
         tint(c,0.88,0.79),0.75);
  },
 65=>{ // Shard Bulwark: paired interlocking gem petals of a thick barrier.
   for i in 0..9{
    let u=(i as f64-4.)/4.;
    let height=(2.6+0.8*(1.-u.abs()))*g;
    let root=p(o,u*2.5*g,-0.5,0.);
    crystal(s,root,p(root,0.20*u*g,height,-0.12*u*g),
        (0.48+0.2*(1.-u.abs()))*g,0.17*i as f64,tint(c,0.69,0.84));
    if i%2==0{
     wing(s,p(root,0.,0.8*g,0.),PI*0.5,
        0.8*g,2.1*g,tint(c,0.83,0.54));
    }
   }
  },
 85=>{ // Mistcloak: diffuse droplet-skin silhouette, NOT thin flat rings.
   for layer in 0..5{
    let a=TAU*layer as f64/5.;
    curtain(s,p(o,a.cos()*0.65*g,0.,a.sin()*0.65*g),
          1.6*g+0.24*layer as f64*g,3.6*g,
          age,a,tint(c,0.67,0.15+0.032*layer as f64));
   }
   for i in 0..13 {
    let a=TAU*i as f64/13.+t;
    let h=(0.5+(i%5) as f64*0.62)*g;
    let center=p(o,1.3*g*a.cos(),h,1.3*g*a.sin());
    orb(s,center,[0.21*g,0.31*g,0.20*g],
        age,tint(c,0.87,0.53),0.59);
   }
  },
 105=>{ // Firefly Veil: living motes inside a 3D verdant shield, not ring.
   for layer in 0..3 {
    curtain(s,p(o,(layer as f64-1.)*0.35*g,0.,0.),
          1.85*g,3.5*g,age,layer as f64*0.87,
          tint(c,0.52,0.24));
   }
   for i in 0..19{
    let a=TAU*i as f64/19.+t+(i%3) as f64*0.15;
    let h=(0.5+(i%7) as f64*0.45)*g;
    let r=(0.55+(i%5) as f64*0.19)*g;
    let center=p(o,r*a.cos(),h,r*a.sin());
    crystal(s,p(center,0.,-0.18*g,0.),
      p(center,0.,0.25*g,0.),0.14*g,a,
      tint(c,0.97,0.84));
   }
  },
 125=>{ // Storm Coil: three twisting 3D thick conducting CAGES with mass.
   for strand in 0..3{
    let mut prev=p(o,0.,0.1*g,0.);
    for i in 0..9{
     let u=(i+1) as f64/9.;
     let a=TAU*strand as f64/3.+TAU*u*1.65+t;
     let next=p(o,r*u.sqrt()*a.cos(),(0.25+3.2*u)*g,
                             r*u.sqrt()*a.sin());
     let mid=add(prev,scale(sub(next,prev),0.58));
     let lateral=[-a.sin()*0.50*g,0.,a.cos()*0.50*g];
     s.membrane(prev,add(mid,lateral),next,sub(mid,lateral),
         tint(c,0.72+0.03*strand as f64,0.73));
     prev=next;
    }
   }
   orb(s,p(o,0.,1.6*g,0.),[0.49*g,0.74*g,0.5*g],age,tint(c,0.92,0.70),0.67);
  },
 145=>{ // Rift Slice: deep thick broken crescent with face and back.
   for j in 0..10{
    let a=-0.95+j as f64*1.9/9.;
    let x=2.4*g*a.sin();
    let y=1.8*g+1.7*g*a.cos();
    let from=p(o,x,y,-0.32*g);
    let end=p(o,x,y,0.42*g);
    let wedge=p(o,x+0.35*g*a.cos(),y+0.24*g,-0.10*g);
    let inner=p(o,x-0.48*g*a.sin(),y-0.73*g*a.cos(),0.25*g);
    s.membrane(from,wedge,end,inner,
      tint(c,0.58+0.027*j as f64,0.81));
    crystal(s,from,end,0.23*g,a,tint(c,0.72,0.80));
   }
  },
 165=>{ // Temporal Mark: heavy hourglass carving + solid hour pips.
   body(s,p(o,0.,1.4*g,0.),[1.58*g,1.74*g,0.43*g],tint(c,0.57,0.85));
   for j in 0..11{
     let a=TAU*j as f64/11.;
     let point=p(o,1.12*g*a.cos(),1.4*g+1.12*g*a.sin(),0.53*g);
     body(s,point,[0.12*g,0.21*g,0.15*g],tint(c,0.85,0.94));
   }
   crystal(s,p(o,0.,0.55*g,0.66*g),p(o,0.,2.6*g,0.66*g),
           0.31*g,t,tint(c,0.92,0.79));
  },
 185=>{ // Rib Cage: thick interlocked bone ribs surrounding its protected center.
   for side in [-1.,1.]{
    for j in 0..6{
     let t=j as f64/5.;
     let base=p(o,side*(1.2+0.25*t)*g,-0.45*g,(j as f64-2.5)*0.65*g);
     let elbow=p(base,side*0.55*g,1.6*g,0.15*g);
     let high=p(o,side*0.17*g,(2.3+0.23*(j%3) as f64)*g,
                        (j as f64-2.5)*0.53*g);
     crystal(s,base,elbow,0.29*g,PI*0.3,tint(c,0.82,0.90));
     crystal(s,elbow,high,0.26*g,PI*0.7,tint(c,0.70,0.87));
    }
   }
   body(s,p(o,0.,-0.1*g,0.),[0.84*g,0.24*g,1.95*g],tint(c,0.52,0.82));
  },
  _=>unreachable!("dedicated charged Standard ID only"),
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((stage,progress))=phase(id,age)else{return;};
 let g=match stage{
  Stage::Telegraph=>0.14+0.20*progress,
  Stage::Cast=>0.33+0.67*progress,
  Stage::Impact=>1.04+0.09*progress,
  Stage::Afterglow=>(1.0-progress).max(0.07)
 };
 let base=catalog200::WORLD_COLORS[catalog200::school(cast.kind)];
 let before=s.facets.len();
 shape(s,id,cast.center,age,g,base);
 if stage==Stage::Afterglow{
  for (_,col) in &mut s.facets[before..]{
   let a=((*col>>24)&255) as f64/255.;
   *col=(*col&0x00FFFFFF)|
     (((a*(1.-progress).powf(0.7)*255.).round() as u32)<<24);
  }
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn ten_charged_standard_shapes_are_distinct_finite_and_nonempty(){
  let mut signatures=std::collections::BTreeSet::new();
  for (school,id) in IDS.into_iter().enumerate(){
   assert_eq!((id-1)/20,school as u16);
   let slot=(id-1) as usize;
   let age=(0..104).find(|t|matches!(
    phase(id,*t as f64),Some((Stage::Cast,p)) if p>0.55
   )).unwrap() as f64;
   let mut scene=Scene::new(500);
   render(Cast{kind:slot,variant:4,start:0,center:[0.,64.,0.],
            direction:[0.,0.,1.]},age,&mut scene);
   assert!(scene.facets.len()>=16,"invisible spell {id}");
   assert!(scene.facets.len()<=500);
   assert!(scene.facets.iter().all(|(q,_)|
      q.iter().flatten().all(|v|v.is_finite())));
   let shape=scene.facets.iter().flat_map(|(q,_)|
      q.iter().flat_map(|v|v.iter().map(|w|(*w*100.).round() as i64))
   ).collect::<Vec<_>>();
   assert!(signatures.insert(shape),"duplicate geometry {id}");
  }
  assert_eq!(signatures.len(),10);
 }
}
