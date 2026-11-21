//! M35: seeded, bounded procedural VFX emitters for EVERY registry school.
//!
//! This is the base of an authored material+particle engine, NOT 200 polished
//! spell scenes and NOT 200 Blender objects. Each emitter is independently
//! animated in 3D; an OPTIONAL Horizon WGSL PipelineQuad shades each small
//! camera-facing quad into a soft flame, ice spark, droplet or ghost wisp.
//! On pipeline failure lib.rs falls back to ONLY the original solid geometry
//! so the user never sees raw rectangles. No server gameplay claims.
//!
//! Camera-relative, depth-tested particles use normal Horizon guest quads;
//! we are NOT claiming GPU instancing, full-screen bloom or native post access.
use super::{Cast,Scene,add,sub,scale,cross,length,norm};
use crate::catalog200;
use std::f64::consts::{PI,TAU};

const RGB:[u32;10]=[
    0x00F66B23, // fire: hot ember
    0x0081D5EB, // ice: spindrift
    0x00C0EAD4, // air: turbulent mist
    0x00A7A091, // stone: dust motes
    0x0031B8E8, // water: dancing droplets
    0x006CDA8B, // flora: pollen/fireflies
    0x0096BDF8, // storm: ion sparks
    0x00AD78E5, // astral: dimensional dust
    0x00E6BE7A, // time: timeline echoes
    0x00B5C9E7, // souls: wandering wisps
];
/// Alpha is a small tagged material band, NOT a user's final opacity:
/// 0.25 fire, 0.31 crystalline star, 0.36 droplet/dust, 0.43 wisp.
/// The optional WGSL pipeline soft-clips the quad boundary to alpha zero.
/// Plain guest WorldQuad must never receive these material-only quads.
const MATERIAL_ALPHA:[u8;10]=[64,79,110,91,91,110,79,110,79,110];

fn hash(x:f64)->f64 {
    let z=(x*12.9898+78.233).sin()*43758.5453123;
    z-z.floor()
}
fn rnd(slot:usize,i:usize,k:usize)->f64{
    hash(1.0+slot as f64*23.621+i as f64*79.137+k as f64*7.913)
}
fn finite(v:[f64;3])->bool {v.iter().all(|v|v.is_finite())}
pub(super) fn sprite(
    scene:&mut Scene,camera:[f64;3],at:[f64;3],
    width:f64,height:f64,roll:f64,rgb:u32,material:u8
)->bool {
    if !finite(at)||!finite(camera)||!width.is_finite()
        ||!height.is_finite()||!roll.is_finite()
        ||width<=0.0||height<=0.0{return false;}
    let to_eye=sub(camera,at);
    if length(to_eye)<0.001{return false;}
    let forward=norm(to_eye);
    let guide=if forward[1].abs()<0.93 {[0.0,1.0,0.0]}
        else {[0.0,0.0,1.0]};
    let right=norm(cross(guide,forward));
    let up=norm(cross(forward,right));
    let (s,c)=roll.sin_cos();
    let x=add(scale(right,c*width),scale(up,s*width));
    let y=add(scale(up,c*height),scale(right,-s*height));
    let a=sub(sub(at,x),y);
    let b=add(sub(at,y),x);
    let c=add(add(at,x),y);
    let d=add(sub(at,x),y);
    // Positive normal faces the camera, including vertical camera shots.
    let face=cross(sub(b,a),sub(d,a));
    if face.iter().zip(forward).map(|(a,b)|a*b).sum::<f64>()<=0.0 {
        return false;
    }
    let before=scene.facets.len();
    scene.quad([a,b,c,d],(u32::from(material)<<24)|(rgb&0x00ff_ffff));
    scene.facets.len()>before
}
fn movement(
    school:usize,origin:[f64;3],q:f64,life:f64,
    rx:f64,ry:f64,rz:f64,pulse:f64
)->[f64;3] {
    let u=q*TAU;
    let lateral=2.0*rx-1.0;
    let depth=2.0*rz-1.0;
    let age=life;
    let delta=match school{
        0=>[1.4*lateral+0.55*(u+age*5.3).sin()*life,
            0.12+4.1*life+0.60*(u*2.0+age*6.3).cos()*life,
            1.0*depth+0.45*(u-age*3.1).cos()*life],
        1=>[2.6*lateral+0.27*(u+age*3.8).sin(),
            0.9+1.8*ry-1.8*life+0.23*(u*1.7+age).sin(),
            2.2*depth+0.41*(age*7.2+u).cos()],
        2=>[4.0*(life-0.4)+1.1*lateral,
            1.3*ry+0.6*(age*4.0+u).sin(),
            1.4*depth+0.76*(age*7.6+u).cos()],
        3=>[1.8*lateral+0.24*(u+age*4.0).sin(),
            2.3*ry+1.15*life-3.6*life*life,
            1.8*depth+0.3*(u*1.4+age*3.1).cos()],
        4=>[2.7*lateral+1.5*life,
            0.25+3.1*ry*life-3.8*life*life,
            1.9*depth+0.28*(u+age*5.1).sin()],
        5=>[1.5*lateral+0.55*(u+life*4.1).sin()*life,
            0.35+2.3*life+0.45*(age*3.8+u).sin(),
            1.4*depth+0.55*(u+age*4.1).cos()*life],
        6=>[2.5*lateral+0.38*(u+age*13.0).sin(),
            3.0*ry+0.25*(u*2.0+age*17.0).sin(),
            2.0*depth+0.27*(u+age*17.0).cos()],
        7=>[1.8*lateral*(1.0-0.7*life)+0.55*(u+age*2.8).sin(),
            1.6*ry+1.0*life+0.32*(u+age*4.0).cos(),
            1.8*depth*(1.0-0.7*life)+0.55*(age*2.6+u).sin()],
        8=>[2.4*lateral+0.26*((age*4.0).floor()*0.5+u).sin(),
            0.7+1.7*ry-0.68*life,
            2.2*depth-1.15*life+0.42*((age*6.0).floor()+u).cos()],
        _=>[1.1*lateral+1.35*(u+age*2.0).sin()*life,
            0.7+2.6*life+0.40*(age*4.4+u).sin(),
            1.0*depth+1.15*(u*0.7-age*2.4).cos()*life]
    };
    add(origin,scale(delta,pulse))
}
/// Fully deterministic: no client RNG/asset IO or authoritative server state.
/// School-specific physical motion is coupled to canonical 200 spell metadata.
/// Returns the number of actually scheduled procedurally masked quads.
pub(super) fn render(
    cast:Cast,age:f64,camera:[f64;3],scene:&mut Scene
)->usize {
    if cast.kind>=catalog200::SPELL_COUNT||!age.is_finite()
        ||!(0.0..104.0).contains(&age)
        ||!finite(cast.center)||!finite(cast.direction)||!finite(camera)
    {return 0;}
    let before=scene.facets.len();
    let school=catalog200::school(cast.kind);
    let rank=catalog200::rank(cast.kind);
    let n=if scene.reduced_detail{24}else{48+rank%4*4};
    let pulse=(0.55+0.45*(age/13.0).clamp(0.0,1.0))*
        ((104.0-age)/15.0).clamp(0.0,1.0);
    for i in 0..n {
        if scene.facets.len()>=scene.budget {break;}
        let rx=rnd(cast.kind,i,0);
        let ry=rnd(cast.kind,i,1);
        let rz=rnd(cast.kind,i,2);
        let offset=rnd(cast.kind,i,3);
        let born=offset*82.0;
        let life_ticks=13.0+18.0*rnd(cast.kind,i,4);
        let t=(age-born)/life_ticks;
        if !(0.0..1.0).contains(&t){continue;}
        let fade=(PI*t).sin().max(0.0)*pulse;
        if fade<0.065{continue;}
        let position=movement(school,cast.center,rx,t,rx,ry,rz,pulse);
        let size=(0.12+0.19*ry)*(0.58+0.42*fade)*
            if rank>=17{1.22}else{1.0};
        let aspect=match school{
            0|6=>1.95+0.6*rz, // tapered flame/spark
            1|3|8=>0.8+0.5*rx,
            2|5|7|9=>1.25+0.6*ry,
            _=>1.1+0.8*rz,
        };
        let angle=match school{
            0=>0.18*(t*12.0+rx*TAU).sin(),
            6=>(rx-0.5)*0.8+0.14*(age*0.7).sin(),
            2|5|9=>(rx-0.5)*2.0+t*1.6,
            _=>rx*TAU*0.37+t*0.2,
        };
        let _=sprite(scene,camera,position,size,size*aspect,angle,
                     RGB[school],MATERIAL_ALPHA[school]);
    }
    scene.facets.len()-before
}
#[cfg(test)]
mod tests {
    use super::*;
    fn cast(id:usize)->Cast{
        Cast{kind:id,variant:(id%20) as u8,start:0,
            center:[0.,64.,0.],direction:[0.12,-0.31,0.94]}
    }
    #[test]
    fn all_two_hundred_identities_have_real_seeded_evolution_and_hard_budget(){
        for slot in 0..200{
            let mut total=0;
            for age in [8.0,19.0,43.0,70.0,91.0]{
                let mut a=Scene::with_lod(110,false);
                let mut b=Scene::with_lod(110,false);
                let na=render(cast(slot),age,[7.,69.,12.],&mut a);
                let nb=render(cast(slot),age,[7.,69.,12.],&mut b);
                assert_eq!(na,nb);
                assert_eq!(a.facets,b.facets,"unstable VFX for slot {slot}");
                assert_eq!(a.group_ids.len(),a.facets.len());
                assert!(na<=60);
                assert!(a.facets.iter().all(|(v,_)|
                    v.iter().flatten().all(|a|a.is_finite())));
                total+=na;
            }
            assert!(total>3,"no actual particles for #{slot}");
        }
    }
    #[test]
    fn every_school_has_a_distinct_spatial_trajectory(){
        let mut means=Vec::new();
        for school in 0..10{
            let mut s=Scene::new(100);
            assert!(render(cast(school*20+5),42.0,[8.,68.,9.],&mut s)>0);
            let avg=s.facets.iter().flat_map(|(q,_)|q.iter())
                .fold([0.,0.,0.],|a,b|add(a,*b));
            let avg=scale(avg,1.0/(s.facets.len()*4) as f64);
            means.push(avg);
        }
        for (i,a) in means.iter().enumerate(){
            for b in means.iter().skip(i+1){
                assert!(length(sub(*a,*b))>0.015);
            }
        }
    }
    #[test]
    fn each_quad_is_camera_facing_and_never_becomes_an_opaque_card(){
        let cast=cast(41);
        for camera in [[8.,68.,9.],[0.,92.,0.],[-9.,63.,-8.]]{
            let mut s=Scene::new(100);
            let n=render(cast,44.0,camera,&mut s);
            assert!(n>0);
            for (quad,color) in s.facets {
                let center=scale(quad.into_iter().fold([0.,0.,0.],add),0.25);
                let normal=cross(sub(quad[1],quad[0]),sub(quad[3],quad[0]));
                assert!(normal.iter().zip(sub(camera,center)).map(|(a,b)|a*b)
                    .sum::<f64>()>0.0);
                let a=(color>>24)&255;
                assert!(a>55&&a<118, "shader-only material mask lost");
            }
        }
    }
    #[test]
    fn respects_zero_budget_and_invalid_geometry(){
        let mut s=Scene::new(0);
        assert_eq!(render(cast(0),42.,[5.,66.,5.],&mut s),0);
        let mut s=Scene::new(100);
        assert_eq!(render(cast(200),42.,[5.,66.,5.],&mut s),0);
        assert_eq!(render(cast(0),f64::NAN,[5.,66.,5.],&mut s),0);
        assert_eq!(render(cast(0),42.,[f64::NAN,66.,5.],&mut s),0);
        assert!(s.facets.is_empty());
    }
    #[test]
    fn reduced_detail_at_most_twenty_four_particles_per_school(){
        for school in 0..10{
            let mut s=Scene::with_lod(250,true);
            let n=render(cast(school*20),42.,[8.,68.,9.],&mut s);
            assert!(n<=24);
        }
    }
}
