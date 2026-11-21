# M36 — 100 named procedural spell VFX, source-verified art QA

**Date:** 2026-11-21. **Status: DEVELOPMENT / ART NOT APPROVED**. This is a continuation of the user's explicit request to produce **100 visually and graphically complete spells** using the newly verified procedural VFX approach, not Blender models.

## Current factual achievement

- **100 explicitly authored profiles** built from 10 different schools, the first 10 registry spells in each school. These are the **real named registry entries**, not placeholders, e.g. Ember Needle, Pyre Serpent, Rime Bridge, Twister Seed, Faultline, Rainweaver, Root Snare, Arc Net, Rift Slice, Moment Slip, Carrion Swarm.
- **48 different spatial movement/silhouette families** in `src/authored_100_fx.rs`: branching/sweeping arcs, interlocking plates, fractal cracks, erupting roots, swelling clouds, multiple rotating petals, flocking motes, plunging rain, time echoes, solid lance trajectories, cage ribs, flowing liquid paths and more. Each of the 100 rows has its **own animation parameters**, not a simple RGB recolor or cloned OBJ.
- **12 independent optional WGSL fragment-mask classes** in `assets/aetheria_preview/shaders/pipeline/soft_particle.wgsl`: flame, ice glint, droplet, spectral wisp, smoke, faceted ice shard, petal, jagged plasma, flowing filament, broken sigil, temporal fracture, bone wisp. All quads fade to transparent at the edges. This is **not actual HDR screen-space postprocessing**.
- The existing 200-slot registry and two true 3D Blender-bodied originals #021 / #022 remain intact. 100 authored profiles use `authored_100_fx`, other 100 M35 procedural emitter backgrounds; the old solid base effects are still rendered underneath. This is **not a claim that the old generic solid geometry was all replaced**.
- All graphics emitted through the verified *optional* `PipelineQuad` / `WorldDepth::Tested` API; no shaderless rectangular sprites on fallback. Finite coordinates, bounded 80-ish particles/spell and global 144-particle reservation, 900-quads total cap. FPS **not measured in game**.

## Exact source implementation

- `src/authored_100_data.rs`: 100 curated name-matched profiles and 48 unique motion pattern identifiers.
- `src/authored_100_fx.rs`: 48 procedural 3D trajectory generators, separate 12-category shader material tags, camera-facing quads, staged per-particle births/fades, limited-distance/tick scope.
- `src/lib.rs`: only slots 1–10, 21–30, ..., 181–190 dispatch to these profiles. Outside these 100, the working M35 emitter still runs. All retain the existing M32 body renderer pending genuine appearance QA.
- `src/shader_validation.rs`: pinned Naga shader parsing/validation of the real material file and required new mask families.
- `asset-pipeline/sharp-projectile-v2/rust-qa/src/lib.rs`: compiles the **exact source files**, checks 100 named profiles at five ages, finite/depth-facing quads, group limits, stable deterministic output, dynamic trajectories/48 patterns, and generates source-geometry previews.
- `.github/workflows/aetheria-core.yml`: standalone actual Rust geometry test and Rapier; uploads **10 SVG overview sheets** generated from Rust's real world-quad geometry (one for each school, two ages per spell, 100 spell panels total). Note these are **offscreen Rust source projections with an SVG soft-mask style approximation**, **not actual GPU screenshots** and **not a real Horizon client capture**.

## First completed test gate and genuine visual inspection

- [Core CI run 44906337416](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44906337416) **PASS**: 19 actual-source tests including M36 and 79 portable RPG core tests; Rapier physics job PASS.
- Downloaded artifact `Aetheria-M36-100-Spells-Source-Geometry-Review` (10 genuine source-derived school diagrams, artifact ID 14707559724). Inspected Pyromancy and Hydromancy thumbnails: recognizable **different spatial motions**, but at this stage **much too many small dots/simple strokes** to qualify as polished/beautiful cinematic spells. This is an explicit **art FAIL**, not hidden behind passing test counts.
- Improvement underway: individual WGSL material families expanded from 4 to 12 to avoid all effects looking like identical tinted sparks. New material visuals must pass pinned Naga + Horizon SDK and in-game screenshot before promoting any of them to quality approved.
- **No Horizon player screenshot or RX580 FPS for M36.** No collision, server freeze/damage, real shield reflection, saving, loot or authoritative multiplayer for all 100.

## Hard acceptance gates for 100 "fully done"

1. Test all 100 actual named spells in a working Horizon Nightly client with user-level screenshots/videos, including camera angles and overlapping casts. Source/GPU previews alone do not suffice.
2. Visually approve 100 distinct silhouettes, phase transitions, not just names/palettes/particles or 48 formulas. Reject wire rods, static hoops, generic cards and washed-out overbright blooms.
3. Verify sound, mana, authority, gameplay effects, collisions/world interactions, multiplayer ownership and server cleanup where each spell promises them.
4. Test framerate/latency on user's target RX580/Ryzen 1600 at reduced and full LOD and document the actual results.
5. Do not merge to stable, publish, or send the user an incomplete `.hmod` as **finished 100**. Draft CI artifact is internal.

## User direction and constraints

Read `docs/M35_PROCEDURAL_VFX_DIRECTION.md`, `docs/memory/AI_DEV_LOG.md`, `docs/VALIDATION.md`, prior messages and source videos before next iterations. The user explicitly asked not to waste the remaining remote PC budget and not to produce Blender copies or fake screenshots. All M36 work is done via **GitHub and GitHub Actions**, not Remote Desktop Commander.
