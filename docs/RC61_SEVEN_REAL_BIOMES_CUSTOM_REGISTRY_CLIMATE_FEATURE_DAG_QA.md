# RC61 — real custom biome registration (Overworld / caves / Nether)

**9 Nov 2026 | PRIVATE STATIC CANDIDATE | DATAPACK-ONLY SCOPE | NO SHADER WORK**

This batch responds to the user's prioritization: **finish the datapack before working on Photon or engine fixes**. It corrects a major overlooked missing feature in all four actual RC59 candidate datapacks.

## Root cause: RC59 had no custom biome IDs

In the physical RC59 ZIP, the true biome registry directory `data/*/worldgen/biome/*.json` contained **42 vanilla IDs, all under `data/minecraft`**. The other **117** files that naive substring searches counted as biomes were in fact **TREK biome TAG files** (`data/trek/tags/worldgen/biome/`), **not registered biomes**. The Overworld and Nether `minecraft:multi_noise` sources named only original `minecraft:*` biome IDs; the End used `minecraft:the_end`, which resolves to the five original End biome IDs in the pinned Horizon implementation.

Thus previous versions had reworked decorations and cave features but did NOT create any new named environment shown in F3. This is now corrected for seven active worldgen slots.

## Seven real new biome registry IDs and climate source links

| Region | New biome ID | Source biome template | Changes |
| --- | --- | --- | --- |
| Overworld | `thalvoryn:ancient_woodlands` | forest | old spruce, forest colors, rain/temperature |
| Overworld | `thalvoryn:crystal_meadows` | meadow | richer flowering trees, colors, weather |
| Caves | `thalvoryn:glowroot_caverns` | lush_caves | reduced excess moss, amethyst, luminous fungus |
| Caves | `thalvoryn:calcite_grotto` | dripstone_caves | calcite/dripstone visuals, purple fog |
| Nether | `thalvoryn:ember_fissures` | nether_wastes | ash/cinder vegetation, ember fog |
| Nether | `thalvoryn:azure_grotto` | warped_forest | deeper blue fog, luminous details |
| Nether | `thalvoryn:obsidian_cinders` | basalt_deltas | rarefied ash, dark fog, magma detail |

All seven have their own `data/thalvoryn/worldgen/biome/*.json` and are **actually referenced by the appropriate dimension's `minecraft:multi_noise` biome_source**.

- In the Overworld, four existing 6D climate ranges have been **partitioned** (not removed): forest weirdness at 0.72; meadow humidity at 0.55; lush cave temperature at 0.42; dripstone cave temperature at 0.52. There are **53 → 57 climate entries**, with original biome regions still available.
- In the Nether, exactly **six of 49 existing climate grid slots** replace vanilla IDs, assigning two each to ember, azure and obsidian biomes. Remaining 43 vanilla assignments are unchanged. The basalt delta cell count reduces 4→2, without changing the biome generation noise settings.
- Four pre-existing TREK biome tag files are extended, and two new additive vanilla `minecraft:is_overworld` / `minecraft:is_nether` biome tag files identify all new IDs. **71 structure JSON definitions** have their allowed biome HolderSets extended by the new IDs; **no existing allowed biome was deleted**.
- Original monster spawn data from source biomes is preserved, and new flora references are all actual placed_feature resources.

### Independent worldgen cycle correction

The first *intermediate* attempt at new flora ordering introduced a detected cycle in the vanilla-style cross-biome `FeatureSorter` dependency graph. Before release, the builder was amended to topologically order new flora using a rank computed from the original acyclic feature graph. The **actual final candidate passes**: 246 unique placed-feature IDs, **348 directed dependency edges**, **zero detected cycles**.

## Four real and independently tested ZIPs

One combined private archive:  
`Thalvoryn_RC61_7_SKUTECNYCH_BIOMU_JAVA_RUST_1K_2K_PRIVATE_STATIC.zip`  
**38,652,726 bytes**, SHA256 **`5a1d8abbab7afc897372b3c4612bf239e684b21a7e533d4ebd6c25f18281fd2c`**

Contains four independent datapack ZIPs: Java Nether1024 and Nether2032, Rust Nether1024 and Nether2032; exact manifest/SHA256, source builder, wholly independently written validator, QA JSON, installation commands and limitations. **The Photon shader and Horizon engine patches are NOT included**, per user priority.

The **independent validator** opens the physical RC59 sources and each RC61 ZIP separately and verifies:
- ZIP CRC, no duplicate filenames, no duplicate JSON keys: **1,716 JSON / metadata files per candidate** (old 1,707 + 9 added).
- Exactly 7 actual new biome registry files; 49 biome JSON definitions total; all seven selected by actual dimension biome source; climate split bounds independently compared, Nether grid 6 exact reassigned cells.
- Exactly **71 previously defined structures** acquire additional biome eligibility; all other jigsaw/loot/placement fields unchanged.
- 4 existing TREK biome tags expanded only with the new IDs + 2 new additive global tags.
- **1,378/1,378 original NBT template files byte-identical**, `pack.png`, all loot tables, noise/density functions, dimension heights, all structure sets and the full End dimension definition byte-identical.
- **246 placed-feature nodes / 348 edges / no cycle**, all user-namespace referenced features have actual placed_feature JSON, `pack_format=121`.
- Comparison across the four variant ZIPs checks the new biome definitions are byte-identical; each prior variant's Rust-only flags and Nether1k/2k differences are retained.

Files and scripts uploaded privately to **ChatGPT Library `/Horizon/Thalvoryn/RC61_DATAPACK_BIOMES`**. Imported third-party NBT assets have **not been published publicly** pending license/attribution verification.

## What cannot be certified yet

These are real registered, climate-selected custom biome IDs, **but have not yet been witnessed in a generated world or F3 in running Horizon Nightly/Java 26.3**. The environment still has Java 21 and no Rust toolchain/Horizon binary. Claiming the live distributions, performance or structure frequencies have been tested would be false. Only newly generated chunks can reflect the changes.

**End limitation:** The pinned Horizon `minecraft:the_end` source [hardcodes the five standard End biome IDs](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-world/src/worldgen/biome_source/runtime.rs). Replacing it with a generic climate source would risk destroying the origin/dragon-safe End selection. This update **does not** ship untested custom End biome registration; previous End island/structure changes remain as in RC59. Dedicated End custom IDs still need a safe source extension or a verified alternative.

Also unfinished: guaranteed structure whole-footprint terrain/water validation, measured cave connectivity/biome diversity and actual loot rolls, game world load tests, Nether2032 acceptance, engine LOD and portal fixes and complete native Photon. **Photon is deliberately deferred until the datapack is finished.**

The user requested progress reported in 10-point milestones. Do **not** claim an unsupported 60%+ or Stable. The last reported milestone is 50%; RC61 adds substantial worldgen functionality but requires runtime validation before raising completion claims. Keep [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) OPEN.