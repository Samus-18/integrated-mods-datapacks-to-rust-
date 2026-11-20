# M30 — Actual Horizon WGSL material and intact glacial haze

Date: 2026-11-20. Internal development, not a user-facing build.

## Grounded rendering API

Inspected exact pinned Horizon SDK commit dd67b76ff23e28c6d8e21859df600d31572f81a0, especially docs/modding/src/how-to/client/render-pipelines.md. The guest world-quad PipelineInput has uv, linear color, camera-relative position in blocks, and frame.time in seconds; optional depth-tested/translucent render pipelines are supported. It has NO scene color/depth texture, fullscreen stage, per-spell uniforms or camera matrices; without a separate native Rust bridge, cinematic post raymarch, refraction and true HDR world lighting cannot be claimed.

The native-only horizon-mod-render/src/render.rs *does* expose WorldDrawn with GPU device/encoder and world depth when requested. That is a **separate trusted native build route**, not the existing WASM preview. Also researched Horizon shader API 0.2 full-screen passes: these require an independent shader pack and a real uniform handoff for selected spells. Do not invent a bridge or port to C++/Iris/OptiFine.

## Actual M30 code

- The real packaged preview material asset, preview-5spells-wasm/assets/aetheria_preview/shaders/pipeline/ethereal.wgsl, has five original animated procedural expressions: upward curled fire tongues, jagged glacial cleavage, growing living sap, forked lightning and broken astral void faults. They shade the source 3D scene surfaces and respect different per-face ARGB pigments instead of a generic single noise recolor. The output remains capped to 1.14 linear brightness and max 0.88 alpha to avoid whiteout.
- WGSL dpdx/dpdy of the actual interpolated world position provides fragment face normals and a small grazing-rim/facet term, for directional 3D volume readability. It is not world light, shadow casting or physically correct Fresnel.
- shader_validation.rs mock prelude now tests the documented PipelineInput.position and Naga parsing/validation plus five distinct material functions.
- hero_vfx.rs glacial_haze now emits each full animated curved atmospheric sheet atomically. In crowded scenes, full sheet tessellation decreases from 10x8 to 6x5 rather than punching holes. A dedicated Rust test checks full groups in the M29 900-face scheduler.

## Truth / remaining

This is functional WASM guest **material code** and connected source geometry, not an integrated HDR GPU postpass. No new cinematic user video or native in-game screenshot evidence is claimed. The 200/200 renderer routes are still draft visuals; 170 DesignIntent mechanics remain server-rejected; 0/100 end-to-end gameplay accepted. No incomplete test ZIP. Actual frame-time and visual reference comparison require Horizon Nightly and GPU review.

## Verification

M29 baseline: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44754956222 — 37/37 tests, original WASM package. **M30 pinned Horizon WASM CI run 44758785121: SUCCESS — 38/38 preview Rust tests passed** (0 failed). Real pinned-sdk WASM .hmod compiled; 30 original OGG audio cues, validated WGSL/Naga shaders, ZIP manifest and SHA512 checks all passed. The added frozen haze high/low atomic-component test and actual M30 new material syntax test both passed. This tests the procedural material source language and module packaging, **NOT** actual Windows on-screen shader quality, physically correct HDR bloom or GPU FPS. Source commit: a3baa351ecfbcbfaa824b3ba6b40aac48b216137.

CI: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44758785121