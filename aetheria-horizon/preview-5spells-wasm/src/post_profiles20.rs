//! Original first *batch* of 20 cross-school VFX post profiles.
//! Mapping to the canonical 200 entries is stable, but the shader bridge
//! requires an independent NATIVE Horizon mod and .hshader. These are
//! NOT 20 finished spells: audio, combat, unlock, visuals need actual QA.
#[allow(dead_code)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub(super) struct PostProfile {
    pub spell_id:u16, pub profile:u8, pub name:&'static str,
    pub motion:&'static str, pub tint:[f32;3],
    pub radius:f32, pub charge_s:f32,pub sustain_s:f32,pub dissolve_s:f32,
}
pub(super) const POST20:[PostProfile;20]=[
    PostProfile{spell_id:1,profile:1,name:"Ember Needle",motion:"bent fire spear/heat tongues",tint:[1.0,0.28,0.02],radius:5.2,charge_s:0.55,sustain_s:0.95,dissolve_s:0.65},
    PostProfile{spell_id:2,profile:2,name:"Cinder Fan",motion:"seven opening flame wings",tint:[0.95,0.22,0.04],radius:5.4,charge_s:0.65,sustain_s:1.35,dissolve_s:0.82},
    PostProfile{spell_id:21,profile:3,name:"Frost Needle",motion:"hexagonal cracked ice spear",tint:[0.26,0.72,1.0],radius:4.9,charge_s:0.52,sustain_s:1.00,dissolve_s:0.70},
    PostProfile{spell_id:22,profile:4,name:"Rime Bridge",motion:"six solid ascending frost plates",tint:[0.47,0.78,1.0],radius:6.2,charge_s:0.80,sustain_s:2.60,dissolve_s:1.0},
    PostProfile{spell_id:41,profile:5,name:"Gale Palm",motion:"compressed air pressure-front bloom",tint:[0.40,0.91,0.84],radius:4.5,charge_s:0.35,sustain_s:0.55,dissolve_s:0.63},
    PostProfile{spell_id:42,profile:6,name:"Razor Crescent",motion:"sharp curved wind blade",tint:[0.37,0.83,0.78],radius:5.0,charge_s:0.37,sustain_s:1.0,dissolve_s:0.70},
    PostProfile{spell_id:61,profile:7,name:"Stone Knuckle",motion:"four articulated rocks into giant fist",tint:[0.62,0.45,0.29],radius:4.6,charge_s:0.55,sustain_s:0.85,dissolve_s:0.85},
    PostProfile{spell_id:62,profile:8,name:"Crystal Traverse",motion:"rising mineral faceted staircase",tint:[0.63,0.48,0.88],radius:5.9,charge_s:1.0,sustain_s:2.7,dissolve_s:1.2},
    PostProfile{spell_id:81,profile:9,name:"Tide Needle",motion:"two braided liquid cores",tint:[0.21,0.70,1.0],radius:5.2,charge_s:0.45,sustain_s:0.95,dissolve_s:0.70},
    PostProfile{spell_id:82,profile:10,name:"Flowstep",motion:"traveling breaking wave floor",tint:[0.31,0.87,0.95],radius:5.0,charge_s:0.55,sustain_s:1.25,dissolve_s:0.85},
    PostProfile{spell_id:101,profile:11,name:"Thorn Dart",motion:"living dense seven-lobed leaf bud",tint:[0.36,0.87,0.19],radius:4.7,charge_s:0.50,sustain_s:0.90,dissolve_s:0.82},
    PostProfile{spell_id:102,profile:12,name:"Root Snare",motion:"tangled earth roots and leaf canopy",tint:[0.55,0.76,0.14],radius:5.1,charge_s:0.85,sustain_s:2.25,dissolve_s:1.05},
    PostProfile{spell_id:121,profile:13,name:"Spark Fork",motion:"three-fork violet thunder",tint:[0.58,0.78,1.0],radius:4.7,charge_s:0.35,sustain_s:0.62,dissolve_s:0.55},
    PostProfile{spell_id:122,profile:14,name:"Static Halo",motion:"charged plasma torus and core",tint:[0.39,0.78,1.0],radius:5.0,charge_s:0.74,sustain_s:1.34,dissolve_s:0.85},
    PostProfile{spell_id:141,profile:15,name:"Star Shard",motion:"purple fractured star crystal",tint:[0.70,0.49,1.0],radius:4.8,charge_s:0.55,sustain_s:1.2,dissolve_s:0.75},
    PostProfile{spell_id:142,profile:16,name:"Gravity Thread",motion:"dark lensing folded gravity well",tint:[0.54,0.18,0.91],radius:5.6,charge_s:0.92,sustain_s:1.65,dissolve_s:1.12},
    PostProfile{spell_id:161,profile:17,name:"Second Hand",motion:"disjoint chrono dial fragments",tint:[0.91,0.72,0.24],radius:5.4,charge_s:0.65,sustain_s:2.2,dissolve_s:0.90},
    PostProfile{spell_id:162,profile:18,name:"Moment Slip",motion:"four torn moving temporal silhouettes",tint:[0.54,0.68,1.0],radius:5.4,charge_s:0.70,sustain_s:1.1,dissolve_s:1.1},
    PostProfile{spell_id:181,profile:19,name:"Bone Thorn",motion:"hooked skeletal branches",tint:[0.68,0.37,0.79],radius:5.0,charge_s:0.60,sustain_s:1.4,dissolve_s:1.0},
    PostProfile{spell_id:182,profile:20,name:"Soul Lantern",motion:"filled spirit vessel plus floating wisps",tint:[0.46,0.73,0.98],radius:5.1,charge_s:0.83,sustain_s:2.3,dissolve_s:1.15},
];
#[allow(dead_code)]
pub(super) fn profile_for_spell(spell_id:u16)->Option<&'static PostProfile>{
    POST20.iter().find(|v|v.spell_id==spell_id)
}
/// Wire contract for future native Hooks::shader_params, NOT called by guest.
/// center is ABSOLUTE world, camera is ABSOLUTE world; shader reads delta.
#[allow(dead_code)]
pub(super) fn pack_frame_rows(
    profile:&PostProfile, phase:f32,power:f32, center:[f32;3],
    camera:[f32;3],dir:[f32;3]
)->[[f32;4];8]{
    let mut out=[[0.0_f32;4];8];
    if !phase.is_finite() || !power.is_finite() ||
        !center.iter().chain(camera.iter()).chain(dir.iter()).all(|x|x.is_finite())
        {return out;}
    let d2=dir.iter().map(|x|x*x).sum::<f32>();
    if d2<0.0001{return out;}
    let root=d2.sqrt();
    let forward=[dir[0]/root,dir[1]/root,dir[2]/root];
    out[0]=[f32::from(profile.profile),phase.clamp(0.0,1.0),power.clamp(0.0,2.0),1.0];
    out[1]=[center[0]-camera[0],center[1]-camera[1],
            center[2]-camera[2],profile.radius];
    out[2]=[forward[0],forward[1],forward[2],0.0];
    out[3]=[profile.tint[0],profile.tint[1],profile.tint[2],0.0];
    out
}
/// One canonical duration/validation routine shared by the runtime hook
/// and offline tests. Bad/future/expired cast events emit *zero* uniforms.
#[allow(dead_code)]
pub(super) fn packet_from_cast(
    spell_id:u16,age_ticks:f32,center:[f32;3],
    camera:[f32;3],direction:[f32;3]
)->[[f32;4];8]{
    let zero=[[0.0_f32;4];8];
    let Some(profile)=profile_for_spell(spell_id) else {return zero;};
    if !age_ticks.is_finite() || age_ticks<0.0{return zero;}
    let duration=(profile.charge_s+profile.sustain_s+profile.dissolve_s)*20.0;
    if !duration.is_finite() || duration<1.0 || age_ticks>duration {return zero;}
    pack_frame_rows(profile,age_ticks/duration,1.0,center,camera,direction)
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test] fn exactly_twenty_first_two_from_all_ten_schools(){
        let mut ids=std::collections::BTreeSet::new();
        let mut names=std::collections::BTreeSet::new();
        for (index,a) in POST20.iter().enumerate(){
            assert_eq!(a.spell_id as usize,1+20*(index/2)+(index%2));
            assert_eq!(a.profile as usize,index+1);
            assert!(names.insert(a.name));
            assert!(ids.insert(a.spell_id));
            assert!(a.radius>=4.0 && a.radius<=9.0);
            assert!(a.charge_s>0.0&&a.sustain_s>0.0&&a.dissolve_s>0.0);
            assert_ne!(a.motion,"");
            assert_eq!(profile_for_spell(a.spell_id),Some(a));
        }
        assert_eq!(ids.len(),20);assert_eq!(names.len(),20);
        assert!(profile_for_spell(199).is_none());
    }
    #[test] fn all_twenty_real_cast_packets_and_release_tails_are_safe(){
        let camera=[0.0_f32,67.0,0.0];
        let center=[1.0_f32,67.0,8.0];
        for p in POST20 {
            let length=(p.charge_s+p.sustain_s+p.dissolve_s)*20.0;
            let data=packet_from_cast(p.spell_id,length*0.54,center,
                camera,[0.0,0.1,1.0]);
            assert_eq!(data[0][0],f32::from(p.profile));
            assert_eq!(data[0][3],1.0);
            assert!(data.iter().flatten().all(|f|f.is_finite()));
            assert_eq!(packet_from_cast(p.spell_id,length+0.2,center,
                camera,[0.0,0.0,1.0]),[[0.0;4];8]);
            assert_eq!(packet_from_cast(p.spell_id,-1.0,center,
                camera,[0.0,0.0,1.0]),[[0.0;4];8]);
        }
        assert_eq!(packet_from_cast(999,12.0,center,camera,[0.,0.,1.]),
            [[0.0;4];8]);
    }
    #[test] fn zero_bad_inputs_and_unit_forward_never_nan(){
        let a=&POST20[10];
        let good=pack_frame_rows(a,0.45,1.0,[101.0,68.0,-4.0],
            [100.0,65.0,-10.0],[0.0,0.0,4.0]);
        assert_eq!(good[0],[11.0,0.45,1.0,1.0]);
        assert_eq!(good[1],[1.0,3.0,6.0,a.radius]);
        assert_eq!(good[2],[0.0,0.0,1.0,0.0]);
        for bad in [f32::NAN,f32::INFINITY] {
            assert_eq!(pack_frame_rows(a,bad,1.0,[0.0;3],[0.0;3],[0.0,0.0,1.0]),[[0.0;4];8]);
        }
        assert_eq!(pack_frame_rows(a,0.4,1.0,[0.0;3],[0.0;3],[0.0;3]),[[0.0;4];8]);
    }
}
