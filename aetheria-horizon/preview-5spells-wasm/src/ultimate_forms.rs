//! M21: Twenty DISTINCT original cinematic-ultimate base compositions.
//! This is first-pass CPU world-space geometry, not a finished HDR/camera
//! effect or live server/terrain action. Every one of the 20 registry Ultimates
//! has a manually selected sculpt, never a recolored generic cube/orb/ring.
use super::{Cast,Scene,add,sub,scale,shade_argb};
use crate::catalog200;
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{TAU,PI};
type P=[f64;3];
fn p(o:P,x:f64,y:f64,z:f64)->P{add(o,[x,y,z])}
fn mix(a:P,b:P,t:f64)->P{add(a,scale(sub(b,a),t))}
fn tint(c:u32,b:f64,a:f64)->u32{
  let x=shade_argb(c,b);
  (x&0x00FF_FFFF)|(((a.clamp(0.,1.)*255.).round() as u32)<<24)
}
fn body(s:&mut Scene,origin:P,half:P,c:u32){
  let pts=[
    p(origin,-half[0],-half[1],-half[2]),p(origin,half[0],-half[1],-half[2]),
    p(origin,half[0],half[1],-half[2]),p(origin,-half[0],half[1],-half[2]),
    p(origin,-half[0],-half[1],half[2]),p(origin,half[0],-half[1],half[2]),
    p(origin,half[0],half[1],half[2]),p(origin,-half[0],half[1],half[2])
  ];
  for (i,idx) in [[0,1,2,3],[4,7,6,5],[0,4,5,1],
                   [3,2,6,7],[1,5,6,2],[0,3,7,4]].iter().enumerate(){
    s.quad(idx.map(|j|pts[j]),tint(c,0.54+i as f64*0.066,0.89));
  }
}
fn crystal(s:&mut Scene,base:P,top:P,width:f64,spin:f64,c:u32){
  s.solid_crystal(base,top,width,spin,c,6);
}
fn wing(s:&mut Scene,o:P,angle:f64,spread:f64,h:f64,c:u32){
  let stem=p(o,0.,0.2,0.);
  let tip=p(o,spread*angle.cos(),h,spread*angle.sin());
  let middle=mix(stem,tip,0.53);
  let flank=[-angle.sin()*spread*0.24,0.2,angle.cos()*spread*0.24];
  s.membrane(stem,add(middle,flank),tip,sub(middle,flank),c);
}
fn floor_slab(s:&mut Scene,o:P,r:f64,depth:f64,spin:f64,c:u32){
  // Solid broken terrain segments, not a hollow ring.
  let n=8;
  for i in 0..n{
    let a=TAU*i as f64/n as f64+spin;
    let b=TAU*(i+1) as f64/n as f64+spin;
    let center=p(o,0.,-depth,0.);
    let low=p(center,0.,-0.23,0.);
    let aa=p(o,r*a.cos(),-0.08,r*a.sin());
    let bb=p(o,r*b.cos(),-0.08,r*b.sin());
    let lowa=p(low,r*a.cos()*0.94,0.,r*a.sin()*0.94);
    let lowb=p(low,r*b.cos()*0.94,0.,r*b.sin()*0.94);
    s.quad([center,aa,bb,center],tint(c,0.56+0.04*(i%3) as f64,0.81));
    s.quad([aa,lowa,lowb,bb],tint(c,0.49,0.93));
  }
}
fn orb(s:&mut Scene,o:P,rad:P,age:f64,c:u32,opacity:f64){
  let n=14usize;let m=9usize;
  let v=|i:usize,j:usize|{
    let theta=TAU*i as f64/n as f64+age*0.008;
    let phi=PI*j as f64/m as f64;
    p(o,rad[0]*phi.sin()*theta.cos(),
      rad[1]*phi.cos(),rad[2]*phi.sin()*theta.sin())
  };
  for j in 0..m{
    for i in 0..n{
      let ni=(i+1)%n;
      s.quad([v(i,j),v(ni,j),v(ni,j+1),v(i,j+1)],
        tint(c,0.59+0.23*((i*3+j)%7) as f64/7.,opacity));
    }
  }
}
fn curtain(s:&mut Scene,o:P,r:f64,height:f64,age:f64,phase:f64,c:u32){
  // Broad non-planar translucent atmosphere with spatial depth.
  let rings=8usize;let cols=10usize;
  let v=|j:usize,i:usize|{
    let t=j as f64/rings as f64;
    let u=i as f64/cols as f64*2.-1.;
    let a=phase+u*1.8+age*0.015+0.34*(t*TAU+age*0.031).sin();
    p(o,r*t*a.sin(),height*t,
       r*t*a.cos()+0.44*(u*TAU+age*0.034).sin())
  };
  for j in 0..rings{
    for i in 0..cols{
      s.quad([v(j,i),v(j,i+1),v(j+1,i+1),v(j+1,i)],
        tint(c,0.57+0.22*((j+i)%4) as f64/4.,0.30));
    }
  }
}
fn arch(s:&mut Scene,o:P,a:f64,r:f64,h:f64,c:u32){
  // Thick anchored stone arch, not an outlined circle.
  let n=5;
  for i in 0..n{
    let t=i as f64/n as f64;
    let u=(i+1) as f64/n as f64;
    let place=|v:f64|p(o,r*(2.*v-1.)*a.cos(),
               h*(1.-(2.*v-1.).abs().powf(1.6)),r*(2.*v-1.)*a.sin());
    let from=place(t);let to=place(u);
    let mid=mix(from,to,0.5);
    let dist=((from[0]-to[0]).powi(2)+(from[1]-to[1]).powi(2)
         +(from[2]-to[2]).powi(2)).sqrt();
    body(s,mid,[0.39+dist*0.12,dist*0.52,0.45],
        tint(c,0.76,0.90));
  }
}
fn shaped(s:&mut Scene,id:u16,o:P,age:f64,g:f64,c:u32) {
  let h=6.1*g;let r=4.4*g;
  match id {
    19=>{ // CATACLYSMIC CALDERA: layered 3D volcanic crater
      for tier in 0..4{
        let radius=r*(0.43+tier as f64*0.19);
        floor_slab(s,p(o,0.,tier as f64*0.6-1.3,0.),radius,0.5,
                   tier as f64*0.09,c);
      }
      for j in 0..9{
        let a=j as f64*TAU/9.0;
        crystal(s,p(o,r*0.5*a.cos(),-0.8,r*0.5*a.sin()),
          p(o,r*0.65*a.cos(),h*(0.68+(j%3) as f64*0.1),r*0.65*a.sin()),
          0.51*g,a,tint(c,0.85,0.89));
      }
      curtain(s,o,r*0.68,h,age,0.3,tint(c,0.86,0.49));
    },
    20=>{ // SUNHEART REBIRTH: wide unfolding phoenix with dense plume
      crystal(s,p(o,0.,-0.6,0.),p(o,0.,h*0.95,0.),1.1*g,0.,c);
      for side in [-1.,1.] {
        for i in 0..8{
          let a=(if side<0.{PI}else{0.})+i as f64*0.08*side;
          wing(s,p(o,side*0.35,1.2*g,0.),a,r*(0.6+i as f64*0.1),
               h*(0.64+i as f64*0.055),tint(c,0.67+0.03*i as f64,0.85));
        }
      }
      curtain(s,o,r*0.5,h,age,0.6,tint(c,0.72,0.22));
    },
    39=>{ // WHITE SILENCE: colossal broad ice vault, not 3 blank cubes
      orb(s,p(o,0.,h*0.52,0.),[r*0.83,h*0.53,r*0.83],age,c,0.48);
      for j in 0..12{
        let a=TAU*j as f64/12.;
        let foot=p(o,r*0.8*a.cos(),-0.5,r*0.8*a.sin());
        crystal(s,foot,p(o,r*0.44*a.cos(),h*0.85,r*0.44*a.sin()),
                0.48*g,a,tint(c,0.79,0.86));
      }
    },
    40=>{ // ABSOLUTE ZERO CATHEDRAL: arched spatial snow cathedral
      for i in 0..7{
        let a=TAU*i as f64/7.;
        arch(s,o,a,r*0.88,h*1.06,tint(c,0.82,0.9));
        let foot=p(o,r*0.73*a.cos(),-0.42,r*0.73*a.sin());
        crystal(s,foot,p(foot,0.,h,0.),0.53*g,a,tint(c,0.91,0.89));
      }
      crystal(s,p(o,0.,-0.8,0.),p(o,0.,h*1.36,0.),0.85*g,0.2,c);
    },
    59=>{ // WORLDBREATH: colossal wind sheets spin around core
      for i in 0..8 {curtain(s,o,r*(0.7+i as f64*0.045),
          h*(0.8+i as f64*0.045),age,TAU*i as f64/8.,
          tint(c,0.8,0.42));}
    },
    60=>{ // VACUUM REQUIEM: nested compressed spheroid with torn shells
      orb(s,p(o,0.,h*0.44,0.),[r*0.7,h*0.44,r*0.7],age,c,0.77);
      for i in 0..10{
        let a=TAU*i as f64/10.;
        crystal(s,p(o,r*a.cos(),h*0.3,r*a.sin()),
          p(o,r*0.35*a.cos(),h*0.52,r*0.35*a.sin()),
          0.30*g,a,tint(c,0.88,0.70));
      }
    },
    79=>{ // EARTHSHAKER THRONE: tiered enormous suspended stone platform
      for level in 0..5{
        let t=level as f64/5.;
        floor_slab(s,p(o,0.,h*t*0.7,0.),r*(0.78-0.13*t),0.38,t,c);
      }
      for i in 0..8{
        let a=TAU*i as f64/8.;
        body(s,p(o,r*a.cos(),h*0.38,r*a.sin()),
             [0.52*g,0.63*g,0.57*g],c);
      }
    },
    80=>{ // CATHEDRAL OF ROOTS: anchored massive interwoven rock arches
      floor_slab(s,o,r*0.83,0.45,0.1,c);
      for i in 0..9{
        let a=TAU*i as f64/9.;
        arch(s,o,a,r*(0.65+(i%3) as f64*0.08),
             h*(0.72+(i%4) as f64*0.06),tint(c,0.82,0.92));
      }
    },
    99=>{ // KING TIDE: tall cresting blue waves, real broad membranes
      for row in 0..6{
        let a=row as f64*0.36;
        curtain(s,p(o,(row as f64-2.5)*0.68,0.,0.),
            r*(0.78+0.1*a),h*(0.78+0.1*a),age,a,c);
      }
    },
    100=>{ // DEEPHEART: irregular coral reef sanctuary
      floor_slab(s,o,r*0.64,0.37,0.4,c);
      for i in 0..16{
        let a=TAU*i as f64/16.;
        let x=r*(0.33+(i%3) as f64*0.1)*a.cos();
        let z=r*(0.33+(i%3) as f64*0.1)*a.sin();
        crystal(s,p(o,x,-0.6,z),p(o,x+0.4*a.cos(),h*(0.65+(i%5) as f64*0.08),z+0.4*a.sin()),
                0.42*g,a,tint(c,0.65+0.02*(i%5) as f64,0.85));
      }
      orb(s,p(o,0.,h*0.55,0.),[r*0.52,h*0.44,r*0.52],age,c,0.19);
    },
    119=>{ // WORLD TREE: deep-rooted trunk and substantial leafy canopy
      crystal(s,p(o,0.,-1.2,0.),p(o,0.,h*1.18,0.),1.4*g,0.,c);
      for branch in 0..18{
        let a=TAU*branch as f64/18.;
        wing(s,p(o,0.,h*0.66,0.),a,r*(0.72+(branch%5) as f64*0.1),
             h*(0.8+(branch%4) as f64*0.1),tint(c,0.62,0.81));
      }
      for i in 0..10 {
        let a=TAU*i as f64/10.;
        crystal(s,p(o,r*0.78*a.cos(),-0.67,r*0.78*a.sin()),
                 p(o,r*0.22*a.cos(),h*0.5,r*0.22*a.sin()),
                 0.37*g,a,tint(c,0.59,0.9));
      }
    },
    120=>{ // GAIA EMBRACE: ancient spirit wrapped in living petals
      orb(s,p(o,0.,h*0.54,0.),[r*0.28,h*0.54,r*0.29],age,c,0.72);
      for i in 0..14{
        let a=TAU*i as f64/14.;
        wing(s,p(o,0.,h*0.18,0.),a,r*(0.75+(i%3) as f64*0.11),
             h*(0.8+(i%4) as f64*0.05),tint(c,0.62,0.84));
      }
    },
    139=>{ // JUDGEMENT: layered thunder canopy with solid branch mass
      crystal(s,p(o,0.,-0.9,0.),p(o,0.,h,0.),1.05*g,0.,c);
      for i in 0..15{
        let a=TAU*i as f64/15.;
        crystal(s,p(o,0.,h*0.45,0.),p(o,r*a.cos(),h*(0.71+(i%4) as f64*0.09),r*a.sin()),
           0.42*g,a,tint(c,0.87,0.92));
      }
      curtain(s,o,r*0.55,h,age,0.3,tint(c,0.84,0.23));
    },
    140=>{ // HEART OF THUNDER: enclosed rotating charged storm core
      orb(s,p(o,0.,h*0.53,0.),[r*0.67,h*0.49,r*0.67],age,c,0.60);
      for i in 0..12{
        let a=TAU*i as f64/12.+age*0.025;
        crystal(s,p(o,r*0.57*a.cos(),h*0.2,r*0.57*a.sin()),
          p(o,r*0.76*a.cos(),h*(0.8+(i%3) as f64*0.08),
                     r*0.76*a.sin()),0.35*g,a,tint(c,0.84,0.9));
      }
    },
    159=>{ // STARFALL LITURGY: visible staggered giant falling star stones
      for i in 0..12{
        let a=TAU*i as f64/12.+age*0.005;
        let dist=r*(0.4+(i%4) as f64*0.21);
        let at=p(o,dist*a.cos(),h*(0.53+(i%5) as f64*0.14),dist*a.sin());
        let low=p(at,-a.cos()*0.7,-1.6*g,-a.sin()*0.7);
        crystal(s,low,at,0.55*g,a,tint(c,0.7+0.02*(i%4) as f64,0.91));
        wing(s,low,a+0.6,1.3*g,2.2*g,tint(c,0.75,0.42));
      }
    },
    160=>{ // HOLLOW SKY: broken heavy lunar shell with actual dark center
      for i in 0..12 {
        let a=TAU*i as f64/12.;
        let b=TAU*(i+1) as f64/12.;
        let k=(i%4) as f64;
        let low=p(o,r*0.52*a.cos(),h*0.48,r*0.52*a.sin());
        let far=p(o,r*(0.9+0.025*k)*a.cos(),h*0.48,
                    r*(0.9+0.025*k)*a.sin());
        let far2=p(o,r*(0.9+0.025*k)*b.cos(),h*0.48,
                    r*(0.9+0.025*k)*b.sin());
        let near2=p(o,r*0.52*b.cos(),h*0.48,r*0.52*b.sin());
        s.quad([low,far,far2,near2],tint(c,0.43,0.84));
        crystal(s,far,p(far,0.3*a.cos(),0.5*g,0.3*a.sin()),0.26*g,a,
             tint(c,0.78,0.79));
      }
      // Folded depth below the lunar frame, kept thin enough for occlusion.
      curtain(s,o,r*0.44,h,age,0.45,tint(c,0.55,0.20));
    },
    179=>{ // THIRTEENTH HOUR: cathedral clock with thick frame & teeth
      body(s,p(o,0.,h*0.54,0.),[r*0.68,h*0.59,0.74*g],c);
      for i in 0..12 {
        let a=TAU*i as f64/12.;
        let x=r*0.59*a.cos();
        let y=h*0.57+r*0.59*a.sin();
        crystal(s,p(o,x,y,-0.63*g),p(o,x*1.1,y*1.04,-0.73*g),
            0.27*g,a,tint(c,0.83,0.92));
      }
      crystal(s,p(o,0.,h*0.57,-0.8*g),
        p(o,0.1*g,h*1.0,-0.9*g),0.15*g,age*0.013,tint(c,0.92,0.88));
    },
    180=>{ // EPOCH REVERSAL: inverted hourglass-waterfall of spatial ribbons
      orb(s,p(o,0.,h*0.61,0.),[r*0.47,h*0.49,r*0.47],age,c,0.37);
      for i in 0..9{
        curtain(s,p(o,0.,0.,0.),r*(0.25+(i%4) as f64*0.12),
                h*(0.72+(i%5) as f64*0.09),-age,TAU*i as f64/9.,
                tint(c,0.69,0.46));
      }
    },
    199=>{ // KINGDOM OF ASH: mausoleum with broad solid tomb arches
      floor_slab(s,o,r*0.88,0.5,0.2,c);
      for i in 0..8{
        let a=TAU*i as f64/8.;
        arch(s,o,a,r*(0.56+(i%3) as f64*0.1),h*(0.64+(i%4) as f64*0.08),
                tint(c,0.73,0.87));
      }
      curtain(s,o,r*0.5,h,age,0.2,tint(c,0.73,0.24));
    },
    200=>{ // LAST REQUIEM: large spectral guardian with solid mantle wings
      orb(s,p(o,0.,h*0.65,0.),[r*0.33,h*0.45,r*0.28],age,c,0.77);
      orb(s,p(o,0.,h*1.11,0.),[r*0.17,h*0.20,r*0.16],age,c,0.87);
      for side in [-1.,1.]{
        for i in 0..8{
          let a=if side<0.{PI}else{0.}+(i as f64*0.085*side);
          wing(s,p(o,0.,h*0.83,0.),a,
               r*(0.8+(i%4) as f64*0.06),
               h*(0.45+i as f64*0.075),
               tint(c,0.68+0.02*(i%3) as f64,0.74));
        }
      }
    },
    _=>{}
  }
}
pub fn render(cast:Cast,age:f64,scene:&mut Scene){
  if cast.kind>=200 || catalog200::rank(cast.kind)<18{return;}
  if !(0.0..104.0).contains(&age) || !age.is_finite(){return;}
  let spec=catalog200::spec(cast.kind);
  let timeline=match for_spell(spec.id){Some(x)=>x,None=>return};
  let mut t=age/104.0*timeline.total_ticks() as f64;
  let mut state=(Stage::Telegraph,0.0);
  for stage in timeline.phases {
    if t<stage.duration_ticks as f64{
      state=(stage.kind,(t/stage.duration_ticks as f64).clamp(0.,1.));
      break;
    }
    t-=stage.duration_ticks as f64;
  }
  let grow=match state.0{
    Stage::Telegraph=>0.10+0.2*state.1,
    Stage::Cast=>0.28+0.72*state.1,
    Stage::Impact=>1.0+0.12*(state.1*TAU).sin(),
    Stage::Afterglow=>(1.0-0.7*state.1).max(0.05),
  };
  let school=catalog200::school(cast.kind);
  let mut c=tint(catalog200::WORLD_COLORS[school],0.90,0.9);
  if state.0==Stage::Afterglow {
    c=tint(c,0.95,(1.0-state.1).powf(1.2));
  }
  shaped(scene,spec.id,cast.center,age,grow,c);
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn all_20_ultimates_have_distinct_dense_unscaled_source_derived_sculptures(){
    use std::collections::BTreeSet;
    let mut ids=BTreeSet::new();
    let mut geometries=BTreeSet::new();
    for school in 0..10{
      for local in [18,19]{
        let slot=school*20+local;
        let spec=catalog200::spec(slot);
        assert_eq!(spec.role,"Ultimate");
        let mut scene=Scene::new(700);
        render(Cast{kind:slot,variant:local as u8,start:0,
              center:[0.,64.,0.],direction:[0.,0.,1.]},58.,&mut scene);
        assert!(scene.facets.len()>=60,
           "ultimate {} did not have a real sculpt; {} faces",spec.id,scene.facets.len());
        assert!(scene.facets.len()<=700);
        assert!(scene.facets.iter().all(|(q,_)|
           q.iter().flatten().all(|n|n.is_finite())));
        let signature=scene.facets.iter().take(36).flat_map(|(q,_)|
          q.iter().flat_map(|v|v.iter().map(|x|(x*100.0) as i32))).collect::<Vec<_>>();
        assert!(geometries.insert(signature),"duplicate ultimate 3D sculpture id {}",spec.id);
        assert!(ids.insert(spec.id));
      }
    }
    assert_eq!(ids.len(),20);
 }
}
