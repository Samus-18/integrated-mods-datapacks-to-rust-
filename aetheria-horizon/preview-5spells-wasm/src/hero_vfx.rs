//! M17: surface-first hero VFX — Shard Chorus (Cryomancer #3, registry ID 23).
//! The user rejected wireframe rods/rings. This is an original fully faceted,
//! stagger-grown glacier formation with volumetric-looking fog/aurora surfaces.
//! All meshes are built through the pinned Horizon world-quad guest API;
//! not an HDR renderer, off-screen camera effect or dynamic world light.
use super::{Cast,Scene,add,sub,scale,alpha,shade_argb,norm};
use crate::cinematic_beams::visual_phase;
use aetheria_horizon::vfx::timeline::Stage;
use std::f64::consts::TAU;

type Vec3=[f64;3];
fn at(o:Vec3,x:f64,y:f64,z:f64)->Vec3{add(o,[x,y,z])}
fn lerp(a:Vec3,b:Vec3,t:f64)->Vec3{add(a,scale(sub(b,a),t))}
fn translucent(c:u32,opacity:f64)->u32{
    let a=(opacity.clamp(0.0,1.0)*255.0).round() as u32;
    (c&0x00FF_FFFF)|(a<<24)
}
fn crystal_palette(index:usize,layer:usize,age:f64,opacity:f64)->u32{
    // Deliberately calibrated violet-teal-blue glass, not a single recolor.
    let p=[
        0xFF9CEBEC_u32, // frost vein
        0xFF5CB7E0_u32, // cold blue shadow
        0xFF8D8AE5_u32, // iridescent violet shadow
        0xFFB8F9F0_u32, // glint
        0xFF547EB5_u32, // translucent icy interior
    ];
    let base=p[(index*7+layer*3)%p.len()];
    let glint=0.79+0.12*((age*0.036+index as f64*1.71+layer as f64).sin()).abs();
    translucent(shade_argb(base,glint),opacity)
}
/// Closed 3D prismatic crystal with a midsection "cleavage" and a second
/// smaller faceted internal vein. Neither part is a flat crossed billboard.
fn glacier_spire(scene:&mut Scene,o:Vec3,index:usize,phase:f64,grow:f64){
    if grow<0.025{return;}
    let j=index as f64;
    let angle=TAU*j*0.3819660112501051+0.24*(j*2.3).sin();
    let radius=if index==0 {0.0}else{1.2+0.36*(index%5) as f64};
    let lean=(0.18+0.24*(j*1.93).sin().abs())*grow;
    let base=at(o,
        angle.cos()*radius,
        -0.63-0.13*(index%3) as f64,
        angle.sin()*radius);
    let height=if index==0 {8.6}else{3.5+(j*1.721).sin().abs()*3.3};
    let tip=at(base,
        angle.cos()*lean,
        (height*grow*(1.0+0.05*(phase*0.16+j).sin())).max(0.17),
        angle.sin()*lean);
    let thickness=if index==0 {1.24}else{0.58+0.16*(j*0.91).cos().abs()};
    let c0=crystal_palette(index,0,phase,0.87);
    let c1=crystal_palette(index,1,phase,0.72);
    scene.solid_crystal(base,tip,thickness*grow.max(0.25),angle*0.4,c0,6);
    // Faceted internal cleavage is offset and shorter; changes silhouette
    // from all camera angles rather than drawing a thin highlight line.
    if grow>0.43 {
        let b=lerp(base,tip,0.19);
        let t=lerp(base,tip,0.92);
        scene.solid_crystal(
            at(b,0.12*(angle+0.5).cos(),0.,0.12*(angle+0.5).sin()),
            t,thickness*0.31,angle+0.7,c1,5
        );
    }
}
fn crystal_bloom(scene:&mut Scene,o:Vec3,age:f64,energy:f64) {
    // Crown of broad angular ice faces around an opaque central spire.
    // These are connected frozen wings, never hair-thin tubes.
    for n in 0..7{
        let theta=TAU*n as f64/7.0+age*0.006;
        let g=energy.clamp(0.03,1.0);
        let inner=at(o,theta.cos()*0.51,1.6*g,theta.sin()*0.51);
        let outer=at(o,theta.cos()*2.0,4.0*g,theta.sin()*2.0);
        let wing=at(o,(theta+0.46).cos()*1.52,5.2*g,
                     (theta+0.46).sin()*1.52);
        let root=at(o,(theta-0.16).cos()*0.98,2.2*g,
                     (theta-0.16).sin()*0.98);
        scene.membrane(inner,outer,wing,root,
            crystal_palette(n,1,age,0.67));
    }
}
/// Sheets of crystal dust / cold air; almost transparent, structured and
/// spatially anchored to the base. These are broad flowing surfaces, not
/// smoke billboards or decorative rings. Original source video #6 shows
/// broad lavender-blue lit atmosphere around rigid crystals.
fn glacial_haze(scene:&mut Scene,o:Vec3,age:f64,energy:f64){
    let long=9usize;let wide=8usize;
    for sheet in 0..3 {
        let theta=sheet as f64*TAU/3.0+0.29;
        let (ca,sa)=(theta.cos(),theta.sin());
        let base_height=0.12+sheet as f64*0.20;
        for z in 0..long{
            let progress=z as f64/long as f64;
            let next=(z+1) as f64/long as f64;
            for x in 0..wide{
                let u=x as f64/wide as f64*2.0-1.0;
                let v=(x+1) as f64/wide as f64*2.0-1.0;
                let point=|w:f64,t:f64|{
                    let radial=(0.65+2.8*t)*energy.max(0.08);
                    let lateral=2.45*w;
                    let lift=base_height+0.28*(t*TAU*2.0+age*0.048+w*3.0).sin()
                       +0.52*t*energy+(sheet as f64*0.18);
                    at(o,radial*ca-lateral*sa,lift,radial*sa+lateral*ca)
                };
                // Deterministic small scale face-color alternation imitates
                // rough granular illuminated mist without a texture atlas.
                let shade=0.75+0.21*((x*7+z*3+sheet*2)%5) as f64/5.0;
                let color=translucent(
                    shade_argb(if sheet==2 {0xFF92D5EE}else{0xFF8B86DF},shade),
                    0.09+0.10*energy
                );
                scene.quad([point(u,progress),point(v,progress),
                            point(v,next),point(u,next)],color);
            }
        }
    }
}
fn ground_cleavage(scene:&mut Scene,o:Vec3,energy:f64,age:f64){
    // Fractured solid frost slabs in *terrain*, NOT a thin circle outline.
    for n in 0..9{
        let a=TAU*n as f64/9.0;
        let r=0.5+energy*(2.6+0.2*(n%4) as f64);
        let x=a.cos()*r;let z=a.sin()*r;
        let w=0.52+0.22*(n%3) as f64;
        let lift=0.08+0.33*energy*(a*2.0+age*0.04).sin().abs();
        let origin=at(o,x,-0.55,z);
        let c=crystal_palette(n,3,age,0.62);
        scene.membrane(
            at(origin,-w,0.,-w*0.5),
            at(origin,w,0.,-w*0.65),
            at(origin,w*0.8,lift,w),
            at(origin,-w*1.1,lift,w*0.8),
            c);
    }
}
pub(super) fn render_shard_chorus(cast:Cast,age:f64,scene:&mut Scene){
    // Stable registry routing: slot #06 is Cryomancer rank 3 / spell ID 23.
    if cast.kind!=5 || !age.is_finite(){return;}
    let Some((stage,progress))=visual_phase(cast.kind,age) else{return;};
    let o=cast.center;
    let q=progress.clamp(0.0,1.0);
    let fade=match stage{
        Stage::Afterglow=>(1.0-q).powf(1.25),
        _=>1.0
    };
    let energy=match stage{
        Stage::Telegraph=>0.08+0.17*q,
        Stage::Cast=>0.22+0.78*q,
        Stage::Impact=>1.0+0.05*(q*TAU).sin(),
        Stage::Afterglow=>1.0-0.66*q,
    };
    let before=scene.facets.len();
    // Ground cleavage starts early, telegraphs the blast with solid mass
    // rather than an empty circular glyph.
    ground_cleavage(scene,o,energy,age);
    const SPIRES:usize=13;
    for i in 0..SPIRES {
        // center first, then phased outward crystal eruption
        let lag=if i==0 {0.0}else{0.09+0.047*(i%5) as f64};
        let grow=((energy-lag)/(1.0-lag)).clamp(0.0,1.0);
        glacier_spire(scene,o,i,age,grow);
    }
    if energy>0.32{
        crystal_bloom(scene,o,age,(energy-0.27)*1.37);
    }
    if energy>0.45{
        glacial_haze(scene,o,age,(energy-0.35)*1.4);
    }
    if fade<0.999{
        // Preserve geometry budget and physically fade the whole formation,
        // including its heavy chunks, rather than abruptly deleting shards.
        for (_,c) in &mut scene.facets[before..] {
            let opacity=((*c>>24)&255) as f64/255.0;
            *c=translucent(*c,opacity*fade);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shard_chorus_is_full_3d_grounded_without_ring_scaffolds(){
        let id=5;
        for age in [5.,28.,46.,67.,94.]{
            let mut s=Scene::new(900);
            render_shard_chorus(Cast{kind:id,variant:2,start:0,
                center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut s);
            assert!(!s.facets.is_empty());
            assert!(s.facets.len()<=900);
            let mut mins=[f64::INFINITY;3];
            let mut maxs=[f64::NEG_INFINITY;3];
            for (quad,_) in &s.facets{
                for v in quad{
                    for k in 0..3{
                        assert!(v[k].is_finite());
                        mins[k]=mins[k].min(v[k]);maxs[k]=maxs[k].max(v[k]);
                    }
                }
            }
            assert!(mins[1]<64.0,"ice feet are floating above ground");
            assert!(maxs[1]>mins[1]+0.1);
        }
        let mut peak=Scene::new(900);
        let age=(0..104).find(|t|matches!(
           visual_phase(id,*t as f64),Some((Stage::Impact,p)) if p>0.25
        )).unwrap() as f64;
        render_shard_chorus(Cast{kind:id,variant:2,start:0,
            center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut peak);
        let has_bulk=peak.facets.iter().filter(|(q,_)|{
            let len=|a:Vec3,b:Vec3|{
                let d=sub(a,b);
                (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
            };
            len(q[0],q[1])>0.3&&len(q[1],q[2])>0.3
        }).count();
        assert!(has_bulk>35,"glacier reduced to thin outlines");
        assert!(peak.facets.len()<900);
    }
}
