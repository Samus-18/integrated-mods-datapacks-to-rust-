//! M26 — twenty individually choreographed standard fantasy scenes (local 10–11).
//! First-pass world-space geometry, NOT polished HDR nor implemented gameplay.
//! Their canonical cast actions remain DesignIntent pending world authority.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,orb,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
const IDS:[u16;20]=[10,11,30,31,50,51,70,71,90,91,110,111,130,131,150,151,170,171,190,191];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let timeline=for_spell(id)?;
 let clock=age/104.0*timeline.total_ticks() as f64;
 let mut prev=0.0;
 for stage in timeline.phases{
  let end=prev+stage.duration_ticks as f64;
  if clock<end{return Some((stage.kind,((clock-prev)/stage.duration_ticks as f64).clamp(0.,1.)))}
  prev=end;
 }
 None
}
fn scene(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
 let t=age*0.024;let radius=2.6*g;
 match id{
 10=>{ // SEARING JAVELIN: substantial pierced magma spear + ember fins.
  let root=p(o,-2.5*g,1.85*g,-0.25*g);
  let tip=p(o,3.8*g,1.85*g,0.5*g);
  crystal(s,root,tip,0.52*g,t,tint(c,0.92,0.95));
  for i in 0..7{
   let u=i as f64/7.;
   let anchor=p(o,(-2.0+5.5*u)*g,1.85*g,(0.1+0.4*u)*g);
   wing(s,anchor,PI*0.45+(i%2) as f64*PI,
        (0.78+0.13*(i%3) as f64)*g,(0.75+0.17*(i%4) as f64)*g,
        tint(c,0.69+0.03*(i%5) as f64,0.76));
  }
  },
 11=>{ // KINDLE CHAIN: 6 broad linked coal hooks with real hooked claws.
  for i in 0..6{
   let a=TAU*i as f64/6.+t*0.34;
   let x=2.0*g*a.cos();let z=2.0*g*a.sin();
   let base=p(o,x,0.7*g,z);
   let point=p(o,1.4*g*a.cos(),(2.1+(i%3) as f64*0.36)*g,1.4*g*a.sin());
   crystal(s,base,point,0.29*g,a,tint(c,0.56,0.95));
   crystal(s,point,p(point,-0.65*g*a.sin(),0.8*g,0.65*g*a.cos()),
       0.30*g,a+0.7,tint(c,0.92,0.84));
   let middle=p(o,0.,1.3*g,0.);
   s.membrane(base,p(base,0.,0.25*g,0.),middle,
         p(middle,0.,-0.28*g,0.),tint(c,0.75,0.55));
  }
  },
 30=>{ // SNOWBLIND: three layered blizzard bodies, no thin empty hoops.
   for i in 0..3{
    let a=i as f64*TAU/3.+t;
    curtain(s,p(o,0.,0.,0.),(1.6+0.44*i as f64)*g,
       (3.2+0.22*i as f64)*g,age,a,tint(c,0.77,0.22));
   }
   for i in 0..11{
    let a=TAU*i as f64/11.+t;
    let at=p(o,radius*a.cos(),(0.9+0.35*(i%5) as f64)*g,radius*a.sin());
    crystal(s,p(at,0.,-0.4*g,0.),p(at,0.26*g*a.sin(),0.4*g,0.),
       0.22*g,a,tint(c,0.97,0.81));
   }
  },
 31=>{ // FROZEN TETHER: actual jagged ice-link segments suspended in a bridge.
   for i in 0..10 {
    let u=i as f64/9.;
    let middle=p(o,(u-0.5)*6.0*g,
        (1.3+0.32*(u*TAU*2.+t).sin())*g,
        (u*TAU+t).sin()*0.55*g);
    let end=p(middle,0.53*g,(0.4+0.14*(i%3) as f64)*g,0.22*g);
    crystal(s,middle,end,0.35*g,t+u*TAU,tint(c,0.70,0.92));
    wing(s,middle,t+u*PI,0.47*g,0.54*g,tint(c,0.91,0.65));
   }
  },
 50=>{ // THUNDERHEAD DRAFT: solid rain-cloud heavy underside and moving cap.
   for i in 0..9{
    let a=TAU*i as f64/9.;
    let at=p(o,(1.1+0.35*(i%3) as f64)*g*a.cos(),
       (2.9+0.27*(i%3) as f64)*g,
       (0.9+0.19*(i%4) as f64)*g*a.sin());
    body(s,at,[0.64*g,0.44*g,0.54*g],tint(c,0.54+0.02*i as f64,0.63));
   }
   for i in 0..10{
    let a=TAU*i as f64/10.;
    let at=p(o,(1.2+0.12*(i%3) as f64)*g*a.cos(),
       1.7*g,(1.1+0.14*(i%3) as f64)*g*a.sin());
    crystal(s,at,p(at,0.,-0.7*g,0.),0.13*g,a,tint(c,0.89,0.57));
   }
  },
 51=>{ // SUCTION LATTICE: three pressure panels converging into physical focus.
   let apex=p(o,0.,2.75*g,0.);
   for i in 0..9{
    let a=TAU*i as f64/9.+t*0.17;
    let b=TAU*(i+1) as f64/9.+t*0.17;
    let edge=p(o,2.2*g*a.cos(),0.15*g,2.2*g*a.sin());
    let edge2=p(o,2.2*g*b.cos(),0.15*g,2.2*g*b.sin());
    s.membrane(edge,edge2,apex,p(o,0.,1.05*g,0.),
         tint(c,0.60+0.028*(i%4) as f64,0.32));
   }
   crystal(s,p(o,0.,0.35*g,0.),apex,0.64*g,t,tint(c,0.89,0.87));
  },
 70=>{ // OBSIDIAN NEEDLE: colossal *closed* chipped monolith with black face.
   let base=p(o,0.,-0.65*g,0.);
   let tip=p(o,0.65*g,5.6*g,-0.16*g);
   crystal(s,base,tip,0.94*g,0.16,tint(c,0.36,0.98));
   for i in 0..5{
    let a=TAU*i as f64/5.;
    let at=p(o,0.71*g*a.cos(),0.15*g,0.71*g*a.sin());
    crystal(s,at,p(at,0.36*a.cos()*g,1.7*g,0.36*a.sin()*g),
       0.39*g,a,tint(c,0.64,0.90));
   }
  },
 71=>{ // TREMOR ECHO: three contiguous ground ruptures, offset delayed blocks.
   for layer in 0..3{
    for i in 0..7{
     let u=i as f64/6.;
     let x=(u-0.5)*7.3*g;
     let z=(layer as f64-1.)*0.96*g+
            0.42*g*(u*TAU+t*layer as f64).sin();
     let h=(0.23+0.26*(i%4) as f64)*g;
     body(s,p(o,x,-0.52*g+h*0.30,z),
        [0.48*g,0.21*g+h*0.30,0.49*g],
        tint(c,0.53+0.04*(layer%3) as f64,0.93));
    }
   }
  },
 90=>{ // WHIRLPOOL: three deep SCULPTED liquid crests, not outline rings.
   for i in 0..5{
     let a=TAU*i as f64/5.+t;
     let at=p(o,2.2*g*a.cos(),0.16*g,2.2*g*a.sin());
     curtain(s,at,1.4*g,(1.4+0.25*(i%3) as f64)*g,
          age,a,tint(c,0.74,0.47));
     wing(s,at,a+PI*0.38,0.89*g,0.72*g,tint(c,0.91,0.74));
   }
  },
 91=>{ // BRINE MARK: crystalline salt rosettes coating a water-stained plinth.
   floor_slab(s,o,2.1*g,0.30*g,t,tint(c,0.63,0.67));
   for i in 0..12{
    let a=TAU*i as f64/12.;
    let at=p(o,1.2*g*a.cos(),0.13*g,1.2*g*a.sin());
    crystal(s,at,p(at,0.35*g*a.cos(),(0.65+0.08*(i%3) as f64)*g,
                   0.35*g*a.sin()),
       (0.18+0.06*(i%4) as f64)*g,a,tint(c,0.90,0.83));
   }
   body(s,p(o,0.,-0.02*g,0.),[0.39*g,0.17*g,0.43*g],tint(c,0.85,0.76));
  },
 110=>{ // MUSHROOM LANTERN: articulated stalk + thick luminescent fungus crown.
   body(s,p(o,0.,1.3*g,0.),[0.40*g,1.36*g,0.41*g],tint(c,0.70,0.89));
   for i in 0..11{
    let a=TAU*i as f64/11.;
    let at=p(o,1.34*g*a.cos(),2.45*g,1.34*g*a.sin());
    wing(s,at,a+PI,(1.2+0.14*(i%3) as f64)*g,
       0.68*g,tint(c,0.87,0.91));
   }
   for i in 0..7{
    let a=TAU*i as f64/7.;
    let at=p(o,1.05*g*a.cos(),1.47*g,1.05*g*a.sin());
    crystal(s,at,p(at,0.,0.66*g,0.),0.23*g,a,tint(c,0.97,0.73));
   }
  },
 111=>{ // SAPLING CHAIN: branches connect five actual miniature grown trees.
   for i in 0..5{
    let a=TAU*i as f64/5.;
    let at=p(o,2.0*g*a.cos(),-0.36*g,2.0*g*a.sin());
    crystal(s,at,p(at,0.,2.3*g,0.),0.42*g,a,tint(c,0.57,0.88));
    for j in 0..3{
     wing(s,p(at,0.,(0.9+0.3*j as f64)*g,0.),
         a+0.8*j as f64,0.9*g,0.7*g,tint(c,0.84,0.73));
    }
    s.membrane(at,p(at,0.,0.22*g,0.),
       p(o,0.,0.15*g,0.),p(o,0.,-0.13*g,0.),
       tint(c,0.64,0.62));
   }
  },
 130=>{ // BOLT WEAVER: zigzag woven electrified 3D ground cracks.
   let mut prev=p(o,-3.2*g,-0.30*g,-1.2*g);
   for i in 0..12{
    let u=i as f64/11.;
    let next=p(o,(u*6.4-3.2)*g,-0.16*g,
       (if i%2==0{-0.9}else{0.9})*g);
    let side=p(next,0.,0.32*g,(if i%2==0{0.32}else{-0.32})*g);
    s.membrane(prev,side,next,p(next,0.,-0.25*g,0.),
       tint(c,0.78,0.75));
    if i%3==0{
      crystal(s,next,p(next,0.,0.96*g,0.),0.21*g,u,tint(c,0.96,0.91));
    }
    prev=next;
   }
  },
 131=>{ // CONDUCTIVE SEAL: carved copper octagon enclosing a heavy obelisk.
   floor_slab(s,o,2.2*g,0.37*g,t,tint(c,0.59,0.89));
   for i in 0..8{
    let a=TAU*i as f64/8.;
    let at=p(o,1.70*g*a.cos(),0.16*g,1.70*g*a.sin());
    body(s,at,[0.25*g,0.34*g,0.28*g],tint(c,0.65,0.85));
   }
   crystal(s,p(o,0.,-0.28*g,0.),p(o,0.,3.15*g,0.),
      0.66*g,t,tint(c,0.91,0.97));
  },
 150=>{ // ORBITING MOONS: 3 asymmetric closed lunar bodies, crater facets.
   for i in 0..3{
    let a=TAU*i as f64/3.+t*0.29;
    let at=p(o,2.18*g*a.cos(),(2.0+0.34*i as f64)*g,
        2.18*g*a.sin());
    orb(s,at,[(0.68+0.10*i as f64)*g,0.71*g,0.68*g],
        age+i as f64*4.,tint(c,0.67+0.04*i as f64,0.82),0.79);
    crystal(s,p(at,-0.18*g,-0.2*g,0.3*g),
       p(at,0.30*g,0.42*g,0.22*g),0.22*g,a,tint(c,0.92,0.78));
   }
  },
 151=>{ // CONSTELLATION SNARE: 7 physical star shards connected by cloth planes.
   for i in 0..7{
    let a=TAU*i as f64/7.+t*0.18;
    let at=p(o,1.75*g*a.cos(),(1.55+0.43*(i%3) as f64)*g,
       1.75*g*a.sin());
    crystal(s,p(at,0.,-0.35*g,0.),p(at,0.,0.52*g,0.),
      0.29*g,a,tint(c,0.92,0.93));
    s.membrane(p(o,0.,1.1*g,0.),p(at,0.,0.28*g,0.),at,
       p(at,0.,-0.31*g,0.),tint(c,0.71,0.36));
   }
  },
 170=>{ // REWIND STITCH: broad reverse-flow golden fabric, no clock outline.
   for k in 0..5{
    let mut prev=p(o,-2.7*g,0.4*g,(k as f64-2.0)*0.5*g);
    for i in 0..7{
     let u=(i+1) as f64/7.;
     let next=p(o,(u*5.4-2.7)*g,
       (1.7+0.7*(u*TAU+t).sin())*g,
       ((k as f64-2.)*0.55+0.39*(u*TAU+t).cos())*g);
     let mid=add(prev,scale(sub(next,prev),0.5));
     s.membrane(prev,p(mid,0.,0.31*g,0.),next,
         p(mid,0.,-0.31*g,0.),tint(c,0.73+0.03*k as f64,0.58));
     prev=next;
    }
   }
  },
 171=>{ // TIMEGLASS SNARE: frozen arrow-shards inside two deep time funnels.
   for i in 0..7{
     let a=TAU*i as f64/7.;
     let at=p(o,1.3*g*a.cos(),1.25*g,1.3*g*a.sin());
     crystal(s,at,p(o,0.,3.6*g,0.),0.33*g,a,tint(c,0.66,0.65));
     crystal(s,p(o,0.,3.6*g,0.),
       p(o,1.1*g*a.cos(),5.4*g,1.1*g*a.sin()),
       0.24*g,a,tint(c,0.89,0.72));
   }
   for i in 0..6{
    let a=TAU*i as f64/6.+t;
    let at=p(o,0.89*g*a.cos(),2.8*g,0.89*g*a.sin());
    crystal(s,p(at,-0.33*g,0.,0.),p(at,0.46*g,0.,0.),
       0.20*g,a,tint(c,0.95,0.84));
   }
  },
 190=>{ // CRIMSON POOL: dark 3D blood-crystal reservoir & submerged wedges.
   floor_slab(s,o,2.1*g,0.22*g,t,tint(c,0.54,0.72));
   for i in 0..8{
    let a=TAU*i as f64/8.;
    let at=p(o,1.2*g*a.cos(),-0.25*g,1.2*g*a.sin());
    s.membrane(p(o,0.,-0.08*g,0.),at,
       p(at,0.,0.31*g,0.),p(o,0.,0.19*g,0.),
       tint(c,0.80,0.54));
    crystal(s,at,p(at,0.,(0.48+0.08*(i%3) as f64)*g,0.),
       0.22*g,a,tint(c,0.87,0.83));
   }
  },
 191=>{ // SPIRIT BELL: full tapering spectral bronze bell with clapper.
   for i in 0..10{
    let a=TAU*i as f64/10.;let b=TAU*(i+1) as f64/10.;
    let top=p(o,0.61*g*a.cos(),3.18*g,0.61*g*a.sin());
    let top2=p(o,0.61*g*b.cos(),3.18*g,0.61*g*b.sin());
    let bottom=p(o,1.54*g*a.cos(),0.83*g,1.54*g*a.sin());
    let bottom2=p(o,1.54*g*b.cos(),0.83*g,1.54*g*b.sin());
    s.quad([bottom,bottom2,top2,top],tint(c,0.54+0.03*(i%5) as f64,0.62));
    crystal(s,bottom,top,0.15*g,a,tint(c,0.80,0.76));
   }
   crystal(s,p(o,0.,0.29*g,0.),p(o,0.,2.11*g,0.),
       0.33*g,t,tint(c,0.98,0.84));
   body(s,p(o,0.,3.35*g,0.),[0.41*g,0.25*g,0.42*g],tint(c,0.87,0.93));
  },
  _=>unreachable!("M26 source-grounded 20 Standard signatures"),
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((stage,progress))=phase(id,age)else{return;};
 let g=match stage{
  Stage::Telegraph=>0.15+progress*0.21,
  Stage::Cast=>0.35+progress*0.65,
  Stage::Impact=>1.02+progress*0.09,
  Stage::Afterglow=>(1.-progress).max(0.07)
 };
 let old=s.facets.len();
 let c=catalog200::WORLD_COLORS[catalog200::school(cast.kind)];
 scene(s,id,cast.center,age,g,c);
 if stage==Stage::Afterglow{
  for (_,col) in &mut s.facets[old..] {
   let a=((*col>>24)&255) as f64/255.;
   *col=(*col&0x00FFFFFF)|
    (((a*(1.-progress).powf(0.75)*255.).round() as u32)<<24);
  }
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn twenty_named_scenes_have_full_finite_distinct_geometry_without_clipping(){
  let mut forms=std::collections::HashSet::new();
  for (n,id) in IDS.iter().enumerate(){
   assert_eq!((*id-1)/20,n as u16/2);
   let slot=(*id-1) as usize;
   let age=(0..104).find(|t|matches!(
    phase(*id,*t as f64),Some((Stage::Cast,p)) if p>0.50
   )).unwrap() as f64;
   let mut s=Scene::new(500);
   render(Cast{kind:slot,variant:4,start:0,center:[0.,64.,0.],
       direction:[0.,0.,1.]},age,&mut s);
   assert!(s.facets.len()>12,"insufficient shape for {id}");
   assert!(s.facets.len()<500,"truncated at 500 quads for {id}");
   assert!(s.facets.iter().all(|(q,_)|q.iter().flatten().all(|v|v.is_finite())));
   let geo=s.facets.iter().flat_map(|(q,_)|
      q.iter().flat_map(|v|v.iter().map(|x|(*x*100.).round() as i64))
   ).collect::<Vec<_>>();
   assert!(forms.insert(geo),"geometry clone not allowed for {id}");
  }
  assert_eq!(forms.len(),20);
 }
}
