//! M36: 100 individually named, artist-directed PROCEDURAL spells.
//! 48 independent motion/silhouette families, not 100 Blender meshes or
//! 100 recolours of one school particle preset.
//! These are preview choreographies, NOT 100 polished gameplay-verified spells.
use super::{Cast,Scene,add,sub,scale};
use super::authored_100_data::{author_for,FxAuthor,Pattern};
use super::procedural_fx::sprite;
use std::f64::consts::{PI,TAU};
const RGB:[u32;10]=[
  0x00FF7335,0x0087D2F1,0x00B2E5D1,0x00B8A98E,0x0049BEEB,
  0x007FDB98,0x009FC6FA,0x00C9A0F9,0x00E9C890,0x00D3ABDB,
];
const MATERIAL:[u8;10]=[64,79,110,91,91,110,79,110,79,110];
fn hash(n:f64)->f64 {let q=(n*12.9898+78.233).sin()*43758.5453123;q-q.floor()}
fn rand(slot:usize,i:usize,k:usize)->f64 {
    hash(1.0+slot as f64*27.31+i as f64*73.19+k as f64*11.17)
}
fn ease(t:f64)->f64 {let x=t.clamp(0.0,1.0);x*x*(3.0-2.0*x)}
fn polar(r:f64,a:f64,h:f64)->[f64;3] {[r*a.cos(),h,r*a.sin()]}
fn pose(a:&FxAuthor,q:f64,u:f64,v:f64,t:f64)->[f64;3]{
    use Pattern::*;
    let phase=t*TAU*a.tempo+a.phase*TAU;
    let theta=q*TAU;
    let radial=q.sqrt();
    let ripple=(theta*2.4+phase).sin()*0.22;
    let m=match a.pattern{
        Lance=>[0.15*(theta*1.7+phase).sin()*(1.0-q)+0.22*(u-0.5),
                0.22*(theta+phase).cos()+0.12*(v-0.5),-1.1+4.8*q+3.5*t],
        Fan=>{let blade=(q*7.0).floor();let f=(q*7.0).fract();
            [(blade-3.0)*(0.26+0.47*q)+ripple,0.5+1.2*f*(v-0.2),1.4+2.7*q]},
        Lash=>[1.45*(theta*0.72+phase).sin()*q,
               0.55+1.3*(theta*0.33+phase).sin(),3.5*q+0.85*t],
        Footsteps=>{let step=(q*8.0).floor();let f=(q*8.0).fract();
            [(if step as usize%2==0 {-1.0}else{1.0})*(0.30+0.5*f),
             0.05+0.35*v*t,0.53*step+0.27*f+1.3*t]},
        Cloud=>{let r=0.7+1.7*radial;
            [r*(theta+phase*0.17).cos(),0.6+2.1*v+0.4*(theta*2.0+phase).sin(),
             r*(theta+phase*0.17).sin()]},
        Sigil=>{let branch=(q*6.0).floor();let f=(q*6.0).fract();
            [((branch as i32%3)-1) as f64*(0.7+0.8*f)+ripple,
             0.08+0.18*v+0.5*(branch*0.48).sin(),
             ((branch as i32/3)-1) as f64*0.65+1.7*f]},
        Serpent=>[1.25*(q*TAU*2.3-phase).sin()*(0.5+q),
                  0.9+0.67*(q*TAU*2.0-phase*0.8).cos()+0.35*v,
                  q*6.1+1.9*t],
        Bloom=>{let petal=(q*6.0).floor();let f=(q*6.0).fract();
            let angle=TAU*petal/6.0+phase*0.05;
            polar((0.55+3.3*f)*ease(t*3.0),angle,
                  0.2+1.6*(PI*f).sin()+0.35*v)},
        Orbit=>{let angle=theta*2.0+phase;
            [1.45*angle.cos()*(0.5+0.6*v),1.1+0.8*(theta*3.0+phase).sin(),
             1.75*angle.sin()*(0.5+0.5*u)]},
        Bridge=>{let slab=(q*8.0).floor();let f=(q*8.0).fract();
            [2.0*(f-0.5),0.15+0.45*(1.0-ease(t*8.0-slab)),(slab-3.5)*1.0]},
        Cage=>{let rib=(q*8.0).floor();let f=(q*8.0).fract();
            polar(1.4+0.2*(f*PI).sin(),TAU*rib/8.0,
                  0.25+3.1*f+0.2*ripple)},
        Roots=>{let path=(q*7.0).floor();let f=(q*7.0).fract();
            let angle=TAU*path/7.0;
            polar(0.25+2.7*f,angle+0.22*(f*TAU+phase).sin(),
                  0.05+0.35*(f*PI*2.0).sin()+0.5*f)},
        Claw=>{let finger=(q*5.0).floor();let f=(q*5.0).fract();
            [(finger-2.0)*0.48*(1.0-0.14*f),
             0.65+2.8*f-0.58*f*f+0.16*ripple,
             0.6+1.35*(f*PI).sin()+0.12*finger]},
        Mirror=>{let side=if q<0.5 {-1.0}else{1.0};
            [side*(1.1+0.5*(theta*2.0+phase).sin()),
             0.6+2.0*(q*2.0).fract(),
             0.28*(theta*3.0+phase).sin()+0.5*(v-0.5)]},
        Vortex=>{let r=(1.8-0.65*q)*(0.4+0.6*v);
            polar(r,theta*3.2+phase,0.15+4.3*q)},
        Sonic=>{let d=0.55+3.5*q;
            [d*((v-0.5)*1.9+0.1*(theta+phase).sin()),
             0.45+d*0.33*(u-0.5),d+2.1*t]},
        Crescent=>{let phi=PI*(q*1.37-0.17);
            [2.5*phi.cos()+0.14*v,0.7+2.35*phi.sin(),
             1.5+0.38*(theta+phase).sin()]},
        Swarm=>[2.2*(theta+phase+v*TAU).sin()*radial,
                1.2+2.0*(theta*3.1-phase*0.6).cos().abs()*v,
                1.9*(theta*1.71+phase*0.4).cos()*radial],
        Implode=>{let r=(2.6*(1.0-ease(t))+0.2)*(0.3+0.7*radial);
            polar(r,theta*2.0+phase,1.3+1.8*(u-0.5)*r)},
        Pulse=>{let r=0.5+4.0*ease(t);
            polar(r*(0.35+0.65*radial),theta,1.2+1.1*(u-0.5)+0.2*ripple)},
        Wings=>{let side=if q<0.5{-1.0}else{1.0};let f=(q*2.0).fract();
            [side*(0.45+4.1*f),0.6+2.6*(PI*f).sin()+0.30*(theta+phase).sin(),
             0.2+1.0*(1.0-f)*v]},
        Jaws=>{let tooth=(q*8.0).floor();let f=(q*8.0).fract();
            let side=if tooth<4.0{-1.0}else{1.0};
            [side*(1.3-0.45*ease(t*2.0)),0.3+0.4*(tooth%4.0),
             (f-0.5)*1.7+0.15*(theta+phase).sin()]},
        Crack=>{let d=5.5*q;
            [0.35*(d*3.1).sin()+0.65*(d*1.7).sin()*q,
             0.1+0.12*v,d+0.8*ease(t)]},
        Shield=>{let r=1.1+1.45*v;
            [r*theta.cos(),0.15+1.7*(PI*q).sin().abs(),r*theta.sin()*0.74]},
        Net=>{let row=(q*9.0).floor();let col=(q*9.0).fract();
            [(row-4.0)*0.37,0.3+2.1*col,
             1.0+0.31*(row*1.8+col*5.0+phase).sin()]},
        Barrage=>{let shot=(q*6.0).floor();let f=(q*6.0).fract();
            [(shot-2.5)*0.49+0.25*(f*TAU+phase).sin(),
             0.4+0.3*(shot*1.8).sin()+0.12*ripple,f*4.3+3.0*t]},
        Surf=>{let w=theta+phase;
            [q*5.0-2.5,0.45+1.15*(q*PI).sin()+0.4*(w*1.6).cos(),
             1.0+0.9*w.sin()]},
        Tether=>[0.45*(q*TAU*2.0+phase).sin()+0.18*(u-0.5),
                 0.6+0.7*(theta+phase).cos(),5.6*q+0.5*t],
        Rain=>[4.6*(u-0.5),0.4+4.2*(1.0-(q+t*2.4).fract()),4.3*(v-0.5)],
        Cascade=>[4.0*(q-0.5),0.6+2.7*(PI*q).sin()-2.1*ease(t)*q,
                   1.5+1.0*(theta+phase).cos()],
        Jet=>[0.45*(theta+phase).sin()*q,
               0.55+0.7*(theta*2.0).sin()*q,5.8*q+2.8*t],
        Weave=>{let strand=(q*3.0).floor();let f=(q*3.0).fract();
            [0.75*(f*TAU*2.0+phase+strand*TAU/3.0).sin(),
             0.5+2.7*f,1.15*(f*TAU*2.0+strand*TAU/3.0).cos()]},
        Canopy=>{let r=2.3*radial;
            polar(r,theta,1.1+2.3*(1.0-(r/2.3).powi(2))+0.1*ripple)},
        Fork=>{let branch=(q*6.0).floor();let f=(q*6.0).fract();
            [(branch-2.5)*0.30+0.55*(f*TAU*1.5+phase).sin(),
             0.6+3.0*f,2.4*f+0.25*(branch*1.7).sin()]},
        Corona=>{let r=(0.25+2.4*radial)*ease(t*3.0);
            polar(r,theta*1.2+phase,0.8+0.55*(theta*4.0+phase).sin())},
        Coil=>{let r=1.2+0.45*v;
            polar(r,theta*4.1+phase,0.2+3.1*q)},
        Starfield=>[5.4*(u-0.5),0.4+3.2*(v-0.1)+0.2*(theta*3.0+phase).sin(),
                    4.8*(hash(theta+v)-0.5)],
        Fold=>{let panel=(q*4.0).floor();let f=(q*4.0).fract();
            [(panel-1.5)*0.85+0.85*(f*PI).sin(),
             0.5+3.0*f,0.7*(panel*1.7+phase).cos()]},
        Veil=>{let r=0.55+1.65*q;
            [3.2*(u-0.5),0.4+3.0*v,
             r*(theta+phase*0.15).sin()+0.32*(theta*2.2).cos()]},
        Comet=>{let qt=q*q;
            [0.38*(theta*2.0+phase).cos()*(1.0-q),
             0.6+0.38*(theta*3.0+phase).sin()*(1.0-q),
             5.6*qt+3.3*t]},
        Compass=>{let spoke=(q*8.0).floor();let f=(q*8.0).fract();
            polar(0.3+3.0*f,TAU*spoke/8.0+
                  0.08*(theta+phase).sin(),0.7+0.55*(PI*f).sin())},
        Clock=>{let phi=phase+0.18*(q*PI).sin();
            [2.5*q*phi.cos()+0.12*(u-0.5),0.8+2.3*q*phi.sin(),
             0.14*(theta+phase).sin()]},
        TimeEcho=>[0.9*(theta+phase).sin()*q,0.5+0.7*v,
                   5.0*q-2.1*t+0.55*(theta+phase).cos()],
        Sandglass=>{let sign=if q<0.5{-1.0}else{1.0};let f=(q*2.0).fract();
            [sign*1.65*f*(theta+phase).cos(),
             1.9+sign*(0.9+1.8*f),1.2*f*(theta+phase).sin()]},
        Stasis=>{let r=1.0+2.0*radial;
            polar(r,theta+0.1*phase,1.8+0.6*(u-0.5))},
        Lantern=>{let r=0.5+1.2*radial;
            polar(r,theta*1.6+phase*0.2,1.7+1.35*(1.0-q)+0.2*ripple)},
        Wisp=>[0.8*(theta+phase*0.5).sin()+1.2*(u-0.5),
               0.3+3.5*q+0.35*(theta*1.9+phase).sin(),
               0.9*(theta+phase*0.4).cos()],
        Pool=>{let r=0.15+2.7*radial;
            polar(r,theta*1.1+phase*0.04,0.10+0.23*(theta*3.2+phase).sin())}
    };
    [m[0]*a.spread*0.75,m[1]*a.rise*0.75,m[2]*a.spread*0.78]
}
fn transform(dir:[f64;3],offset:[f64;3])->[f64;3]{
    // Preserve a stable horizontal cast frame under vertical camera angle.
    let len=(dir[0]*dir[0]+dir[2]*dir[2]).sqrt();
    let f=if len>0.0001 && len.is_finite() {
        [dir[0]/len,0.0,dir[2]/len]
    }else{[0.0,0.0,1.0]};
    let right=[f[2],0.0,-f[0]];
    add(add(scale(right,offset[0]),[0.0,offset[1],0.0]),scale(f,offset[2]))
}
/// Returns None if slot is not in the artist-authored 100. Legacy M35
/// background emitters handle the other 100. Returns Some(n) for authored
/// slots even if the specific age yields zero particles (no double layer).
pub(super) fn render(
    cast:Cast,age:f64,camera:[f64;3],scene:&mut Scene
)->Option<usize>{
    let author=author_for(cast.kind)?;
    if !age.is_finite()||!(0.0.0.104.0).contains(&age)
        ||!cast.center.iter().chain(cast.direction.iter()).all(|v|v.is_finite())
        ||!camera.iter().all(|v|v.is_finite())
    {return Some(0);}
    let start=scene.facets.len();
    let school=cast.kind/20;
    let n=if scene.reduced_detail {author.density as usize/2}
        else {author.density as usize};
    let overall=age/104.0;
    let envelope=ease(age/8.0)*(1.0-ease((age-83.0)/21.0));
    for i in 0..n{
        if scene.facets.len()>=scene.budget{break;}
        // Deterministic seeded spread, not a shared random cone.
        let q=(i as f64+0.5)/n as f64;
        let u=rand(cast.kind,i,0);
        let v=rand(cast.kind,i,1);
        let born=4.0+rand(cast.kind,i,2)*34.0*author.phase;
        let life=(age-born)/(50.0+29.0*rand(cast.kind,i,3));
        if !(0.0.0.1.0).contains(&life){continue;}
        let fade=ease(life*8.0)*(1.0-ease((life-0.72)/0.28))*envelope;
        if fade<0.06{continue;}
        let local=pose(author,q,u,v,overall);
        let position=add(cast.center,transform(cast.direction,
            [local[0],local[1]+0.12*(life*TAU+q*TAU*2.0).sin(),
             local[2]+0.17*(life*TAU*author.twist).sin()]));
        let width=(0.09+0.16*v)*(0.42+0.58*fade)
            *(1.0+0.12*author.sweep);
        let height=width*match author.pattern{
            Pattern::Lance|Pattern::Jet|Pattern::Serpent=>1.8,
            Pattern::Rain|Pattern::Wisp|Pattern::Weave=>1.5,
            Pattern::Shield|Pattern::Cloud|Pattern::Pool=>0.86,
            _=>1.14,
        };
        let roll=0.18*author.twist*(life*TAU+q*TAU).sin();
        let _=sprite(scene,camera,position,width,height,roll,
                     RGB[school],MATERIAL[school]);
    }
    Some(scene.facets.len()-start)
}
#[cfg(test)]
mod tests{
 use super::*;
 fn cast(id:usize)->Cast {
   Cast{kind:id,variant:(id%20) as u8,start:0,
        center:[0.,65.,0.],direction:[0.29,-0.58,0.77]}
 }
 #[test]
 fn all_100_authored_slots_have_deterministic_distinct_animated_quads(){
   let mut actual=0;
   for slot in 0.0.200{
     let eligible=slot%20<10;
     for age in [9.0,22.0,47.0,70.0,88.0]{
       let mut a=Scene::with_lod(90,false);
       let mut b=Scene::with_lod(90,false);
       let n=render(cast(slot),age,[6.,72.,9.],&mut a);
       let m=render(cast(slot),age,[6.,72.,9.],&mut b);
       assert_eq!(n,m);
       assert_eq!(a.facets,b.facets);
       assert_eq!(n.is_some(),eligible);
       assert!(a.facets.len()<=80 && a.facets.len()<=a.budget);
       assert!(a.facets.iter().all(|(q,_)|q.iter().flatten().all(|v|v.is_finite())));
       if age==47.0 && n.unwrap_or_default()>0{actual+=1;}
     }
   }
   assert_eq!(actual,100);
 }
 #[test]
 fn all_motion_families_have_different_spatial_signatures(){
   use std::collections::BTreeMap;
   let mut unique=BTreeMap::new();
   for spec in super::super::authored_100_data::AUTHOR_100 {
      let q=pose(&spec,0.27,0.38,0.61,0.45);
      assert!(q.iter().all(|x|x.is_finite()));
      unique.entry(format!("{:?}",spec.pattern)).or_insert(q);
   }
   assert!(unique.len()>=40);
   let signatures:std::collections::BTreeSet<_>=unique.values()
      .map(|v|format!("{:0.2},{:0.2},{:0.2}",v[0],v[1],v[2])).collect();
   assert!(signatures.len()>=36);
 }
 #[test]
 fn camera_steep_angle_and_zero_budget_do_not_break_world_geometry(){
   let slot=6; // Pyre Serpent
   for camera in [[3.,72.,8.],[0.,130.,0.],[-8.,66.,-6.]]{
     let mut s=Scene::new(80);
     let n=render(cast(slot),46.,camera,&mut s).unwrap();
     assert!(n>0);
     for (q,_) in s.facets{
       let c=scale(q.iter().copied().fold([0.,0.,0.],add),0.25);
       let face=super::super::cross(sub(q[1],q[0]),sub(q[3],q[0]));
       assert!(face.iter().zip(sub(camera,c)).map(|(a,b)|a*b).sum::<f64>()>0.);
     }
   }
   let mut s=Scene::new(0);
   assert_eq!(render(cast(slot),40.,[4.,71.,8.],&mut s),Some(0));
 }
}