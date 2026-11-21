// M37 fastest independent test of EXACT source WGSL in Naga 30.0.1.
// A pass only proves syntax/validation, never beauty in Horizon GPU.
#[test]
fn m37_name_specific_real_wgsl_matches_naga(){
    let shader=include_str!(
        "../../../../preview-5spells-wasm/assets/aetheria_preview/shaders/pipeline/signature_sdf.wgsl"
    );
    let prelude=r#"
        struct PipelineInput {
            uv: vec2<f32>,
            color: vec4<f32>,
            position: vec3<f32>,
        };
        struct TestFrame { time: f32 };
        var<private> frame: TestFrame;
    "#;
    let full=format!("{prelude}\n{shader}");
    let m=naga::front::wgsl::parse_str(&full)
        .unwrap_or_else(|e|panic!("{}",e.emit_to_string(&full)));
    let names=["a37_shape","a37_hash","a37_noise","a37_fbm",
               "a37_line","a37_ellipse","a37_seg","a37_edge",
               "pipeline_fragment"];
    for n in names{
        assert!(m.functions.iter().any(|(_,f)|f.name.as_deref()==Some(n)),
            "Missing per-pixel shape function: {n}");
    }
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(),
       naga::valid::Capabilities::all()).validate(&m)
       .expect("EXACT M37 shader fails Naga 30.0.1 validation");
}