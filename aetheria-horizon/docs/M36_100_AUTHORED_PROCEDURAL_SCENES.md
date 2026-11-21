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


## M36.2 review (2026-11-21) — four phases, 100 previews, verified optional glow

**Source and tests after the first M36 art FAIL:**
- Per-profile VFX now has four authored lifecycle phases: condensation, animated growth/travel, impact/sustain, dissipating afterglow. Structural shapes (bridges/cages/shields/pools) hold instead of exploding; projectile/impact shapes expand. The 100 profiles use 48 trajectory families and 12 soft WGSL silhouettes, plus larger but bounded source-geometry hero cores. Existing 198 draft solid art is **still visible** underneath as it was before; more work is necessary.
- Standalone GitHub core [run 44907311604](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907311604): **20/20** actual-source renderer tests and **79/79** portable RPG tests PASSED; Rapier physics passed. After new hero cores, [run 44907516076](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516076) **core-tests and physics PASSED**.
- Downloaded and inspected **latest v4 actual Rust-source material approximation**: ten school panels, 100 identified spell names, two phases each. This is not GPU shader output, Horizon frames, an AI image or a beauty acceptance. Despite larger hero quads and 12 material families, **v4 is still too visually basic** — flat orange swooshes, simple sparkling clusters and uniform strokes do not match God Wand/Photon-like cinematic spells. Our count of actually in-game-vetted polished spells remains **0/100**. The latest full source collage was created from the generated v4 PNGs locally without using the user's PC.
- A separate **optional** official Horizon shaderpack API **0.2** has been built in `shaderpacks/aetheria_spell_glow/`: 8-tap restrained chroma-aware global bloom poststage. [Pinned official `horizon-shader check` + `pack` run 44907197852](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907197852) **SUCCESS**, generated `.hshader` artifact. **This is NOT attached to or automatically enabled by .hmod.** Horizon can run only one own-format shaderpack at a time and installing it could replace an existing look. No user PC installation, no FPS data.
- Latest full **M36 `.hmod` pinned WASM** build [run 44907516110](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516110) was **still in progress** when checked; do not label that exact revision compiled until success reported.

**Future acceptance gates:** actual native Horizon 26.3 captures of all 100 cast phases, performance on RX580, gameplay-authoritative damage/status/collision/save, richer layered VFX hero silhouettes and true GPU shader materials, synchronization of 100 distinct sound/impact profiles. This source-geometry milestone must not be published as a visually completed 100-spell mod. **No Remote Desktop Commander connection used.**

### M36.2 Pinned Horizon QA — final verified 2026-11-21 result

Full official pinned SDK [Actions run 44907516110](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516110) **COMPLETED SUCCESS**. **58/58** actual preview Rust tests (including all 200 cast ID and 100 authored source scenarios) and no failures, verified original 10-body Rime mesh and Frost Needle outward normals, original 30 OGG sound stems, separate 12-class soft particle WGSL/Naga validation, correctly compiled WASM `aetheria_preview.hmod`, package SHA512 and ZIP integrity. Internal artifact ID `14708039590`; built `.hmod` SHA256 `7a592a197704f32f361f317b3b7ac102f03441218ef3066c99632d1312d2112f`. Also emitted source visual-audit artifact ID `14709215259` (offscreen geometry, **not** in-game capture). A separate official shaderpack run `44907197852` validated the OPTIONAL `aetheria_spell_glow` with `horizon-shader check` and pack, which remains unrelated to automatic guest module load.

**The above is a compiled development preview, not 100 visually/gameplay completed spells.** Actual in-game mod runtime and shader appearance, RX580 benchmark, server mechanics and users' reference quality remain unverified. No user's PC connector used. PR stays DRAFT. Do not distribute or merge as a finished 100-spell product.
