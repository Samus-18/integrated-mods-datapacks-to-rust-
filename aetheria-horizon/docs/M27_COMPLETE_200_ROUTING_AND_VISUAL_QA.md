# M27 — Last 30 Standard spells: 200/200 explicit visual dispatch coverage

**Date:** 2026-11-19. **Status:** internal technical milestone (NOT a polished mod release).

## Corrected inventory

The earlier M26 progress note accidentally stated **150 individually scripted**. Reading the actual `preview-5spells-wasm/src/lib.rs` dispatch, canonical catalog and source files shows the correct previous count was **170**. M27 adds the final 30 dedicated standard designs. The actual dispatch now covers:

| Category | Local rank per school | Count | Renderer |
|---|---|---:|---|
| Curated | 1–3 | 30 | `cinematic_beams.rs` / dedicated `hero_vfx.rs` |
| Standard | 4–5 | 20 | `standard_forms.rs`, `charged_standard_forms.rs` |
| Standard | 6–7 | 20 | `standard_channelled.rs`, `standard_instant.rs` |
| Standard | 8–9 | 20 | `standard_combo.rs`, `standard_charged2.rs` |
| Standard | 10–11 | 20 | `standard_m26.rs` |
| Standard | 12–14 | 30 | **NEW:** `standard_m27a.rs`, `standard_m27b.rs` |
| Awakening | 15–18 | 40 | `awakening_forms.rs` |
| Ultimate | 19–20 | 20 | `ultimate_forms.rs` |
| **Total** | **All 20 × 10** | **200** | **All IDs use an individually selected renderer path** |

This means **zero Standard IDs fall back to the generic semantic `mass_forms::render_expansion` path** (confirmed by `explicit_coverage.rs`), NOT that 200 finished cinematic-quality spells exist. Some individual paths still use shared geometry-building primitives; they are deliberately composed per name but the material/lighting quality remains primitive.

## Thirty newly scripted spell compositions (source-grounded IDs)

- **Pyromancer:** 012 Basalt Burst (volcanic 3D fan), 013 Heat Mirage (tall distorted curtains), 014 Infernal Anvil (stepped forged anvil with molten embossing).
- **Cryomancer:** 032 Icicle Rain (hanging glacier chandelier), 033 Crystal Fault (zigzag raised ridge), 034 Winter Breath (wide veiled frost cone).
- **Aeromancer:** 052 Skyward Hook (physical thick curved wind ribbon and hook), 053 Soundbreak (three-layer 3D pressure wedges), 054 Windwall Gallery (sculpted spatial arches).
- **Geomancer:** 072 Quartz Garden (9 branching quartz clusters), 073 Mountain Shoulder (massive staggered cliff blocks), 074 Sandglass Trap (deep covered swirling ground funnel).
- **Hydromancer:** 092 River Serpent (segmented fins and watery body), 093 Tidal Mirror (three curved reflecting planes), 094 Undertow Hand (five articulated prismatic water fingers and deep-water swell).
- **Druid:** 112 Pollen Cyclone (floral pollen cloud with broad moving petals), 113 Carnivore Bloom (large twelve-petal two-sided jaw flower), 114 Spore Burst (eight varied 3D seed sacs).
- **Electromancer:** 132 Skyfire Crown (9-part cloud and branching lightning with broad membranes), 133 Pulse Cage (closed full-height hexagonal force-cell walls, NOT stacked empty rings), 134 Storm Feathers (17 broad 3D energy wings).
- **Astral:** 152 Starlight Gate (solid arched threshold with continuous magical screen), 153 Void Spiral (four wide volumetric spiral ribbons), 154 Falling Constellation (11 comet shards and tails).
- **Chronomancer:** 172 Future Flicker (three staggered physical body echoes), 173 Pendulum Edge (two double-sided angled time-blades), 174 Chronal Bloom (twelve non-identical time petals).
- **Necromancer:** 192 Spectral Scythe (broad crescent with thickness), 193 Gravetide (three rolling ghost sheets and substantial grave bodies), 194 Bone Palisade (joined bone plates and arched ribs).

## Verified compile and regression results

Final pinned Horizon preview run: [GitHub Actions 44629836762](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762), status **SUCCESS**:
- **28 passed, 0 failed** Rust unit tests, including 200/200 exact authored-route coverage, 15 M27A individual-geometry tests, 15 M27B individual-geometry tests, and a new **120-view** M27 Cast/Impact × front-oblique/back-oblique projection regression.
- **860 original CPU-world-geometry SVG projections** (170 extended spells × Cast/Impact × 2 views plus 30 curated × 3 phases × 2 views) preserved as Actions artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal`.
- Pinned SDK `.hmod` package compiles, SHA512/ZIP/WGSL/30 original OGG files verified. These are internal build checks, not a gameplay playtest.
- The added M27 projection test rejects total invisibility, collapsed edge-on strands, exaggerated world scale and silent 500-face truncation in all 30 new scenes, at both cameras and both tested phases.

## Manual direct source-frame art review: before and after

Downloaded and opened the REAL geometry artifact, made side-by-side multi-angle contact sheets for all thirty new spells, and rejected too-thin / generic constructs. Repaired seven weak cases after viewing the first results:

| Spell | Initial summed projected-face-area proxy (min of 2 views, 2 stages) | Final | Visual change |
|---|---:|---:|---|
| 014 Infernal Anvil | 0.838 | 0.609 | More recognizably connected heavy anvil; deliberately removed distracting floating spike halo |
| 052 Skyward Hook | 0.076 | 0.151 | Toothpick zigzag replaced by broad 3D enclosed air fabric |
| 113 Carnivore Bloom | 0.599 | 0.756 | Fork-like branch replaced by full 12-petal living jaw |
| 132 Skyfire Crown | 0.351 | 0.749 | Single blocky cloud + thin bolts replaced by layered thundercloud and substantial branched sheet lightning |
| 133 Pulse Cage | 0.475 | 0.848 | Three stacked wire rings replaced by full-height eight-sided prismatic force enclosure |
| 173 Pendulum Edge | 0.232 | 0.587 | Bare angled rods replaced by broad front/back sculpted time blades |
| 194 Bone Palisade | 0.380 | 0.458 | Picket spikes connected into continuous ossified plates; **486 of 500** face budget now used |

**Metric warning:** `projected_coverage` is the *sum of projected polygon areas*, including overlap; values can exceed 1. It is neither precise visible-pixel coverage nor aesthetic quality. The final PNGs still visibly lack high-end fantasy lighting; some shapes remain very blocky. User specifically forbids claiming that passing numerical geometry tests proves visual beauty.

## Unfinished work / release gate

1. The original reference-video look depends on HDR-like emission, volumetric fog/rays, camera-aware scene depth/occlusion, richer unique WGSL materials, particle trails and the proper native GPU post path. The Naga-validated prototype in `native-gpu-vfx/aurora_volume.wgsl` is **not connected to Horizon**.
2. Current extended `SpellAction::DesignIntent` effects are *server-rejected*, so 200 actual gameplay abilities, persistent unlocking, damage/healing, terrain/world edits and multiplayer are **not implemented**.
3. No real Horizon Nightly in-game visual QA, performance timings or final GUI-world-scope review took place. CPU SVG is NOT a Horizon screenshot.
4. Final 194 Bone Palisade uses 486/500 faces and should be reworked for budget headroom before release. Not all 200 authored silhouettes pass professional art direction. Work continues on source-video-quality light/material/cinematic staging, not more generic templates.

**No half-built ZIP is to be delivered to the user.**
