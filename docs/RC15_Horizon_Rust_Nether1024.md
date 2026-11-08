# Thalvoryn RC15 — Horizon Rust 26.3 / Nether 1024

**Status: local experimental test build only; NOT published as a GitHub release/download.** This document records the revised build for review. The local ZIP contains third-party supplied assets; their public redistribution rights have **not** been established. Do not rehost them until permission is confirmed.

## Changes since RC13

- **Nether:** dimension/noise height 1,024 blocks, no hard bedrock roof, taper terrain density toward air above approximately Y700–735. Extra themed structures and greater frequency of existing Nether landmarks, with some basalt/blackstone contrast in Nether Wastes.
- **Overworld floating structures:** Sky Garden uses WORLD_SURFACE_WG projection plus height offset 47–65; tuned to be around 2.5–3× as likely in suitable biomes. Existing TREK aerial farms also use surface-relative placement instead of height from the top of the dimension. **This does not guarantee a strict world Y180 cap on mountains.**
- **End:** reduced excessive chorus and random decorative pillar feature placements; more frequent existing landmarks; six original distinct floating-island ruins.
- **Trees:** 48 standalone tree templates extracted from the supplied 50-autumn-trees schematic. Mod-only foliage mapped to vanilla leaves. Four grove variants; no new biomes are registered.
- **Loot:** replaced lazy LootTable linkage in 2,262 eligible TREK/Thalvoryn chest/barrel/dispenser containers across 314 structure templates with 2–4 modest deterministic item stacks. Vanilla End City templates left intact. New map builds use only six stocked barrels each.
- **Added map structures:** three complete bounding-box builds extracted from the supplied All Builds 1.19.2 map, one Overworld, one Nether, and one End. Three older 1.11–1.12 map collections were not imported due to legacy metadata/orientation issues.
- **Modern palette migration:** 910 NBT structure templates migrated from legacy `Name`/`Properties` to `id`/`properties` block-state keys.

## Static build verification

- 2,378 files; 992 NBT templates; 1,311 JSON files.
- All 992 NBT templates were parsed during the palette migration; ZIP CRC/integrity passed.
- Structure graph: 188 structures, 191 structure-set references, 881 template references; **0 missing target resources**.
- ZIP size: 6,626,365 bytes. SHA-256: `416cb440adec84ecba36f499cc9a0bd7ea05c894c7389958d9adecf8b74233b8`.
- **Horizon Rust in-game test: NOT RUN.** Generation frequency, absolute island height, biome placements, underground fluid behavior, block entities, mob behavior and collision safety need actual runtime testing.
- Existing generated chunks will not regenerate. Create a fresh test world, disable RC13, and install exactly one edition/Nether-height variant.

## Licensing and publishing

The source world and schematics were supplied by a user but do not carry verified redistribution grants. Download availability or purchase is **not** sufficient permission to redistribute original builds publicly. Please verify each creator's license/permission before publishing the binary as a public GitHub release.
