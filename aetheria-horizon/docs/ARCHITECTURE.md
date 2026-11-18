# Implementation architecture

Pure-Rust core: `src/core` (player data, save format, engine bridge), `src/magic` (registry and casting), `src/vfx` (procedural mesh, GPU budgets), `src/ui` (spell wheel), `src/sfx` (audio cue profiles). `assets/shaders` holds original standalone WGSL experiments.

## Technical compatibility
- Official first native mod tutorial: https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/start/first-mod.md
- Official native package docs: https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/reference/manifest.md
- Native mod source mixin template: https://github.com/horizon-rust/horizon-native-mod-template (different build route)
- Orbital Laser: https://github.com/horizon-rust/orbital-laser (fork-specific SDK; VFX reference ONLY)
- Shader API: https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md

## Nonnegotiable engineering constraints
- No Rust GC does not imply zero stalls; GPU, allocations, shaders and world mutation still cost frame time.
- No assumption of unrestricted `wgpu::Device` handles in installed mods. Check published interfaces before binding.
- Bloom requires HDR textures + suitable post-processing; setting emissive values alone does not guarantee glow.
- Do not slow multiplayer server ticks for client-only ultimate cutscenes. Use client camera effects.
- Mutation budget and rollback needed before enabling terrain destruction; no persistent damage on default settings.
- This M0 milestone is core/source scaffolding, not an installable or tested `.hmod`.
