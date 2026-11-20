# Aetheria Horizon — Fantasy Magic RPG for Minecraft 26.3 (Horizon Rust)

Fantasy-only, original Minecraft RPG magic. **Goal: 10 distinct classes × 20 spells = 200 complete spells**, not 200 color-swapped effects. Each class plans 14 standard, 4 awakenings and 2 cinematic ultimates. **No sci-fi or orbital weapons**; Orbital Laser is a rendering-code reference only. Development happens via this chat and GitHub, no Codex or paid asset generation.

## Status — 2026-11-19

| Area | Verified state |
| --- | --- |
| Master spell registry | **200 designed**. Individual meshes, mechanics, SFX, API calls are not all complete |
| Portable Rust RPG gameplay | **20 distinct spell intents**, 39 core tests passed at M3; not yet synced to Horizon player state |
| First in-game preview v0.2.0 | **5 fantasy visual effects** loaded on the user's Windows/Horizon and visually demonstrated in the provided 100-second video |
| Visual polish v0.3.0 | **15 3D visual silhouettes across those same 5 spells** (3 per spell key), reduced additive white glare, fairly bounded geometry. **Not yet user-tested** |
| Actual RPG casting | Mana, cooldown, world impact, multiplayer, full spellbooks/HUD/SFX still need integration into the playable build |
| Final 200-spell release | **Not completed.** Do not call this a 200-spell playable mod |

The game-visible prototype is compiled from Rust to **WASM** and runs in Horizon's sandbox/render APIs. The native Rust route is also kept, but the user's launcher had a native-cache lock problem; the wasm preview can be loaded **without Build**.

## Repository guide

- [Master project specification](docs/PROJECT_SPECIFICATION.md) — goals, 10 classes, requirements, performance, audio and phased acceptance
- [USER_LOG](docs/memory/USER_LOG.md) — exact user quotations, decisions and corrections
- [AI_DEV_LOG](docs/memory/AI_DEV_LOG.md) — milestones, code/CI checks, limitations
- [SPELL_REGISTRY](docs/memory/SPELL_REGISTRY.md) — all 200 spell design entries and accurate implementation statuses
- [PLAYTEST_LOG](docs/memory/PLAYTEST_LOG.md) — time-coded evidence from actual in-game footage, improvement backlog
- [Original reference videos analysis](docs/VIDEO_REFERENCE_ANALYSIS.md)
- [Source: 5-spell precompiled WASM preview](preview-5spells-wasm/) — Z/X/C/V/B; 0.3.0 contains three distinct versions of each effect
- [Source: native preview](preview-5spells/) — compiled by compatible native launcher SDK, not used for the successful local WASM gameplay test

## Install the 5-spell precompiled WASM preview

Only use a **CI-verified** `aetheria_preview.hmod` containing `mod.wasm` (not a native source ZIP).
1. Remove older Aetheria copies from both `horizon-native-mods/` and `horizon-mods/`.
2. Put this compiled **WASM** `.hmod` in `horizon-mods/`, enable `aetheria_preview`.
3. Start the game with **Play**, not **Build**. In a world press Z / X / C / V / B. Repeat a key to cycle geometric variations in v0.3.0.

**No** in-game confirmation yet for the *new* 0.3.0 variant shapes; CI compilation is not a rendering-quality test.

## Validation

- Portable core M3: [39/39 tests passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44482049153).
- WASM preview 0.2.0: [Rust/WASM compile, .hmod SHA-512 and archive checks passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44492554390); user subsequently confirmed it runs visually in game.
- WASM preview 0.3.0: [successful pinned SDK build and SHA validation](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44496889719), with 15 procedural forms (3 per key) and visual HUD labels. **In-game rendering of 0.3.0 still awaits user verification.**

License / third-party asset redistribution: TBD. No third-party TikTok media or AI-generated paid meshes included.

## M100 half-project milestone (user asked for next test only at 100/200 complete spells)

The user will **not** do more manual playtests of five-spell demos. The next user-requested game test is at 100 complete spells including **real 3D solid spell geometry, cast/damage/progression, a working GUI/acquisition route, animations and original spatial SFX**. Automated Rust/CI compilation is still run regularly.

- [Exact 100-spell delivery board and quality criteria](docs/M100_IMPLEMENTATION_BOARD.md)
- [Spell acquisition and fluid GUI design / core implementation status](docs/ACQUISITION_AND_GUI.md)
- [Original three-layer procedural sound pipeline](docs/SFX_PIPELINE.md)
- [User screenshot reference and verbatim instruction](docs/memory/USER_LOG.md)

**Current actual progress: 0/100 end-to-end integrated spells; 20 standalone Rust action intents, 5 client-only visual demonstrations, and 15 visual shape variations are not counted as finished RPG spells.** New work: 3D crystalline surface geometry in preview, transactional unlock core across all 200 IDs, 10-class constellation view model, and a generated original SFX pipeline. Do not publish another user-facing test ZIP before this milestone.

## M8 volumetric forms and cinematic choreography (internal work, not a release)

The portable core has **ten custom 3D sculpt foundations**, each with a distinct silhouette and animated construction, and **20 distinct future cinematic camera sequences** with reduced-motion settings. Current portable CI is **[53 tests passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44509592898)**. The precompiled WASM preview's second style for each original five spell demos has been connected to the shared volumetric meshes, pending [internal WASM CI](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44510001331).

**No additional user playtest requested.** The ten 3D school scenes and 20 camera trajectories are **not** complete spells or cinematics; M100 remains 0/100 fully integrated/verified. That honesty is deliberate until actual game mechanics, animations, SFX, GUI and persistence work end to end.

## M9 combat and renderer integration (under development)

The internal five-spell preview now imports the **shared 3D Rust sculpt library** and converts closed triangular volumes into the Horizon world's supported quads with face shading, culling and draw limits. Portable class-learning → book equip → cast mana/cooldown validation is consolidated into \`src/core/authority.rs\` for a future authenticated server adapter. [58 Rust core tests passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44510503132).

This is **not a public test release**. World damage/healing, acquired items, actual GUI interaction, camera cutscenes, model animations and multiplayer persistency still need real Horizon integration. The upcoming 100-spell manual playtest gate is unchanged.

## M10 complete player persistence and fluid GUI data

- Full versioned v2 portable save codec: [M10 state/UI details](docs/M10_STATE_AND_UI.md), with class, mana, pages, resources, trials, book equipment/tier and cooldowns restored safely.
- UI model computes fluid mana ripple, 4–8-slot radial spell wheel, cooldown wedges and ten class color palettes.
- Arcane Inscription tiers I–V now increment actual default spellbook capacity from 4 to 8 slots, not just its title.
- This is **portable backend logic and GUI layout data** only; Horizon server-world saves, rendered GUI and full gameplay still need engine integration. It is not a playable 100-spell milestone. No manual test build until M100.

## M11 compact Iron-style picker + real Rapier3D simulation (internal)

[Detailed M11 source/limits](docs/M11_HUD_RAPIER.md). The game-UI source now implements a tiny mana bar and a selected-spell tile, **hold R + mouse wheel** selection (N/M fallback), G quick cast, and a **K**-toggled moving Constellation with 20 original pixel glyphs. Never covers normal gameplay with a permanent 5–8-slot wheel. Vanilla health/armor/food/air remain visible. A *full custom skin* of the health layer awaits complete native integration. Preview MP is client-local and not gameplay-synced.

An independent open-source [Rapier3D](https://github.com/dimforge/rapier) adapter creates real 3D colliders and simulated falling/ricocheting secondary spell debris, with 10 distinct school recipes and finite body budgets. **Rapier does not create nice shaders.** The latest verified tests were [72 portable core + 7 physics tests, all green](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44530777587). The in-game renderer, native health skin, world collisions, casting and complete multiplayer implementation are not finished. User-visible release remains gated on 100 integrated spells.


## M12 — user-requested 30-spell test branch (2026-11-19)

The user's latest request supersedes the earlier requirement to delay all previews until 100 spells: prepare a **30-spell test version with working casting controls, GUI and effects**. Development is isolated to `aetheria-30-spell-test` until the new WASM artifact is compiled and reviewed. In `preview-5spells-wasm/` (kept as a historical folder name), version 0.5.0 now offers the first 3 spells of all ten schools with real shared-core mana/cooldown checks, direct keys, a compact R+scroll selector, a 10-school animated constellation UI and 30 original SFX events. Each school has its own closed 3D sculpt, with 3 distinct spatial spell motifs layered on top. The portable core's gameplay action intent catalog grew from 20 to 30 (ten new actions).

**Qualification:** 30 selectable and locally simulated casts ≠ 30 end-to-end multiplayer/world-impact spells. The source still lacks a Horizon server adapter for damage, healing, real blocks/mobs, authoritative progression, and save synchronization. No playable release should claim these systems are done. The [30-spell WASM preview README](preview-5spells-wasm/README.md) lists exact controls and known limits. User's manual playtest is appropriate only after pinned SDK CI produces a verified artifact; do not pass off source ZIP as executable.


## M15 — recurring *visual* QA for 30 3D spells

The user requested that every VFX iteration be checked for appearance, not merely compile. The internal test now renders the same CPU world-space geometry as the Horizon WASM renderer into 180 diagnostic SVG views (30 identities × 3 spell phases × 2 cameras) and archives them in each [visual-audit GitHub Actions run](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44570867156). After reviewing these actual projections, eight weak silhouettes were rebuilt with larger physical bodies; [read exact before/after measurements and caveats](docs/M15_VISUAL_AUDIT.md). All 6 visual test suite checks pass and the pinned SDK `.hmod` compiles, but actual GPU glow, world shading and the user's in-game Windows rendering **have not been verified**. This work is kept on the `aetheria-30-spell-test` branch as an internal milestone; it is **not** a finished gameplay release, and no early manual playtest is requested.


## M17–M18 — original cinematic visual research / GPU shader path (internal)

The exact user's four original reference MP4s were reopened and independently frame-sampled, then current God Wand, Calamity Spells, Void Manifestations, Arcane Zenith, AdventurePack and the **actual Stink-O/orbital-laser MIT source code** were studied for VFX technology and art direction. See [M17 reference study](docs/M17_VIDEO_RESEARCH_AND_MATERIAL_PASS.md) and [M18 GPU postpass route](docs/M18_NATIVE_GPU_POST_RESEARCH.md). The open-source Horizon reference achieves massive look through depth-aware GPU full-screen post shaders, not simply world quads.

The test branch now has an independent world-space 3D ice cathedral for Shard Chorus and a Naga-tested animated per-face material. There is also **experimental non-integrated** native-gpu-vfx/aurora_volume.wgsl implementing colored volumetric raymarch integration with bounded samples and no engine bindings. It is source-only, **not shipped in the WASM .hmod**. Don't describe it as working HDR/bloom or complete player-facing VFX before the exact pinned Horizon native render hook and actual in-game GPU output can be tested. The 200 unique spell goal still stands; only 30 have portable intents and client-local casting visuals.


## M27 (internal): all 200 spells now have explicit client visual scene dispatch

The complete registry is **30 curated + 110 Standard + 40 Awakening + 20 Ultimate**. After [M27](docs/M27_COMPLETE_200_ROUTING_AND_VISUAL_QA.md), the 30 formerly template-backed Standard IDs have their own bespoke scene code (`standard_m27a.rs` / `standard_m27b.rs`), and a Rust test ensures every ID uses an explicit route. The last [pinned SDK test run](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762) passed **28/28** tests, including 120 angle/stage assertions for the new 30, and archived 860 actual-geometry diagnostic views. **These are draft CPU-local visual compositions, NOT 200 finished cinematic spell effects or a playable 200-magic game mod.** True native GPU bloom/refraction/scene depth, authoritative world effects, save/unlocks/multiplayer and verified player gameplay in Horizon Nightly remain outstanding. Do not release this unfinished internal .hmod as a polished user build.


## M28 — verified geometry fairness and Bone Palisade budget (internal)

[Detailed audit, exact changes, limitations](docs/M28_RENDER_FAIRNESS_AND_AUDIT.md). The actual Horizon WASM VFX render path now builds each individual source scene before fairly distributing the available 900 submitted world quads, recycling unused quotas and sampling from the full shape, not just its first pieces. Solo casts preserve the older 900-quad capacity. Bone Palisade (#194) received less costly tapered 3D rib tubes connected to thick ossified wall plates.

[**Horizon WASM CI 44632013016: SUCCESS, 31/31 tests**](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013016), including pinned SDK WASM .hmod compilation, shader Naga, 30 OGG events, archive SHA512 and 860 CPU projections. **No Windows in-game fidelity, actual world gameplay or multiplayer claimed.** Under heavy concurrent VFX load, sparsely sampling independent quad faces can expose holes: coherent mesh-component LOD is still required. This work remains on the development branch, and the user-requested test/release gate is not met.


## M29 — coherent concurrent 3D spell geometry (internal, 2026-11-20)

[Full M29 implementation and source-image review](docs/M29_COHERENT_SCULPTURE_LOD.md). The original M28 sparse-quad scheduler could punch holes through closed 3D sculptures under crowded spellcasting. The actual Rust Horizon WASM preview now tags complete solid components, selects them atomically under its existing 900-world-quad global cap, fairly redistributes spare headroom and generates lower-density **but complete** orb/veil/time/portal surfaces. Grouped sources include crystals, wings, membranes, boxes, ribs and terrain shells; a further visual pass replaced the fence-like Bone Palisade (#194) with overlapping broad bone armor panels.

[Verified first M29 pinned WASM CI: 36/36 preview tests, artifact and OGG/WGSL/SHA512 checks](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44752358562). A later Bone Palisade art revision needs its own CI and updated two-view image review before aesthetic approval. The reviewed first M29 **real-source** eight-spell SVG projection had **897/900** submitted quads and all atomic panels intact, but still looked overlapped and visually rough; CPU metrics are not an artistic pass or game screenshot. Authentic HDR/depth/native GPU rendering, 170 rejected `DesignIntent` world mechanics, real network/save/unlocks and 200 unique original SFX remain unimplemented. **0/100 complete Horizon-gameplay-verified spells; no user test/release ZIP.**


**M29 final review:** [Pinned Horizon WASM run 44754956222 succeeded
with 37/37 preview Rust tests](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44754956222),
30 original OGG cues, WGSL/Naga and signed manifest/ZIP integrity. The
follow-up Bone Palisade art revision removes the ugly covering rectangle
and adds 390-source-face, double-sided, articulated osseous shields.
Its actual Cast/Impact source SVG projections were compared in **both
camera views**; they show an improved jagged sculpt, but are still
first-pass faceted CPU visuals rather than approved cinematic Horizon
GPU imagery. **No world/server mechanics or release gate improved.**


## M30 — real animated magic shader materials (internal; 2026-11-20)

The actual Horizon WASM source preview now has **distinct dynamic animated surface materials** for flame, ice, living roots, lightning and astral fissures, calculated with original WGSL in its *real* world-quad render pipeline. A geometric normal from GPU derivatives emphasizes angled full-volume crystal facets. Ice spell Shard Chorus keeps all three curved atmosphere sheets whole at both high and crowded LOD rather than removing random panels.

[**Pinned WASM CI 44758785121 — PASS: 38/38 preview tests, pinned .hmod, Naga WGSL, 30 OGG, ZIP/SHA512**](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44758785121) · [M30 implementation/limits](docs/M30_LIVE_SHADER_MATERIALS.md). This does **not** mean actual HDR volumetric light or that the visuals match reference videos yet: the separate native postprocess/depth interface has not been wired, and no Windows GPU screenshot/FPS review was performed. Rust+WGSL development only; no C++/Iris/OptiFine. **0/100 real in-world accepted RPG spells; no incomplete download.**
