//! M23: ten hand-directed Standard spell silhouettes, #4 in each school.
//! Explicitly based on canonical registry mesh + mechanics, not palette swaps.
//! Their server actions are STILL DesignIntent and unimplemented in-world.
use super::{Cast,Scene,add,sub,scale};
use crate::catalog200;
use crate::ultimate_forms::{p,tint,body,crystal,wing,orb,curtain,floor_slab};
use aetheria_horizon::vfx::timeline::{for_spell,Stage};
use std::f64::consts::{PI,TAU};
type V=[f64;3];
const IDS:[u16;10]=[4,24,44,64,84,104,124,144,164,184];
fn phase(id:u16,age:f64)->Option<(Stage,f64)> {
    if !age.is_finite() || !(0.0..104.0).contains(&age) {return None;}
    let timeline=for_spell(id)?;
    let at=age/104.*timeline.total_ticks() as f64;
    let mut passed=0.;
    for part in timeline.phases {
        let n=part.duration_ticks as f64;
        if at<passed+n{return Some((part.kind,((at-passed)/n).clamp(0.,1.)))}
        passed+=n;
    }
    None
}
pub(super) fn dedicated(id:u16)->bool{IDS.contains(&id)}
fn forms(s:&mut Scene,id:u16,o:V,age:f64,g:f64,c:u32){
    let r=3.0*g;
    match id {
        4=>{ // Furnace Step: staggered burning footprints, a spatial dash trail.
            for i in 0..9{
                let t=i as f64/8.;
                let side=if i%2==0{-1.}else{1.};
                let x=side*0.54*g;
                let z=(t-0.5)*7.2*g;
                let loc=p(o,x,-0.3,z);
                body(s,p(loc,0.,0.10*g,0.),
                    [0.39*g,0.17*g,0.65*g],tint(c,0.68+0.028*i as f64,0.89));
                for j in 0..3{
                    let a=j as f64*TAU/3.;
                    wing(s,p(loc,0.,0.08*g,0.),a,
                         (0.7+j as f64*0.18)*g,
                         (1.1+1.5*(t*PI).sin())*g,
                         tint(c,0.76,0.48+0.14*(i%3) as f64));
                }
            }
        }
        24=>{ // Glaze Prison: broad hexagonal prison walls and ice buttresses.
            let sides=6;
            for j in 0..sides{
                let a=TAU*j as f64/sides as f64;
                let b=TAU*(j+1) as f64/sides as f64;
                let base0=p(o,2.05*g*a.cos(),-0.4,2.05*g*a.sin());
                let base1=p(o,2.05*g*b.cos(),-0.4,2.05*g*b.sin());
                let top0=p(base0,0.,4.45*g,0.);
                let top1=p(base1,0.,4.45*g,0.);
                s.quad([base0,base1,top1,top0],tint(c,0.68+0.042*j as f64,0.49));
                crystal(s,base0,top0,0.37*g,a,tint(c,0.8,0.83));
                let cap=p(o,0.,5.4*g,0.);
                s.membrane(top0,top1,cap,p(o,0.,4.5*g,0.),
                           tint(c,0.8,0.67));
            }
        }
        44=>{ // Feather Drift: large broad curved 3D FEATHERS, no sticks.
            for i in 0..12{
                let a=TAU*i as f64/12.+age*0.012;
                let h=(i%4) as f64*0.61*g+0.72*g;
                let r=(0.5+(i%3) as f64*0.22)*g;
                let base=p(o,a.cos()*r,h,a.sin()*r);
                let end=p(base,(0.8+0.32*(i%5) as f64)*g*a.cos(),
                       (0.5+(i%3) as f64*0.12)*g,
                       (0.8+0.32*(i%5) as f64)*g*a.sin());
                let mid=add(base,scale(sub(end,base),0.47));
                let wing_width=0.52*g;
                let sideways=[-a.sin()*wing_width,0.16*g,a.cos()*wing_width];
                s.membrane(base,add(mid,sideways),end,sub(mid,sideways),
                    tint(c,0.64+0.019*i as f64,0.67));
                // Thick central vein INSIDE the leaf, not an exposed rod.
                crystal(s,p(mid,0.,-0.12*g,0.),
                    p(end,0.,0.06*g,0.),0.10*g,a,tint(c,0.93,0.78));
            }
        }
        64=>{ // Faultline: contiguous cracked ground trench in branching Z.
            for i in 0..12{
                let t=i as f64/11.;
                let x=(t*TAU*1.4+age*0.007).sin()*0.87*g;
                let z=(t-0.5)*8.0*g;
                let height=(0.15+0.6*(i%4) as f64/4.)*g;
                body(s,p(o,x,-0.5+height*0.25,z),
                    [0.51*g,0.20*g+height*0.30,0.48*g],
                    tint(c,0.58+0.03*(i%5) as f64,0.96));
                if i%2==0{
                    let tip=p(o,x+(if i%4==0{-1.}else{1.})*1.4*g,
                               0.0,z+0.4*g);
                    crystal(s,p(o,x,-0.6,z),tip,0.33*g,
                            t*TAU,tint(c,0.66,0.9));
                }
            }
        }
        84=>{ // Riptide Snare: swirling thick water *sheets* around victim.
            for j in 0..4{
                let a=TAU*j as f64/4.+age*0.021;
                curtain(s,o,(1.4+0.43*j as f64)*g,3.7*g,age,a,
                        tint(c,0.59+0.07*j as f64,0.34));
            }
            for j in 0..6{
                let a=TAU*j as f64/6.+age*0.013;
                wing(s,p(o,a.cos()*r*0.39,0.2*g,a.sin()*r*0.39),
                    a,1.5*g,2.5*g,tint(c,0.84,0.55));
            }
        }
        104=>{ // Briar Loop: interwoven rooted LIVING BRIAR CANOPY.
            // Previous scene was only skinny spikes with stray top leaves.
            for layer in 0..2 {
              for i in 0..5{
                let a=TAU*i as f64/5.+layer as f64*0.29;
                let radius=(1.7+layer as f64*0.46)*g;
                let base=p(o,radius*a.cos(),-0.55,radius*a.sin());
                let height=(2.2+0.20*(i%5) as f64+layer as f64*0.27)*g;
                let mid=p(o,radius*a.cos()*1.06,height*0.60,radius*a.sin()*1.06);
                let tip=p(o,radius*a.cos()*0.72,height,radius*a.sin()*0.72);
                crystal(s,base,mid,0.39*g,a,tint(c,0.58,0.91));
                crystal(s,mid,tip,0.31*g,a+0.33,tint(c,0.74,0.91));
                let foliage=p(o,radius*a.cos()*0.86,height*0.80,
                                 radius*a.sin()*0.86);
                for petal in 0..3 {
                    wing(s,foliage,a+petal as f64*0.53-0.40,
                      (0.92+0.15*(petal%2) as f64)*g,
                      (0.52+0.13*petal as f64)*g,
                      tint(c,0.74+0.06*petal as f64,0.75));
                }
              }
            }
            floor_slab(s,o,2.1*g,0.23,age*0.007,tint(c,0.61,0.76));
        }
        124=>{ // Arc Net: a 3D triangular electrified MESH, no naked outline.
            let layers=6;
            for h in 0..layers{
                let z=h as f64*0.35*g;
                let width=(1.8+0.2*h as f64)*g;
                for side in [-1.,1.]{
                    let bottom=p(o,side*width,-0.1*g,z-2.4*g);
                    let top=p(o,side*(width*0.48),3.5*g,z-1.2*g);
                    let mid=p(o,0.,1.65*g,z+0.7*g);
                    s.membrane(bottom,top,mid,
                      p(o,side*(width*0.3),0.3*g,z+0.6*g),
                      tint(c,0.70+0.022*h as f64,0.46));
                }
            }
            crystal(s,p(o,0.,0.2*g,-1.7*g),p(o,0.,4.2*g,0.6*g),
                   0.33*g,age*0.015,tint(c,0.93,0.89));
        }
        144=>{ // Moonlit Veil: two silver PHASE drapes, substantial depth.
            for i in 0..3 {
                let a=age*0.009+i as f64*0.83;
                curtain(s,p(o,(i as f64-1.)*0.58*g,-0.2*g,0.),
                    (2.6+0.2*i as f64)*g,4.4*g,age,a,
                    tint(c,0.66+0.06*i as f64,0.31));
            }
            orb(s,p(o,0.,2.2*g,-0.30*g),[0.61*g,1.42*g,0.27*g],
                age,tint(c,0.86,0.77),0.61);
        }
        164=>{ // Sands of Delay: proper DOUBLE INVERTED 3D SANDGLASS.
            // Angular connected chambers with wide top/bottom and tight waist,
            // avoiding the earlier embarrassing two spheres.
            let sides=8usize;
            let chamber=|height:f64,flip:f64,s:&mut Scene|{
                for j in 0..sides {
                    let a=TAU*j as f64/sides as f64;
                    let b=TAU*(j+1) as f64/sides as f64;
                    let base=p(o,1.25*g*a.cos(),height,1.05*g*a.sin());
                    let next=p(o,1.25*g*b.cos(),height,1.05*g*b.sin());
                    let neck=p(o,0.20*g*a.cos(),height+flip*1.36*g,
                                 0.20*g*a.sin());
                    let neck2=p(o,0.20*g*b.cos(),height+flip*1.36*g,
                                 0.20*g*b.sin());
                    s.quad([base,next,neck2,neck],tint(c,0.65+0.03*(j%4) as f64,0.55));
                    crystal(s,base,neck,0.11*g,a,tint(c,0.77,0.76));
                }
            };
            chamber(4.32*g,-1.0,s);
            chamber(1.60*g,1.0,s);
            body(s,p(o,0.,4.4*g,0.),[1.27*g,0.19*g,1.09*g],
                 tint(c,0.82,0.92));
            body(s,p(o,0.,1.52*g,0.),[1.27*g,0.19*g,1.09*g],
                 tint(c,0.74,0.93));
            for i in 0..14{
                let a=TAU*i as f64/14.;
                let y=(1.6+(i%7) as f64*0.37)*g;
                let center=p(o,0.24*g*a.cos(),y,0.24*g*a.sin());
                crystal(s,p(center,0.,-0.16*g,0.),
                        p(center,0.,0.22*g,0.),
                        0.085*g,a,tint(c,0.96,0.88));
            }
        }
        184=>{ // Blood Thread: WIDE braided crimson magical sheets, not rods.
            for strand in 0..4{
                let a=TAU*strand as f64/4.+age*0.015;
                let mut previous=p(o,0.,0.1*g,0.);
                for i in 1..=9{
                    let t=i as f64/9.;
                    let angle=a+t*TAU*1.6;
                    let next=p(o,2.1*g*t*angle.cos(),2.5*g*t,
                                    2.1*g*t*angle.sin());
                    let mid=add(previous,scale(sub(next,previous),0.5));
                    let lateral=[-angle.sin()*0.46*g,0.,angle.cos()*0.46*g];
                    s.membrane(previous,add(mid,lateral),next,sub(mid,lateral),
                               tint(c,0.59+0.06*(strand%3) as f64,0.64));
                    previous=next;
                }
            }
            orb(s,p(o,0.,1.4*g,0.),[0.63*g,0.76*g,0.61*g],
                age,tint(c,0.83,0.76),0.69);
        }
        _=>unreachable!("only 10 hand-directed Standard spell IDs"),
    }
}
pub(super) fn render(cast:Cast,age:f64,s:&mut Scene){
    let id=catalog200::id(cast.kind);
    if !dedicated(id){return;}
    let Some((stage,progress))=phase(id,age)else{return;};
    let g=match stage{
        Stage::Telegraph=>0.12+progress*0.24,
        Stage::Cast=>0.36+progress*0.64,
        Stage::Impact=>1.04+progress*0.07,
        Stage::Afterglow=>(1.0-progress).max(0.06),
    };
    let base=catalog200::WORLD_COLORS[catalog200::school(cast.kind)];
    let before=s.facets.len();
    forms(s,id,cast.center,age,g,base);
    if stage==Stage::Afterglow {
        for (_,color) in &mut s.facets[before..]{
            let a=((*color>>24)&255) as f64/255.;
            *color=(*color&0x00FF_FFFF)|
                (((a*(1.-progress).powf(0.8)*255.).round() as u32)<<24);
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn ten_hand_authored_standard_shapes_have_nondegenerate_unique_3d_forms(){
        use std::collections::BTreeSet;
        let mut signatures=BTreeSet::new();
        for (school,id) in IDS.iter().enumerate(){
            assert_eq!((id-1)/20,school as u16);
            let slot=(*id-1) as usize;
            assert!(dedicated(*id));
            let mut s=Scene::new(500);
            let age=(0..104).find(|t|matches!(
                phase(*id,*t as f64),Some((Stage::Cast,p)) if p>0.5
            )).expect("cast phase") as f64;
            render(Cast{kind:slot,variant:3,start:0,
                center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut s);
            assert!(s.facets.len()>=12,"weak shape: {id}");
            assert!(s.facets.len()<500,"truncated/over budget Standard: {id}");
            assert!(s.facets.iter().all(|(q,_)|q.iter().flatten().all(|f|f.is_finite())));
            let fingerprint=s.facets.iter().flat_map(|(q,_)|
                q.iter().flat_map(|pt|pt.iter().map(|v|(*v*100.).round() as i64))
            ).collect::<Vec<_>>();
            assert!(signatures.insert(fingerprint),"palette swap clone: {id}");
        }
        assert_eq!(signatures.len(),10);
        assert!(!dedicated(5)&&!dedicated(25));
    }
}
