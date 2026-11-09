# RC49 — extra-tree clearance and cave-room encounter balance

**Date:** 9 November 2026. **State: PRIVATE / STATIC-QA PASSED / NO IN-GAME TEST.**  
**Applies to:** Horizon Rust / MC26.3 Thalvoryn Nether 1024. This is incremental to RC48, not another independent datapack to stack alongside it.

## Problem from user videos

1. Overworld: additional tree attempts visibly grow very close to doors, windows and rooftops, alongside denser vanilla/jungle flora. The eight custom **extra vanilla-tree placed features** in RC48 had only rarity, square, surface heightmap and biome filters; none checked nearby air.
2. Caves: user sees mostly Lush-looking rooms; existing Dripstone/Calcite ruins are too difficult to encounter. RC48 had nine cave-room rarity filters ranging from 1/8 to 1/19, split into low and upper altitude layers.

## Implementation

### Local tree clearance, not whole-site protection

Eight extra feature definitions modified: `birch_hollow`, `meadow_birch`, `oak_flower_valley`, `oak_meadow`, `old_spruce`, `savanna_acacia`, `spruce_grove`, `spruce_ridges`.

Between their unchanged `WORLD_SURFACE_WG` heightmap and `minecraft:biome`, insert one `minecraft:block_predicate_filter` with `minecraft:all_of` of **9 separate `minecraft:matching_blocks` air/cave_air samples**. There are three central checks at Y+1,+2,+3, plus six near-trunk samples at Y+2, with X/Z offsets −2..2. These short-distance checks can **reduce** trees on obviously occupied construction squares. Their counts, tree species and original rarity stay unchanged. Vanilla trees, jigsaw trees and all external NBT geometry are untouched.

**Limit:** Not an engine-wide footprint collision guard; foliage can still overlap far-away walls and lava/fire sources remain unprotected. There is no claim to have solved the observed burning trees.

### Cave-room attempts, deliberately not room-spawn guarantees

| Placed feature | Chance old | Chance new | Max. relative attempt multiplier |
| --- | ---: | ---: | ---: |
| trek_dripstone_room | 1/8 | 1/5 | 1.60× |
| calcite_ossuary | 1/9 | 1/6 | 1.50× |
| trek_dripstone_room_upper_rc42 | 1/11 | 1/7 | 1.57× |
| calcite_ossuary_upper_rc42 | 1/13 | 1/9 | 1.44× |
| trek_lush_room | 1/8 | 1/6 | 1.33× |
| moss_library | 1/9 | 1/7 | 1.29× |
| trek_lush_room_upper_rc42 | 1/11 | 1/9 | 1.22× |
| moss_library_upper_rc42 | 1/13 | 1/11 | 1.18× |
| sculk_archive | 1/19 | 1/16 | 1.19× |

No `minecraft:count` changed. The cave-room features already contain biome and surface-relative depth tests, and underlying `minecraft:fossil` configured features limit empty corners. **An increase in attempts alone does not establish more *accessible* rooms or changed cave biome incidence.** Dripstone/Calcite was increased slightly more relative to Lush to reduce green-room repetition where these cave biomes do occur.

## Exact provenance and static QA

- **Base RC48 SHA256:** `b095c9f806bfc383a389a2e36ea60c2bbb6c2841facc5f35c878f3784a93e5eb`
- **Output RC49 SHA256:** `d6a7f9dfc77deaa5ab2a43d1cd93033c334e072c48051bc0bf052752fb30aebb`
- **RC49 size:** `10,177,000` bytes; `3,153` members.
- **Diff:** 18 existing files changed (8 tree feature JSON, 9 cave room feature JSON, 1 pack description); one private README added, **none deleted**.
- **Two independent scripts PASS:** archive CRC, 1,707 JSON/meta parse with duplicate-key detection, all edited coordinates ±2 and height +1..3 within allowed 26.3 `offset` limits, no missing custom biome→feature refs (136 checked), 188 active structure-set entries checked, 8/8 tree and 9/9 cave feature IDs still used by some biomes, all cave JSON changes confined to the one `rarity_filter.chance`.
- **1,378 / 1,378 old NBT byte-identical**, all `data/**/structure_set`, `structure`, `template_pool`, loot tables, terrain density/noise, dimension types, all 42 Minecraft biome JSON, art icon and every unrelated old resource are **byte-identical to RC48**.
- Official current [Minecraft 26.3 release notes](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3) confirm new worldgen block predicate and codec behaviors. It is a **source/schema reasoning check**, not a deserialization or real-world Minecraft/Horizon generation test.

**Private deliverables**, preserved for the user only in ChatGPT Library `/Horizon/Thalvoryn/RC49`: 10.18 MB RC49 ZIP, two independently runnable Python source scripts, their original QA JSON reports and a brief Czech README. Do **not** publish user-imported third-party NBT map templates on public GitHub without obtaining permission/license details. The existing library snapshots RC43–RC48 remain intact.

## Open issues and progress estimate

**Rough global project progress ~45%; ~55% remains.** This is a judgement by task scope, **not a code-test pass percentage**. Although much of the datapack has been statically assembled, the user-required **actual gameplay/acceptance validation remains 0% complete**. In particular the following have NOT been fixed by RC49: full-footprint clearance for massive structures and ordinary trees, burning wood/lava near towns, safe original water/Nether ships and other cluster-spam structures, true multi-layer biome registry/F3 names, high End terrain, rendering LOD (black/white bands), portal arrival safety in 1024-high Nether, rust-native complete Photon shader and Java/Rust 1k/2k final packaged builds. No runtime testing requested of the user at present.

**Next defensible technical step:** separate static fire/lava risk audit of real NBT wooden buildings, improve placement where the decoder permits, or patch/test the actual Horizon Rust engine modules for full-footprint structure support, safe portal spawn and LOD. Never present a static JSON/NBT regression as the final answer to an engine problem.
