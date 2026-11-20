//! M17 shader QA: parse and validate the actual guest WGSL against the Naga
//! version already pinned in the official Horizon source's Cargo.lock.
//! This is a material-language check, NOT a real GPU draw or bloom test.
#[cfg(test)]
mod tests {
    #[test]
    fn original_native_aurora_raymarch_shader_validates_under_horizon_naga(){
        // This file is **not** packaged into the current WASM preview:
        // validation proves shader-language correctness and bounded-loop
        // design, NOT that the game exposes a full-screen post hook.
        let source=include_str!("../../native-gpu-vfx/aurora_volume.wgsl");
        // Quality guard for the user's RX 580: a ray march must remain
        // explicitly bounded until real GPU timing validates a larger cap.
        assert!(source.contains("let steps=clamp(quality,6u,24u);"));
        let parsed=naga::front::wgsl::parse_str(source)
            .unwrap_or_else(|e|panic!(
                "Aetheria experimental 3D raymarch parse error:\n{}",
                e.emit_to_string(source)
            ));
        assert!(parsed.functions.iter().any(|(_,f)|
            f.name.as_deref()==Some("aetheria_integrate_magic_volume")
        ));
        let mut validator=naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all()
        );
        validator.validate(&parsed)
            .expect("Native fantasy raymarch WGSL failed Naga validation");
    }
    #[test]
    fn aetheria_material_wgsl_parses_and_validates_with_horizon_naga(){
        let actual=include_str!("../assets/aetheria_preview/shaders/pipeline/ethereal.wgsl");
        // The Horizon mod pipeline supplies PipelineInput and the frame
        // uniform when compiling the body. This synthetic prelude tests the
        // fragment syntax and typing; the runtime still must provide exact
        // bindings and semantics for the user's pinned Nightly build.
        let mock_prelude=r#"
            struct PipelineInput {
                uv: vec2<f32>,
                color: vec4<f32>,
                position: vec3<f32>,
            };
            struct AetheriaTestFrame {
                time: f32,
            };
            var<private> frame: AetheriaTestFrame;
        "#;
        let source=format!("{mock_prelude}\n{actual}");
        let parsed=naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error|panic!(
                "M17 cinematic material WGSL parse error:\n{}",
                error.emit_to_string(&source)
            ));
        // Names of the ORIGINAL source-shaped material motions. Parsing
        // against the real pinned PipelineInput.position and time contract
        // is more useful than a pure text-check for "pretty shader".
        for name in [
            "pipeline_fragment","aetheria_ember_flow",
            "aetheria_glacier_cleavage","aetheria_living_sap",
            "aetheria_lightning_branches","aetheria_rift_fault",
            "aetheria_haze_density","aetheria_integrate_haze"
        ] {
            assert!(parsed.functions.iter().any(|(_,f)|
                f.name.as_deref()==Some(name)
            ),"missing M30 material expression: {name}");
        }
        assert!(actual.contains("input.position"),
            "M30 must use the documented camera-relative world position");
        assert!(actual.contains("let steps=select(6u,10u,distance<34.0);"),
            "M31 fog raymarch must have distance LOD 6 or 10 steps");
        assert!(actual.contains("for(var i=0u;i<10u;i=i+1u)"),
            "M31 raymarch samples MUST remain bounded for older Radeon GPUs");
        assert!(actual.contains("if(input.color.a>0.005 && input.color.a<0.22)"),
            "M31 fog must shade transparent atmosphere, not opaque crystals");
        assert!(actual.contains("input.uv,input.position,t"),
            "M31 fog must use actual local face UV and camera-relative distance");

        assert!(actual.contains("1.14"),
            "M30 light energy must remain capped instead of white-out");
        assert!(!actual.contains("textureSample(") && !actual.contains("discard;"),
            "Guest pipeline has no scene depth/texture; do not fake refraction");
        let mut validator=naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all()
        );
        validator.validate(&parsed)
            .expect("M17 shader is not accepted by pinned Horizon Naga");
    }
}
