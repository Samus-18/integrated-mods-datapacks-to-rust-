# M40 — replacement of failed billboard with genuine world-space botanical geometry
2026-11-21 / Aetheria 0.8.8 (INTERNAL QA, not final).

**Observed reason for change:** M39 REAL Horizon/Radeon RX 580 screenshots
show Thorn Dart as a narrow but still flat neon-green arrow.
M39 pixel SDF was rendered on a single camera-facing quad.
It had neither real thickness nor convincing impact/phase differentiation.

**Implemented in source:** `preview-5spells-wasm/src/thorn_living.rs`
- Named living seed -> travelling six-/seven-sided woody shaft, pointed
  green multi-face spearhead, staggered reverse barbs and leaf fins.
- Uses actual Horizon world-space quads on closed 3D component surfaces,
  not image generation, Blender mockups, 2D SDF hero, sticks/rings.
- Impact changes the whole shape into a three-dimensional spreading
  root/leaf splinter rosette, with a separate fading after-effect.
- Actual cast direction and evolving age drive world position and form.
- Low-count bounded glow spores use optional soft-particle WGSL; no
  diffuse vertical generic fog, no unmasked billboard fallback.
- M40 deliberately BYPASSES id-100 signature_sdf 2D hero even when
  the shader loads. Other 11 signature materials stay unchanged.
- Component quads respect geometry budget; regression tests cover
  stage geometry, travel, afterglow and budget.

**Evidence and non-claims:** This is a source implementation, not yet
approved game art. Compile/CI result pending. ACTUAL RX580 Vulkan screenshots
and multi-angle QA must be reviewed before art acceptance. No new native GPU
instancing, HDR scene texture, server hit effects or 100 completed spells.
Other 199 spell routes unchanged; user M32 installation untouched.

## Actual Horizon game result: M40 0.8.8 FAIL — 2026-11-21
GitHub pinned CI 44924606657 PASS, package SHA256:
e519c2b505ffd0c5222197abe01418ec484028bd5c5f5b5a54949c545159e642
Actual isolated Horizon Nightly 2026-11-20, MC 26.3, Vulkan RX580.
Observed FPS after cast ~67.7, no game/world errors.
Photos M40_03 (flight), M40_04 (impact), M40_05 (afterglow) under
docs/qa/actual-horizon/, along with QA_M40_real_game_report.json.
Art verdict FAIL: sparse nine-spoke brown/green star, not a living blossom.
Root cause: blade width was PARALLEL to radial growth direction,
collapsing full leaves into tiny triangles/toothpicks. M41 fixes local
tangent, fills core with faceted seed and overlapping broad petals.
M41 still requires new actual game screenshot; no approval claimed.
