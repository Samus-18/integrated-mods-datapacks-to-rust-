# M33 — Aetheria Horizon: first Blender-authored spell physically connected to live VFX

Date: 2026-11-20. Active branch: `aetheria-30-spell-test`.

## Why this is a real implementation rather than another test-counter milestone

Source research confirmed the **real** 2026-11-20 Horizon SDK supports guest `Hooks::submit_custom_geometry`, `WorldQuad` and an optional fragment WGSL pipeline; it does not give a WASM guest an unrestricted native depth/color fullscreen post. Current Horizon shader packs support an API 0.2 post path; cross-runtime spell state to it needs separately reviewed native integration. Third-party Java/Fabric/NeoForge spell mods show useful staged VFX, particle trajectories and casting UX, but their assets and APIs are not directly reusable for Horizon Rust. We are not copying their code/meshes.

The user's 2026-11-20 real Horizon video rejected the old generic thin rods, empty rings and overexposed cards. The original Frost Needle Blender v1 preview still had a very long narrow tip and empty ring. It is preserved intact as an archived experiment, **not used in the new live spell**.

### New v2 original artist asset

- `asset-pipeline/sharp-projectile-v2/frost_needle_v2.py`: independently created **three closed 3D solids** (one wide irregular ten-sided ice glaive/needle blade, two broad closed shoulder fins) with authored icy material boundaries and UV0. NOT crossed planes, Minecraft placed blocks, generic spheres, empty rune rings or Photoshop images.
- `asset-pipeline/sharp-projectile-v2/export/frost_needle_v2.blend`, `.glb`, `.obj`, material definitions, **two actual Blender Evee camera renders** and their source geometry QA. Readable source is preserved; v1 assets remain unchanged.
- `asset-pipeline/sharp-projectile/export_mesh_to_rust.py`: reproducible conversion of the actual OBJ into a constant Rust face/vertex list. It validates watertight incidence (two faces/edge), material identity and finite coordinates; every even-sided cap becomes joined quads instead of an omitted or degenerate top face.
- Resulting embedded author geometry: **130 vertices / 124 closed quads**, with materials 64 dark solid ice, 40 vivid glass and 20 mint-lit facets. Rust source carries the SHA-256 hash of the original OBJ so it can be audited. No GLB asset-host or unsupported GPU mesh API invented.

### Actual guest code path

- `preview-5spells-wasm/src/sharp_projectile.rs` renders those source vertices using verified `Scene::quad` and Horizon world-space quads, oriented in the real camera/cast direction. It is NOT a dummy GLB path or stand-alone simulator.
- `preview-5spells-wasm/src/lib.rs` now sends **spell ID 021, Cryomancer Frost Needle** into that module FIRST. It no longer calls the 18-family generic school body for this one ID. The remaining 199 slots retain their old draft renderer, not silently claimed done.
- Four visible phases: solid compact charge (0–9 ticks), forward-flying and rolling 3D projectile with full smaller echo shells (10–30), recognizable shattering into whole solid original ice fragments (31–59), short quiet remnant decay (60–103).
- Each complete source mesh is an **atomic** 124-quad scheduler component: fair global 900 world-quad budget never snips a face out of the closed model. Lower-detail mode omits complete echo/shard bodies instead of drawing holes. Depth-tested fragment WGSL from the existing pipeline remains active; color/rim contrast is restrained to avoid blown-out white.
- Native HDR, true GPU Fresnel/scene depth post and world lighting are **not claimed**. Material lighting is a color/shading approximation and this is still a client-only experimental visual spell. No freeze/damage/server effect, multiplayer sync or persisted spell ownership yet.

### Actual validation, not art approval

- Blender 5.2 export succeeded; viewed two *real Blender rendered* angles of v2. The initial v2 was rejected because it looked like a potato with rectangular fins; reshaped into a longer, wider tapered crystal with pointed asymmetric shoulders and re-rendered. Still a working art pass, not yet user-approved cinematic reference quality.
- `python asset-pipeline/sharp-projectile/export_mesh_to_rust.py`: watertight OBJ/quad conversion, generated Rust geometry, SHA identity — PASS.
- `python asset-pipeline/sharp-projectile-v2/validate_source.py`: 3 GLB meshes and 3 authored material IDs, binary GLB integrity, two Blender previews, Rust source hash and **live dispatch order** — PASS.
- Rust stage tests for all 4 phases, geometric extent/finite values and atomic lower-LOD scenes added. Pinned Horizon SDK WASM and Rust tests are run in GitHub Actions; record actual run result below after checking it.
- **Real Horizon Nightly 2026-11-20 game test NOT run for this new asset**, no screenshots or Windows Radeon RX 580 FPS. Do not download/test half-finished builds.

### Research (independent, no source-copy)

- Horizon official modding: https://github.com/horizon-rust/horizon/blob/main/docs/mods.md
- Official shader API 0.2 and post stages: https://github.com/horizon-rust/horizon/blob/main/docs/api/shaders.md
- SpellEngine (animation/shape presets): https://github.com/ZsoltMolnarrr/SpellEngine
- Iron's Spells RPG spellbook/mana/UI: https://iron.wiki/
- Magic Particles Lib (shaped surface particles): https://www.curseforge.com/minecraft/mc-mods/magic-particles-lib
- GeckoLib (keyframed animation): https://modrinth.com/mod/geckolib
- Open-source shader examples (API 0.2): https://github.com/hildlabs/horizon-shaders

### Next engineering sequence (before user testing)

1. Verify pinned SDK M33 CI and resolve any actual compiler failures.
2. Improve observable world-space material gradients/wing breakup and author a shader technique **supported by actual guest/custom pipeline**; investigate native postpass only with exact verified API.
3. Replace next individual **non-identical** spells with source-authored geometry, 3D choreography and sound; stop treating 200 draft paths as acceptable art.
4. Integrate server-authoritative magic actions, acquisition and saves; no preview-only spell claims.
5. Gate in-game visual beauty and RX 580 FPS on recorded actual Horizon gameplay once a meaningful feature milestone exists.

**Current acceptance gate still 0/100 fully integrated-and-game-verified spells.**

### Critical independent normal-winding repair
A fresh OBJ signed-volume and outward-face-normal inspection exposed an actual error the original Blender two-camera renders could not show: **all 108 faces across all 3 v2 solids faced INWARD**. This can make models disappear under Horizon's back-face culling, as previously observed in Roblox. Fixed the original Blender generator by reversing face vertex order before export and rebuilt GLB/OBJ/Blender source/renders and generated Rust faces from the corrected source. New `validate_normals.py` tests all three components have strictly POSITIVE signed volume and that all face normals point outward. Observed core volume +6.24761, two fins +0.65005 each; 108/108 outward faces. Added this as a required CI gate. Shader screens and GPU in-game quality remain unverified.
