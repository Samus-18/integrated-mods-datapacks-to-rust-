# RC50 — Targeted Overworld lava fire-safety repair (PRIVATE, STATIC QA)

**9 Nov 2026 | Horizon Rust / Minecraft 26.3 | Nether 1024.**  
**Status: BUILT, independently static QA PASS, NOT tested in game, NOT STABLE.**  
The original gameplay, launcher and saves were not altered.

## Why the RC49 fire alert was insufficient

The RC49 coarse audit marked 9 NBT structures as containing lava within a Chebyshev distance of ≤4 from potentially flammable wood. That is **not by itself a fire-spread proof**: enclosed lava behind glass/stone often has no reachable air cell that can ignite wood.

RC50 implements a more selective, conservative *local NBT geometry* check reflecting [Java Edition lava ignition behavior](https://minecraft.wiki/w/Lava#Fire_spread): it detects an **explicit air/cave_air cell** within X/Z ±1 at Y+1, or ±2 at Y+2 relative to a lava block, immediately adjacent (6-axis) to a specifically classified flammable block. This excludes hypothetical terrain not encoded in the NBT and does not make claims about actual random ticks.

### Actual changed blocks

Only **5 NBT** files were modified, and only the **101 source lava blocks** independently confirmed to match that local ignition pattern:

| NBT template | Risky lava blocks replaced | Safe native material | Lava blocks left |
| --- | ---: | --- | ---: |
| `minecraft:stronghold/stronghold_big` | 95 | magma block | 1,090 |
| `trek:overworld/rare/maya` | 1 | sea lantern | 125 |
| `trek:village/plains/houses/plains_weaponsmith_1` | 2 | lit campfire | 0 |
| `trek:village/savanna/houses/savanna_toolsmith_1` | 1 | magma block | 0 |
| `trek:village/taiga/houses/taiga_weaponsmith_1` | 2 | lit campfire | 0 |

The magma, lit campfire and sea-lantern blocks do not behave like lava fire sources. The remaining lava pools and the intentionally glazed lava displays are retained. **Other village template lava was left unchanged when the more exact NBT geometric test found no evidenced ignition channel.**

For four templates the change is a four-byte NBT block-state **index-only** change at target positions; in the savanna template, the one and only lava palette entry is renamed to `minecraft:magma_block` (same block position). No NBT block positions, tile inventories, entities, source structures, chest metadata, text, doors, terrain, dimensions, feature weights or art assets changed.

## Exact reproducibility and independent QA

- Input RC49 ZIP SHA256: `d6a7f9dfc77deaa5ab2a43d1cd93033c334e072c48051bc0bf052752fb30aebb`.
- RC50 ZIP: **`Thalvoryn_Horizon_Rust_RC50_TARGETED_LAVA_FIRE_SAFETY_PRIVATE_STATIC_TEST.zip`**, **10,206,060 bytes**. SHA256 `723c34d3445d8b4641aff9566a593da9c7ec759aa7665555987a044aeba1f111`.
- Members: **3,154**. Exactly **6 original files modified** (5 NBT and `pack.mcmeta` version string); **one text README added**; 0 deleted.
- **1,707/1,707 JSON / pack metadata parsed**, with duplicate JSON keys rejected.
- **1,378 original NBT retained**; **1,373/1,373 untouched NBT byte-identical**.
- Independent NBT binary decoder confirmed all 5 edited templates have unchanged dimensions, positions, tile-NBT inventories, entities and non-target states.
- Independent lava/fire air-cell scan confirmed **101 original hazards → 0 after replacement in the five modified templates**.
- ZIP CRC PASS; **136 own biome placed-feature refs still resolve**; `pack.png`, all generated biome/terrain/structure/loot JSON, all dimension heights (Overworld −64..2031 / Nether 0..1023 / End 0..511) unchanged.
- Both builder and **a separate second validator** are packaged with machine-readable logs and original evidence in **private ChatGPT Library `/Horizon/Thalvoryn/RC50`**. No binary ZIP uploaded to public GitHub while rights/attribution of supplied NBT maps remain unresolved.

## Limitations and next work

This corrects a demonstrable NBT-local source of fire hazards in 5 existing models, **not every cause of the burning tree seen in the Overworld video**. The fire in that video may come from **natural lava outside a structure or intersecting worldgen placements** and is therefore not proven fixed. The checker does not model every potentially flammable block, materials overwritten after template placement, rotation, missing cells or naturally generated terrain.

No actual Horizon Nightly / MC 26.3 runtime execution, lava ignition tick test, FPS/TPS benchmark, portal/LOD test or seed-local structure placement test happened. Existing chunks are unchanged by datapack updates. **The user currently cannot test and is not being asked to.**

**Remaining major tasks:** whole-footprint clearance for all buildings and tree categories, terrain-vs-wood/lava interference, safe water ships and complex lava structures, accessible cave entrances/varied underground places, high Nether/End biome geometry, custom biome registry/F3, Horizon LOD and portal placement, complete Rust-only native Photon WGSL and independently verified Java/Rust+Nether-height deliverables. See [full user requirement audit](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md).

**Progress estimate:** roughly **46% of overall scoped project work**, very approximate, not a passing-test ratio; actual full in-game acceptance remains unverified.
