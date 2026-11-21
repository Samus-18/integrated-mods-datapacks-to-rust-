# Aetheria M43: First 20 original post-processing spell profiles

This is a **prototype HDR-capable Horizon API 0.2 .hshader** with four passes
(half-res depth-clipped volumetric emission, blur H, blur V, sharp scene composite).
It is not a finished 20-spell game mod and does not modify your existing Horizon
shaderpack or replace Photon.

- Disabled by default. To preview one profile, choose `aetheria_demo` 1–20;
  to inspect all automatically, turn on `aetheria_cycle`. These demo controls
  are for INTERNAL visual QA, not real gameplay spell casting.
- Actual gameplay connection requires a **native** Horizon mod implementing
  `Hooks::shader_params` to supply eight `vec4` rows via `frame.mods`.
  This shaderpack does **not** have that bridge yet; the existing Aetheria
  guest `.hmod` cannot magically register full-screen post passes.
- Actual named 200-spell IDs are mapped to 20 profile IDs in the separately
  tested pure-Rust adapter `post_profiles20.rs`, not copied from God Wand.
- `aetheria_quality = 0` is the RX580-oriented lower-cost starting point.
  No measured performance claims until real Vulkan screenshots and timings.
- A second selected shaderpack (e.g. Photon) cannot be active simultaneously:
  test using an isolated game directory, never uninstall/overwrite Photon.
- Art/sound/gameplay/impact/LOD and multi-angle QA still pending. Do NOT
  count these twenty as finished skills or release a user install ZIP.

Every shader and formula here is original Aetheria source; God Wand's
All-Rights-Reserved assets are not included.
