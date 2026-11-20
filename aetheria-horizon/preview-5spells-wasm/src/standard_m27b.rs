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
 113=>{ // CARNIVORE BLOOM: huge flowering JAWS with broad thick living petals.
  crystal(s,p(o,0.,-0.65*g,0.),p(o,0.,1.80*g,0.),
    0.73*g,t,tint(c,0.46,0.95));
  // Twelve independent broad recurved leaves make a single blooming maw,
  // not two disconnected arms or a thicket of vertical toothpicks.
  for petal in 0..12{
    let a=TAU*petal as f64/12.+age*0.008;
    let inner=p(o,0.53*g*a.cos(),1.35*g,0.53*g*a.sin());
    let mid=p(o,1.38*g*a.cos(),(2.78+0.20*(petal%3) as f64)*g,
                     1.38*g*a.sin());
    let tip=p(o,(2.3+0.15*(petal%4) as f64)*g*a.cos(),
                 (2.27+0.26*(petal%3) as f64)*g,
                 (2.3+0.15*(petal%4) as f64)*g*a.sin());
    let flank=[-0.53*g*a.sin(),0.17*g,0.53*g*a.cos()];
    s.membrane(inner,add(mid,flank),tip,sub(mid,flank),
       tint(c,0.59+0.03*(petal%6) as f64,0.87));
    s.membrane(p(inner,0.,-0.18*g,0.),
         add(mid,scale(flank,0.87)),
         p(tip,0.,-0.20*g,0.),
         sub(mid,scale(flank,0.87)),
         tint(c,0.44+0.03*(petal%4) as f64,0.72));
    if petal%3==0{
      let fang=p(mid,0.,0.15*g,0.);
      crystal(s,fang,p(fang,0.17*g*a.cos(),-0.72*g,0.17*g*a.sin()),
       0.17*g,a,tint(c,0.91,0.87));
    }
  }
  body(s,p(o,0.,2.16*g,0.),[0.60*g,0.20*g,0.59*g],tint(c,0.32,0.95));
 }
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
 132=>{ // SKYFIRE CROWN: complex layered thundercloud with a full branching bolt.
  // Replace the flat roof box with irregular sculpted cloud masses.
  for i in 0..9{
    let a=TAU*i as f64/9.;
    let r=(0.6+0.37*(i%4) as f64)*g;
    let height=(4.85+0.19*(i%3) as f64)*g;
    body(s,p(o,r*a.cos(),height,r*a.sin()),
       [(0.53+0.10*(i%3) as f64)*g,
        (0.39+0.08*(i%4) as f64)*g,
        (0.52+0.09*(i%3) as f64)*g],
       tint(c,0.36+0.042*(i%5) as f64,0.66));
  }
  let top=p(o,0.,4.66*g,0.);
  let bottom=p(o,0.19*g,1.60*g,0.08*g);
  crystal(s,top,bottom,0.73*g,t,tint(c,0.96,0.94));
  for i in 0..8 {
    let a=TAU*i as f64/8.;
    let joint=p(o,0.21*g*a.cos(),2.68*g,0.21*g*a.sin());
    let branch=p(o,1.48*g*a.cos(),
      (1.32+0.25*(i%3) as f64)*g,1.48*g*a.sin());
    crystal(s,joint,branch,0.26*g,a,tint(c,0.83,0.88));
    let fork=p(branch,0.54*g*a.cos(),-0.73*g,0.54*g*a.sin());
    crystal(s,branch,fork,0.17*g,a,tint(c,0.99,0.92));
    s.membrane(joint,p(joint,-0.24*g*a.sin(),0.37*g,0.24*g*a.cos()),
      branch,p(branch,0.20*g*a.sin(),0.07*g,-0.20*g*a.cos()),
      tint(c,0.85,0.53));
  }
 }
 133=>{ // PULSE CAGE: volumetric octagonal force-field chamber, NOT 3 rings.
  // Full height faceted glass panels form one recognisable solid enclosure.
  for i in 0..8 {
    let a=TAU*i as f64/8.+t*0.12;
    let b=TAU*(i+1) as f64/8.+t*0.12;
    let r=1.92*g;
    let foot=p(o,r*a.cos(),-0.14*g,r*a.sin());
    let foot2=p(o,r*b.cos(),-0.14*g,r*b.sin());
    let top=p(o,0.89*r*a.cos(),3.70*g,0.89*r*a.sin());
    let top2=p(o,0.89*r*b.cos(),3.70*g,0.89*r*b.sin());
    s.quad([foot,foot2,top2,top],
      tint(c,0.58+0.03*(i%4) as f64,0.53));
    let crown=p(o,0.,4.36*g,0.);
    s.membrane(top,top2,crown,p(o,0.54*g*a.cos(),4.04*g,0.54*g*a.sin()),
      tint(c,0.79,0.68));
    // Broad, interlocking material seams without a hollow outline frame.
    if i%2==0{
      let mid=p(o,1.31*g*a.cos(),2.18*g,1.31*g*a.sin());
      crystal(s,foot,mid,0.24*g,a,tint(c,0.87,0.81));
      crystal(s,mid,top2,0.19*g,a+0.38,tint(c,0.95,0.85));
    }
  }
  crystal(s,p(o,0.,0.3*g,0.),p(o,0.,3.45*g,0.),
     0.63*g,t,tint(c,0.99,0.87));
 }
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
 173=>{ // PENDULUM EDGE: two connected wide sweeping time-blades.
  // The previous version read as two thin sticks from both cameras.
  for blade in 0..2{
    let side=if blade==0{-1.0}else{1.0};
    let phase=age*0.036+blade as f64*1.75;
    let a=side*(0.35+phase.sin()*0.44);
    let root=p(o,side*1.13*g,3.77*g,0.);
    let tip=p(root,(2.6*a.sin())*g,-(2.22+0.21*blade as f64)*g,
        0.40*g*side);
    let flank=[0.81*g*side,0.13*g,0.49*g];
    let mid=add(root,scale(sub(tip,root),0.54));
    // Broad substantial swept blade with both a front and a back.
    s.membrane(root,add(mid,flank),tip,sub(mid,flank),
       tint(c,0.80+0.09*blade as f64,0.89));
    s.membrane(p(root,0.,0.,-0.25*g),
       add(mid,scale(flank,0.85)),p(tip,0.,0.,-0.27*g),
       sub(mid,scale(flank,0.85)),tint(c,0.65,0.78));
    crystal(s,root,tip,0.35*g,a,tint(c,0.92,0.92));
    for i in 0..5{
      let u=(i as f64+0.3)/5.;
      let at=add(root,scale(sub(tip,root),u));
      wing(s,at,a+PI*0.4,0.46*g,0.25*g,
         tint(c,0.96,0.71));
    }
  }
  body(s,p(o,0.,3.80*g,0.),[1.37*g,0.26*g,0.39*g],tint(c,0.54,0.84));
 }
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
 194=>{ // BONE PALISADE: overlapping ribcage bastion, NOT nine bare pickets.
   // The old CPU projection showed a generic fence. Seven broad overlapping
   // ossicle shields now sit on a thick continuous backbone, with hooked
   // shoulder bones and irregular cranial crests at the rim.
   body(s,p(o,0.,0.76*g,-0.26*g),[3.82*g,1.04*g,0.71*g],tint(c,0.55,0.94));
   for i in 0..7{
     let x=(i as f64-3.)*1.05*g;
     let shift=0.13*g*((i%3) as f64-1.);
     let peak=(2.25+0.18*(i%4) as f64)*g;
     let front=(0.64+0.05*(i%3) as f64)*g;
     body(s,p(o,x,0.88*g,-0.08*g+shift),
       [0.56*g,0.93*g,0.49*g],tint(c,0.56+0.04*(i%4) as f64,0.95));
     body(s,p(o,x,1.46*g,0.13*g+shift),
       [0.51*g,0.35*g,0.42*g],tint(c,0.72+0.03*(i%3) as f64,0.90));
     // Front-and-back solid plated shield, not a gap between narrow posts.
     s.membrane(
       p(o,x-0.57*g,0.23*g,front),
       p(o,x-0.42*g,1.65*g,front+0.18*g),
       p(o,x+0.13*g,peak,front+0.25*g),
       p(o,x+0.57*g,0.27*g,front),
       tint(c,0.71+0.035*(i%3) as f64,0.93));
     crystal(s,p(o,x-0.06*g,0.72*g,front+0.08*g),
       p(o,x+0.13*g,peak+0.17*g,front+0.20*g),
       0.31*g,0.14*i as f64,tint(c,0.86,0.97));
     for side in [-1.,1.]{
       s.membrane(
         p(o,x+side*0.36*g,0.54*g,front-0.24*g),
         p(o,x+side*0.63*g,1.42*g,front-0.02*g),
         p(o,x+side*0.47*g,1.81*g,front-0.45*g),
         p(o,x+side*0.12*g,0.83*g,front-0.60*g),
         tint(c,0.60+0.05*((i+1)%3) as f64,0.89));
     }
   }
   // Four asymmetric broad crown horns protrude from the solid rampart.
   for i in 0..4{
     let x=(i as f64-1.5)*1.78*g;
     let lean=(if i%2==0{-0.23}else{0.23})*g;
     crystal(s,p(o,x,1.48*g,-0.14*g),
       p(o,x+lean,(2.58+0.14*(i%3) as f64)*g,-0.46*g),
       0.38*g,i as f64*0.39,tint(c,0.83,0.95));
   }
   body(s,p(o,0.,-0.38*g,-0.07*g),
     [3.94*g,0.28*g,0.82*g],tint(c,0.50,0.94));
  }
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
    if id==194 {assert!(scene.facets.len()<=400,
        "Bone Palisade exceeded M28 safety headroom: {} faces",scene.facets.len());}
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
