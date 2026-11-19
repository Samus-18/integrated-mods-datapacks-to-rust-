# M21 — 20 uniquely scripted Ultimate-spell 3D drafts, visual review

2026-11-19. Branch: `aetheria-30-spell-test`. Internal preview v0.6.0, **NOT a completed cinematic mod**.

## Purpose and quality rule

The user demanded **200 actually different beautiful spells without generic “slop.”** Previously only 30 had curated world geometry while 170 had initial large semantic-template drafts. An actual SVG projection audit of the 170 was conducted; many looked like plain cubes, orbs and sparse rods. Those are **not acceptable as final artwork**, even if test code proves finite coordinates or distinct geometry.

This milestone allocates a **dedicated handwritten scene for every Ultimate** instead of scaling up a Standard attack or reusing school-colored rings. Each scene derives from the canonical authored spell's original name, `mesh` and `mechanic` in `src/magic/registry.rs`. Source lives in `preview-5spells-wasm/src/ultimate_forms.rs` and is routed from `render_spell` for exactly the last two spells of each class.

## 20 independent scene directions (all CPU/world-quad DRAFTS)

| ID | Authored original spell | Distinct main composition |
|---|---|---|
| 019 | Cataclysmic Caldera | Raised terraced volcanic crater, large staggered magma crystal heights and smoke curtains |
| 020 | Sunheart Rebirth | Large unfurling dense phoenix wings with faceted fiery torso and flowing light |
| 039 | White Silence | Ice sanctuary/vault with physical crystal buttresses |
| 040 | Absolute Zero Cathedral | Enclosing prismatic ice cathedral, arches and icy spires |
| 059 | Worldbreath Spiral | Layered large twisted pressure-vortex sheets |
| 060 | Vacuum Requiem | Central compressed gravity globe with inward fractured orbiting debris |
| 079 | Earthshaker Throne | **Revised after image review:** suspended broad seat, high carved stone backrest, armrests and broken rocks |
| 080 | Cathedral of Roots | Heavy connected stone/earth arches and central cavern rib canopy |
| 099 | King Tide | Tall connected flowing water wave crests |
| 100 | Deepheart Communion | Coral sanctuary crown of asymmetric growing stems and an inner luminous vault |
| 119 | World Tree Awakening | Giant faceted tree trunk, expansive canopy and deep root spires |
| 120 | Gaia Embrace | **Revised after image review:** distinct humanoid ancestral spirit (head, torso, shoulders, arms, antlers, down-hanging leaf mantle), not another spiky tree |
| 139 | Judgement of Storms | Tall branching thunder tree with faceted charged canopy |
| 140 | Heart of Thunder | Heavily faceted churning storm heart and spatial lightning shards |
| 159 | Starfall Liturgy | Spatially offset sequence of original star-stone fragments with wide tails |
| 160 | Hollow Sky | **Revised after image review:** deep, broken, asymmetric moon fragments with thick side walls and dim dimensional depth, not a thin perfect circle |
| 179 | Thirteenth Hour | **Revised after image review:** 13-piece deep carved cathedral clock, distinct 3D pointers, raised hour-mark facets and anchored stone buttresses, not a plain monolithic slab |
| 180 | Epoch Reversal | **Revised after image review:** two faceted upside-down hourglass chambers, a narrow throat and ascending timefall curtains, not a basic egg |
| 199 | Kingdom of Ash | Broad dark mausoleum arches and rising grave dust |
| 200 | Last Requiem | Physical large spectral guardian with thick layered cloak/wings |

## Verified and unverified

- The first M21 full client build [run 44604619242](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44604619242) **passed**: **15/15** preview tests, including twenty individually selected ultimate geometry tests, and 860 diagnostic SVG views. .hmod package, SHA512, 30 OGG and shader were valid.
- Actual first-pass all-20 contact sheets **were inspected**, and problems observed: Thirteenth Hour looked like a slab, Epoch Reversal like an egg, Gaia Embrace like a tree, Earthshaker Throne like a drum, and Hollow Sky like a thin uniform ring. Those FIVE designs were explicitly rebuilt in `ultimate_forms.rs` (commit `af77cb63d3179a04698cb9a6092e652537180405`).
- The NEW iteration requires another full pinned CI and multi-angle review; only report a passing result with actual run ID after its completion.
- **Passing tests DO NOT prove cinematic beauty**, reference-video equivalence, user-world terrain effect, dynamic lighting, world-save persistence, network authority or safe 60 FPS on RX 580.
- The current 20 ULTIMATES are source-grounded hand-composed **3D layout drafts**. They still need excellent shader HDR/bloom where supported, atmospheric material, destruction/repair mechanics, real sound choreography, camera staging and scene-aware rendering. Do not ship them as “fully polished 20 Ultimates.”
- All 200 spells have registered cast intents and four phases. Only the original first 30 have custom named action variants. Remaining 170 are **DesignIntent**, server rejected until world effect implementation. GUI is an internal local-only selectable design preview.

## Quality gates for next iteration

Visual quality assessment is **per spell**, not merely total polygons, projected area or RGB fingerprint. Review from front, diagonal and side at charge/cast/impact/afterglow, check that main action is recognizable in 1 second, avoid thin rods/empty rings/blank boxes/spheres, preserve unique silhouettes and do not blow out to flat white. Audio/terrain and true GPU postpass should be separately verified on the exact Horizon build. No demand for user manual intermediate tests.


## M21 second-pass visual evidence — verified, not inferred

Second source commit: `af77cb63d3179a04698cb9a6092e652537180405`. Full pinned WASM build [run 44605922355](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44605922355) completed **SUCCESS**: **15/15** native-host preview Rust tests (0 failures), 200-entry registry validation, 20 individually authored Ultimate mesh tests, 860 offscreen diagnostic SVG images, complete original 30 OGG and .hmod SHA512/integrity. The matching fresh visual artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal` was downloaded and **20 Ultimate Cast views were actually inspected again**.

Observed second pass: #079 has now a visible chair/throne backrest rather than an anonymous tiered disc; #120 reads as a cloaked humanoid forest spirit (more distinct from #119); #160 reads as thick broken lunar shards and a real dark center rather than a uniform thin hoop; #179 has a deeply recessed 13-part clock mechanism instead of a featureless brown plaque; #180 reads as a two-chamber inverted hourglass instead of a plain gold egg. This is **genuine silhouette improvement**.

Still **NOT approved** on the user's cinematic beauty scale: simple polygons, uniform material class colors, weak surface shading, some Ultimates too similar (#119 tree vs. #139 thunder tree), GPU atmospheric/bloom and actual depth-aware post are not attached. Portals, wave, clouds and time bodies are still first-pass shapes. The 860 SVGs are generated from CPU worldquad geometry and NOT Horizon screenshots. No manual user playtest requested and no user-ready polished build should be sent.
