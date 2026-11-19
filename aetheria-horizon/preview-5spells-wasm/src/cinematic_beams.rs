//! M13: 30 bounded original fantasy VFX silhouettes.
 //! Source: user's four re-uploaded videos (3–6), see docs/VIDEO_REFERENCE_ANALYSIS.md.
 //! Uses Horizon world-space quads with actual 5/6-sided spatial surfaces; NOT
 //! actual GPU volumetrics, dynamic lighting, game damage or permanent edits.
 use super::{Cast, Scene, add, sub, scale, norm, cross, shade_argb, alpha};
 use crate::catalog30;
 use std::f64::consts::TAU;
 use aetheria_horizon::vfx::timeline::{for_spell,Stage};

 type V=[f64;3];
 fn p(o:V,x:f64,y:f64,z:f64)->V{add(o,[x,y,z])}
 fn mix(a:V,b:V,t:f64)->V{add(a,scale(sub(b,a),t))}
 fn basis(axis:V)->(V,V){
     let d=norm(axis);
     let anchor=if d[1].abs()<0.85 {[0.,1.,0.]}else{[1.,0.,0.]};
     let r=norm(cross(d,anchor));
     (r,norm(cross(r,d)))
 }
 fn color(school:usize,age:f64)->u32 {
     // Alpha deliberately capped, preserving colored material instead of whiteout.
     alpha(catalog30::WORLD_COLORS[school],age,104.)
 }
 /// Actual 5-sided cylinder, not the two crossed 2D ribbons of early VFX.
 fn beam(s:&mut Scene,from:V,to:V,r:f64,phase:f64,color:u32){
     let dir=sub(to,from);
     let (right,up)=basis(dir);
     for j in 0..5 {
         let t=j as f64/5.;
         let u=(j+1) as f64/5.;
         let bend=|v:f64|{
             let drift=(phase+v*TAU*1.5).sin()*r*0.32;
             add(mix(from,to,v),add(scale(right,drift),scale(up,drift*0.27)))
         };
         let a=bend(t);let b=bend(u);
         let fade=1.-0.43*t;
         s.tube(a,b,r*fade,r*(fade-0.07),phase+t,shade_argb(color,0.78),5);
         s.tube(a,b,r*fade*0.36,r*fade*0.33,phase+t+0.2,color,5);
     }
 }
 /// A toroidal runic conduit with real 5-sided radial cross section.
 fn ring(s:&mut Scene,o:V,axis:V,r:f64,thickness:f64,n:usize,spin:f64,color:u32){
     if r<=0.03||thickness<=0.01{return;}
     let (side,up)=basis(axis);
     let n=n.clamp(6,18);
     for j in 0..n {
         let a=TAU*j as f64/n as f64+spin;
         let b=TAU*(j+1) as f64/n as f64+spin;
         let place=|angle:f64|add(o,add(scale(side,r*angle.cos()),scale(up,r*angle.sin())));
         s.tube(place(a),place(b),thickness,thickness,spin+a,
             shade_argb(color,0.68+0.30*(j%3) as f64/2.),5);
     }
 }
 fn rune_seal(s:&mut Scene,o:V,axis:V,r:f64,phase:f64,color:u32){
     ring(s,o,axis,r,0.075,12,phase,color);
     ring(s,o,axis,r*0.76,0.036,12,-phase,shade_argb(color,0.68));
     let (right,up)=basis(axis);
     for j in 0..8 {
         let a=TAU*j as f64/8.+phase*0.52;
         let direction=add(scale(right,a.cos()),scale(up,a.sin()));
         let start=add(o,scale(direction,r*0.87));
         let end=add(o,scale(direction,r*1.13));
         s.solid_crystal(start,end,0.09,a,shade_argb(color,0.83),5);
     }
 }
 fn sparks(s:&mut Scene,o:V,count:usize,age:f64,r:f64,color:u32){
     for j in 0..count.min(16) {
         let a=TAU*j as f64/count.max(1) as f64+age*0.035;
         let h=0.4+(j%5) as f64*0.42+0.3*(age*0.05+a).sin();
         let point=p(o,r*a.cos(),h,r*a.sin());
         let tip=p(point,0.28*a.cos(),0.75+(j%3) as f64*0.16,0.28*a.sin());
         s.solid_crystal(point,tip,0.12,a,color,5);
     }
 }
 fn shards(s:&mut Scene,o:V,age:f64,rows:usize,color:u32){
     for row in 0..rows {
         let elapsed=age-row as f64*5.2;
         if elapsed<=0. {continue;}
         let grow=(elapsed/17.).clamp(0.,1.).powf(0.72);
         for j in 0..5 {
             let x=j as f64-2.;
             let seed=row as f64*2.3+j as f64*1.31;
             let base=p(o,x*(0.72+0.16*row as f64),-0.55,row as f64*1.25-1.5);
             let tip=p(base,0.48*seed.sin(),(2.1+3.9*seed.sin().abs())*grow,
                       0.65*seed.cos());
             s.solid_crystal(base,tip,0.32+0.13*seed.cos().abs(),seed,color,6);
         }
     }
 }
 fn tendril(s:&mut Scene,o:V,age:f64,n:usize,height:f64,color:u32){
     for strand in 0..n {
         let phase=TAU*strand as f64/n.max(1) as f64;
         let mut last=p(o,0.,-0.3,0.);
         for j in 1..=9 {
             let u=j as f64/9.;
             let a=phase+u*TAU*1.55+age*0.02;
             let r=0.33+2.4*u;
             let next=p(o,r*a.cos(),height*u,r*a.sin());
             s.tube(last,next,0.18*(1.-u)+0.05,0.14*(1.-u)+0.04,a,color,5);
             if j%3==0 {s.solid_crystal(next,p(next,a.cos()*0.5,0.8,a.sin()*0.5),0.13,a,color,5);}
             last=next;
         }
     }
 }
 fn broken_sky(s:&mut Scene,o:V,age:f64,color:u32){
     let rise=(age/20.).clamp(0.,1.);
     let center=p(o,0.,3.8*rise,0.);
     rune_seal(s,center,[0.2,0.94,0.2],3.3,age*0.013,color);
     // Radial fracturing in world depth, no skybox edit or copied aperture texture.
     for i in 0..10 {
         let a=TAU*i as f64/10.+0.09*(i as f64*2.).sin();
         let origin=p(center,2.9*a.cos(),0.3*(i%3) as f64,2.9*a.sin());
         let outer=p(center,(5.1+(i%4) as f64*0.56)*a.cos(),
                     -0.8+(i%3) as f64*0.46,
                     (5.1+(i%4) as f64*0.56)*a.sin());
         beam(s,origin,outer,0.09,age*0.02+a,shade_argb(color,0.65));
     }
     sparks(s,center,9,age,2.0,shade_argb(color,0.85));
 }
 fn storm(s:&mut Scene,o:V,age:f64,color:u32){
     // Forked bolts have spatial cross section and branches: no straight laser column.
     for i in 0..5 {
         let a=TAU*i as f64/5.;
         let start=p(o,a.cos()*3.8,6.5,a.sin()*3.8);
         let mut at=start;
         for j in 0..6 {
             let u=(j+1) as f64/6.;
             let jitter=((i*19+j*11) as f64+age*0.055).sin();
             let end=p(o,a.cos()*3.8+jitter*0.58,6.5*(1.-u),a.sin()*3.8-jitter*0.31);
             s.tube(at,end,0.10,0.073,a+j as f64,color,5);
             if j==3 {
                 let branch=p(end,jitter*1.5,-0.65,-jitter*1.1);
                 s.tube(end,branch,0.066,0.03,a,color,5);
             }
             at=end;
         }
     }
 }
 fn pillar(s:&mut Scene,o:V,age:f64,color:u32){
     // Reference (3): a grounded ritual with a tall spatial magical column.
     let open=(age/21.).clamp(0.,1.);
     rune_seal(s,p(o,0.,0.15,0.),[0.,1.,0.],3.4,age*0.015,color);
     ring(s,p(o,0.,2.2*open,0.),[0.,1.,0.],2.0,0.075,10,-age*0.025,color);
     for i in 0..7 {
         let a=TAU*i as f64/7.+age*0.014;
         let base=p(o,a.cos()*0.9,0.,a.sin()*0.9);
         let top=p(o,a.cos()*0.56,9.0*open,a.sin()*0.56);
         beam(s,base,top,0.10,age*0.018+a,color);
     }
     sparks(s,o,9,age,2.8,color);
 }
 /// Map the 30 individual core timelines to a visual lifetime capped at 104 ticks.
 /// Keeps the original choreography proportions, without pretending that the
 /// client preview implements damage, actual server time or charged controls.
 pub(super) fn visual_phase(slot:usize,age:f64)->Option<(Stage,f64)> {
     if !age.is_finite() || !(0.0..104.0).contains(&age) {return None;}
     let timeline=for_spell(catalog30::id(slot))?;
     let at=age/104.0*timeline.total_ticks() as f64;
     let mut elapsed=0.0;
     for part in timeline.phases {
         let length=f64::from(part.duration_ticks);
         if at<elapsed+length {
             let progress=((at-elapsed)/length).clamp(0.,1.);
             return Some((part.kind,progress));
         }
         elapsed+=length;
     }
     None
 }
 /// School-identity telegraphs; little material appears before any big impact.
 /// An individual spell's rank changes physical layer density and radius.
 fn anticipation(s:&mut Scene,o:V,school:usize,rank:usize,age:f64,progress:f64,c:u32){
     let reveal=0.16+progress*0.84;
     let width=0.8+rank as f64*0.37;
     let spin=age*(0.007+rank as f64*0.005);
     match school {
         0=>{ring(s,p(o,0.,0.16,0.),[0.,1.,0.],width*reveal,0.085,10,spin,c);
              sparks(s,o,2+rank,age,width,c);}
         1=>{for j in 0..(2+rank) {
              let a=TAU*j as f64/(2+rank) as f64;
              let from=p(o,a.cos()*width,-0.5,a.sin()*width);
              let to=p(from,0.2*a.cos(),(0.35+0.9*progress)*(j as f64*0.19+0.8),0.2*a.sin());
              s.solid_crystal(from,to,0.22,a,c,6);
             }}
         2=>{for j in 0..2 {ring(s,p(o,0.,0.25+j as f64*0.5,0.),[0.,1.,0.],
              (1.1+j as f64*0.45)*reveal,0.045,8,spin+j as f64,c);}}
         3=>{for j in 0..(3+rank) {
              let a=TAU*j as f64/(3+rank) as f64;
              s.solid_crystal(p(o,width*a.cos(),-0.6,width*a.sin()),
                   p(o,width*a.cos(),0.7*progress,width*a.sin()),0.20,a,c,5);
             }}
         4=>{ring(s,p(o,0.,0.35,0.),[0.4,1.,0.],width*reveal,0.06,10,spin,c);}
         5=>{tendril(s,o,age,2+rank,1.6*progress,c);}
         6=>{ring(s,p(o,0.,0.6,0.),[0.,1.,0.],width*reveal,0.065,8,spin,c);
              for j in 0..2 {let a=j as f64*TAU/2.+spin;
              s.tube(p(o,a.cos(),0.2,a.sin()),p(o,0.,1.4*progress,0.),0.08,0.04,a,c,5);}}
         7=>{rune_seal(s,p(o,0.,0.85,0.),[0.3,0.85,0.1],width*reveal,spin,c);}
         8=>{ring(s,p(o,0.,1.,0.),[0.,0.2,1.],width*reveal,0.055,10,spin,c);}
         _=>{sparks(s,o,3+rank,age,width,c);}
     }
 }
 /// A residual world-space echo, not an opaque static leftover. It fades in
 /// GPU alpha AND reduces its silhouette after the climax.
 fn afterglow(s:&mut Scene,o:V,school:usize,rank:usize,age:f64,progress:f64,c:u32){
     let shrink=(1.-progress).max(0.08);
     let r=(1.+rank as f64*0.49)*shrink;
     let spin=age*0.013;
     match school {
         0=>{sparks(s,o,3+rank,age,r,c);}
         1=>{for j in 0..4 {let a=j as f64*TAU/4.;
             s.solid_crystal(p(o,1.4*a.cos(),-0.55,1.4*a.sin()),
                 p(o,1.4*a.cos(),-0.3+3.0*shrink,1.4*a.sin()),0.19,a,c,6);}}
         2=>{ring(s,p(o,0.,1.+shrink,0.),[0.,1.,0.],r,0.038,8,spin,c);}
         3=>{for j in 0..3 {let a=TAU*j as f64/3.;
             s.solid_crystal(p(o,1.4*a.cos(),-0.3,1.4*a.sin()),p(o,0.,0.5*shrink,0.),0.17,a,c,5);}}
         4=>{ring(s,p(o,0.,0.6,0.),[0.8,0.2,0.3],r,0.035,10,spin,c);}
         5=>{tendril(s,o,age,2,2.0*shrink,c);}
         6=>{sparks(s,o,5,age,r,c);}
         7=>{ring(s,p(o,0.,1.0,0.),[0.5,0.6,0.6],r,0.045,9,spin,c);}
         8=>{ring(s,p(o,0.,0.9,0.),[0.,0.2,1.],r,0.03,10,-spin,c);}
         _=>{sparks(s,o,4,age,r,c);}
     }
 }
 /// An impact halo is only a complement to the unique spell geometry.
 /// 20–60 world quads at most, with remaining budget reserved by the caller.
 fn shockwave(s:&mut Scene,o:V,school:usize,rank:usize,age:f64,progress:f64,c:u32){
     let r=(1.0+4.2*progress)*(0.7+rank as f64*0.16);
     match school {
         0|4|6=>ring(s,p(o,0.,0.18,0.),[0.,1.,0.],r,0.055,10,age*0.012,c),
         1|3=>{for j in 0..6 {
             let a=TAU*j as f64/6.;
             let at=p(o,r*a.cos(),-0.5,r*a.sin());
             s.solid_crystal(at,p(at,0.0,0.7+progress*1.8,0.0),0.18,a,c,5);
         }}
         2|5=>sparks(s,o,7,age,r,c),
         7=>ring(s,p(o,0.,2.,0.),[0.2,0.8,0.3],r,0.06,10,age*0.017,c),
         8=>ring(s,p(o,0.,0.8,0.),[0.4,0.25,0.9],r,0.045,9,-age*0.018,c),
         _=>sparks(s,o,5,age,r,c)
     }
 }

 /// Bound per-spell procedural geometry; spawn only during a finite timeline.
 /// The caller owns a separate fair-budget Scene per concurrent spell.
 pub(super) fn draw(cast:Cast,age:f64,s:&mut Scene){
     let Some((scene_phase,progress))=visual_phase(cast.kind,age) else{return;};
     let school=catalog30::school(cast.kind);
     let rank=catalog30::rank(cast.kind);
     let mut color=color(school,age);
     let o=cast.center;
     // All spells have a legible telegraph, a physical cast, an impact and a
     // restrained echo. Times differ according to each registry spell ID.
     match scene_phase {
         Stage::Telegraph => {
             color=shade_argb(color,0.47+progress*0.39);
             anticipation(s,o,school,rank,age,progress,color);
             return;
         }
         Stage::Afterglow => {
             let fade=(1.-progress).clamp(0.,1.);
             let a=((color>>24) as f64*fade).round() as u32;
             color=(color&0x00FF_FFFF)|(a<<24);
             afterglow(s,o,school,rank,age,progress,color);
             return;
         }
         _=>{}
     }
     let f=norm(cast.direction);
     let phase=age*0.026+rank as f64*0.83;
     let stage=match scene_phase {
         Stage::Impact=>1.0,
         _=>progress.clamp(0.0,1.0)
     };
     match (school,rank) {
         // Pyromancy: swirling phoenix wake, spreading fiery fan, rising magma serpent.
         (0,0)=>{pillar(s,o,age,color);}
         (0,1)=>{
             for j in 0..7 {
                 let a=(j as f64-3.)*0.31;
                 let tip=p(o,6.2*stage*a.sin(),1.4+3.2*stage*a.cos(),
                           5.2*stage*a.cos());
                 beam(s,o,tip,0.17,phase+a,color);
             }
             sparks(s,o,9,age,2.8,color);
         }
         (0,_)=>{tendril(s,o,age,4,6.2*stage,color);ring(s,p(o,0.,3.,0.),[0.,1.,0.],2.5,0.1,11,phase,color);}
         // Cryomancy: towering ice field, raised bridge-like crystals, asymmetric shard chorus.
         (1,0)=>{shards(s,o,age,4,color);}
         (1,1)=>{
             for step in 0..8 {
                 let x=step as f64-3.5;
                 let origin=p(o,x*0.7,-0.2,step as f64*0.6);
                 s.solid_crystal(origin,p(origin,0.15,2.1*stage,0.9),
                                 0.4,step as f64*0.2,color,6);
             }
             ring(s,p(o,0.,1.4,2.),[0.,1.,0.],2.1,0.08,10,phase,color);
         }
         (1,_)=>{
             for i in 0..13 {
                 let a=TAU*i as f64/13.;
                 let tip=p(o,a.cos()*(2.4+i as f64*0.09),4.3*stage+(i%4) as f64*0.45,a.sin()*2.4);
                 s.solid_crystal(p(o,0.,-0.2,0.),tip,0.27,a,color,6);
             }
         }
         // Aeromancy: twisting pressure vortex, three crescent blades, rising updraft.
         (2,0)=>{tendril(s,o,age,3,6.4*stage,color);}
         (2,1)=>{
             for j in 0..3 {
                 let h=j as f64*1.4;
                 for k in 0..9 {
                     let a=(k as f64-4.)*0.27+age*0.015;
                     let base=p(o,a.sin()*2.6,h,a.cos()*2.6);
                     let tip=p(base,2.1*a.sin(),0.2+stage,2.1*a.cos());
                     s.tube(base,tip,0.19,0.06,a,color,5);
                 }
             }
         }
         (2,_)=>{for j in 0..4{ring(s,p(o,0.,j as f64*1.5,0.),[0.,1.,0.],0.8+j as f64*0.55,0.055,12,phase+j as f64*0.8,color);}}
         // Geomancy: linked stone monoliths, solid suspended steps, closing jaws.
         (3,0)=>{shards(s,o,age,3,color);}
         (3,1)=>{for j in 0..8 {let a=phase+j as f64*0.82;let base=p(o,a.cos()*3.,-0.3+j as f64*0.45,a.sin()*3.);
             s.solid_crystal(base,p(base,0.,2.0*stage,0.),0.52,a,color,6);}}
         (3,_)=>{for j in 0..10 {let a=TAU*j as f64/10.;let base=p(o,3.*a.cos(),-0.6,3.*a.sin());
             s.solid_crystal(base,p(o,1.0*a.cos(),2.9*stage,1.0*a.sin()),0.5,a,color,6);}}
         // Hydromancy: braided crest, wave procession, protective pearl.
         (4,0)=>{tendril(s,o,age,3,3.3*stage,color);beam(s,p(o,0.,1.,0.),p(o,0.,1.,6.*stage),0.18,phase,color);}
         (4,1)=>{for j in 0..5 {let a=j as f64*0.42;ring(s,p(o,0.,0.6+j as f64*0.23,j as f64*1.1),[0.,0.45,1.],
            1.4+a,0.055,13,phase+j as f64*0.25,color);}}
         (4,_)=>{for j in 0..4 {ring(s,p(o,0.,1.7,0.),[j as f64*0.15+0.22,0.9,0.4],2.1,0.08,13,phase+j as f64*0.65,color);}}
         // Druid: rooted canopy, binding roots and flowering healing sanctuary.
         (5,0)=>{tendril(s,o,age,5,5.9*stage,color);}
         (5,1)=>{tendril(s,o,age,6,2.7*stage,color);for j in 0..6 {let a=TAU*j as f64/6.;s.leaf(p(o,2.3*a.cos(),2.2,2.3*a.sin()),[a.cos(),0.7,a.sin()],1.2,color);}}
         (5,_)=>{rune_seal(s,p(o,0.,0.2,0.),[0.,1.,0.],3.6,phase,color);sparks(s,o,10,age,2.3,color);}
         // Electromancy: branching thunder, charged halo and precision needle.
         (6,0)=>{storm(s,o,age,color);}
         (6,1)=>{for j in 0..3 {rune_seal(s,p(o,0.,1.0+j as f64*0.8,0.),[0.2,1.,0.],1.3+j as f64*0.65,phase+j as f64,color);}}
         (6,_)=>{beam(s,p(o,0.,1.,0.),p(o,f[0]*10.,1.+f[1]*10.,f[2]*10.),0.14,phase,color);storm(s,p(o,0.,0.,5.),age,shade_argb(color,0.62));}
         // Astral: sky fracture, collapsing gravity lens, folded space frames.
         (7,0)=>{broken_sky(s,o,age,color);}
         (7,1)=>{rune_seal(s,o,[0.3,0.7,0.2],2.4,phase,color);rune_seal(s,p(o,0.,2.3,0.),[0.2,0.7,0.7],1.6,-phase,color);}
         (7,_)=>{for j in 0..7 {let a=TAU*j as f64/7.;let start=p(o,a.cos()*3.,(j%4) as f64*0.7,a.sin()*3.);
             let end=p(o,-a.sin()*1.3,3.3*stage,a.cos()*1.3);beam(s,start,end,0.075,phase+a,color);}}
         // Chronomancy: moving gear arcs, trailing delay disks, time pulse.
         (8,0)=>{rune_seal(s,o,[0.,0.1,1.],2.6,phase,color);beam(s,o,p(o,2.1*phase.cos(),2.1*phase.sin(),0.),0.11,phase,color);}
         (8,1)=>{for j in 0..5{let t=j as f64/5.;ring(s,p(o,-3.*t,0.8*t,1.4*t),[0.,1.,0.],0.8+1.2*t,0.055,10,phase-t,color);}}
         (8,_)=>{for j in 0..4 {ring(s,p(o,0.,j as f64*0.75,0.),[0.,1.,0.],2.5-j as f64*0.4,0.09,12,phase+j as f64*0.4,color);}}
         // Necromancy: rising bone spikes, soul lantern, swirling wisp vortex.
         (9,0)=>{shards(s,o,age,2,color);sparks(s,o,7,age,2.2,color);}
         (9,1)=>{rune_seal(s,p(o,0.,2.3,0.),[0.1,0.9,0.1],2.4,phase,color);tendril(s,o,age,3,3.4*stage,shade_argb(color,0.65));}
         (9,_)=>{tendril(s,o,age,4,4.4*stage,color);ring(s,p(o,0.,0.1,0.),[0.,1.,0.],3.2,0.055,12,phase,color);}
         _=>{}
     }
     if scene_phase==Stage::Impact {
         shockwave(s,o,school,rank,age,progress,color);
     }
 }
 #[cfg(test)]
 mod tests{
     use super::*;
     #[test]
     fn all_thirty_magic_motifs_draw_finite_bounded_world_geometry(){
         for id in 0..30{
             let mut scene=Scene::new(380);
             draw(Cast{kind:id,variant:0,start:0,center:[0.,64.,0.],direction:[0.,0.,1.]},31.,&mut scene);
             assert!(!scene.facets.is_empty(),"no 3D geometry for {id}");
             assert!(scene.facets.len()<=380,"budget overrun for {id}");
             assert!(scene.facets.iter().all(|(corners,_)|corners.iter().flatten().all(|v|v.is_finite())));
         }
     }
    #[test]
    fn thirty_spells_have_finite_distinct_choreography_and_no_lingering_mesh(){
        for slot in 0..30 {
            let p=|a:f64|{
                let mut scene=Scene::new(600);
                draw(Cast{kind:slot,variant:0,start:0,center:[0.,64.,0.],direction:[0.,0.,1.]},a,&mut scene);
                scene
            };
            for time in [8.,34.,56.,90.] {
                let scene=p(time);
                assert!(!scene.facets.is_empty(),"slot {slot} / age {time} empty");
                assert!(scene.facets.len()<=600,"over budget slot {slot} / age {time}");
                assert!(scene.facets.iter().all(|(points,_)|points.iter().flatten().all(|x|x.is_finite())));
            }
            assert!(p(104.).facets.is_empty());
        }
    }
    #[test]
    fn phases_differ_across_schools_and_rank(){
        let ids=[0,1,2,3,5,9,14,18,21,26,29];
        let mut timelines=std::collections::BTreeSet::new();
        for slot in ids {
            let signature=(0..104).filter_map(|age|visual_phase(slot,age as f64)
                .map(|(stage,_)|stage as u8)).collect::<Vec<_>>();
            assert_eq!(signature.len(),104);
            assert!(timelines.insert(signature),"duplicate rhythm slot {slot}");
        }
        assert!(visual_phase(0,f64::NAN).is_none());
        assert!(visual_phase(0,-1.).is_none());
        assert!(visual_phase(0,104.).is_none());
    }

 }