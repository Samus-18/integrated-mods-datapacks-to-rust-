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

 /// Filled fantasy dimensional veil made of hundreds of shaded 3D quad faces.
 /// NO beam(), ring(), tube() or stick scaffold for Blink Fold itself.
 /// Two thick, asymmetrically curved sheets give the illusion of folded space.
 /// In contrast to the older ring + small spikes, most geometry fills the
 /// actual visual silhouette instead of tracing an empty border.
 fn fold_veil(s:&mut Scene,o:V,side:f64,age:f64,grow:f64,color:u32){
     let nx=8usize;let ny=10usize;
     let time=age*0.048;
     let tint=|u:f64,v:f64| {
         let h=(u*2.7+time).sin()*(v*3.9-time*0.7).cos();
         shade_argb(color,(0.46+0.23*h+0.12*(1.0-u*u)).clamp(0.25,0.86))
     };
     let place=|iu:usize,iv:usize,back:bool|{
         let u=iu as f64/nx as f64*2.0-1.0;
         let v=iv as f64/ny as f64;
         // Broad asymmetrical draped silhouette, not a geometric ring.
         let taper=(0.79+0.21*(v*TAU+side).sin())
                   *(0.93+0.07*(u*TAU+time).cos());
         let x=side*2.25+u*(1.34+0.19*(v*7.0).sin())*taper;
         let y=0.17+v*4.9*grow+0.20*(u*4.8+time).sin()*v;
         let z=side*1.22
             +0.85*(1.0-u*u)*((v*TAU+time).sin())
             +0.24*(v*11.0+time*0.67).sin()
             +if back{-0.32-0.12*(u*TAU).cos().abs()}else{0.0};
         p(o,x,y,z)
     };
     // Each side is a genuinely contiguous 3D tessellated membrane: no
     // omitted center, no transparent portal hole, no outline-only mesh.
     for j in 0..ny {
         for i in 0..nx {
             let a=place(i,j,false);
             let b=place(i+1,j,false);
             let c=place(i+1,j+1,false);
             let d=place(i,j+1,false);
             let u=(i as f64+0.5)/nx as f64*2.-1.;
             let v=(j as f64+0.5)/ny as f64;
             let col=tint(u,v);
             s.quad([a,b,c,d],col);
             let a2=place(i,j,true);
             let b2=place(i+1,j,true);
             let c2=place(i+1,j+1,true);
             let d2=place(i,j+1,true);
             s.quad([d2,c2,b2,a2],shade_argb(col,0.61));
             // Wide moving folds across the veil itself; built as surfaces,
             // not separate rods or tubes.
             if (i+j)%5==0 && i+1<nx && j+1<ny {
                 let inner=place(i,j+1,false);
                 let middle=place(i+1,j,true);
                 s.quad([a,middle,c,inner],shade_argb(color,0.72));
             }
         }
     }
     // Solid rim *walls* close the volume, avoiding ghostly flat billboard
     // appearance from an oblique camera. These are quadrilateral surfaces.
     for i in 0..nx {
         for v in [0,ny] {
             s.quad([
                 place(i,v,false),place(i+1,v,false),
                 place(i+1,v,true),place(i,v,true)
             ],shade_argb(color,0.52));
         }
     }
     for j in 0..ny {
         for u in [0,nx] {
             s.quad([
                 place(u,j,false),place(u,j+1,false),
                 place(u,j+1,true),place(u,j,true)
             ],shade_argb(color,0.53));
         }
     }
 }
 /// A wide 3D folded-space ribbon physically bridges the two portal veils.
 /// It sweeps across a quadrilateral surface with a thick back-side, not
 /// four thin beam lines connecting circles.
 fn fold_bridge(s:&mut Scene,o:V,age:f64,grow:f64,color:u32){
     let sections=12usize;let rows=4usize;
     let time=age*0.06;
     let point=|i:usize,j:usize,back:bool|{
         let t=i as f64/sections as f64;
         let u=j as f64/rows as f64*2.0-1.0;
         let bend=(t*TAU*0.72+time).sin();
         let wave=(t*TAU*1.4-time*0.63).sin();
         let width=0.55+0.62*(t*std::f64::consts::PI).sin();
         let x=(t*4.5-2.25)+u*0.12*bend;
         let y=(1.4+1.4*(t*std::f64::consts::PI).sin()
              +u*width*0.68*grow+0.34*wave)*grow;
         let z=(t*2.44-1.22)+u*width*0.78*grow+0.55*bend
             +if back{-0.24}else{0.0};
         p(o,x,y,z)
     };
     for i in 0..sections {
         for j in 0..rows {
             let a=point(i,j,false);
             let b=point(i+1,j,false);
             let c=point(i+1,j+1,false);
             let d=point(i,j+1,false);
             s.quad([a,b,c,d],
                 shade_argb(color,0.58+0.25*((i+j)%3) as f64/2.0));
             s.quad([point(i,j+1,true),point(i+1,j+1,true),
                 point(i+1,j,true),point(i,j,true)],
                 shade_argb(color,0.43));
         }
         for j in [0,rows] {
             s.quad([
                 point(i,j,false),point(i+1,j,false),
                 point(i+1,j,true),point(i,j,true)
             ],shade_argb(color,0.52));
         }
     }
 }

 fn broken_sky(s:&mut Scene,o:V,age:f64,color:u32){
     let rise=(age/20.).clamp(0.,1.);
     let center=p(o,0.,3.8*rise,0.);
     let rotation=age*0.013;
     // A physical tilted astral iris in three dimensions: ten dark glass
     // petals around a dimensional core. Do not merely draw a thin circle.
     for j in 0..10 {
         let a=TAU*j as f64/10.+rotation*0.6;
         let b=TAU*(j+1) as f64/10.+rotation*0.6;
         let outer_a=p(center,2.8*a.cos(),0.20*a.sin(),2.8*a.sin());
         let outer_b=p(center,2.8*b.cos(),0.20*b.sin(),2.8*b.sin());
         let inner_a=p(center,0.75*a.cos(),-0.28,0.75*a.sin());
         let inner_b=p(center,0.75*b.cos(),-0.28,0.75*b.sin());
         s.membrane(inner_a,outer_a,outer_b,inner_b,shade_argb(color,0.36));
         // Raised prismatic rims give the aperture real spatial depth.
         s.tube(outer_a,outer_b,0.19,0.15,a,color,5);
     }
     s.solid_crystal(p(center,0.,-1.25,0.),p(center,0.,1.5,0.),
                     0.72,rotation,color,6);
     rune_seal(s,center,[0.2,0.94,0.2],3.3,rotation,color);
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
             // Seven broad curved ember feathers with both front/back and
             // real edge walls. Distinguish a fan from a bare laser comb.
             for j in 0..7 {
                 let a=(j as f64-3.)*0.32;
                 let spread=(1.35+a.cos()*3.9)*stage;
                 let end=p(o,a.sin()*spread*1.7,1.15+2.5*stage*a.cos(),
                           4.7*stage*a.cos());
                 let inner=p(o,a.sin()*0.3,0.4,0.5);
                 let wing=0.27+0.22*(1.0-a.abs()/1.0).max(0.0);
                 // Broad curved crescent surface with real thickness and
                 // an irregular multi-sided ember core. Screen coverage is
                 // measured from BOTH oblique views in our visual audit.
                 let mid=p(inner,0.62*(end[0]-inner[0]),
                     0.64*(end[1]-inner[1]),0.62*(end[2]-inner[2]));
                 let left=p(mid,-wing*2.5,0.37,wing*0.65);
                 let right=p(mid,wing*2.5,-0.34,-wing*0.65);
                 s.membrane(inner,left,end,right,shade_argb(color,0.86));
                 s.solid_crystal(inner,end,0.38+wing*0.36,a,color,6);
                 beam(s,inner,end,0.18,phase+a,color);
                 s.solid_crystal(end,p(end,0.23*a.sin(),0.95*stage,0.5),
                                 0.30,a,color,5);
             }
             sparks(s,o,9,age,2.8,color);
         }
         (0,_)=>{tendril(s,o,age,4,6.2*stage,color);ring(s,p(o,0.,3.,0.),[0.,1.,0.],2.5,0.1,11,phase,color);}
         // Cryomancy: towering ice field, raised bridge-like crystals, asymmetric shard chorus.
         (1,0)=>{shards(s,o,age,4,color);}
         (1,1)=>{
             // Rime Bridge must look like interlocked *walkable ice plates*,
             // not just an airy chain of tiny spikes.
             for step in 0..8 {
                 let t=step as f64;
                 let x=(t-3.5)*0.55;
                 let z=(t-3.0)*0.83;
                 let rise=(0.4+(t*1.7).sin().abs()*0.44)*stage;
                 let a=p(o,x-0.8,rise,z-0.52);
                 let b=p(o,x+0.8,rise,z-0.52);
                 let c=p(o,x+0.9,rise,z+0.53);
                 let d=p(o,x-0.9,rise,z+0.53);
                 s.membrane(a,b,c,d,shade_argb(color,0.9));
                 let root=p(o,x,-0.65,z);
                 s.solid_crystal(root,p(o,x,rise+0.12,z),0.42,t*0.28,color,6);
                 // Underlying staggered blue icicles provide mass and depth.
                 for side in [-1.0,1.0] {
                     let edge=p(o,x+0.66*side,rise,z);
                     s.solid_crystal(edge,p(edge,side*0.2,-0.95-(step%3) as f64*0.28,0.12),
                                     0.18,t,color,5);
                 }
             }
             ring(s,p(o,0.,0.6,2.),[0.,1.,0.],1.7,0.075,10,phase,color);
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
         (4,1)=>{
             // Flowstep is a cresting spatial wave with translucent *surface*
             // volume, not five empty hoops viewed from an angle.
             for j in 0..6 {
                 let t=j as f64/5.;
                 let wave=1.8+(t*TAU+phase).sin()*0.56;
                 let width=1.4+0.7*(1.0-t);
                 let start=p(o,-width,0.2+0.7*t,5.0*t-1.8);
                 let end=p(o,width,0.2+0.7*t,5.0*t-1.8);
                 let left=p(o,-width*0.62,wave*stage,5.0*t-1.6);
                 let right=p(o,width*0.62,(wave+0.2)*stage,5.0*t-1.6);
                 s.membrane(start,left,right,end,shade_argb(color,0.73));
                 s.tube(start,left,0.095,0.055,phase+t,color,5);
                 s.tube(end,right,0.095,0.055,phase-t,color,5);
             }
             tendril(s,o,age,2,2.1*stage,shade_argb(color,0.8));
             sparks(s,o,7,age,2.2,color);
         }
         (4,_)=>{for j in 0..4 {ring(s,p(o,0.,1.7,0.),[j as f64*0.15+0.22,0.9,0.4],2.1,0.08,13,phase+j as f64*0.65,color);}}
         // Druid: rooted canopy, binding roots and flowering healing sanctuary.
         (5,0)=>{tendril(s,o,age,5,5.9*stage,color);}
         (5,1)=>{tendril(s,o,age,6,2.7*stage,color);for j in 0..6 {let a=TAU*j as f64/6.;s.leaf(p(o,2.3*a.cos(),2.2,2.3*a.sin()),[a.cos(),0.7,a.sin()],1.2,color);}}
         (5,_)=>{
             // Healing sanctuary: a living flower of thick leaves and a
             // rising seed crystal, not a lone decorative ring.
             rune_seal(s,p(o,0.,0.2,0.),[0.,1.,0.],3.1,phase,color);
             for petal in 0..9 {
                 let a=TAU*petal as f64/9.+phase*0.15;
                 let root=p(o,0.55*a.cos(),0.2,0.55*a.sin());
                 let middle=p(o,1.8*a.cos(),1.0+0.5*stage,1.8*a.sin());
                 let tip=p(o,2.9*a.cos(),1.35+0.8*stage,2.9*a.sin());
                 let left=p(middle,-0.42*a.sin(),0.08,0.42*a.cos());
                 let right=p(middle,0.42*a.sin(),0.02,-0.42*a.cos());
                 s.membrane(root,left,tip,right,shade_argb(color,0.86));
                 s.tube(root,middle,0.12,0.065,a,color,5);
             }
             s.solid_crystal(p(o,0.,0.45,0.),p(o,0.,3.7*stage,0.),
                             0.64,phase,color,6);
             sparks(s,o,10,age,2.3,color);
         }
         // Electromancy: branching thunder, charged halo and precision needle.
         (6,0)=>{storm(s,o,age,color);}
         (6,1)=>{
             // The charged crown needs substantial orbiting electrical
             // shards and spatial joints, not just thin empty wire hoops.
             for j in 0..3 {
                 rune_seal(s,p(o,0.,0.8+j as f64*0.92,0.),
                     [0.15+j as f64*0.18,1.,0.32],1.1+j as f64*0.58,
                     phase+j as f64,color);
             }
             for j in 0..12 {
                 let a=TAU*j as f64/12.+phase*0.52;
                 let h=0.9+(j%4) as f64*0.66;
                 let base=p(o,2.2*a.cos(),h,2.2*a.sin());
                 let tip=p(base,0.55*a.cos(),0.95*stage,0.55*a.sin());
                 s.solid_crystal(base,tip,0.24,a,color,6);
             }
             sparks(s,o,8,age,2.0,shade_argb(color,0.9));
         }
         (6,_)=>{
             let strike=p(o,f[0]*9.3,1.+f[1]*9.3,f[2]*9.3);
             // An energized spear with a wide many-sided hard core.
             rune_seal(s,p(o,0.,1.,0.),f,1.2,phase,color);
             beam(s,p(o,0.,1.,0.),strike,0.44,phase,color);
             for j in 0..5 {
                 let t=j as f64/5.;
                 let base=p(o,f[0]*9.3*t,1.+f[1]*9.3*t,f[2]*9.3*t);
                 let top=p(o,f[0]*9.3*(t+0.17),1.+f[1]*9.3*(t+0.17),
                             f[2]*9.3*(t+0.17));
                 s.solid_crystal(base,top,0.47*(1.0-t)+0.13,phase+j as f64,color,6);
             }
             s.solid_crystal(p(o,f[0]*6.2,1.+f[1]*6.2,f[2]*6.2),
                 p(strike,f[0]*0.9,f[1]*0.9,f[2]*0.9),0.58,phase,color,6);
             for j in 0..8 {
                 let a=TAU*j as f64/8.+phase;
                 let mid=p(strike,a.cos()*0.6,0.35,a.sin()*0.6);
                 s.tube(strike,mid,0.085,0.04,a,color,5);
                 s.solid_crystal(mid,p(mid,0.5*a.cos(),0.65*stage,0.5*a.sin()),
                                 0.12,a,color,5);
             }
             storm(s,strike,age,shade_argb(color,0.62));
         }
         // Astral: sky fracture, collapsing gravity lens, folded space frames.
         (7,0)=>{broken_sky(s,o,age,color);}
         (7,1)=>{rune_seal(s,o,[0.3,0.7,0.2],2.4,phase,color);rune_seal(s,p(o,0.,2.3,0.),[0.2,0.7,0.7],1.6,-phase,color);}
         (7,_)=>{
             // BLINK FOLD: two filled, distorted 3D portals with a contiguous
             // dimensional river between them. Deliberately zero rings,
             // rods, beam primitives and stacked thin crystals here.
             for side in [-1.0,1.0] {
                 fold_veil(s,o,side,age,stage.max(0.14),color);
             }
             fold_bridge(s,o,age,stage.max(0.14),color);
         }
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
    fn every_cast_has_distinct_geometry_not_only_different_rgb(){
        use std::collections::BTreeSet;
        let mut geometries=BTreeSet::new();
        for slot in 0..30 {
            // Compare at the middle of each spell's individual Cast stage.
            // Ignore all colors: equal shapes with different palettes fail.
            let age=(0..104).find(|tick|matches!(
                visual_phase(slot,*tick as f64),
                Some((Stage::Cast,p)) if p>0.55
            )).expect("Cast stage missing") as f64;
            let mut scene=Scene::new(900);
            draw(Cast{kind:slot,variant:0,start:0,
                 center:[0.0,64.0,0.0],direction:[0.0,0.0,1.0]},age,&mut scene);
            assert!(!scene.facets.is_empty(),"slot {slot} has no geometry");
            let shape=scene.facets.iter().flat_map(|(corners,_)|
                corners.iter().flat_map(|point|point.iter().map(|v|
                    (v*100.0).round() as i64
                ))
            ).collect::<Vec<_>>();
            assert!(geometries.insert(shape),"same full spatial mesh at slot {slot}");
        }
        assert_eq!(geometries.len(),30);
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

    #[test]
    fn blink_fold_has_dense_filled_surfaces_in_both_oblique_views(){
        let slot=7*3+2;
        let age=(0..104).find(|t|matches!(
            visual_phase(slot,*t as f64),Some((Stage::Cast,p)) if p>0.55
        )).unwrap() as f64;
        let mut scene=Scene::new(900);
        draw(Cast{kind:slot,variant:2,start:0,
             center:[0.0,64.0,0.0],direction:[0.0,0.0,1.0]},age,&mut scene);
        // Two 8x10 front/back sheets and a 12x4 dimensional bridge are
        // genuine filled panels; previous ring+stick model did not have this.
        assert!(scene.facets.len()>=300,"blink fold devolved to a stick outline");
        assert!(scene.facets.len()<=900);
        for eye in [[0.5,0.38,1.0],[-0.8,0.7,0.55]] {
            let right=norm(cross([0.0,1.0,0.0],eye));
            let up=norm(cross(eye,right));
            let mut area=0.0;
            let mut broad=0;
            for (corners,_) in &scene.facets{
                let q=corners.map(|p|[
                    p.iter().zip(right).map(|(a,b)|a*b).sum::<f64>(),
                    p.iter().zip(up).map(|(a,b)|a*b).sum::<f64>()
                ]);
                let mut twice=0.0;
                for i in 0..4{
                    let j=(i+1)%4;
                    twice+=q[i][0]*q[j][1]-q[j][0]*q[i][1];
                }
                let a=twice.abs()*0.5;
                area+=a;
                if a>0.015{broad+=1}
            }
            assert!(area>9.0,"blink fold has too little filled area");
            assert!(broad>120,"too many edge-on or degenerate strands");
        }
    }

 }