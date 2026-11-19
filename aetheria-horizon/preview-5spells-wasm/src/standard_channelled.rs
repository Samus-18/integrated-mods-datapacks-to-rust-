//! M24: ten source-specific Channeled Standard spell designs (local rank 6).
//! Their geometry is a bespoke visual DRAFT. Server DesignIntent is still
//! explicitly rejected; no actual block edits, teleport or authoritative damage.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,orb,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};

const IDS:[u16;10]=[6,26,46,66,86,106,126,146,166,186];
pub(super) fn dedicated(id:u16)->bool {IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
    if !age.is_finite()||!(0.0..104.0).contains(&age){return None;}
    let t=for_spell(id)?;
    let tick=age/104.*t.total_ticks() as f64;
    let mut last=0.;
    for v in t.phases {
        let finish=last+v.duration_ticks as f64;
        if tick<finish{return Some((v.kind,((tick-last)/(finish-last)).clamp(0.,1.)))}
        last=finish;
    }
    None
}
fn scene(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
    let time=age*0.023;
    match id {
      6=>{ // FIREBRAND SIGIL: a forged, engraved disk of solid molten floor.
          // Not a naked glowing circle: seven irregular fire-etched metal plates.
          floor_slab(s,o,2.65*g,0.44*g,time*0.09,tint(c,0.54,0.91));
          for i in 0..7{
              let a=TAU*i as f64/7.;
              let r=(1.04+0.16*(i%3) as f64)*g;
              let center=p(o,r*a.cos(),-0.19*g,r*a.sin());
              body(s,center,[0.29*g,0.10*g,0.57*g],tint(c,0.64+0.028*i as f64,0.87));
              let spike=p(center,0.,(0.58+0.14*(i%4) as f64)*g,0.);
              crystal(s,center,spike,0.22*g,a,tint(c,0.91,0.86));
          }
          crystal(s,p(o,0.,-0.45*g,0.),p(o,0.,2.0*g,0.),
                  0.60*g,time,tint(c,0.95,0.79));
      },
      26=>{ // HOARFROST TRAIL: icy floor path, split crystalline branches.
          for step in 0..9{
              let t=step as f64/8.;
              let z=(t-0.5)*8.2*g;
              let skew=(t*TAU*1.35+time).sin()*0.49*g;
              let center=p(o,skew,-0.35*g,z);
              body(s,center,[0.71*g,0.22*g,0.57*g],tint(c,0.67+0.021*step as f64,0.79));
              for side in [-1.,1.] {
                  let base=p(center,side*0.48*g,0.08*g,0.);
                  let tip=p(base,side*(0.68+0.12*(step%3) as f64)*g,
                            (0.62+0.18*(step%4) as f64)*g,0.18*g);
                  crystal(s,base,tip,0.32*g,side*0.45,tint(c,0.82,0.86));
              }
          }
      },
      46=>{ // CYCLONE STRIDE: lateral stacked current-plates follow a dash.
          for step in 0..9 {
              let t=step as f64/8.;
              let a=t*TAU*1.3+time;
              let z=(t-0.5)*8.0*g;
              let x=1.2*g*a.sin();
              let base=p(o,x,0.09*g,z);
              body(s,base,[0.52*g,0.13*g,0.71*g],tint(c,0.63+0.02*step as f64,0.62));
              for side in [-1.,1.] {
                  let tip=p(base,side*1.08*g,(1.28+0.21*(step%3) as f64)*g,
                            0.57*g);
                  let mid=p(base,side*0.67*g,0.64*g,-0.29*g);
                  s.membrane(base,mid,tip,p(base,side*0.47*g,0.62*g,0.47*g),
                      tint(c,0.72,0.45));
              }
          }
      },
      66=>{ // GRANITE ORBIT: protective swarm of real chunky stone objects.
          for i in 0..11{
              let a=TAU*i as f64/11.+time*0.63;
              let r=(1.5+0.19*(i%4) as f64)*g;
              let center=p(o,r*a.cos(),(1.5+0.27*(i%5) as f64)*g,r*a.sin());
              body(s,center,[(0.37+0.11*(i%3) as f64)*g,
                    (0.46+0.08*(i%4) as f64)*g,
                    (0.38+0.11*(i%2) as f64)*g],
                    tint(c,0.59+0.027*(i%4) as f64,0.93));
              if i%3==0 {
                  crystal(s,p(center,0.,0.,0.),p(center,0.19*g,0.80*g,0.),
                     0.22*g,a,tint(c,0.79,0.88));
              }
          }
      },
      86=>{ // SURGE FAN: sideways breaking sheets with foaming free edges.
          for i in 0..7 {
              let t=i as f64/6.;
              let x=(i as f64-3.)*0.52*g;
              let phase=time+t*1.5;
              let root=p(o,x,-0.38*g,-0.4*g);
              let left=p(o,x-0.66*g,1.06*g,1.14*g);
              let tip=p(o,x+0.2*g,(1.98+0.20*phase.sin())*g,2.82*g);
              let right=p(o,x+0.74*g,0.73*g,1.53*g);
              s.membrane(root,left,tip,right,tint(c,0.64+0.04*(i%3) as f64,0.54));
              let foam=p(tip,0.10*g,0.29*g,0.24*g);
              crystal(s,tip,foam,0.14*g,phase,tint(c,0.96,0.82));
          }
          floor_slab(s,p(o,0.,-0.38*g,1.0*g),2.5*g,0.23*g,
                     0.17,tint(c,0.57,0.54));
      },
      106=>{ // SEEDBURST: ten stagger-growth seedpods with opening thick leaves.
          for pod in 0..10 {
              let a=TAU*pod as f64/10.;
              let radius=(1.25+0.27*(pod%4) as f64)*g;
              let location=p(o,a.cos()*radius,-0.26*g,a.sin()*radius);
              let local=(g-0.07*(pod%4) as f64).max(0.12);
              let high=(0.62+0.24*(pod%4) as f64)*local;
              let mid=p(location,0.,0.56*high,0.);
              crystal(s,location,p(location,0.,high,0.),0.36*local,a,
                  tint(c,0.74,0.90));
              for leaf in 0..4 {
                  let u=a+leaf as f64*TAU/4.+time*0.09;
                  wing(s,mid,u,(0.62+0.08*leaf as f64)*local,
                      (0.42+0.09*(pod%3) as f64)*local,tint(c,0.82,0.82));
              }
          }
      },
      126=>{ // FLASHSTEP: volumetric trailing silhouettes, not a dotted line.
          for k in 0..5 {
              let t=k as f64/4.;
              let x=(t-0.5)*5.4*g;
              let z=0.28*g*(k%2) as f64;
              let alpha=0.80-0.13*k as f64;
              let center=p(o,x,1.45*g,z);
              body(s,center,[0.40*g,0.85*g,0.34*g],tint(c,0.69,alpha));
              body(s,p(center,0.,1.27*g,0.),
                  [0.38*g,0.39*g,0.34*g],tint(c,0.85,alpha));
              for side in [-1.,1.]{
                  crystal(s,p(center,side*0.41*g,0.52*g,0.),
                      p(center,side*0.83*g,-0.56*g,0.11*g),
                      0.25*g,0.2*side,tint(c,0.72,alpha));
              }
          }
      },
      146=>{ // COMET PEARL: big luminous faceted stone with falling fragments.
          orb(s,p(o,0.,3.05*g,0.),[1.15*g,1.20*g,1.02*g],
              age,tint(c,0.77,0.86),0.80);
          for i in 0..9 {
              let a=TAU*i as f64/9.+time;
              let t=i as f64/9.;
              let dist=(1.3+2.7*t)*g;
              let center=p(o,dist*a.cos(),(1.25+1.75*(1.-t))*g,
                            dist*a.sin()-1.2*g);
              crystal(s,p(center,0.,-0.30*g,0.),
                  p(center,0.25*a.cos()*g,0.43*g,0.25*a.sin()*g),
                  (0.20+0.04*(i%3) as f64)*g,a,tint(c,0.90,0.78));
          }
      },
      166=>{ // ECHO STEP: physical footprint afterimages and a path memory.
          for k in 0..8 {
              let t=k as f64/7.;
              let x=(k%2) as f64*1.06*g-0.53*g;
              let z=(t-0.5)*8.0*g;
              let center=p(o,x,-0.3*g,z);
              body(s,center,[0.38*g,0.13*g,0.62*g],
                  tint(c,0.71+0.02*k as f64,0.48+0.05*k as f64));
              if k%2==0 {
                  // Temporal fold appears as a low broad slab, not clock rings.
                  s.membrane(p(center,-0.53*g,0.19*g,0.),
                    p(center,-0.43*g,1.04*g,0.),
                    p(center,0.49*g,1.15*g,0.42*g),
                    p(center,0.57*g,0.12*g,0.42*g),tint(c,0.91,0.38));
              }
          }
      },
      186=>{ // WRAITH STEP: a passing ghost mantle and protective floor phasing.
          for k in 0..4 {
              let t=k as f64/3.;
              let loc=p(o,(t-0.5)*4.8*g,0.,(t*PI).sin()*0.24*g);
              let a=0.67-0.13*k as f64;
              body(s,p(loc,0.,1.55*g,0.),
                    [0.60*g,1.26*g,0.42*g],tint(c,0.64,a));
              orb(s,p(loc,0.,2.78*g,0.),
                    [0.43*g,0.50*g,0.36*g],age,
                    tint(c,0.78,0.42),0.42);
              wing(s,p(loc,0.,1.75*g,-0.12*g),PI*0.18,
                   1.04*g,1.42*g,tint(c,0.72,0.35));
          }
      },
      _=>unreachable!("one of exactly ten Channeled Standard spell IDs"),
    }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
    let id=catalog200::id(cast.kind);
    if !dedicated(id){return;}
    let Some((stage,progress))=phase(id,age)else{return;};
    let g=match stage{
        Stage::Telegraph=>0.12+0.23*progress,
        Stage::Cast=>0.35+0.65*progress,
        Stage::Impact=>1.04+0.12*progress,
        Stage::Afterglow=>(1.0-progress).max(0.06)
    };
    let start=s.facets.len();
    scene(s,id,cast.center,age,g,
        catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
    if stage==Stage::Afterglow {
        for (_,col) in &mut s.facets[start..]{
            let a=((*col>>24)&255) as f64/255.;
            *col=(*col&0x00FFFFFF)|
                (((a*(1.-progress).powf(0.82)*255.).round() as u32)<<24);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channelled_scenes_are_distinct_finite_3d_and_not_budget_clipped(){
        use std::collections::BTreeSet;
        let mut signatures=BTreeSet::new();
        for id in IDS {
            let kind=(id-1) as usize;
            let age=(0..104).find(|tick|matches!(
                phase(id,*tick as f64),Some((Stage::Cast,p)) if p>0.63
            )).unwrap() as f64;
            let mut s=Scene::new(500);
            render(Cast{kind,variant:0,start:0,
                center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut s);
            assert!((24..500).contains(&s.facets.len()),
                "too little geometry or clipped scene: {id}, quads={}",s.facets.len());
            assert!(s.facets.iter().all(|(q,_)|
                q.iter().flatten().all(|v|v.is_finite())));
            let signature=s.facets.iter().flat_map(|(q,_)|
                q.iter().flat_map(|pt|pt.iter().map(|v|(*v*100.).round() as i64))
            ).collect::<Vec<_>>();
            assert!(signatures.insert(signature),
                "palette swap or cloned geometry: {id}");
        }
        assert_eq!(signatures.len(),10);
    }
}
