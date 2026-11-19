//! M27B: final fifteen source-named Standard rank 12–14 effects, schools 6–10.
//! Individually sculpted world-space 3D drafts, not functioning world spells.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
pub(super) const IDS:[u16;15]=[112,113,114,132,133,134,152,153,154,172,173,174,192,193,194];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn stage(id:u16,age:f64)->Option<(Stage,f64)>{
 if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
 let sequence=for_spell(id)?;
 let time=age/104.*sequence.total_ticks() as f64;
 let mut passed=0.;
 for part in sequence.phases{
  let finish=passed+part.duration_ticks as f64;
  if time<finish{
    return Some((part.kind,((time-passed)/part.duration_ticks as f64).clamp(0.,1.)));
  }
  passed=finish;
 }
 None
}
fn draw(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32) {
 let t=age*0.026;
 match id{
 112=>{ // POLLEN CYCLONE: twisting floral cyclone filled with layered petals.
  for i in 0..3{
    curtain(s,p(o,0.,0.1*g,0.),(1.5+0.28*i as f64)*g,3.0*g,
      age,i as f64*1.7,tint(c,0.72,0.24));
  }
  for i in 0..14{
    let a=TAU*i as f64/14.+t;
    let h=(0.40+0.21*i as f64)*g;
    let r=(0.42+0.10*i as f64)*g;
    let base=p(o,r*a.cos(),h,r*a.sin());
    wing(s,base,a,0.69*g,0.41*g,tint(c,0.83+0.02*(i%5) as f64,0.64));
    if i%3==0{crystal(s,base,p(base,0.,0.38*g,0.),0.12*g,a,tint(c,0.98,0.89));}
  }
 },
 113=>{ // CARNIVORE BLOOM: plant jaws split like living clamping petals.
   crystal(s,p(o,0.,-0.65*g,0.),p(o,0.,1.72*g,0.),
     0.72*g,t,tint(c,0.52,0.97));
   for jaw in [-1.0,1.0]{
    for i in 0..7{
      let a=(i as f64-3.)*0.31;
      let x=(0.3+0.22*i as f64)*g;
      let root=p(o,jaw*0.40*g,1.04*g,0.);
      let mid=p(o,jaw*1.13*g,(2.0+0.21*a.sin())*g,a.cos()*0.36*g);
      let tip=p(o,jaw*(1.8+0.4*a.sin())*g,
          (2.85+0.50*a.cos())*g,(i as f64-3.)*0.37*g);
      let flank=[-jaw*0.33*g,0.17*g,0.26*g];
      s.membrane(root,add(mid,flank),tip,sub(mid,flank),
         tint(c,0.62+0.037*(i%5) as f64,0.85));
      if i%2==0{
        crystal(s,p(mid,0.,0.1*g,0.),p(mid,0.,0.68*g,0.),
          0.17*g,a,tint(c,0.96,0.90));
      }
      let _=x;
    }
   }
   body(s,p(o,0.,1.55*g,0.),[0.56*g,0.22*g,0.53*g],tint(c,0.82,0.96));
 },
 114=>{ // SPORE BURST: seven chunky split seed-sacs and scattered spores.
  for i in 0..8{
    let a=TAU*i as f64/8.+t*0.06;
    let r=(0.8+0.23*(i%3) as f64)*g;
    let center=p(o,r*a.cos(),(0.6+0.23*(i%3) as f64)*g,r*a.sin());
    let crown=p(center,0.,(0.78+0.18*(i%3) as f64)*g,0.);
    crystal(s,p(center,0.,-0.25*g,0.),crown,
       (0.34+0.09*(i%3) as f64)*g,a,tint(c,0.62+0.035*(i%5) as f64,0.91));
    for leaf in 0..3 {
     let b=a+leaf as f64*TAU/3.;
     wing(s,p(center,0.,0.1*g,0.),b,0.75*g,0.64*g,
         tint(c,0.84,0.71));
    }
  }
  floor_slab(s,o,1.9*g,0.20*g,t*0.1,tint(c,0.55,0.64));
 },
 132=>{ // SKYFIRE CROWN: physically branched thunder tree from a cloud base.
  body(s,p(o,0.,4.9*g,0.),[1.55*g,0.45*g,1.28*g],tint(c,0.49,0.68));
  let root=p(o,0.,4.6*g,0.);
  let trunk=p(o,0.23*g,1.5*g,0.11*g);
  crystal(s,root,trunk,0.60*g,t,tint(c,0.95,0.95));
  for i in 0..7{
    let a=TAU*i as f64/7.;
    let from=p(o,0.18*g*a.cos(),2.6*g,0.18*g*a.sin());
    let fork=p(o,1.10*g*a.cos(),(1.45+0.13*(i%3) as f64)*g,
                 1.10*g*a.sin());
    crystal(s,from,fork,0.25*g,a,tint(c,0.77,0.88));
    crystal(s,fork,p(fork,0.66*g*a.cos(),-0.76*g,0.66*g*a.sin()),
        0.13*g,a+0.27,tint(c,0.98,0.91));
  }
 },
 133=>{ // PULSE CAGE: dense dynamic multi-sided containment crystal panes.
  for ring in 0..3{
   let y=(0.28+ring as f64*1.12)*g;
   let next=(0.95+ring as f64*1.14)*g;
   for j in 0..8 {
    let a=TAU*j as f64/8.+t*0.16;
    let b=TAU*(j+1) as f64/8.+t*0.16;
    let ra=(1.8+0.16*(ring%2) as f64)*g;
    let aa=p(o,ra*a.cos(),y,ra*a.sin());
    let ab=p(o,ra*b.cos(),y,ra*b.sin());
    let ac=p(o,(ra-0.17*g)*b.cos(),next,(ra-0.17*g)*b.sin());
    let ad=p(o,(ra-0.17*g)*a.cos(),next,(ra-0.17*g)*a.sin());
    s.quad([aa,ab,ac,ad],tint(c,0.62+0.035*(j%4) as f64,0.53));
    if j%2==0 {
     crystal(s,aa,ad,0.23*g,a,tint(c,0.87,0.90));
    }
   }
  }
  crystal(s,p(o,0.,0.3*g,0.),p(o,0.,3.6*g,0.),
    0.29*g,t,tint(c,0.94,0.82));
 },
 134=>{ // STORM FEATHERS: curved broad high-energy blades, no naked sticks.
  for i in 0..17{
    let a=TAU*i as f64/17.+t*0.24;
    let r=(0.40+0.16*(i%5) as f64)*g;
    let root=p(o,r*a.cos(),(0.6+0.33*(i%5) as f64)*g,r*a.sin());
    let upper=p(root,0.37*g*a.cos(),0.75*g,0.31*g*a.sin());
    let tip=p(upper,(1.15+0.15*(i%3) as f64)*g*a.cos(),
                  0.44*g,(1.10+0.15*(i%3) as f64)*g*a.sin());
    let tangent=[-0.39*g*a.sin(),0.10*g,0.39*g*a.cos()];
    s.membrane(root,add(upper,tangent),tip,sub(upper,tangent),
       tint(c,0.64+0.025*(i%5) as f64,0.77));
    if i%3==0{
     crystal(s,p(root,0.,0.2*g,0.),upper,0.13*g,a,tint(c,0.95,0.80));
    }
  }
 },
 152=>{ // STARLIGHT GATE: an actually filled 3D dimensional arch doorway.
  for side in [-1.0,1.0]{
    let foot=p(o,side*2.0*g,-0.35*g,0.);
    let shoulder=p(foot,0.,3.15*g,0.);
    crystal(s,foot,shoulder,0.53*g,side,tint(c,0.53,0.95));
    let cap=p(o,0.,4.75*g,0.);
    s.membrane(shoulder,p(shoulder,-side*0.23*g,0.35*g,0.),
       cap,p(o,side*0.35*g,4.20*g,0.),
       tint(c,0.69,0.81));
  }
  for i in 0..8 {
    let u=i as f64/8.;let v=(i+1) as f64/8.;
    let place=|t:f64,y:f64|{
      p(o,(t*3.22-1.61)*g,y*g,
       (0.31*(t*TAU+age*0.035).sin()+0.09*y)*g)
    };
    s.quad([place(u,0.35),place(v,0.35),
       place(v,3.95),place(u,3.95)],
       tint(c,0.67+0.03*(i%4) as f64,0.61));
  }
 },
 153=>{ // VOID SPIRAL: physically broad torn space ribbons around dark well.
  for strand in 0..4{
    let mut previous=p(o,0.,0.2*g,0.);
    for i in 0..11{
      let u=(i+1) as f64/11.;
      let angle=TAU*strand as f64/4.+u*TAU*1.38+t;
      let r=(0.4+2.0*u)*g;
      let next=p(o,r*angle.cos(),(0.3+3.8*u)*g,
                         r*angle.sin());
      let mid=add(previous,scale(sub(next,previous),0.52));
      let side=[-angle.sin()*0.60*g,0.17*g,angle.cos()*0.60*g];
      s.membrane(previous,add(mid,side),next,sub(mid,side),
          tint(c,0.44+0.055*(strand%3) as f64,0.69));
      previous=next;
    }
  }
  crystal(s,p(o,0.,0.54*g,0.),p(o,0.,3.30*g,0.),
     0.88*g,t,tint(c,0.33,0.96));
 },
 154=>{ // FALLING CONSTELLATION: scattered distinct faceted comets and wakes.
   for i in 0..11{
     let a=TAU*i as f64/11.;
     let r=(0.44+0.19*(i%5) as f64)*g;
     let x=r*a.cos();let z=r*a.sin();
     let top=p(o,x,(3.0+0.24*(i%7) as f64)*g,z);
     let bottom=p(top,0.,-(1.15+0.28*(i%4) as f64)*g,0.);
     crystal(s,top,bottom,(0.23+0.06*(i%3) as f64)*g,
       a,tint(c,0.75+0.035*(i%5) as f64,0.91));
     let middle=add(top,scale(sub(bottom,top),0.40));
     wing(s,middle,a,0.72*g,1.17*g,
       tint(c,0.94,0.60));
   }
 },
 172=>{ // FUTURE FLICKER: offset translucent human-scale spectral echoes.
  for echo in 0..3{
    let shift=(echo as f64-1.)*1.35*g;
    let alpha=0.32+0.23*echo as f64;
    let at=p(o,shift,0.,echo as f64*0.20*g);
    body(s,p(at,0.,1.68*g,0.),[0.38*g,0.78*g,0.24*g],
       tint(c,0.56+0.06*echo as f64,alpha));
    body(s,p(at,0.,2.72*g,0.),[0.33*g,0.34*g,0.31*g],
       tint(c,0.84,alpha+0.07));
    for side in [-1.0,1.0]{
      body(s,p(at,side*0.50*g,1.59*g,0.),[0.15*g,0.61*g,0.19*g],
         tint(c,0.69,alpha));
      body(s,p(at,side*0.21*g,0.55*g,0.),[0.16*g,0.52*g,0.18*g],
         tint(c,0.76,alpha));
    }
    wing(s,p(at,0.,1.64*g,0.),PI*0.28+echo as f64,
      0.66*g,1.8*g,tint(c,0.80,alpha*0.78));
  }
 },
 173=>{ // PENDULUM EDGE: pair of broad scything time blades, different clocks.
   for blade in 0..2{
     let side=if blade==0{-1.0}else{1.0};
     let a=side*(0.35+(age*0.043+blade as f64*1.3).sin()*0.48);
     let start=p(o,side*1.15*g,3.40*g,0.);
     let tip=p(start,(2.9*a.sin())*g,-(2.0+0.3*blade as f64)*g,
       0.47*g*side);
     crystal(s,start,tip,0.53*g,a,tint(c,0.66+0.13*blade as f64,0.93));
     for i in 0..5{
        let u=i as f64/5.;let v=(i+1) as f64/5.;
        let at=add(start,scale(sub(tip,start),u));
        let next=add(start,scale(sub(tip,start),v));
        let broad=p(at,0.57*g*side,0.13*g,-0.39*g);
        let broad2=p(next,0.47*g*side,0.28*g,0.29*g);
        s.quad([at,broad,broad2,next],tint(c,0.77,0.68));
     }
   }
   body(s,p(o,0.,3.55*g,0.),[1.5*g,0.23*g,0.41*g],tint(c,0.53,0.82));
 },
 174=>{ // CHRONAL BLOOM: phase-specific broad clock-flower petals.
   crystal(s,p(o,0.,-0.45*g,0.),p(o,0.,2.32*g,0.),
     0.62*g,t,tint(c,0.86,0.88));
   for i in 0..12{
     let a=TAU*i as f64/12.+age*0.006;
     let root=p(o,0.47*g*a.cos(),1.02*g,0.47*g*a.sin());
     let mid=p(o,1.41*g*a.cos(),(2.5+0.31*(i%3) as f64)*g,1.41*g*a.sin());
     let tip=p(o,(2.4+0.12*(i%4) as f64)*g*a.cos(),
        (2.05+0.25*(i%3) as f64)*g,
        (2.4+0.12*(i%4) as f64)*g*a.sin());
     let flank=[-0.42*g*a.sin(),0.15*g,0.42*g*a.cos()];
     s.membrane(root,add(mid,flank),tip,sub(mid,flank),
        tint(c,0.62+0.03*(i%5) as f64,0.84));
   }
 },
 192=>{ // SPECTRAL SCYTHE: sweeping solid crescent blade dominates the scene.
   let center=p(o,0.,2.12*g,0.);
   for i in 0..13{
      let a=-1.12+i as f64*2.24/13.;
      let b=-1.12+(i+1) as f64*2.24/13.;
      let outer=|theta:f64|p(center,2.4*g*theta.cos(),2.4*g*theta.sin(),0.);
      let inner=|theta:f64|p(center,(1.33+0.32*theta.cos())*g*theta.cos(),
                          (1.33+0.32*theta.cos())*g*theta.sin(),0.52*g);
      s.quad([outer(a),outer(b),inner(b),inner(a)],
           tint(c,0.64+0.02*(i%6) as f64,0.89));
      s.quad([p(outer(a),0.,0.,-0.33*g),p(outer(b),0.,0.,-0.33*g),
         outer(b),outer(a)],tint(c,0.48,0.84));
   }
   crystal(s,p(o,-1.15*g,-0.42*g,0.15*g),
      p(o,0.36*g,3.0*g,0.18*g),0.25*g,0.,tint(c,0.68,0.91));
 },
 193=>{ // GRAVETIDE: roiling dense ghost tide and anchored gravestones.
   for sheet in 0..3{
     curtain(s,p(o,(sheet as f64-1.)*1.10*g,-0.36*g,0.),
       (1.75+0.21*sheet as f64)*g,2.15*g,age,
       sheet as f64*0.72,tint(c,0.61+0.08*sheet as f64,0.23));
   }
   for i in 0..5 {
     let a=TAU*i as f64/5.;
     let at=p(o,2.03*g*a.cos(),0.,2.03*g*a.sin());
     body(s,p(at,0.,0.65*g,0.),[0.40*g,0.70*g,0.21*g],
       tint(c,0.50+0.05*(i%4) as f64,0.88));
     crystal(s,p(at,0.,1.15*g,0.),p(at,0.,1.60*g,0.),
         0.27*g,a,tint(c,0.77,0.80));
   }
 },
 194=>{ // BONE PALISADE: nine irregular thick interlocking vertebral walls.
  for i in 0..9{
    let x=(i as f64-4.)*0.76*g;
    let y=(2.0+0.22*(i%4) as f64)*g;
    let root=p(o,x,-0.5*g,0.);
    let joint=p(o,x,y*0.58,0.30*g*((i%3) as f64-1.));
    let crown=p(o,x+0.21*g,y,0.12*g);
    crystal(s,root,joint,0.36*g,0.22*i as f64,tint(c,0.62,0.95));
    crystal(s,joint,crown,0.27*g,0.35*i as f64,tint(c,0.78,0.94));
    for side in [-1.0,1.0]{
      let rib=p(o,x+0.38*g*side,(1.35+0.15*(i%3) as f64)*g,0.7*g);
      crystal(s,joint,rib,0.20*g,0.25*i as f64,
        tint(c,0.87,0.90));
    }
  }
  body(s,p(o,0.,-0.46*g,0.),[3.7*g,0.23*g,0.44*g],tint(c,0.49,0.86));
 },
 _=>unreachable!("M27B only accepts fifteen ID-specific scenes")
 }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
 let id=catalog200::id(cast.kind);
 if !dedicated(id){return;}
 let Some((kind,progress))=stage(id,age)else{return;};
 let growth=match kind{
  Stage::Telegraph=>0.13+progress*0.27,
  Stage::Cast=>0.40+progress*0.60,
  Stage::Impact=>1.04+0.04*progress,
  Stage::Afterglow=>(1.0-progress).max(0.07)
 };
 let prev=s.facets.len();
 draw(s,id,cast.center,age,growth,
    catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
 if kind==Stage::Afterglow{
   for (_,col) in &mut s.facets[prev..]{
     let alpha=((*col>>24)&255) as f64/255.;
     *col=(*col&0x00FFFFFF)|
        (((alpha*(1.-progress).powf(0.75)*255.).round() as u32)<<24);
   }
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn fifteen_filled_sculptures_render_in_3d_without_cloning_or_clipping(){
  let mut unique=std::collections::BTreeSet::new();
  for (index,id) in IDS.iter().copied().enumerate(){
    assert_eq!((id-1)/20,index as u16/3+5);
    let slot=(id-1) as usize;
    let age=(0..104).find(|tick|matches!(
      stage(id,*tick as f64),Some((Stage::Cast,p)) if p>0.55
    )).unwrap() as f64;
    let mut scene=Scene::new(500);
    render(Cast{kind:slot,variant:4,start:0,
      center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut scene);
    assert!(scene.facets.len()>=14,"not enough visible faces {id}");
    assert!(scene.facets.len()<500,"hard capped; incomplete scene {id}");
    assert!(scene.facets.iter().all(|(q,_)|
      q.iter().flatten().all(|v|v.is_finite())),"not finite {id}");
    let signature=scene.facets.iter().flat_map(|(q,_)|
      q.iter().flat_map(|v|v.iter().map(|n|(*n*100.).round() as i64))
    ).collect::<Vec<_>>();
    assert!(unique.insert(signature),"duplicate shape independent of color {id}");
  }
  assert_eq!(unique.len(),15);
 }
}
