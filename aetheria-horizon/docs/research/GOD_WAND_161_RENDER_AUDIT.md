# Aetheria M42 — God Wand 1.6.1 render architecture audit (not a port)

Date: 2026-11-21. **Primary evidence: user-supplied exact JAR** `godwand-neoforge-26.3-1.6.1.jar` (sha256 `91eaaaa6f57e9f420271528121115d475bc48ed8dea5fa28c86eae06fc961196`). Statical analysis only; the foreign mod was NOT executed. Supplemental evidence: public source `HyIsNoob/god-wand` (default branch README may lag behind the user's 1.6.1 archive). Target: original Aetheria mechanics and visuals in Horizon Nightly 2026-11-20 / MC 26.3, not Minecraft Java NeoForge.

## Exact artifacts in JAR
- 442 ZIP entries, 289 Java `.class`, 37 `.json`, 34 `.ogg`, 11 `.png`, six `shaders/program/*.fsh`, one `shaders/core/*.fsh`, one `.vsh`. Does NOT consist of hundreds of premade effect models.
- `META-INF/neoforge.mods.toml`: God Wand 1.6.1, NeoForge >=26.3.0.99, MC 26.3. Description ('six spells') is stale: language registry has skill IDs 0..9 plus Blue.
- Key class families:
  - world render: `AtomicRenderer`, `FugaRenderer`, `GateDraw/GateModel/GateCircles/GateBeams/GateFx`, `BlueRedDraw/BlueRedRenderer`, `SupernovaAccretionRenderer`, `StoneCannon*Renderer`, `ImpactDebrisRenderer`, `DrillWindRenderer`, `SpatialAuraRenderer`.
  - GPU compositing: `GodWandPostFxManager`, `AtomicPostFx`, `GatePostFx`, `BlueRedShader`, `net.godwand.client.postfx.GodWandPostChain`.
  - geometry/texture generation: `GateAtlas`, `BlueRedAtlas`, `AtomicRenderer$EnergyTexture`, `FugaRenderer$FlameTexture`.
  - timed camera/pose: `AtomicCinematic`, `FugaCinematic`, `SilentGateCinematic`, `GodWandCinematicEngine`, `AtomicPoseFrames`, `GatePose`.
  - gameplay/authority: `AtomicManager/Sequence`, `SupernovaManager/Sequence`, `SilentGateManager/Sequence`, `BlueRedManager`, `SpatialDisplacementManager`, `SkillCooldowns`, `SpellDamage`, many explicit network payloads.
  - audio: `FugaSoundscape`, `SilentGateSoundscape`, `SupernovaSoundscape`, `BlueRedSounds`, 34 .ogg stems.
  - Minecraft integration: `GameRendererMixin`, `PostPassUniformsAccessor`, `PostEffectProcessorAccessor`, 20+ mixins, NeoForge client event registrations. These hooks cannot be copied directly into Rust/Horizon.

## The actual visual secret — several coordinated rendering methods, not only polygon meshes
1. **Screen-depth reconstruction and screen-space world VFX**: `assets/minecraft/shaders/program/godwand_impact.fsh` (~848 lines) gets `DiffuseSampler`, `MainDepthSampler`, inverse projection/view, camera and spell center; reconstructs 3D pixel rays and scene hits, tests terrain occlusion, and performs black-hole lensing, rift/sky replacement, radial pulses, color distortion and glow. `godwand_impact.json` has two passes; `godwand_nova.json` has five (scene, emission, two blur axes, composite).
2. **Volumetric-looking procedural energy**: `godwand_atomic.fsh` (~833 lines), `godwand_atomic.json` **six passes** (half-res Fuga volume, scene, emission, blur X, blur Y, composite). 3D ray/analytic intersections and volume marching with noise, jitter, extinction and depth clipping, driven by uniform phase/tick values; NOT a lone 3D asset nor one alpha-cut quad.
3. **Artist-specific parametric world meshes**: `GateModel` plus `GateDraw` generate shaped gate panels and drawn light shafts, textured ribbons, gradient surfaces, crystals; `BlueRedDraw` has >100 methods for continuously updated plasma arms, swirling orbs, currents, pressure fronts and impacts. The CPU surfaces and GPU post effects reinforce one another.
4. **Procedural alpha-atlases**: `GateAtlas` generates **1024x1024** art in memory, with regions for runes, swirling linework, seals, filigree, feather, smoke, wisp, rays, pixel stone/panels; tinted by authored vertex colors. This is why there need not be a static model for each ornate effect. `BlueRedAtlas`, Atomic energy and Fuga flame generators similarly make textures. The JAR contains only 11 PNGs.
5. **A complete time/action system**: e.g. `AtomicSequence` has charge/compression/blackout/flash/beam/shock/afterglow timings; synchronized camera and arm pose, targeted terrain/scars and original layered audio. `SpatialDisplacementManager` records displaced terrain and restoration; separate server managers own damage, cooldowns, tasks, target selection and network sends.
6. **Performance protections**: cache/prewarm shaders (`AtomicPostFx.prewarm()`), downsample emission/volume, bounded effects and sound lifetimes, depth clipping, fallback rendering. Their effectiveness on RX580 is **not benchmarked** by inspecting a JAR alone.

## What M39-M41 Aetheria got wrong
- 12 signature SDFs and many 100-name source effects often use one camera-facing quad as the 'hero'. Even if mathematically shaded, this produces flat shapes. The M40 actual game screenshot FAIL confirmed skinny brown-green spokes. Adding facets and more seeds did not fix the absent *scene compositing*.
- World geometry and optional per-quad `pipeline_fragment(PipelineInput)` are not the same graphics stage as God Wand's scene-depth post effects.
- Existing code has 200 portable spell IDs, 100 authored visual *drafts*, and only a few tested models, with incomplete server gameplay. **0 of the 100 required fully playable polished spells** may be certified from this evidence.
- Stop using '100 rendered' or number of quads/tests as acceptance metrics for art beauty. Freeze the one-skill tweaking loop until common GPU compositor is proven.

## Verified pinned Horizon route (SDK commit 0a875da91dbb0fdcb7ba8c42c11341aa229af51a)
**A restricted .hmod pipeline is insufficient alone.** In `crates/horizon-client-render/src/mod_pipelines.rs`, `PipelineInput` only contains quad UV, linear color and camera-relative vertex position; it cannot sample scene-color/depth nor add arbitrary full-screen passes.

**Horizon .hshader API 0.2 DOES provide the missing stages**, in its `[[post]]` manifest:
- `fn post(input: PostInput)->vec4<f32>`, read `input.depth`, `scene_color`, `scene_depth`, `previous`, `depth_at`, `view_position(uv,depth)`, named scaled targets, frame camera/time, HDR and optional textures; it allows up to 12 post passes.
- A native Rust mod's `Hooks::shader_params` can feed `frame.mods` (eight vec4 per frame) **to its own bundled pack** named `native-<mod id>`. This bridge is in `crates/horizon-mod-vocab/src/hooks/shader_params.rs` and `crates/horizon-client/src/game/native_hooks`. Its suitability for an existing WASM-only Aetheria build must not be assumed.
- Only one shader pack can be selected at once, so a Photon integration/composition policy is required. Do NOT overwrite the user's Photon or silently disable their selected shader pack.
- The pack shader is WGSL, **not** God Wand's GLSL. Reimplement original math and visuals; do not transplant NeoForge bytecode/mixins.

## Concrete M42 engine deliverables, gates and budgets
1. Native/client VFX event bridge with a deterministic priority queue, network-authoritative time/position; output a bounded slot record per active large spell into `frame.mods`. Need separate proof that the live hook fires before promising integration.
2. **Shared dedicated full-screen WGSL compositor**, not hundreds of .blend models: low-res 3D analytic SDF/volume emission clipped by `scene_depth`, optional luminous world mesh, horizontal+vertical bloom and compositing into scene. Separate pass profiles for spell identities, with per-spell authored shape functions and timings.
3. Typed VFX grammar: each of 200 spell assets describes `charge -> release -> hit -> lingering`, physical geometry, noise/field type, camera pressure, light colors, original unique sound envelope, server action and player animation. This is shared infrastructure with genuinely different spell silhouettes, never recolor clones.
4. Distinguish the close-up structural needs: actual ice formations, gate doors, moving rubble, projectile weapons require original closed mesh/physics, not a fullscreen false volume; use hybrid composition as God Wand does.
5. Test a **cross-school** matrix, not another week on Thorn Dart: e.g. fire plume, solid ice cathedral, living roots, moving lightning, water vortex, sky rupture, gate, ground crash, soul fog and chrono fracture. Capture original Horizon RX580 Vulkan frames, compare FPS, measure render pass cost and visibility/occlusion from 3 camera angles.
6. Only count a spell 'done' when real visuals, audio, mana/cooldown, hit logic, unlock/equip/UI, multiplayer and recovery tests pass. The user's test gate remains 100/200 finished, no early ZIP.

## Licensing
The JAR's `LICENSE`: Java source and Java compatibility adapters MIT (retain copyright/permission notice if reused). **All custom shaders, textures, music/SFX and other resources in `assets/` are All Rights Reserved**, and God Wand branding/logo are not MIT. The uploaded JAR is study-only; this project must write its own original WGSL, atlas, sound and branding. No extracted assets copied into Aetheria, no re-upload of the God Wand compiled JAR.

Sources: user-uploaded SHA-256 above; https://github.com/HyIsNoob/god-wand (reference source version may lag); https://github.com/horizon-rust/horizon/blob/0a875da91dbb0fdcb7ba8c42c11341aa229af51a/docs/api/shaders-reference-0.2.md and pinned `mod_pipelines.rs` / `hooks/shader_params.rs`.
