# Thalvoryn RC19 — Horizon Rust 26.3, Nether 1024, terrain/End experiment

**Status: private experimental ZIP, static verification ONLY.** This is a continuation of the RC18 test build, **not a confirmed in-game fix or public release**.

## Concrete RC18 -> RC19 changes
- Fixed three Nether density-function files that still referenced the old 2,032-block world: `nether_caves.json`, `nether_final_density.json`, `nether_floating_islands.json`. Remapped the floating-island vertical band from Y1270–2032 to **Y340–705**. These are generator parameters, not a screenshot/gameplay validation.
- Increased spacing-based chance of existing TREK Nether and End landmarks and the imported RC18 structure atlases.
- Introduced weighted placement sets with **9 existing Nether surface structures** and **14 existing End island/ruin structure variants**. Original NBT geometry is preserved; selection frequency and actual biome/terrain suitability require an in-game check.
- Reduced rarity-filter occurrence of synthetic End obsidian, purpur and amethyst pillars and excess chorus; tiny repeated generic End islands were also reduced in favor of authored ruins.
- **Ancient debris:** fixed inverted low/high placement attempts from RC18. Low Y6–380 is now a rare `1/40` attempt per chunk; higher Y300–725 remains a `1/16` attempt per chunk (~6.25%). Not a per-block chance, guaranteed deposit count, or strict 3–4 maximum.

## Static verification
- ZIP passes CRC and contains **2,755 entries**.
- **1,574** JSON/pack metadata records parsed; **100** structure sets validated for spacing > separation.
- Structure sets: 0 missing owned structure IDs; **295** start-pool links and **991** pool-template links resolve without missing custom targets.
- **1,102 NBT templates** preserved byte-for-byte from RC18.
- SHA-256: `9ea887db181ed0c8dd83ce62268a14a2a13528bbe3272e7b06796e00c1219ccb`
- Local ZIP: `Thalvoryn_Horizon_Rust_RC19_Nether1024_TERRAIN_END_TEST.zip` (8,888,322 bytes).

## Still unresolved
- **Horizon runtime/in-game testing: not run.** There is no guarantee that the user-observed Nether clipping, actual structure spawn frequency, biome appearance, mobs, or inventory behavior is fixed in their build.
- No new F3 biome identifiers: Horizon skips datapack-defined new biome IDs in current worldgen. The variation is through current IDs, terrain/material rules, features and structure sets.
- Guaranteeing portal arrival on the surface is engine-level work, not a datapack change.
- Structure / tree collisions and imported map-entity conversions need an in-game audit.
- **Photon** is a separate task; it was **not ported** to native Horizon shader format by RC19.
- Source map owners' redistribution permissions remain unverified; do not publish the third-party asset-containing binary to a public GitHub Release without permission.

Use a new world with only RC19 enabled. Previous RC18 and earlier packs must not be stacked; generated chunks are not retrospectively rewritten.

Horizon documentation: https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md
