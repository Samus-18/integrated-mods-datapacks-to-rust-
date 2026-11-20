//! M34 Rime Bridge: ORIGINAL independent Blender-authored cryomancy geometry.
//! Eight individually sculpted watertight ice flagstones and two deep
//! glacial buttresses, staged to grow from the targeted ground point.
//! CLIENT VFX ONLY: no fake collision/platform or server world blocks.
//! Does not borrow the Frost Needle mesh, scale a rod, or reuse ring cards.
use super::{Cast, Scene, add, scale, shade_argb};

mod original_mesh {
    include!("rime_bridge_mesh_generated.rs");
}
const SPELL_SLOT:usize=21; // Registry #022, Cryomancer Rime Bridge
const BODY_COLORS:[u32;3]=[0x003F7D9E,0x0064B8DA,0x0080E0D4];

fn ease(x:f64)->f64{
    let q=x.clamp(0.0,1.0);
    q*q*(3.0-2.0*q)
}
fn frame(dir:[f64;3])->([f64;3],[f64;3],[f64;3]){
    // Support pitching the camera straight down: never align the bridge
    // with the vertical cast ray or create infinite/NaN coordinates.
    let horizontal=[dir[0],0.0,dir[2]];
    let l=(horizontal[0]*horizontal[0]+horizontal[2]*horizontal[2]).sqrt();
    let f=if l.is_finite() && l>0.0001 {
        [horizontal[0]/l,0.0,horizontal[2]/l]
    }else{[0.0,0.0,1.0]};
    let right=[f[2],0.0,-f[0]];
    (f,right,[0.0,1.0,0.0])
}
fn quad_color(material:u8,index:usize,opacity:f64)->u32{
    let base=BODY_COLORS[(material as usize).min(2)];
    let alpha=((255.0*opacity.clamp(0.0,1.0)).round() as u32).min(255);
    let brightness=0.83+0.16*(0.5+0.5*(index as f64*2.71).sin());
    shade_argb(base|(alpha<<24),brightness)
}
fn draw_part(
    scene:&mut Scene,part:usize,cast:Cast,age:f64,
    progress:f64,opacity:f64
)->bool {
    if !(0.0..=1.0).contains(&progress)||!opacity.is_finite()
        ||!cast.center.iter().chain(cast.direction.iter()).all(|v|v.is_finite())
        ||part>=original_mesh::PARTS.len() {return false;}
    let (first,count)=original_mesh::PARTS[part];
    let Some(previous)=scene.begin_component(count) else{return false;};
    let (forward,right,up)=frame(cast.direction);
    let plate=part<8;
    let lift=if plate {
        -0.82*(1.0-progress)
    }else{
        -0.51*(1.0-progress)
    };
    let fade=opacity*if age<82.0 {1.0}else{
        (1.0-ease((age-82.0)/22.0))
    };
    let jostle=if plate {0.043*(age*0.16+part as f64*0.75).sin()}
               else {0.02*(age*0.08+part as f64).sin()};
    let scale_z=if plate {1.0}else{0.35+0.65*progress};
    let center=add(cast.center,scale(up,lift+jostle));
    let point=|id:usize| {
        // OBJ already uses Y as world-up and Z as original Blender long axis.
        // Retain the source vertex normals/winding by using a right-handed
        // forward/right/up frame with determinant positive.
        let v=original_mesh::VERTICES[id];
        add(center,add(scale(right,v[0]),
            add(scale(up,v[1]),scale(forward,v[2]*scale_z))))
    };
    for (i,(ids,mat)) in original_mesh::QUADS[first..first+count].iter().enumerate(){
        scene.quad(ids.map(point),quad_color(*mat,i+first,fade));
    }
    scene.end_component(previous);
    true
}
/// Four readable phases: the plates manifest, interlock from near to far,
/// glacial side wings unfurl, then the entire structure slowly dissolves.
/// The visual bridge has no collision and does NOT grant traversability.
pub(super) fn render(cast:Cast,age:f64,scene:&mut Scene)->bool{
    if cast.kind!=SPELL_SLOT{return false;}
    if !age.is_finite() || !(0.0..104.0).contains(&age){return true;}
    if !cast.center.iter().chain(cast.direction.iter()).all(|v|v.is_finite()){return true;}
    for i in 0..8 {
        let born=4.0+i as f64*5.1;
        if age<born {continue;}
        let progress=ease((age-born)/9.2);
        if progress<0.01 {continue;}
        let opacity=0.66+0.28*progress;
        let _=draw_part(scene,i,cast,age,progress,opacity);
    }
    for i in 8..10 {
        let born=29.0+(i-8) as f64*7.4;
        if age<born {continue;}
        let progress=ease((age-born)/18.0);
        if progress<0.01 {continue;}
        // Wide solid supports, never lines or holographic empty hoops.
        let _=draw_part(scene,i,cast,age,progress,0.90);
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;
    fn cast()->Cast{Cast{
        kind:SPELL_SLOT,variant:1,start:0,
        center:[0.,70.,3.0],direction:[0.37,-0.68,0.68],
    }}
    #[test]
    fn original_bridge_has_nine_distinct_closed_mesh_components(){
        assert_eq!(original_mesh::PARTS.len(),10);
        assert_eq!(original_mesh::VERTICES.len(),456);
        assert_eq!(original_mesh::QUADS.len(),436);
        let mut end=0;
        for (i,(first,n)) in original_mesh::PARTS.iter().enumerate(){
            assert_eq!(*first,end);
            assert_eq!(*n,if i<8 {32}else{90});
            end+=n;
        }
        assert_eq!(end,436);
        assert!(original_mesh::VERTICES.iter().flatten().all(|v|v.is_finite()));
        for (ids,mat) in original_mesh::QUADS {
            assert!((mat as usize)<3);
            assert!(ids.into_iter().all(|id|id<original_mesh::VERTICES.len()));
        }
    }
    #[test]
    fn stages_produce_full_anatomical_faces_and_change_with_time(){
        let mut counts=Vec::new();
        for age in [11.0,25.0,44.0,61.0,95.0]{
            let mut s=Scene::with_lod(900,false);
            assert!(render(cast(),age,&mut s));
            assert!(!s.facets.is_empty());
            assert!(s.facets.len()<=436);
            assert_eq!(s.facets.len(),s.group_ids.len());
            // Every source mesh stays wholly rendered; zero lopped-off faces.
            let mut groups=std::collections::BTreeMap::new();
            for id in s.group_ids{
                *groups.entry(id).or_insert(0usize)+=1;
            }
            for (_,n) in groups {
                assert!(n==32||n==90,"Unexpected partial Blender mesh {n}");
            }
            counts.push(s.facets.len());
        }
        assert!(counts[0]<counts[1]);
        assert!(counts[1]<counts[2]);
        assert!(counts[3]>=counts[2]);
    }
    #[test]
    fn hard_quads_budget_safe_for_rx580_distance_lod(){
        for cap in [0,32,96,224,256,300,380,900]{
            for reduced in [false,true]{
                let mut s=Scene::with_lod(cap,reduced);
                assert!(render(cast(),58.0,&mut s));
                assert!(s.facets.len()<=cap);
                let mut groups=std::collections::BTreeMap::new();
                for (id,(poly,color)) in s.group_ids.iter().zip(s.facets.iter()){
                    assert!(poly.iter().flatten().all(|v|v.is_finite()));
                    assert!(*color>0);
                    *groups.entry(id).or_insert(0usize)+=1;
                }
                assert!(groups.values().all(|n|*n==32||*n==90));
            }
        }
    }
    #[test]
    fn only_correct_registry_slot_and_finite_aim(){
        let mut s=Scene::new(900);
        assert!(!render(Cast{kind:20,..cast()},60.0,&mut s));
        assert!(s.facets.is_empty());
        assert!(render(Cast{direction:[f64::NAN,0.,1.],..cast()},60.0,&mut s));
        assert!(s.facets.is_empty());
        assert!(render(Cast{direction:[0.,-1.,0.],..cast()},60.0,&mut s));
        assert!(s.facets.len()>0);
    }
}
