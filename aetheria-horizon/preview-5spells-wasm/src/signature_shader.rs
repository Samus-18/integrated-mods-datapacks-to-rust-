//! M37.2 true procedural PIXEL SHAPES for eleven specifically named spells.
//! The source of the silhouette is a separate shader, NOT Blender models,
//! not source-quad rods/rings, and not another cloned particle dot field.
//! This CPU module contributes exactly ONE camera-facing bounded proxy quad
//! per signature, carrying material identity in alpha byte for WGSL SDFs.
//! It never submits unmasked proxies to WorldQuad; fallback is original
//! 3D signature_forms whenever this optional shader fails/does not load.
//! NOTE: shader masking is screen-aligned 2.5D, not a full-screen world post,
//! and cannot claim depth-volume or beauty until verified on Horizon GPU.
use super::{Cast,Scene,add,scale};
use super::procedural_fx::sprite;
const SIGNATURES:[usize;11]=[0,6,29,47,63,86,101,123,144,161,186];
const BASE_RGB:[u32;11]=[
    0x00FF8B32, // Ember Needle
    0x00FF5D16, // Pyre Serpent
    0x0093DCFF, // Snowblind
    0x00A4F5C1, // Twister Seed
    0x00FFAC4B, // Faultline
    0x0045C3F9, // Rainweaver
    0x008DE36E, // Root Snare
    0x0088B8FF, // Arc Net
    0x00C378FB, // Rift Slice
    0x00FFD998, // Moment Slip
    0x00B588EB, // Carrion Swarm
];
// Material IDs separated by 21/255, quantized independent of themed RGB.
pub(super) fn material_for(slot:usize)->Option<(usize,u8,u32)>{
    SIGNATURES.iter().position(|id|*id==slot)
        .map(|index|(index,(23+index*21) as u8,BASE_RGB[index]))
}
pub(super) fn is_signature(slot:usize)->bool {
    material_for(slot).is_some()
}
fn smooth(t:f64)->f64{let t=t.clamp(0.0,1.0);t*t*(3.0-2.0*t)}
/// A single solidly shaded mask whose *pixel shader* performs all shape
/// synthesis, turbulence, emissive flow and feathering. No impostor
/// full-screen bloom; optional extra .hshader is separately installed.
pub(super) fn render(cast:Cast,age:f64,camera:[f64;3],scene:&mut Scene)->usize{
    let Some((id,tag,rgb))=material_for(cast.kind)else{return 0;};
    if !age.is_finite() || !(0.0..104.0).contains(&age)
        ||!cast.center.iter().chain(cast.direction.iter()).all(|v|v.is_finite())
        ||!camera.iter().all(|v|v.is_finite()) {return 0;}
    let formation=smooth((age-2.0)/13.0);
    let dissolution=1.0-smooth((age-78.0)/26.0);
    let grow=formation*dissolution;
    if grow<0.02{return 0;}
    // A hero is appropriately large in world space. The last attempt drew
    // tiny 100px glyphs at a normal 20-block third-person distance.
    let shapes:[(f64,f64,f64,f64);11]=[
        (2.6,5.5,1.3,0.44),   // spear, diagonal along target aim
        (5.2,5.9,2.0,-0.08), // curled head+body fire serpent
        (6.5,6.2,1.8,0.03),  // sweeping blizzard+snowflake core
        (5.5,6.3,2.3,-0.03), // tall tapered storm funnel
        (8.1,4.5,0.1,-0.18), // long jagged earth rupture
        (6.0,5.3,2.2,0.10),  // three large braided streams
        (5.0,5.7,2.1,0.02),  // growing roots around target
        (6.3,5.5,2.2,0.08),  // branching joined electrical net
        (7.0,6.1,2.0,-0.34), // diagonal torn dimensional slash
        (5.1,5.4,2.3,0.02),  // broken chrono glyph and echoes
        (6.4,5.8,2.0,0.08),  // ghost skull-bird swarm
    ];
    let (width,height,lift,roll)=shapes[id];
    let center=add(cast.center,[0.0,lift,0.0]);
    let before=scene.facets.len();
    let _=sprite(scene,camera,center,width*grow.max(0.08),
        height*grow.max(0.08),roll,
        rgb,tag);
    scene.facets.len()-before
}
#[cfg(test)]
mod tests {
    use super::*;
    fn cast(slot:usize)->Cast {
        Cast{kind:slot,variant:0,start:0,center:[0.0,65.0,1.0],
             direction:[0.24,-0.2,0.92]}
    }
    #[test]
    fn exactly_eleven_name_bound_shader_material_ids_no_generic_colour_clones(){
        assert_eq!(SIGNATURES.len(),11);
        let tags:Vec<_>=(0..200)
          .filter_map(|s|material_for(s).map(|(_,m,_)|(s,m))).collect();
        assert_eq!(tags.len(),11);
        assert_eq!(tags.iter().map(|(_,m)|*m)
           .collect::<std::collections::BTreeSet<_>>().len(),11);
        assert!(tags.iter().all(|(_,m)|*m>=23 && *m<=233));
        assert_eq!(tags[1].0,6);
    }
    #[test]
    fn all_eleven_actual_proxies_camera_facing_bounded_and_animated(){
        for slot in SIGNATURES {
            for camera in [[9.0,72.0,10.0],[-6.0,75.0,-9.0],
                           [0.0,113.0,1.0]] {
                let mut early=Scene::new(3);
                let mut main=Scene::new(3);
                assert_eq!(render(cast(slot),10.0,camera,&mut early),1);
                assert_eq!(render(cast(slot),46.0,camera,&mut main),1);
                assert_ne!(early.facets,main.facets);
                assert_eq!(main.group_ids.len(),1);
                let q=main.facets[0].0;
                assert!(q.iter().flatten().all(|v|v.is_finite()));
                let center=scale(q.iter().copied().fold([0.0;3],add),0.25);
                let normal=super::super::cross(
                    super::super::sub(q[1],q[0]),
                    super::super::sub(q[3],q[0]));
                assert!(normal.iter().zip(super::super::sub(camera,center))
                    .map(|(a,b)|a*b).sum::<f64>()>0.0);
            }
        }
    }
    #[test]
    fn no_proxy_when_shader_is_absent_or_budget_zero(){
        let mut scene=Scene::new(0);
        assert_eq!(render(cast(6),35.0,[0.0,75.0,8.0],&mut scene),0);
        assert!(scene.facets.is_empty());
        assert!(!is_signature(199));
        let mut scene=Scene::new(1);
        assert_eq!(render(cast(199),40.0,[0.0,75.0,8.0],&mut scene),0);
    }
}
