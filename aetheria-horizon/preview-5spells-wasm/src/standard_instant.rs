//! M24: ten distinct source-specific Instant Standard spell VFX (local rank 7).
//! The outer mass is solid and recognizable, not generic thin ring outlines.
//! Stage animation remains client-only: mechanics are DesignIntent until adapter.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,curtain,orb,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{TAU,PI};

const IDS:[u16;10]=[7,27,47,67,87,107,127,147,167,187];
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn phase(id:u16,age:f64)->Option<(Stage,f64)>{
    if !age.is_finite()||!(0.0..104.).contains(&age){return None;}
    let t=for_spell(id)?;
    let clock=age/104.*t.total_ticks() as f64;
    let mut offset=0.;
    for part in t.phases{
        let end=offset+part.duration_ticks as f64;
        if clock<end{return Some((part.kind,((clock-offset)/(end-offset)).clamp(0.,1.)))}
        offset=end;
    }
    None
}
fn scene(s:&mut Scene,id:u16,o:[f64;3],age:f64,g:f64,c:u32){
    let time=age*0.025;
    match id {
      7=>{ // PYRE SERPENT: serpentine solid body with distinct raised head.
          for segment in 0..12 {
              let t=segment as f64/11.;
              let a=t*TAU*1.25+time;
              let x=(t-0.5)*6.5*g;
              let y=(1.1+0.55*a.sin()+0.7*t)*g;
              let z=(0.78*a.cos())*g;
              let center=p(o,x,y,z);
              let width=(0.48-0.18*t)*g;
              body(s,center,[0.41*g,width,width*1.12],
                  tint(c,0.63+0.027*(segment%6) as f64,0.91));
              if segment%2==0{
                  wing(s,p(center,0.,width*0.7,0.),PI*0.35,
                       0.7*g,(0.42+0.12*(segment%3) as f64)*g,
                       tint(c,0.89,0.77));
              }
          }
          let head=p(o,-3.3*g,(1.75+0.32*time.sin())*g,0.78*g);
          crystal(s,p(head,0.,-0.25*g,0.),
                     p(head,-0.93*g,0.35*g,0.16*g),
                     0.62*g,0.27,tint(c,0.95,0.95));
          for side in [-1.,1.] {
              crystal(s,p(head,-0.43*g,0.18*g,side*0.26*g),
                       p(head,-0.74*g,0.79*g,side*0.41*g),
                       0.18*g,side*0.5,tint(c,0.99,0.89));
          }
      },
      27=>{ // PERMAFROST GRIP: articulated five-finger crystal HAND.
          // Initial Cast review: a blank vertical ice block hiding the fingers.
          // Now use a defined wrist, palm, five raised knuckles and claw tips.
          body(s,p(o,0.,0.52*g,0.05*g),[0.49*g,0.67*g,0.37*g],
               tint(c,0.68,0.94));
          body(s,p(o,0.,1.73*g,0.),[1.00*g,0.75*g,0.44*g],
               tint(c,0.73,0.82));
          for finger in 0..5{
              let side=(finger as f64-2.)*0.45;
              let root=p(o,side*g,2.15*g,0.15*g);
              let knuckle=p(root,0.11*side*g,
                     (1.01+0.19*(finger%3) as f64)*g,0.17*g);
              let claw=p(knuckle,0.08*side*g,0.13*g,0.55*g);
              crystal(s,root,knuckle,0.23*g,0.30*finger as f64,
                      tint(c,0.89,0.96));
              crystal(s,knuckle,claw,0.19*g,0.30*finger as f64,
                      tint(c,0.98,0.96));
          }
          // Separate oversized thumb bends toward the enemy at the front.
          crystal(s,p(o,-0.84*g,1.75*g,0.),
                     p(o,-1.54*g,1.98*g,0.53*g),
                     0.42*g,0.15,tint(c,0.80,0.89));
          crystal(s,p(o,-1.54*g,1.98*g,0.53*g),
                     p(o,-1.68*g,1.56*g,0.99*g),
                     0.30*g,0.35,tint(c,0.96,0.89));
      },
      47=>{ // BANSHEE WHISTLE: wide sculpted pressure TRUMPET, not empty rings.
          for i in 0..8{
              let a=TAU*i as f64/8.;
              let b=TAU*(i+1) as f64/8.;
              let back=p(o,0.31*g*a.cos(),1.6*g+0.31*g*a.sin(),0.15*g);
              let back_next=p(o,0.31*g*b.cos(),1.6*g+0.31*g*b.sin(),0.15*g);
              let tip=p(o,(2.2+0.12*(i%3) as f64)*g*a.cos(),
                    1.6*g+1.7*g*a.sin(),4.35*g);
              let tip_next=p(o,(2.2+0.12*((i+1)%3) as f64)*g*b.cos(),
                    1.6*g+1.7*g*b.sin(),4.35*g);
              s.quad([back,back_next,tip_next,tip],
                tint(c,0.50+0.031*(i%4) as f64,0.40));
              let tip_inner=p(tip,-0.10*g,0.15*g,-0.35*g);
              crystal(s,p(back,0.,0.,0.),tip_inner,
                 0.18*g,a,tint(c,0.73,0.66));
          }
          body(s,p(o,0.,1.60*g,0.14*g),[0.50*g,0.53*g,0.46*g],
               tint(c,0.88,0.87));
      },
      67=>{ // ORE SENSE: fractured floating mineral *exposure*.
          // Source review found a featureless dark dinner plate: remove
          // the dominating disk and show actual embedded luminous ore seams.
          for i in 0..11{
              let a=TAU*i as f64/11.;
              let dist=(0.45+0.26*(i%5) as f64)*g;
              let center=p(o,dist*a.cos(),-0.25*g,dist*a.sin());
              body(s,center,[0.44*g,0.17*g,0.37*g],
                 tint(c,0.54+0.04*(i%4) as f64,0.84));
              let lean=0.21*(i%3) as f64;
              let tip=p(center,lean*a.sin()*g,
                          (0.93+0.35*(i%4) as f64)*g,
                          lean*a.cos()*g);
              crystal(s,center,tip,(0.24+0.11*(i%3) as f64)*g,a,
                    tint(c,0.93,0.96));
              if i%3==0 {
                  let side=p(center,0.57*a.cos()*g,0.41*g,0.57*a.sin()*g);
                  crystal(s,center,side,0.20*g,a+0.3,
                      tint(c,0.73,0.91));
              }
          }
      },
      87=>{ // RAINWEAVER: small sculpted raincloud, thick cloud fringe and drops.
          for clump in 0..6 {
              let a=TAU*clump as f64/6.+time*0.18;
              let center=p(o,1.0*g*a.cos(),3.45*g+0.12*g*(clump%3) as f64,
                  0.87*g*a.sin());
              body(s,center,[0.70*g,0.43*g,0.58*g],tint(c,0.67,0.55));
          }
          for i in 0..24 {
              let a=TAU*i as f64/24.;
              let radial=(0.5+0.12*(i%6) as f64)*g;
              let y=(0.5+0.36*(i%7) as f64+0.06*(age*0.19+i as f64).sin())*g;
              let center=p(o,radial*a.cos(),y,radial*a.sin());
              crystal(s,p(center,0.,-0.19*g,0.),
                  p(center,0.,0.22*g,0.),0.13*g,a,
                  tint(c,0.89,0.73));
          }
      },
      107=>{ // VINE SWING: thick rooted grapple vine arches with leaf growth.
          let count=11;
          let mut prev=p(o,-3.1*g,4.1*g,-0.75*g);
          for i in 0..count {
              let t=(i+1) as f64/count as f64;
              let x=(-3.1+6.2*t)*g;
              let y=(1.5+2.6*(1.-(2.*t-1.).abs()).powf(0.73))*g;
              let z=(0.50*(t*TAU+time).sin()-0.75)*g;
              let next=p(o,x,y,z);
              // Vine body has physical radial width rather than hairline.
              s.tube(prev,next,0.25*g,0.31*g,
                    t*TAU,tint(c,0.64,0.92),6);
              if i%2==0 {
                  for side in [-1.,1.] {
                      wing(s,p(next,0.,0.,0.),t*TAU+side*0.63,
                          (0.75+0.08*(i%3) as f64)*g,
                          (0.47+0.1*(i%4) as f64)*g,
                          tint(c,0.89,0.73));
                  }
              }
              prev=next;
          }
          body(s,p(o,-3.2*g,4.0*g,-0.75*g),
                 [0.45*g,0.52*g,0.46*g],tint(c,0.5,0.93));
      },
      127=>{ // MAGNET STONE: single heavy levitating octahedron + flying ore.
          crystal(s,p(o,0.,0.3*g,0.),p(o,0.,4.1*g,0.),
                  1.32*g,time,tint(c,0.58,0.91));
          for i in 0..13 {
              let a=TAU*i as f64/13.+time*0.62;
              let dist=(1.45+0.18*(i%5) as f64)*g;
              let center=p(o,dist*a.cos(),
                    (0.7+0.28*(i%7) as f64)*g,
                    dist*a.sin());
              body(s,center,[0.25*g,0.23*g,0.28*g],
                  tint(c,0.60+0.035*(i%4) as f64,0.89));
          }
      },
      147=>{ // ASTRAL COMPASS: four-armed physical star pointer and inner gem.
          body(s,p(o,0.,2.0*g,0.),[0.42*g,0.45*g,0.56*g],
               tint(c,0.87,0.94));
          for i in 0..8{
              let a=TAU*i as f64/8.+time*0.20;
              let from=p(o,0.20*g*a.cos(),2.0*g,0.20*g*a.sin());
              let end=p(o,(1.7+0.27*(i%2) as f64)*g*a.cos(),
                       (2.0+0.25*(i%3) as f64)*g,
                       (1.7+0.27*(i%2) as f64)*g*a.sin());
              crystal(s,from,end,(0.38+0.07*(i%2) as f64)*g,a,
                  tint(c,0.73+0.026*(i%4) as f64,0.90));
          }
          crystal(s,p(o,0.,1.14*g,0.),
              p(o,0.,3.2*g,0.),0.48*g,time,
              tint(c,0.98,0.95));
      },
      167=>{ // AGING TOUCH: separate weathered stone PALM with hand-like
          // silhouette and cracked *front surface* sundial time seal.
          // Not the former tall featureless slab with little spikes above.
          body(s,p(o,0.,0.48*g,0.),[0.45*g,0.66*g,0.35*g],
               tint(c,0.60,0.94));
          body(s,p(o,0.,1.64*g,0.),[0.97*g,0.79*g,0.47*g],
               tint(c,0.69,0.93));
          for finger in 0..5{
              let side=(finger as f64-2.)*0.42;
              let base=p(o,side*g,2.16*g,0.09*g);
              let high=p(base,0.08*side*g,
                         (0.95+0.26*(finger%3) as f64)*g,
                         0.14*g);
              crystal(s,base,high,(0.22+0.03*(finger%2) as f64)*g,
                      0.17*finger as f64,tint(c,0.81,0.96));
              if finger%2==0{
                  crystal(s,high,p(high,0.11*side*g,0.11*g,0.32*g),
                          0.15*g,0.27*finger as f64,tint(c,0.96,0.94));
              }
          }
          for j in 0..12 {
              let a=TAU*j as f64/12.;
              let center=p(o,0.63*g*a.cos(),
                           1.68*g+0.58*g*a.sin(),0.57*g);
              body(s,center,[0.10*g,0.15*g,0.10*g],
                   tint(c,0.92,0.90));
          }
          // Physical branching fracture across the inlaid time face.
          crystal(s,p(o,-0.55*g,1.22*g,0.56*g),
                    p(o,0.58*g,2.16*g,0.57*g),
                    0.15*g,time,tint(c,0.43,0.85));
          crystal(s,p(o,0.,1.30*g,0.62*g),
                    p(o,0.,2.04*g,0.62*g),
                    0.24*g,time,tint(c,0.94,0.89));
      },
      187=>{ // CARRION SWARM: textured spiritual MOTH WINGS around cloudy core.
          for i in 0..15 {
              let a=TAU*i as f64/15.+time*0.78;
              let radius=(0.75+0.28*(i%5) as f64)*g;
              let center=p(o,a.cos()*radius,
                     (0.95+0.34*(i%6) as f64)*g,
                     a.sin()*radius);
              let flutter=0.25*(time*5.0+i as f64*1.8).sin();
              for side in [-1.,1.] {
                  let base=p(center,0.,0.,0.);
                  let tip=p(center,side*0.50*g,
                    (0.45+flutter)*g,0.20*g);
                  let mid=p(center,side*0.34*g,0.25*g,-0.22*g);
                  s.membrane(base,mid,tip,p(center,side*0.17*g,0.35*g,0.30*g),
                      tint(c,0.72+0.06*(i%3) as f64,0.49));
              }
              body(s,center,[0.13*g,0.19*g,0.11*g],
                  tint(c,0.90,0.78));
          }
      },
      _=>unreachable!("one of exactly ten Instant Standard IDs"),
    }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
    let id=catalog200::id(cast.kind);
    if !dedicated(id){return;}
    let Some((stage,progress))=phase(id,age)else{return;};
    let g=match stage {
        Stage::Telegraph=>0.10+0.22*progress,
        Stage::Cast=>0.33+0.67*progress,
        Stage::Impact=>1.01+0.12*progress,
        Stage::Afterglow=>(1.-progress).max(0.05)
    };
    let from=s.facets.len();
    scene(s,id,cast.center,age,g,
          catalog200::WORLD_COLORS[catalog200::school(cast.kind)]);
    if stage==Stage::Afterglow{
        for (_,color) in &mut s.facets[from..]{
            let a=((*color>>24)&255) as f64/255.;
            *color=(*color&0x00FFFFFF)|
                (((a*(1.-progress).powf(0.77)*255.).round() as u32)<<24);
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn ten_instant_spell_bodies_are_distinct_finite_and_budgeted(){
        use std::collections::BTreeSet;
        let mut signatures=BTreeSet::new();
        for id in IDS {
            let age=(0..104).find(|tick|matches!(
                phase(id,*tick as f64),Some((Stage::Cast,p)) if p>0.63
            )).unwrap() as f64;
            let mut s=Scene::new(500);
            render(Cast{kind:(id-1) as usize,variant:0,start:0,
                center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut s);
            assert!((24..500).contains(&s.facets.len()),
                "thin, empty, or clipped geometry for spell {id}: {}",s.facets.len());
            assert!(s.facets.iter().all(|(q,_)|
                q.iter().flatten().all(|x|x.is_finite())));
            let coords=s.facets.iter().flat_map(|(q,_)|
                q.iter().flat_map(|v|v.iter().map(|v|(*v*100.).round() as i64))
            ).collect::<Vec<_>>();
            assert!(signatures.insert(coords),"geometry clone for {id}");
        }
        assert_eq!(signatures.len(),10);
    }
}
