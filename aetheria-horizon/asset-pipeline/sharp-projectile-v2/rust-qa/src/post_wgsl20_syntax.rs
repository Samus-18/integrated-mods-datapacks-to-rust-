//! Fast local Naga parser for *exact* M43 production shader code.
//! Official Horizon check in CI remains the final authority for bindings.
const SHARED:&str=include_str!("../../../../shaderpacks/aetheria_spell20/profiles20.wgsl");
const EMISSION:&str=include_str!("../../../../shaderpacks/aetheria_spell20/post/emission.wgsl");
const BLUR_H:&str=include_str!("../../../../shaderpacks/aetheria_spell20/post/blur_h.wgsl");
const BLUR_V:&str=include_str!("../../../../shaderpacks/aetheria_spell20/post/blur_v.wgsl");
const COMPOSITE:&str=include_str!("../../../../shaderpacks/aetheria_spell20/post/composite.wgsl");
const PREVIEW_MOD:&str=include_str!("../../../../preview-5spells-wasm/src/lib.rs");
// Naga parses the same WGSL grammar as Horizon, with lightweight stand-ins
// for the Horizon post-only prelude provided by the official pack loader.
const PRELUDE:&str=r#"
struct PostInput {
    uv:vec2<f32>,pixel:vec2<f32>,
    color:vec4<f32>,depth:f32,
};
struct Frame {
    mods:array<vec4<f32>,8>,
    view_inverse:mat4x4<f32>,time:f32,headroom:f32,camera_position:vec3<f32>,
};
@group(3) @binding(0) var<uniform> frame:Frame;
@group(3) @binding(1) var scene_color:texture_2d<f32>;
@group(3) @binding(4) var linear_sampler:sampler;
@group(3) @binding(32) var aetheria_emission:texture_2d<f32>;
@group(3) @binding(33) var aetheria_blur_h:texture_2d<f32>;
@group(3) @binding(34) var aetheria_blur_v:texture_2d<f32>;
fn view_position(uv:vec2<f32>,depth:f32)->vec3<f32> {
    return vec3<f32>(uv*2.0-vec2<f32>(1.0),depth);
}
"#;
#[test]
fn all_four_real_post_stages_parse_in_naga_30(){
    for (name,stage) in [
        ("emission",EMISSION),("blur_h",BLUR_H),
        ("blur_v",BLUR_V),("composite",COMPOSITE)
    ] {
        let shader=format!("{PRELUDE}\n{SHARED}\n{stage}");
        naga::front::wgsl::parse_str(&shader).unwrap_or_else(|err|
            panic!("{name} WGSL:\n{}",err.emit_to_string(&shader)));
    }
}
#[test]
fn original_twenty_spells_unique_shapes_and_no_invalid_reserved_identifiers(){
    let branches:Vec<usize>=SHARED.lines()
        .filter_map(|line|{
            let prefix="if(id==";
            let s=line.trim_start().strip_prefix(prefix)?;
            let digits=s.split(')').next()?;
            digits.parse::<usize>().ok()
        }).collect();
    assert_eq!(branches,(1..=20).collect::<Vec<_>>());
    for reserved in ["let from=","let active=","let type=","let pass="]{
        assert!(!SHARED.contains(reserved),"WGSL reserved: {reserved}");
    }
    assert_eq!(SHARED.matches("fn fx_profile(").count(),1);
    assert!(SHARED.contains("frame.mods[1].xyz"));
    assert!(SHARED.contains("let center=fixed_world-frame.camera_position;"));
    assert!(!SHARED.contains("let center=aim*8.0"));
    assert!(SHARED.contains("fn fx_is_solid("));
    assert!(SHARED.contains("fn fx_shaded_surface("));
    assert!(COMPOSITE.contains("rgb=mix(input.color.rgb,vol.light,vol.coverage)"));
    assert!(SHARED.contains("terrain_depth=input.depth"));
    assert!(SHARED.contains("view_position(input.uv,terrain_depth)"));
}

#[test]
fn m44_first_twenty_cannot_render_old_camera_facing_hero_proxies(){
    assert!(PREVIEW_MOD.contains("fn use_legacy_signature_proxy(slot:usize)->bool"));
    assert!(PREVIEW_MOD.contains("post_profiles20::profile_for_spell(catalog200::id(slot)).is_none()"));
    assert!(PREVIEW_MOD.contains("if !use_legacy_signature_proxy(cast.kind){continue;}"));
    assert!(PREVIEW_MOD.contains("if !signature_rendered || !use_legacy_signature_proxy(cast.kind)"));
    assert!(PREVIEW_MOD.contains("m44_world_anchored_batch_tests"));
}
