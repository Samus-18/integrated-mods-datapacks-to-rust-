# RC45 — Nether lava-skiff clustering correction (PRIVATE, static QA)

**Date:** 8 Nov 2026  
**State:** Built and independently statically validated, **NOT tested in Horizon Nightly / not stable**, no public NBT assets.

## Why this change

The user's third gameplay video shows several visually similar crimson / purple platforms or boats clustered in one lava lake at ~13–21 seconds. The actual identity of those structures is not provable from the recording (one has a glowstone cross absent in the RC43 lava-skiff NBT), but the RC44 datapack has a genuine clustering risk: four different skiff placed-feature generators; each was configured with `count:3`. Nether Wastes loaded all four generators independently, and crimson / warped forests loaded two each.

The goal is **fewer simultaneous skiff attempts, while preserving other types of Nether structures and their current density**.

## Changed files

- `data/thalvoryn/worldgen/placed_feature/nether/lava_skiff_{crimson,crimson_high,warped,warped_high}.json`: `minecraft:count` 3 → 1. All existing biome, height, downward scan and two-layer (13×17) lava + air-mast predicates retained exactly.
- `data/thalvoryn/worldgen/feature/nether/lava_skiff_mixed_rc45.json` (new): existing MC 26.3 `minecraft:template` feature with a weighted 1:1 choice of the two **preexisting** crimson/warped NBT boats. No new 3D map data.
- `data/thalvoryn/worldgen/placed_feature/nether/lava_skiff_mixed_rc45_{low,high}.json` (new): same footprint validation as the previously proven static codecs; each `count:1`.
- `data/minecraft/worldgen/biome/nether_wastes.json`: replaces four concurrent skiff placed feature references with the two new mixed low/high references. Crimson and warped forests still reference their own themed skiffs; their counts have been lowered.
- Private QA README added at archive root. No other worldgen, NBT, loot or metadata changed.

### Analytical reduction in skiff *attempts* (not actual successful spawns)

| Biome | RC44 attempt expectation per chunk | RC45 | Reduction |
| --- | ---: | ---: | ---: |
| Nether Wastes | 0.86579 | 0.15263 | 82.4% |
| Crimson Forest | 0.45789 | 0.15263 | 66.7% |
| Warped Forest | 0.40789 | 0.13596 | 66.7% |

Each expectation sums `count / rarity_filter.chance` for all biome-referenced skiff generators. **Actual boats are MUCH rarer depending on lava body size, air, Y, biome and runtime support**; this does **not** enforce a minimum distance across neighboring chunks or eliminate every cluster. No other kinds of Nether structures were made sparser by this patch.

## Pre/post static verification

- SHA256 source RC44: `7114c8b3edba8e0ed1e1cefc3ed969c0d315cf2935bc5fce033a5edf79f0fc6e`
- SHA256 RC45: `e5b37baf9be48a4e0a4db21f260f79e242018f4183f8fc3d864b7c7920888a40` (**10,161,907 B**)
- **3,140 archive members**, exactly **5 modified** and **4 added**, 0 removed, no modifications outside the five whitelisted JSON files
- **1,704 JSON/pack metadata parsed** (independent parser), ZIP CRC PASS
- **1,372/1,372 existing NBT templates byte-identical**, including inventories and every user-supplied map
- **133 custom biome placed-feature references validated**
- **6 skiff placed features** checked for 26.3 count, rarity, scan ≤32, offset ≤16, full 13×17×2 lava predicate and original free-air hull clearance
- Weighted mixed templates each point to preserved, existing NBT IDs
- `pack.png`, `pack.mcmeta`, heights and noise definitions unmodified
- **No Minecraft/Horizon game instance was run during this validation.** In particular, no claims about FPS, underwater/flowing lava safety, anti-collision in adjacent chunks or long-range End/Overworld appearance.

## Private artifacts and test plan

The private ZIP plus reproducible build script, independent QA script and test steps are preserved in **ChatGPT Library → /Horizon/Thalvoryn/RC45**. **Do not put the binary datapack on public GitHub** until attribution/licensing of imported world maps is resolved.

For next user test: keep old world backed up, generate a NEW seed with **only RC45** enabled; verify low and high lava layers with coordinates and video; capture `%APPDATA%\horizon\logs\latest.log` after closing the game and any crash report from `%APPDATA%\horizon\crash-reports\`. The Horizon launcher also has **Console → Open logs folder**. The bundled Markdown README explains the sequence.

## Still not completed

Proper multi-chunk minimum separation and whole-footprint placement of all original buildings, Overworld fire/tree collisions, redesigned End islands and distinct caves, high Nether 3D landmarks, full Horizon renderer LOD/portal safety/new registered biome IDs, Rust-only Photon, runtime FPS/TPS, 1k+2k & Java/Rust final deliverables and EXE converter. All ten historic user requirements remain in [the full follow-up audit](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md). Continue one independently QA'd change set at a time and use real game logs when moving into engine fixes.
