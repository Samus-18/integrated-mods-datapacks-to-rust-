# RC51 — Sky Islands relative-height correction + TREK deep-ocean ship placement

**Date: 9 November 2026** · **Status: PRIVATE, BUILT, independently static-verified, NOT runtime tested, NOT stable.**

## Background

The user requested lower and terrain-aware Sky Islands and ships that stay on water. The previous RC50's five active `thalvoryn:sky/*` structures use **`minecraft:jigsaw` with `project_start_to_heightmap: WORLD_SURFACE_WG`** plus a uniform `start_height` with `absolute` anchors. Important: with a heightmap projection, **the returned height is an OFFSET added to the sampled surface** — not absolute world Y. This explains why a variant with `78..102` can appear very high over a high mountain. [Minecraft Java jigsaw structure codec](https://minecraft.wiki/w/Jigsaw_structure).

Another still-active TREK structure, `trek:overworld/medium/ship_pillager`, previously used `#trek:ocean/is_not_frozen_ocean`. That tag expands to **both deep and shallow oceans**, and the jigsaw only samples a single start biome, so coastal/shallows hull interference remains possible. Previously the separate RC47 disabled eight imported ocean ships with unsafe absolute Y68; here we preserve the one existing TREK ship but narrow the biome.

## Exact RC51 changes

| Sky variant (5 structure JSON) | RC50 relative-Y above `WORLD_SURFACE_WG` | RC51 relative-Y |
| --- | ---: | ---: |
| sky_garden | 75–96 | **42–64** |
| verdant_terrace | 57–79 | **37–58** |
| twin_isles | 54–74 | **36–55** |
| crescent_crag | 66–91 | **40–63** |
| amber_ridge | 78–102 | **45–68** |

The biome lists, template pools, structure-set frequency, salts, original NBT geometry, loot variants, sky-island counts and `WORLD_SURFACE_WG` projection were **not changed**. Height on a very tall mountain may still be above the desired Y110–180 range: there is **no global Y clamp**. Lowering islands too close to tree tops could introduce new collisions; this is a known runtime-check item.

**TREK ocean ship:** in exactly one existing structure JSON, change
`"biomes":"#trek:ocean/is_not_frozen_ocean"` →
`"biomes":"#trek:ocean/is_deep_not_frozen_ocean"`.

The target tag exists in the pack and contains vanilla `deep_ocean`, `deep_cold_ocean`, `deep_lukewarm_ocean` and an optional external biome (no shallow ocean values). **This is not full-hull liquid verification**: large ships can still overhang deep-ocean slopes or border a coast; placing original 8 imported ships safely needs engine checks.

`pack.mcmeta` description now says `RC51 • PRIVATE STATIC QA`; the pack format remains unchanged. A private text README was added to the ZIP. **RC51 is a replacement for RC50, not a second datapack to layer on it.**

## Verified exact diff and QA

Input: `Thalvoryn_Horizon_Rust_RC50_TARGETED_LAVA_FIRE_SAFETY_PRIVATE_STATIC_TEST.zip`
- SHA256 `723c34d3445d8b4641aff9566a593da9c7ec759aa7665555987a044aeba1f111`
- 3,154 ZIP files.

Output: `Thalvoryn_Horizon_Rust_RC51_SKY_HEIGHT_DEEP_OCEAN_PRIVATE_STATIC_TEST.zip`
- SHA256 **`4206c1043d746b8126e661837917952ed1ebecaa7a14e96e6f16a75ea6c982c5`**
- **10,206,759 bytes; 3,155 files**
- **Exactly seven files modified** (five sky structure JSON, one TREK ocean ship JSON, one pack description), **one private README added; zero removed**.
- Independent validator reads **both source and output ZIPs**: CRC PASS / no duplicate filenames / **1,707 JSON or pack metadata files parsed with duplicate-key rejection**, all **1,378 / 1,378 original NBT byte-identical**, **188 active structure-set referenced IDs checked** (vanilla IDs resolved by built-in game), all dimension/noise/density, structure sets, biomes, loot tables and `pack.png` byte-identical.
- Diff-of-objects test verifies **only `start_height` changes** in all five sky definitions, and **only `biomes` changes** in the ship definition. New sky offsets strictly lower than the old ones.
- Source builder + independently coded validator + machine-readable reports + Czech README preserved with the private ZIP in ChatGPT Library folder **`/Horizon/Thalvoryn/RC51`**. The binary is deliberately **not** uploaded publicly due to outstanding licensing/attribution for imported map NBT assets.

## What is still open

- **Horizon runtime and acceptance**: no world/chunks were generated, no shaders rendered, no frame-timing or actual ship water-footprint check executed. Prior generated chunks will not update automatically.
- Safe full-footprint placement for imported ocean, lava, End and large Overworld ships/buildings; real anti-overlap guards for ground features, forestry and lava; cave entrances actually connected to varied underground habitats; Nether 1024 high 3D site variety.
- Entirely custom F3 biome IDs require registration in Horizon engine; End natural high topography and LOD black/white strips, Nether portal safety, performance and full native WGSL Photon are still **not** completed. Final Java/Rust plus 1k/2k variants remain unverified.

**Progress:** approx. **47% of the full requested scope**, a qualitative estimate, **not** a game-test pass metric. The user currently cannot test; no test requested. The open [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) remains the tracker.