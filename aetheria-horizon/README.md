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
