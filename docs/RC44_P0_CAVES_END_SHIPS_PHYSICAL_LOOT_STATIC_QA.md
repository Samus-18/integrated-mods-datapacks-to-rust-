# RC44 — P0 corrections after three RC43 gameplay videos

**Date:** 2026-11-08  
**Status:** PRIVATE, STATIC-QA PASSED; **NOT** verified in Horizon Nightly, **NOT STABLE**.  
**Target:** Horizon Rust / Minecraft 26.3, Nether height 1024.

This follows [RC43 three-video findings](RC43_VIDEO_TEST_THREE_CLIPS_ISSUES.md) and [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). The previously distributed RC43 did **not** fix the large cavities seen around Y110–180 because its density taper started only at Y320.

## What changed in the private RC44 test ZIP

1. **Middle-altitude caves:** In `thunderfusion:overworld_overworld_combined`, two nested vanilla 26.3 `minecraft:lerp` / `minecraft:gradient` density functions mix the original RC42 cave density toward the existing surface density. The *surface* fraction is 0 below Y92, approx. 10.7% at Y110, 34.4% at Y150, **52.2% at Y180**, 70% at Y210 and 78% at Y760. This is a mathematical/static reduction in cave density influence, **not proof of correct terrain geometry in game**. Original cave definition is kept.
2. **Unsafe imported End ships:** Removed only `data/thalvoryn/worldgen/structure_set/end/imported/airships_rc23.json`, which was randomly placing eight imported ships at absolute Y145–190 without whole-hull island support. All eight structure definitions, template pools and NBT are **retained for manual placement/future valid placement**. This is deliberately disabling an unsafe spawn path, not implementing terrain-aware ships. Other End structures and naturally generated End City are preserved.
3. **Repetitive tiny End islands:** The existing high-sky `minecraft:end_island` rarity chance denominator was increased 22 → 36; the other island groups and RC43 ecology features remain.
4. **Physical container loot:** Rebalanced **2,033** poor or repeated-ID inventories in **471** structure NBT files (including **784 one-stack inventories** and **984 repetitive-item instances**). Existing functional chests generally get 4–5 context-appropriate distinct item types and deficient storage barrels get 2–3. Empty decorative barrels stay empty. Content is baked into vanilla `Items` tags, **without reliance on unsupported/untested custom world-data-pack LootTable loading**. Already varied and special-item inventories are preserved. Randomness is only through existing template variants, *not* true per-container runtime RNG.

## Exact static QA

| Check | Result |
| --- | --- |
| Source RC43 SHA256 | `501117cdba3247c616b42e0a0143c5afac47814b6d775c33b1adac724d6ce3cd` |
| Private RC44 SHA256 | `7114c8b3edba8e0ed1e1cefc3ed969c0d315cf2935bc5fce033a5edf79f0fc6e` |
| RC44 bytes | 10,159,552 |
| Parsed JSON + pack.mcmeta | **1,701/1,701 PASS** |
| NBT templates | **1,372**, unchanged count |
| Edited NBT | 471, **all 471 non-Items byte streams exactly preserved** |
| Untouched NBT | **901/901 byte-identical** |
| Independent second NBT parser | 18 selected old/new complete semantic comparisons PASS |
| ZIP CRC / unchanged icon / dimensions | PASS |
| OW / Nether / End | −64..2031 / 0..1023 / 0..511 |

**Important limitations:** The three supplied videos show earlier RC43 gameplay, **not** RC44 gameplay. Tests prove syntax and NBT non-inventory preservation, **not** that the new cave shape, End placement, or chest contents work as intended in the current Horizon runtime. Older generated chunks will not change. Existing worlds/saves must not be overwritten with the older RC42 repair overlay without matching the exact world snapshot.

**Still open:** full-footprint placement for large buildings/ships, ordinary-tree collisions, more Nether 3D site variety, cave/End terrain diversity, stable high world FPS/TPS, Horizon native LOD black/white distance artifacts, Nether portal safety, native custom biome IDs, Photon WGSL Rust-only shader and in-game acceptance. The *public* GitHub repository does **not** contain the private RC44 binary because rights for externally supplied NBT map assets have not been verified.

Codec references: [Mojang Java Edition 26.3 release notes](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3) (`lerp`, `gradient`).