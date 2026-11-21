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
