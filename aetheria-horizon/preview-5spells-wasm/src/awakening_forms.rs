//! M22: forty distinct original Awakening *layout drafts* for the 200-entry
//! Aetheria spell library. Each ID uses a source-named, hand-choreographed
//! silhouette rather than the generic semantic-draft family.
//!
//! Critical: world effects (damage, ward, heal, summon, terrain) are still
//! DESIGN INTENTS only; this module emits transient 3D world quad geometry,
//! not server-authoritative mobs, projectiles or solid collision objects.
//! TRUE HDR world light and depth-aware volume shader remain separate work.
use super::{Cast,Scene,add,sub,scale,shade_argb};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,floor_slab,orb,curtain};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{TAU,PI};

type V=[f64;3];
fn crystal_garden(s:&mut Scene,o:V,age:f64,g:f64,n:usize,r:f64,h:f64,c:u32){
    for j in 0..n{
        let a=TAU*j as f64/n as f64+age*0.006;
        let radial=r*(0.64+0.24*((j*17)%7) as f64/7.0);
        let from=p(o,radial*a.cos(),-0.4,radial*a.sin());
        let to=p(from,0.18*a.cos()*g,h*(0.62+(j%4) as f64*0.12)*g,
                 0.18*a.sin()*g);
        crystal(s,from,to,(0.26+0.06*(j%3) as f64)*g,a,
                tint(c,0.56+0.09*(j%4) as f64,0.83));
    }
}
fn cascade(s:&mut Scene,o:V,age:f64,g:f64,paths:usize,r:f64,h:f64,c:u32) {
    for j in 0..paths {
        let a=TAU*j as f64/paths as f64;
        curtain(s,p(o,0.,0.,0.),r*(0.55+0.09*(j%3) as f64)*g,
             h*(0.70+0.08*(j%4) as f64)*g,age,a,
             tint(c,0.63+0.04*(j%3) as f64,0.37));
    }
}
fn shield(s:&mut Scene,o:V,age:f64,g:f64,c:u32){
    for j in 0..3 {
        let xx=(j as f64-1.0)*0.37*g;
        body(s,p(o,xx,2.1*g,j as f64*0.19*g),
            [1.17*g,1.63*g,(0.38+0.08*j as f64)*g],
             tint(c,0.63+0.075*j as f64,0.67));
    }
    crystal_garden(s,o,age,g,7,1.8*g,3.5*g,tint(c,0.92,0.80));
}
fn beast(s:&mut Scene,o:V,age:f64,g:f64,c:u32,coil:f64){
    // Closed faceted dragon/leviathan body masses with a distinct head.
    for segment in 0..10{
        let t=segment as f64/9.;
        let a=t*TAU*coil+age*0.018;
        let x=(t-0.5)*6.0*g;
        let y=(1.6+0.85*(a).sin())*g;
        let z=1.1*a.cos()*g;
        body(s,p(o,x,y,z),[(0.42-0.13*t)*g,(0.55-0.19*t)*g,
             (0.51-0.17*t)*g],
             tint(c,0.65+0.18*t,0.83));
    }
    let head=p(o,-3.*g,1.6*g+0.6*(age*0.018).sin()*g,1.1*g);
    crystal(s,p(head,-0.4*g,-0.15*g,0.),
             p(head,-1.15*g,0.16*g,-0.22*g),
             0.48*g,0.3,tint(c,0.96,0.92));
}
fn guardian(s:&mut Scene,o:V,age:f64,g:f64,c:u32) {
    // Full torso/head/arms: not a stick skeleton.
    orb(s,p(o,0.,2.4*g,0.),[0.95*g,1.38*g,0.65*g],age,
        tint(c,0.62,0.75),0.68);
    orb(s,p(o,0.,4.05*g,-0.10*g),
        [0.56*g,0.64*g,0.51*g],age,tint(c,0.83,0.86),0.91);
    for side in [-1.,1.] {
        crystal(s,p(o,side*0.76*g,3.3*g,0.),
                p(o,side*1.6*g,1.37*g,-0.1*g),
                0.42*g,side*0.43,tint(c,0.76,0.88));
        body(s,p(o,side*0.37*g,0.64*g,0.1*g),
                [0.40*g,0.78*g,0.43*g],tint(c,0.56,0.87));
    }
}
fn form(s:&mut Scene,id:u16,o:V,age:f64,g:f64,c:u32) {
  let r=3.3*g;let h=5.1*g;
  match id {
   15=>{ // Phoenix Mantle: active two-wing feather cloak, no magic ring
      guardian(s,o,age,g*0.62,c);
      for side in [-1.,1.] {
        for j in 0..9 {
          let a=(if side<0.{PI}else{0.})+
              side*((j as f64*0.092)+(age*0.013).sin()*0.025);
          wing(s,p(o,0.,1.5*g,0.),a,
              r*(0.56+(j%4) as f64*0.1),
              h*(0.61+j as f64*0.06),tint(c,0.72,0.80));
        }
      }
   },
   16=>{ // Magma Crown: seven uneven basal basalt columns and molten heart
      crystal_garden(s,o,age,g,7,r,h,c);
      body(s,p(o,0.,0.54*h,0.),[0.78*g,1.05*g,0.74*g],c);
      floor_slab(s,o,r*0.8,0.4,age*0.003,tint(c,0.63,0.78));
   },
   17=>{ // Solar Crucible: self-contained golden heavy furnace bowl
      orb(s,p(o,0.,0.48*h,0.),[1.7*g,1.7*g,1.7*g],age,c,0.57);
      for i in 0..8{
        let a=TAU*i as f64/8.;
        body(s,p(o,2.0*g*a.cos(),2.1*g,2.0*g*a.sin()),
            [0.38*g,0.97*g,0.43*g],tint(c,0.62,0.88));
      }
      crystal(s,p(o,0.,0.46*h,0.),p(o,0.,1.09*h,0.),
              0.43*g,age*0.013,tint(c,0.95,0.93));
   },
   18=>{ // Dragonfire Procession: three separately coiled dragon heads
      for j in 0..3{
         beast(s,p(o,(j as f64-1.0)*1.33*g,0.,j as f64*0.65*g),
               age+j as f64*13.0,g*0.59,tint(c,0.70+j as f64*0.07,0.84),1.2+j as f64*0.22);
      }
   },
   35=>{ // Avalanche Choir: stacked slabs forming advancing snow-front
      for row in 0..6 {
         let t=row as f64/6.0;
         for j in -3..=3{
           let xx=j as f64*0.67*g+(row%2) as f64*0.21*g;
           body(s,p(o,xx,h*(0.17+t*0.69),r*t*0.59),
               [0.43*g,(0.34+t*0.33)*g,0.50*g],tint(c,0.60+0.19*t,0.83));
         }
      }
   },
   36=>shield(s,o,age,g,c), // Diamondheart: layered crystal armor
   37=>{ // Cryostasis Vault: hourglass inside an actual ice dome
      orb(s,p(o,0.,h*0.44,0.),[r*0.76,h*0.46,r*0.77],age,c,0.48);
      for j in 0..10{
        let a=TAU*j as f64/10.;
        crystal(s,p(o,r*0.69*a.cos(),-0.6,r*0.69*a.sin()),
          p(o,r*0.42*a.cos(),h*0.85,r*0.42*a.sin()),0.39*g,a,
           tint(c,0.65+0.02*(j%4) as f64,0.91));
      }
   },
   38=>beast(s,o,age,g,c,1.7), // Glacial Leviathan
   55=>{ // Tempest Staircase: helix of thick hovering traversable sheets
      for j in 0..15 {
        let t=j as f64/15.;
        let a=TAU*t*1.65+age*0.007;
        let width=0.66*g;
        body(s,p(o,r*a.cos()*0.75,h*t,r*a.sin()*0.75),
           [width,0.20*g,0.79*g],tint(c,0.55+0.24*t,0.75));
      }
   },
   56=>{ // Zephyr Doppel: two opposing translucent wind apparitions
      for side in [-1.,1.] {
         guardian(s,p(o,side*1.37*g,0.,0.23*g),age,g*0.61,
                  tint(c,0.55,0.29));
         curtain(s,p(o,side*1.37*g,0.,0.),r*0.54,h,age,
               side*0.5,tint(c,0.76,0.31));
      }
   },
   57=>{ // Storm Eye: transparent still center, folding pressure walls
      for i in 0..5{
        curtain(s,o,r*(0.54+0.075*i as f64),h*0.93,
          age,TAU*i as f64/5.,tint(c,0.63+0.05*(i%3) as f64,0.49));
      }
      orb(s,p(o,0.,h*0.50,0.),[r*0.21,h*0.21,r*0.20],
          age,tint(c,0.44,0.29),0.33);
   },
   58=>{ // Hurricane Harp: visible harp-shaped wooden/air frame and strings
      for i in 0..8{
        let x=(i as f64-3.5)*0.40*g;
        crystal(s,p(o,x,0.0,0.),p(o,x,h*(0.72+(i%3) as f64*0.1),0.),
                0.16*g,0.0,tint(c,0.69,0.71));
      }
      wing(s,o,0.24,r,h,tint(c,0.60,0.51));
      wing(s,o,PI-0.24,r,h,tint(c,0.75,0.47));
   },
   75=>{ // Colossus Gauntlet: twin giant stone FISTS (not just spikes)
      for side in [-1.,1.]{
        let origin=p(o,side*1.8*g,2.3*g,0.);
        body(s,origin,[0.92*g,1.13*g,0.73*g],tint(c,0.7,0.9));
        for finger in 0..4{
          body(s,p(origin,side*(finger as f64-1.5)*0.39*g,-1.02*g,-0.41*g),
             [0.24*g,0.61*g,0.28*g],tint(c,0.61+finger as f64*0.06,0.85));
        }
      }
   },
   76=>{ // Gemheart Armor: crystalline mantle with overlapping
      // gem shoulder plates and separately suspended faceted javelins.
      body(s,p(o,0.,2.2*g,0.),[1.05*g,1.42*g,0.77*g],
           tint(c,0.55,0.93));
      for side in [-1.,1.] {
          for i in 0..4{
              let offset=side*(0.69+i as f64*0.35)*g;
              crystal(s,p(o,offset,3.6*g,0.),
                 p(o,offset+side*0.25*g,5.0*g+(i%3) as f64*0.24*g,-0.3*g),
                 (0.39+0.05*i as f64)*g,
                 side*i as f64*0.2,tint(c,0.80+i as f64*0.02,0.9));
          }
          body(s,p(o,side*1.1*g,2.5*g,-0.2*g),[0.51*g,0.54*g,0.62*g],
               tint(c,0.75,0.95));
      }
   },
   77=>{ // Tectonic Crown: delayed stone columns that tilt outward
      for i in 0..9 {
        let a=TAU*i as f64/9.;
        let root=p(o,r*0.53*a.cos(),-0.62,r*0.53*a.sin());
        crystal(s,root,p(root,0.35*a.cos(),h*(0.66+(i%4) as f64*0.10),
                          0.35*a.sin()),0.51*g,a,tint(c,0.63+0.03*(i%5) as f64,0.94));
      }
      floor_slab(s,o,r*0.82,0.44,0.2,tint(c,0.56,0.88));
   },
   78=>{ // Monolith March: actual spaced giant pillar SENTINELS
       for i in 0..6{
         let x=(i as f64-2.5)*1.28*g;
         let z=(i%2) as f64*0.63*g;
         let height=h*(0.65+(i%4) as f64*0.11);
         body(s,p(o,x,height*0.50,z),
              [0.55*g,height*0.5,0.57*g],tint(c,0.60+i as f64*0.035,0.94));
         crystal(s,p(o,x,height,z),p(o,x,height+0.63*g,z),
                 0.17*g,0.,tint(c,0.94,0.88));
       }
   },
   95=>{ // Ocean Guardian: distinct armored turtle-like spirit shell
      orb(s,p(o,0.,2.05*g,0.),[2.0*g,0.87*g,1.51*g],age,c,0.65);
      for side in [-1.,1.]{
        body(s,p(o,side*1.70*g,1.3*g,1.0*g),[0.62*g,0.49*g,0.70*g],
           tint(c,0.73,0.79));
      }
      orb(s,p(o,0.,1.91*g,-1.8*g),[0.71*g,0.70*g,0.68*g],
           age,tint(c,0.88,0.89),0.85);
   },
   96=>{ // MONSOON CHOIR: THREE DISTINCT restorative fountains.
      // The first draft was an overlapping single funnel, duplicating
      // Storm Eye; three separate sources now create a visible chorus.
      for fountain in 0..3{
        let a=TAU*fountain as f64/3.;
        let base=p(o,2.15*g*a.cos(),0.,2.15*g*a.sin());
        floor_slab(s,base,0.86*g,0.18,age*0.005+a,
                   tint(c,0.65,0.56));
        cascade(s,base,age+fountain as f64*4.0,g*0.76,2,
                0.80*g,4.8*g,tint(c,0.85,0.31));
        crystal(s,p(base,0.,-0.3*g,0.),p(base,0.,3.0*g,0.),
              0.28*g,a,tint(c,0.89,0.64));
      }
   },
   97=>{ // Abyssal Dome: layered submerged pressure sphere
      orb(s,p(o,0.,2.0*g,0.),[2.1*g,1.9*g,2.1*g],age,c,0.45);
      orb(s,p(o,0.,2.0*g,0.),[1.7*g,1.52*g,1.7*g],age,
          tint(c,0.53,0.56),0.65);
      crystal_garden(s,o,age,g,5,1.4*g,2.1*g,tint(c,0.92,0.79));
   },
   98=>{ // LEVIATHAN SPIRAL: a continuous WET CRYSTAL-WATER ribbon
      // dragon, not the same hard block skeleton as Glacial Leviathan.
      let n=16usize;
      let spiral=|t:f64| {
         let a=t*TAU*2.22+age*0.022;
         let rad=(0.65+1.52*t)*g;
         p(o,rad*a.cos(),(0.5+4.6*t)*g,rad*a.sin())
      };
      for i in 0..n {
         let t=i as f64/n as f64;
         let u=(i+1) as f64/n as f64;
         let mid=(t+u)*0.5;
         let start=spiral(t);
         let end=spiral(u);
         let current=spiral(mid);
         let a=mid*TAU*2.22+age*0.022;
         let flank=[-a.sin(),0.,a.cos()];
         let half=(0.7-0.27*t)*g;
         s.membrane(start,p(current,flank[0]*half,-0.12*g,flank[2]*half),
             end,p(current,-flank[0]*half,0.12*g,-flank[2]*half),
             tint(c,0.78+0.09*(i%3) as f64,0.56));
         if i%4==0 {
           wing(s,current,a+0.65,(1.1-0.3*t)*g,0.95*g,
                tint(c,0.68,0.42));
         }
      }
      let head=spiral(1.0);
      crystal(s,head,p(head,0.73*g,0.40*g,-0.12*g),
              0.44*g,age*0.01,tint(c,0.93,0.87));
   },
   115=>{ // Ancient Ent: stocky rooted *walking* tree guardian
      guardian(s,o,age,g,tint(c,0.53,0.91));
      crystal_garden(s,o,age,g,8,r*0.70,h*0.64,tint(c,0.79,0.86));
   },
   116=>{ // Emerald Canopy: broad protective overlapping forest umbrella
      for j in 0..14{
        wing(s,p(o,0.,2.8*g,0.),TAU*j as f64/14.,
              r*(0.85+(j%3) as f64*0.10),
              h*(0.84+(j%4) as f64*0.07),tint(c,0.58+j as f64*0.014,0.74));
      }
      body(s,p(o,0.,1.28*g,0.),[0.47*g,1.58*g,0.51*g],tint(c,0.58,0.91));
   },
   117=>{ // VERDANT COMMUNION: a grounded unfolding medicinal FLOWER.
      // Canopy previously looked like the same vertical green spike fan;
      // these leaves fan horizontally across real ground and bear a heart.
      floor_slab(s,o,r*0.87,0.23,age*0.005,tint(c,0.45,0.68));
      for petal in 0..14 {
        let a=TAU*petal as f64/14.;
        let spread=r*(0.64+(petal%4) as f64*0.11);
        wing(s,p(o,0.,0.18*g,0.),a,spread,
            0.62*g+(petal%3) as f64*0.1*g,
            tint(c,0.55+0.04*(petal%4) as f64,0.81));
      }
      orb(s,p(o,0.,0.63*g,0.),[0.58*g,0.57*g,0.58*g],
          age,tint(c,0.89,0.92),0.85);
      crystal_garden(s,o,age,g*0.49,8,1.5*g,2.0*g,tint(c,0.62,0.56));
   },
   118=>{ // Thornheart Swarm: stout, interwoven bramble forest
      for group in 0..9{
        let a=TAU*group as f64/9.;
        let origin=p(o,2.1*g*a.cos(),-0.4,2.1*g*a.sin());
        crystal(s,origin,p(o,1.0*g*a.cos(),h*(0.59+(group%4) as f64*0.12),
               1.0*g*a.sin()),0.44*g,a,tint(c,0.71,0.92));
        wing(s,origin,a+0.3,1.4*g,2.3*g,tint(c,0.64,0.78));
      }
   },
   135=>{ // Living Thunder: winged arc elemental core with dense mass
      orb(s,p(o,0.,2.3*g,0.),[0.72*g,1.08*g,0.69*g],age,c,0.78);
      for side in [-1.,1.]{
        for j in 0..7{
          let a=(if side<0.{PI}else{0.})+
                (j as f64*0.08*side);
          wing(s,p(o,0.,2.*g,0.),a,r*(0.7+j as f64*0.05),
             h*(0.6+j as f64*0.045),tint(c,0.73,0.81));
        }
      }
   },
   136=>{ // Thunder Drum: heavy covered percussion body, no hollow ring
      body(s,p(o,0.,2.1*g,0.),[1.37*g,1.03*g,1.24*g],tint(c,0.61,0.92));
      for i in 0..8{
        let a=TAU*i as f64/8.;
        crystal(s,p(o,1.3*g*a.cos(),1.9*g,1.2*g*a.sin()),
          p(o,1.77*g*a.cos(),2.7*g,1.56*g*a.sin()),0.31*g,a,
          tint(c,0.84,0.78));
      }
   },
   137=>{ // TEMPEST CONDUCTOR: a striking heavy conductor staff
      // between two faceted copper-magical capacitors. Not the blue
      // winged-thunder elemental from spell 135.
      body(s,p(o,0.,2.4*g,0.),[0.54*g,2.0*g,0.55*g],
          tint(c,0.57,0.87));
      for side in [-1.,1.] {
         let x=side*1.1*g;
         body(s,p(o,x,3.05*g,0.),[0.48*g,0.99*g,0.58*g],
              tint(c,0.69,0.89));
         crystal(s,p(o,x,3.1*g,0.),p(o,x*1.38,4.7*g,0.),
                 0.46*g,0.3*side,tint(c,0.94,0.90));
         for i in 0..3{
            let y=(i as f64+1.)*1.2*g;
            body(s,p(o,side*(0.29+0.16*i as f64)*g,y,0.),
                [0.27*g,0.17*g,0.43*g],tint(c,0.77,0.89));
         }
      }
      crystal(s,p(o,0.,4.23*g,0.),p(o,0.,5.4*g,0.),
              0.32*g,age*0.015,tint(c,0.99,0.93));
   },
   138=>{ // Heavens Rattle: heavy suspended thunder seals/fractures
      for j in 0..7{
        let a=TAU*j as f64/7.;
        body(s,p(o,2.5*g*a.cos(),(2.0+(j%3) as f64*0.57)*g,
                 2.2*g*a.sin()),[0.68*g,0.47*g,0.58*g],
             tint(c,0.66+0.04*(j%4) as f64,0.82));
      }
      orb(s,p(o,0.,2.5*g,0.),[0.87*g,1.11*g,0.97*g],
          age,tint(c,0.80,0.68),0.74);
   },
   155=>{ // NEBULA MANTLE: wide star-strewn cloak, not another vortex
      // funnel. Nine panels spread horizontally from an astral collar.
      let collar=p(o,0.,3.5*g,0.);
      for i in 0..9{
        let a=TAU*i as f64/9.+age*0.005;
        wing(s,collar,a,r*(0.67+(i%4) as f64*0.10),
              -(1.7+(i%3) as f64*0.29)*g,
              tint(c,0.57+0.04*(i%5) as f64,0.43));
      }
      for sheet in 0..3{
        curtain(s,o,r*(0.30+0.1*sheet as f64),h*0.84,
              age,TAU*sheet as f64/3.,tint(c,0.72,0.17));
      }
      crystal_garden(s,o,age,g*0.48,7,1.4*g,2.8*g,tint(c,0.86,0.75));
   },
   156=>{ // Gravity Crown: rotating heavy tectonic orbital slabs
      orb(s,p(o,0.,2.1*g,0.),[1.08*g,1.36*g,1.1*g],age,c,0.63);
      for i in 0..11{
        let a=TAU*i as f64/11.+age*0.018;
        body(s,p(o,2.5*g*a.cos(),2.3*g+0.52*g*a.sin(),
                2.5*g*a.sin()),[0.47*g,0.51*g,0.48*g],
             tint(c,0.53+0.05*(i%4) as f64,0.88));
      }
   },
   157=>{ // CELESTIAL ECHO: two reflective panels set BEHIND
      // distinct full-height astral silhouettes, not just two slabs.
      for side in [-1.,1.] {
        body(s,p(o,side*1.60*g,2.35*g,0.72*g),
             [0.82*g,1.77*g,0.32*g],tint(c,0.41,0.46));
        guardian(s,p(o,side*1.55*g,0.,-0.77*g),
                 age,g*0.58,tint(c,0.79,0.67));
      }
   },
   158=>{ // Lunar Eclipse: partial occlusion of luminous lunar sphere
      orb(s,p(o,0.,2.35*g,0.),[1.78*g,1.78*g,1.56*g],age,
          tint(c,0.88,0.84),0.82);
      orb(s,p(o,0.51*g,2.40*g,-0.41*g),[1.53*g,1.7*g,1.33*g],age,
          tint(c,0.18,0.96),0.91);
   },
   175=>guardian(s,o,age,g,c), // Hour Guardian: large timekeeper
   176=>{ // STASIS LENS: visible suspended projectiles inside
      // frozen time, physically different from a simple hourglass orb.
      orb(s,p(o,0.,2.4*g,0.),[1.8*g,1.9*g,1.8*g],age,c,0.35);
      for j in 0..12 {
        let a=TAU*j as f64/12.;
        let yy=(1.7+(j%4) as f64*0.45)*g;
        crystal(s,p(o,1.17*g*a.cos(),yy,1.17*g*a.sin()),
             p(o,1.17*g*a.cos()-0.53*g,yy+0.28*g,
                    1.17*g*a.sin()-0.15*g),
             0.17*g,a,tint(c,0.92,0.93));
      }
      for frame in [-1.,1.] {
         body(s,p(o,frame*1.42*g,2.4*g,0.),
               [0.23*g,1.68*g,0.41*g],tint(c,0.64,0.79));
      }
   },
   177=>{ // Entropy Ribbon: wide disintegrating temporal cloth
      for j in 0..6{
         curtain(s,p(o,(j as f64-2.5)*0.25*g,0.,0.),
             r*(0.63+0.08*(j%3) as f64),h,age,
             TAU*j as f64/6.,tint(c,0.64,0.38));
      }
   },
   178=>{ // Reprise Chorus: suspended spatial time crystals & split dials
      for i in 0..8{
        let a=TAU*i as f64/8.;
        let root=p(o,2.3*g*a.cos(),0.8*g,2.3*g*a.sin());
        crystal(s,root,p(o,1.6*g*a.cos(),3.8*g,1.6*g*a.sin()),
            0.42*g,a,tint(c,0.75,0.91));
      }
      orb(s,p(o,0.,2.5*g,0.),[1.11*g,1.24*g,1.11*g],
          age,tint(c,0.61,0.35),0.38);
   },
   195=>{ // Grim Shepherd: armored guardian with a broad spectral scythe
      guardian(s,o,age,g,c);
      crystal(s,p(o,1.35*g,0.0,0.),p(o,2.05*g,h,0.),
              0.22*g,0.7,tint(c,0.85,0.9));
      wing(s,p(o,2.0*g,h*0.76,0.),0.78,1.7*g,1.4*g,
          tint(c,0.71,0.59));
   },
   196=>{ // Soul Exchange: two enclosed balanced vessels connected
      // by thick spectral life stream (not a simple solid rod).
      for side in [-1.,1.]{
        orb(s,p(o,side*1.75*g,2.2*g,0.),[0.72*g,1.24*g,0.74*g],
            age,tint(c,0.61+(side+1.)*0.08,0.77),0.84);
      }
      cascade(s,p(o,0.,0.,0.),age,g,3,r*0.60,h*0.70,tint(c,0.81,0.30));
   },
   197=>{ // Crimson Aegis: five real layered angled overlapping
      // ritual shields with a raised central star-clasp, no armor clone.
      for i in 0..5 {
         let a=(i as f64-2.0)*0.24;
         let cx=(i as f64-2.0)*0.55*g;
         body(s,p(o,cx,2.2*g+(i%2) as f64*0.21*g,
                  (i as f64-2.0).abs()*0.24*g),
              [0.62*g,1.38*g,0.34*g],
              tint(c,0.55+0.07*(i%4) as f64,0.85));
         crystal(s,p(o,cx,1.3*g,-0.44*g),
              p(o,cx+a*g,3.3*g,-0.48*g),
              0.29*g,a,tint(c,0.90,0.90));
      }
      orb(s,p(o,0.,2.3*g,-0.65*g),
          [0.55*g,0.63*g,0.24*g],age,tint(c,0.92,0.90),0.84);
   },
   198=>{ // Eidolon Procession: ALL five soldiers must remain
      // visible even at a medium face budget; use a compact solid
      // mannequin per soldier instead of 5 full 300-face guardian orbs.
      for i in 0..5{
         let x=(i as f64-2.0)*1.39*g;
         let z=(i%2) as f64*0.69*g;
         let at=p(o,x,0.,z);
         body(s,p(at,0.,1.8*g,0.),[0.47*g,0.95*g,0.35*g],
              tint(c,0.51+i as f64*0.047,0.74));
         crystal(s,p(at,0.,2.64*g,0.),p(at,0.,3.40*g,0.),
                 0.37*g,0.1*i as f64,tint(c,0.83,0.81));
         for side in [-1.,1.]{
             body(s,p(at,side*0.64*g,1.85*g,0.),
                  [0.24*g,0.75*g,0.27*g],tint(c,0.64,0.77));
         }
         // Shoulder mantle: solid sides to avoid a T-shaped stick figure.
         wing(s,p(at,0.,2.75*g,0.),PI*0.25,
              0.82*g,1.2*g,tint(c,0.62,0.34));
      }
   },
   _=>{}
  }
}
/// Explicit first-draft high-tier animations, using authored source IDs.
pub fn render(cast:Cast,age:f64,scene:&mut Scene) {
  if cast.kind>=200 || !(14..=17).contains(&catalog200::rank(cast.kind)) ||
      !age.is_finite() || !(0.0..104.0).contains(&age){return;}
  let spec=catalog200::spec(cast.kind);
  let Some(timeline)=for_spell(spec.id)else{return;};
  let mut local=age/104.0*timeline.total_ticks() as f64;
  let mut current=Stage::Telegraph;
  let mut progress=0.0;
  for part in timeline.phases {
    if local<part.duration_ticks as f64 {
        current=part.kind;
        progress=(local/part.duration_ticks as f64).clamp(0.,1.);
        break;
    }
    local-=part.duration_ticks as f64;
  }
  let grow=match current{
    Stage::Telegraph=>0.16+progress*0.16,
    Stage::Cast=>0.31+0.69*progress,
    Stage::Impact=>1.0+0.10*(progress*TAU).sin(),
    Stage::Afterglow=>(1.0-0.62*progress).max(0.09),
  };
  let school=catalog200::school(cast.kind);
  let mut c=tint(catalog200::WORLD_COLORS[school],0.9,0.90);
  if current==Stage::Afterglow {
    c=tint(c,0.9,(1.0-progress).powf(1.20));
  }
  form(scene,spec.id,cast.center,age,grow,c);
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn all_40_awakening_ids_have_distinct_non_empty_bounded_layouts(){
   let mut ids=std::collections::BTreeSet::new();
   let mut hashes=std::collections::BTreeSet::new();
   for school in 0..10usize {
     for local in 14..=17usize {
       let slot=school*20+local;
       let spec=catalog200::spec(slot);
       assert_eq!(spec.role,"Awakening");
       let mut scene=Scene::new(700);
       render(Cast{kind:slot,variant:local as u8,start:0,
            center:[0.,64.,0.],direction:[0.,0.,1.]},49.0,&mut scene);
       assert!(scene.facets.len()>=36,
         "Awakening spell {} lacks actual physical composition: {} faces",spec.id,scene.facets.len());
       assert!(scene.facets.len()<=700);
       assert!(scene.facets.iter().all(|(q,_)|
           q.iter().flatten().all(|x|x.is_finite())));
       let geometry=scene.facets.iter().take(28).flat_map(|(q,_)|
           q.iter().flat_map(|p|p.iter().map(|x|(x*100.) as i64)))
           .collect::<Vec<_>>();
       assert!(hashes.insert(geometry),"Awakening duplicate mesh #{}",spec.id);
       assert!(ids.insert(spec.id));
     }
   }
   assert_eq!(ids.len(),40);
 }
}
