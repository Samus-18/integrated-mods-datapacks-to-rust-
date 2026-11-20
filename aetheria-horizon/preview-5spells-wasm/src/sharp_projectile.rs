//! M33: one ACTUAL Blender-authored hero mesh bound to live Horizon guest
//! world-quads. This is a self-contained real source asset, not a rod/ring,
//! spawned Minecraft blocks, or an invented native GPU mesh API.
//! The physical 3D Frost Needle uses every face of the original closed model;
//! four timing phases vary silhouette and flight in world space.
//! This is still a CLIENT PREVIEW: no server freeze/damage/world edit.
use super::{Cast, Scene, add, sub, scale, norm, cross, shade_argb};
use std::f64::consts::{PI,TAU};

mod mesh {
    include!("sharp_projectile_mesh_generated.rs");
}
const SLOT:usize=20; // canonical 1-based spell ID 021, Cryomancer Frost Needle
const QUAD_COUNT:usize=mesh::QUADS.len();

fn clamp01(x:f64)->f64{x.clamp(0.0,1.0)}
fn ease(x:f64)->f64{let t=clamp01(x);t*t*(3.0-2.0*t)}
fn palette(material:u8,face:usize,fade:f64)->u32{
    // Restrained saturated ice BODY, glass facets and softly illuminated mint
    // veins, not a flat white additive shape. The Horizon optional material
    // shader supplies noise/fragment shading; no fake HDR/bloom claim.
    let (rgb,opacity,light)=match material{
        0=>(0x00528EAA,0.94,0.82),
        1=>(0x0067C0EA,0.81,0.96),
        2=>(0x0086E8D6,0.86,1.00),
        _=>return 0
    };
    let alpha=((opacity*fade.clamp(0.0,1.0)*255.0).round() as u32).min(255);
    let rgb_col=rgb|alpha<<24;
    let facets=0.91+0.09*((face as f64*1.239).sin().abs());
    shade_argb(rgb_col,light*facets)
}
fn frame(direction:[f64;3])->([f64;3],[f64;3],[f64;3]) {
    let forward=norm(direction);
    let guide=if forward[1].abs()<0.93 {[0.0,1.0,0.0]} else {[0.0,0.0,1.0]};
    let right=norm(cross(guide,forward));
    let up=norm(cross(forward,right));
    (forward,right,up)
}
/// Draw a complete original crystal mesh as a single atomic LOD component.
/// Object forward is Blender +Z and maps to the player's cast direction.
fn draw_closed_needle(
    scene:&mut Scene,origin:[f64;3],direction:[f64;3],
    radius:f64,length:f64,roll:f64,fade:f64
)->bool{
    if !radius.is_finite()||!length.is_finite()||!roll.is_finite()
        ||!fade.is_finite()||radius<=0.0||length<=0.0||fade<=0.008
        ||!origin.iter().chain(direction.iter()).all(|v|v.is_finite())
        ||direction.iter().map(|v|v*v).sum::<f64>()<0.00001
    {return false;}
    let Some(previous)=scene.begin_component(QUAD_COUNT) else{return false;};
    let (forward,right,up)=frame(direction);
    let c=roll.cos();let s=roll.sin();
    let world=|id:usize|-> [f64;3]{
        let local=mesh::VERTICES[id];
        let x=(local[0]*c-local[1]*s)*radius;
        let y=(local[0]*s+local[1]*c)*radius;
        add(origin,add(scale(right,x),
            add(scale(up,y),scale(forward,local[2]*length))))
    };
    for (i,(verts,material)) in mesh::QUADS.iter().enumerate() {
        scene.quad(verts.map(world),palette(*material,i,fade));
    }
    scene.end_component(previous);
    true
}
/// Four-phase original crystal:
/// 0–9 tick: condensed charged shard; 10–30: propelled, rolls through air;
/// 31–59: physical shards separate at the target; 60–103: quiet collapse.
/// No huge central disc, no billboard, no world-space wire hoop.
pub(super) fn render(cast:Cast,age:f64,scene:&mut Scene)->bool{
    if cast.kind!=SLOT{return false;}
    if !age.is_finite()||!(0.0..104.0).contains(&age){return true;}
    if !cast.center.iter().chain(cast.direction.iter()).all(|v|v.is_finite()){
        return true;
    }
    let (dir,right,up)=frame(cast.direction);
    let impact=add(cast.center,[0.0,0.56,0.0]);
    let start=add(impact,scale(dir,-5.0));
    if age<10.0{
        let t=ease(age/10.0);
        let center=add(start,scale(dir,-0.18+0.42*t));
        let _=draw_closed_needle(scene,center,dir,
            0.36+0.44*t,0.43+0.30*t,age*0.055,0.30+0.61*t);
    }else if age<31.0{
        let t=ease((age-10.0)/21.0);
        let center=add(start,scale(dir,4.95*t));
        let _=draw_closed_needle(scene,center,dir,
            1.00,1.20,age*0.035,0.92);
        // Whole original meshes form a short TRAIL; ghost bodies taper
        // geometrically, rather than placing cheap transparent flat planes.
        if !scene.reduced_detail {
            for j in 1..=2{
                let back=(j as f64)*0.69*(0.32+0.68*t);
                let after=add(center,scale(dir,-back));
                let _=draw_closed_needle(scene,after,dir,
                    0.48/(j as f64).sqrt(),0.58,age*0.035-(j as f64)*0.11,
                    0.15/(j as f64));
            }
        }
    }else if age<60.0{
        let t=ease((age-31.0)/29.0);
        let body_fade=(1.0-t).powf(1.45);
        // The original shell fractures in the same place as the world hit.
        let _=draw_closed_needle(scene,impact,dir,
            1.0-0.55*t,1.15-0.65*t,age*0.038,body_fade);
        let fragments=if scene.reduced_detail{2}else{5};
        for i in 0..fragments {
            let angle=TAU*i as f64/fragments as f64
                +0.18*((i*3+1) as f64).sin();
            let radial=add(scale(right,angle.cos()),scale(up,angle.sin()));
            let displacement=1.85*t*(0.75+0.1*(i%3) as f64);
            let fragment_center=add(impact,
                add(scale(radial,displacement),scale(dir,0.52*t)));
            let tilt=norm(add(scale(dir,0.42+0.20*t),
                scale(radial,0.80+0.20*t)));
            let _=draw_closed_needle(scene,fragment_center,tilt,
                (0.35-0.11*t)*(1.0+0.09*(i%2) as f64),
                0.50-0.12*t,age*0.072+angle,
                (1.0-t).powf(0.75)*0.82);
        }
    }else{
        // Two restrained, slowly descending frozen remnants, not an endlessly
        // spinning runic wire mesh or an opaque overlay in front of the camera.
        let t=ease((age-60.0)/44.0);
        for i in 0..2{
            let angle=(i as f64+0.3)*PI;
            let radial=add(scale(right,angle.cos()),scale(up,angle.sin()));
            let center=add(impact,add(scale(radial,1.1+0.45*t),
                [0.0,-0.40*t,0.0]));
            let aim=norm(add(scale(dir,0.47),scale(radial,0.53)));
            let _=draw_closed_needle(scene,center,aim,0.20,0.30,
                angle+age*0.025,0.30*(1.0-t));
        }
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;
    fn cast()->Cast {Cast{kind:SLOT,variant:0,start:0,
        center:[3.0,71.0,-2.0],direction:[0.21,-0.04,0.98]}}
    fn measure(age:f64,reduced:bool,cap:usize)->Scene{
        let mut scene=Scene::with_lod(cap,reduced);
        assert!(render(cast(),age,&mut scene));
        scene
    }
    #[test]
    fn original_closed_blender_mesh_keeps_all_original_surfaces(){
        assert_eq!(mesh::VERTICES.len(),130);
        assert_eq!(QUAD_COUNT,124);
        assert_eq!(mesh::QUADS.len(),QUAD_COUNT);
        let mut sources=[0;3];
        for (q,material) in mesh::QUADS{
            sources[material as usize]+=1;
            assert!(q.into_iter().all(|i|i<mesh::VERTICES.len()));
            let ab=super::super::sub(mesh::VERTICES[q[1]],mesh::VERTICES[q[0]]);
            let ad=super::super::sub(mesh::VERTICES[q[3]],mesh::VERTICES[q[0]]);
            assert!(super::super::length(super::super::cross(ab,ad))>0.00001);
        }
        assert_eq!(sources,[64,40,20]);
        let zmin=mesh::VERTICES.iter().map(|v|v[2]).fold(f64::INFINITY,f64::min);
        let zmax=mesh::VERTICES.iter().map(|v|v[2]).fold(f64::NEG_INFINITY,f64::max);
        assert!(zmax-zmin>2.0,"Not a flat billboard");
    }
    #[test]
    fn the_four_stages_have_full_meshes_and_distinct_world_positions(){
        let frames=[3.0,20.0,43.0,85.0];
        let mut centroids=Vec::new();
        for t in frames{
            let scene=measure(t,false,900);
            assert!(!scene.facets.is_empty());
            assert_eq!(scene.facets.len()%QUAD_COUNT,0,
                "Never drop an individual surface face");
            assert_eq!(scene.group_ids.len(),scene.facets.len());
            assert!(scene.facets.iter().all(|(q,_)|q.iter().flatten().all(|v|v.is_finite())));
            let mean=scene.facets.iter().flat_map(|(q,_)|q.iter())
                .map(|v|v[2]).sum::<f64>()/(scene.facets.len()*4) as f64;
            centroids.push(mean);
        }
        assert!((centroids[0]-centroids[1]).abs()>0.5);
        assert!((centroids[1]-centroids[2]).abs()>0.3);
    }
    #[test]
    fn reduced_quality_respects_atomic_geometry_and_bad_input(){
        for (time,limit) in [(3.0,QUAD_COUNT),(20.0,QUAD_COUNT),
            (43.0,QUAD_COUNT*3),(80.0,QUAD_COUNT*2)] {
            let scene=measure(time,true,limit);
            assert!(scene.facets.len()<=limit);
            assert_eq!(scene.facets.len()%QUAD_COUNT,0);
        }
        let mut blank=Scene::new(900);
        assert!(!render(Cast{kind:21,..cast()},22.0,&mut blank));
        assert!(blank.facets.is_empty());
        assert!(render(cast(),f64::NAN,&mut blank));
        assert!(blank.facets.is_empty());
        assert!(!draw_closed_needle(&mut blank,[0.,0.,0.],[0.,0.,0.],
            1.,1.,0.,1.));
    }
}
