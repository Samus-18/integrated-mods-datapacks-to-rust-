# RC24 — Horizon Rust 26.3 Nether 1024: conservative placement hardening

**Private experiment, not a completed release.** Origin: RC23. The RC22 user screenshots and video remain documented at [issue #2](../../issues/2).

## Changes made in actual ZIP

- **Overworld ocean ships:** filtered the 27 eligible structures to 16 smaller vessels (up to 65 blocks in X, 30 in Z and 65 in Y); ocean-biome filter remains, and the weighted selection grid is now 96 chunks with separation 42 and attempt frequency 0.30. 11 too-large ships remain available as NBT but are not selected by this structure set. Y62 is provisional relative to the pack's 70 sea level; the code does **not verify the full water footprint**.
- **Overworld imported ground buildings:** only five small templates still selected in random worldgen; single-point terrain projection is unsuitable for full 100-block footprints. Set grid 76/30, frequency 0.23. Original NBT preserved.
- **Nether imported buildings:** disabled the unsafe five-building group (including buildings that appeared in the wrong biome or floating); templates preserved. Previous imported Nether ships were already disabled in RC23. No claim of correct lava-only boat placement.
- **End:** end imported ground buildings restricted to five small templates, large airships filtered to 12 designs and moved to one rarer 86/36 grid (frequency 0.30); end archipelago set 39/14 at frequency 0.50, highland ruins 66/27 at 0.38. Removed 15 redundant independent archipelago/end TREK grids; unified duplicated referenced structures.
- **Nether:** deduplicated native variants and tuned surface-site attempts and pillared-ruin spamming.
- **Sky Garden:** ocean, river and beach biomes removed from its eligible locations.
- **World height:** no speculative engine change. JSON checks confirm **Overworld min_y -64, height 2096** and **Nether min_y 0, height 1024** in both dimension type and corresponding noise settings. User's observed in-game -64..256 limit must be diagnosed from the loaded dimension type and world creation logs. Pack JSON values do not alone prove the active world uses them.

## Independent static verification

- Input: `Thalvoryn_Horizon_Rust_RC23_Nether1024_PLACEMENT_SAFETY_TEST.zip`
- Output: `Thalvoryn_Horizon_Rust_RC24_Nether1024_DEDUP_PLACEMENT_TEST.zip`
- SHA-256: `84d1e66d43075c0c1a42974fbfa68fdae80d3c5b90cd559d6c517cf8a998e98b`
- Size: **8,886,964 bytes**
- ZIP CRC: PASS
- JSON / pack metadata: **1,530 parse successfully**
- Original NBT templates: **1,102 retained byte-for-byte**
- Active structure sets: **56**, active unique structure references: **237**
- No duplicated active `thalvoryn:nether/*` or `thalvoryn:end/*` structure IDs, no duplicate salts and no missing *custom* structure IDs among active placement sets
- This was **NOT tested in the Horizon runtime**, and does not prove placement safety in an actual world.

## Unresolved work (issue #2 stays OPEN)

1. A native structure-start validator that samples **multiple points under the entire structure footprint** (liquid type and depth for ships, max slope and occupancy for buildings, air-only zones for end sky structures) is needed for full reliability. A biome tag and a jigsaw heightmap projection alone cannot guarantee these.
2. Safe Nether portal arrival above lava needs a supported engine fix / native portal-exit placement; this datapack cannot promise that.
3. Large custom biomes with their own F3 names require registered biome IDs and visuals in Horizon; biome recoloring via vanilla IDs is possible, native ID registration is not included.
4. The observed world height and tall-terrain loading need a clean-world test and the actual Horizon log. Compare dimension type, noise height and selected generator.
5. Structure-themed randomized loot across all storage remains a future improvement; no NBT inventories were changed in RC24.
6. End island *geometry* is not yet completely remodeled. RC24 prevents duplicate spawn grids rather than claiming a terrain overhaul.
7. Custom trees require collision-aware placement; old schematics exist but full in-game verification has not been completed.
8. Photon has **not** been natively ported/tested in this build.

## Test procedure

Create a **new world** with **only the RC24 datapack** (do not combine with RC23), then check a sample of ocean ships, non-water world locations, Nether ground structures and End islands. Examine the active world height and save the latest.log. Existing generated chunks are unchanged.

**Copyright:** third-party map/schematic redistribution rights are unverified; do not publish this binary as a public GitHub release without permission. This document contains no original map assets.

Reference: [Horizon world-height documentation](https://github.com/horizon-rust/horizon/blob/main/docs/world-height.md).
