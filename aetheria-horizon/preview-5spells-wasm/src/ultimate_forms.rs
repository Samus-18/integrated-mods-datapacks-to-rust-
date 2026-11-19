//! M21: Twenty DISTINCT original cinematic-ultimate base compositions.
//! This is first-pass CPU world-space geometry, not a finished HDR/camera
//! effect or live server/terrain action. Every one of the 20 registry Ultimates
//! has a manually selected sculpt, never a recolored generic cube/orb/ring.
use super::{Cast,Scene,add,sub,scale,shade_argb};
use crate::catalog200;
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{TAU,PI};
type P=[f64;3];
pub(super) fn p(o:P,x:f64,y:f64,z:f64)->P{add(o,[x,y,z])}
pub(super) fn mix(a:P,b:P,t:f64)->P{add(a,scale(sub(b,a),t))}
pub(super) fn tint(c:u32,b:f64,a:f64)->u32{
  let x=shade_argb(c,b);
  (x&0x00FF_FFFF)|(((a.clamp(0.,1.)*255.).round() as u32)<<24)
}
pub(super) fn body(s:&mut Scene,origin:P,half:P,c:u32){
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
pub(super) fn crystal(s:&mut Scene,base:P,top:P,width:f64,spin:f64,c:u32){
  s.solid_crystal(base,top,width,spin,c,6);
}
pub(super) fn wing(s:&mut Scene,o:P,angle:f64,spread:f64,h:f64,c:u32){
  let stem=p(o,0.,0.2,0.);
  let tip=p(o,spread*angle.cos(),h,spread*angle.sin());
  let middle=mix(stem,tip,0.53);
  let flank=[-angle.sin()*spread*0.24,0.2,angle.cos()*spread*0.24];
  s.membrane(stem,add(middle,flank),tip,sub(middle,flank),c);
}
pub(super) fn floor_slab(s:&mut Scene,o:P,r:f64,depth:f64,spin:f64,c:u32){
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
pub(super) fn orb(s:&mut Scene,o:P,rad:P,age:f64,c:u32,opacity:f64){
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
pub(super) fn curtain(s:&mut Scene,o:P,r:f64,height:f64,age:f64,phase:f64,c:u32){
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
pub(super) fn arch(s:&mut Scene,o:P,a:f64,r:f64,h:f64,c:u32){
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
    79=>{ // EARTHSHAKER THRONE: a suspended, deliberately carved
      // stone throne with actual seat/backrest and drifting rocky crown.
      floor_slab(s,p(o,0.,h*0.29,0.),r*0.76,0.48,0.12,c);
      body(s,p(o,0.,h*0.44,0.29*g),
             [r*0.45,0.20*g,r*0.35],tint(c,0.68,0.91));
      body(s,p(o,0.,h*0.70,r*0.20),
             [r*0.40,h*0.37,0.28*g],tint(c,0.76,0.94));
      for side in [-1.,1.]{
        body(s,p(o,side*r*0.49,h*0.51,0.15*g),
             [0.33*g,h*0.30,0.49*g],tint(c,0.66,0.91));
        crystal(s,p(o,side*r*0.42,h*0.87,r*0.20),
          p(o,side*r*0.57,h*1.13,r*0.20),
          0.35*g,0.22*side,tint(c,0.91,0.90));
      }
      // Rockfall suspended in a broken asymmetrical spiral, not a drum.
      for i in 0..10 {
        let a=TAU*i as f64/10.+age*0.004;
        let dist=r*(0.52+(i%3) as f64*0.17);
        let y=h*(0.30+(i%5) as f64*0.12);
        crystal(s,p(o,dist*a.cos(),y-0.4*g,dist*a.sin()),
           p(o,dist*a.cos()+0.12*g,y+0.4*g,dist*a.sin()),
           (0.29+0.05*(i%3) as f64)*g,a,tint(c,0.54+0.05*(i%4) as f64,0.93));
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
    120=>{ // GAIA EMBRACE: a protective ancestral forest spirit
      // with head, shoulders, mantle, rooted limbs, and living leaves.
      // Distinguish it from the solid trunk and spire of World Tree.
      let torso=p(o,0.,h*0.48,0.);
      orb(s,torso,[r*0.31,h*0.35,r*0.24],
          age,tint(c,0.49,0.87),0.74);
      let head=p(o,0.,h*0.92,-0.13*g);
      orb(s,head,[r*0.15,h*0.16,r*0.15],
          age,tint(c,0.86,0.94),0.88);
      for side in [-1.,1.]{
        // Thick rooted forearms curve around the protected area.
        let shoulder=p(torso,side*r*0.30,h*0.12,0.);
        let hand=p(o,side*r*0.77,h*0.16,-0.8*g);
        crystal(s,shoulder,hand,0.37*g,side*0.45,tint(c,0.58,0.92));
        let antler=p(o,side*r*0.42,h*1.21,0.);
        crystal(s,p(head,side*0.29*g,0.16*g,0.),antler,
             0.16*g,side*1.1,tint(c,0.84,0.88));
      }
      // Seven broad guardian-cloak leaves, deliberately pointed DOWN,
      // so the silhouette is not another upward spiky plant.
      for i in 0..7 {
        let a=TAU*i as f64/7.;
        let collar=p(torso,0.,h*0.21,0.);
        let tip=p(o,r*0.85*a.cos(),-0.35*h,r*0.85*a.sin());
        let center=mix(collar,tip,0.50);
        let flank=[-a.sin()*0.43*r,0.,a.cos()*0.43*r];
        s.membrane(collar,add(center,flank),tip,sub(center,flank),
             tint(c,0.52+0.045*(i%3) as f64,0.61));
      }
      floor_slab(s,o,r*0.53,0.25,age*0.003,tint(c,0.43,0.52));
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
    160=>{ // HOLLOW SKY: DEEP fragmented crescent-moon portal.
      // No clean uniform ring; jagged irregular lunar slabs are angled
      // and physically thick, with dark spatial negative center.
      let n=15usize;
      let center=p(o,0.,h*0.55,0.);
      let rad=r*0.83;
      for i in 0..n {
        let a=TAU*i as f64/n as f64+age*0.003;
        let b=TAU*(i+1) as f64/n as f64+age*0.003;
        let inner=rad*(0.41+0.08*(i%3) as f64);
        let outer=rad*(0.89+0.065*(i%4) as f64);
        let x1=p(center,inner*a.cos(),inner*a.sin(),-0.37*g);
        let x2=p(center,outer*a.cos(),outer*a.sin(),-0.37*g);
        let x3=p(center,outer*b.cos(),outer*b.sin(),-0.37*g);
        let x4=p(center,inner*b.cos(),inner*b.sin(),-0.37*g);
        let depth=(0.37+0.19*(i%3) as f64)*g;
        let back=[p(x1,0.,0.,depth),p(x2,0.,0.,depth),
                  p(x3,0.,0.,depth),p(x4,0.,0.,depth)];
        s.quad([x1,x2,x3,x4],tint(c,0.47+0.08*(i%4) as f64,0.86));
        s.quad([back[3],back[2],back[1],back[0]],
            tint(c,0.35,0.79));
        s.quad([x2,back[1],back[2],x3],tint(c,0.58,0.81));
        s.quad([x4,x3,back[2],back[3]],tint(c,0.43,0.83));
        // Broken oblique moon fragments tilt OUT of the frame.
        if i%2==0 {
          let root=p(center,outer*a.cos(),outer*a.sin(),-0.2*g);
          let tip=p(root,0.36*r*a.cos(),0.22*r*a.sin(),
                    (if i%4==0 {-0.62}else{0.62})*g);
          crystal(s,root,tip,0.28*g,a,tint(c,0.69,0.84));
        }
      }
      // Dim, original dimensional fog behind the Moon aperture.
      for j in 0..2 {
        curtain(s,p(center,0.,-h*0.38,0.),r*(0.35+j as f64*0.09),
             h*0.65,age,j as f64*PI,tint(c,0.41,0.12));
      }
    },
    179=>{ // THIRTEENTH HOUR: a cathedral-scale 13-hour time engine.
      // An actual deep, carved clock mechanism + nave, not a rectangular
      // plaque. 13 carved monumental teeth, forward clock hands and a
      // warm recess. Unlike wire-ring magic, this has thick solid walls.
      let center=p(o,0.,h*0.56,0.);
      let n=13usize;
      let spin=age*0.007;
      let face_z=-0.87*g;
      let back_z=0.47*g;
      for i in 0..n {
        let a=TAU*i as f64/n as f64-PI*0.5;
        let b=TAU*(i+1) as f64/n as f64-PI*0.5;
        let radial=|rad:f64,ang:f64,z:f64|
            p(center,rad*ang.cos(),rad*ang.sin(),z);
        let inner=r*0.56;
        let outer=r*0.82;
        let f0=radial(inner,a,face_z);
        let f1=radial(outer,a,face_z);
        let f2=radial(outer,b,face_z);
        let f3=radial(inner,b,face_z);
        let b0=radial(inner,a,back_z);
        let b1=radial(outer,a,back_z);
        let b2=radial(outer,b,back_z);
        let b3=radial(inner,b,back_z);
        s.quad([f0,f1,f2,f3],tint(c,0.76+0.13*(i%4) as f64/3.,0.96));
        s.quad([b3,b2,b1,b0],tint(c,0.53,0.91));
        s.quad([f1,b1,b2,f2],tint(c,0.66,0.94));
        s.quad([f3,f2,b2,b3],tint(c,0.49,0.91));
        let at=radial(r*0.75,(a+b)*0.5,face_z-0.22*g);
        let at2=radial(r*0.86,(a+b)*0.5,face_z-0.25*g);
        crystal(s,at,at2,0.19*g,a,tint(c,0.95,0.91));
      }
      // Broad readable golden pointers form a 3D crossing shape
      // suspended over the genuinely recessed clock face.
      for hand in 0..2 {
        let a=spin+(if hand==0{0.84}else{-1.19});
        let len=r*(if hand==0{0.65}else{0.44});
        let root=p(center,0.,0.,face_z-0.40*g);
        let tip=p(root,len*a.cos(),len*a.sin(),-0.06*g);
        let left=p(root,-0.19*g*a.sin(),0.19*g*a.cos(),0.);
        let right=p(root,0.19*g*a.sin(),-0.19*g*a.cos(),0.);
        s.membrane(root,left,tip,right,tint(c,0.98,0.97));
      }
      orb(s,p(center,0.,0.,0.13*g),
          [r*0.52,r*0.52,0.24*g],age,tint(c,0.29,0.64),0.24);
      // Three distinct cathedral buttresses anchor its mass.
      for j in -1..=1 {
        let x=j as f64*r*0.77;
        body(s,p(o,x,h*0.25,0.7*g),
             [0.29*g,h*0.26,0.34*g],tint(c,0.72,0.92));
        crystal(s,p(o,x,h*0.49,0.7*g),
          p(o,x,h*(0.61+(j==0) as u8 as f64*0.18),0.7*g),
          0.21*g,j as f64,tint(c,0.87,0.88));
      }
    },
    180=>{ // EPOCH REVERSAL: a fractured inverse timefall sandglass
      // with two asymmetric faceted chambers, visible narrow throat,
      // and a braided vertical current flowing *upward*. No closed egg.
      let segments=12usize;
      let layers=7usize;
      let center=p(o,0.,h*0.52,0.);
      for chamber in 0..2 {
        for j in 0..layers {
          let lo=j as f64/layers as f64;
          let hi=(j+1) as f64/layers as f64;
          let point=|t:f64,i:usize|{
            let a=TAU*i as f64/segments as f64+age*0.008+
                0.08*(t*TAU+chamber as f64).sin();
            let y=if chamber==0 {
               -h*(0.51-t*0.51)
            }else{
               h*t*0.52
            };
            let wide=0.32+1.62*t.powf(0.87);
            let warp=1.0+0.13*(a*3.0+t*7.0+age*0.029).sin();
            p(center,wide*g*warp*a.cos(),y,wide*g*warp*a.sin())
          };
          for i in 0..segments {
            let next=(i+1)%segments;
            s.quad([point(lo,i),point(lo,next),
                    point(hi,next),point(hi,i)],
                tint(c,0.44+0.20*((i+j+chamber)%5) as f64/4.,
                     0.35+0.20*chamber as f64));
          }
        }
      }
      // Solid, floating sandglass bottom and crown, not unsupported rods.
      for layer in [0.,1.]{
        let y=if layer==0. {-0.02*h}else{h};
        for i in 0..8 {
          let a=TAU*i as f64/8.;
          let rad=r*0.27;
          crystal(s,p(o,rad*a.cos(),y-0.17*g,rad*a.sin()),
              p(o,rad*a.cos(),y+0.4*g,rad*a.sin()),
              0.22*g,a,tint(c,0.79,0.83));
        }
      }
      // High-budget flowing surface curtains are the time-reversal
      // waterfall. Each sheet climbs in the opposite direction of gravity.
      for j in 0..4{
        curtain(s,p(o,0.,-0.21*h,0.),r*(0.15+j as f64*0.045),
             h*1.42,-age,TAU*j as f64/4.,tint(c,0.88,0.48));
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
