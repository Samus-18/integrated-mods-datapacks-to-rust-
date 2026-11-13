# Thalvoryn V76 — archaeology loot repair

Status: **4/4 PASS_STATIC_ONLY**. Built directly from V75 (Minecraft 26.3).

Deep loot review found **1,930 brushable suspicious gravel blocks across 67 NBT templates** pointing at two **nonexistent** loot tables: `minecraft:archaelogy/jungle_ruins` and `minecraft:archaelogy/ruins`. These are typos; no vanilla 26.3 loot tables exist at those IDs.

Fixed NBT loot references and created four weighted-random `minecraft:archaeology` tables:

- `thalvoryn:archaeology/jungle_common` — 474 references (ordinary jungle dwellings)
- `thalvoryn:archaeology/jungle_temple` — 502 references (jungle temples, rarer drops)
- `thalvoryn:archaeology/ruin_common` — 842 references (ordinary town ruins)
- `thalvoryn:archaeology/ruin_center` — 112 references (town center and events, rarer drops)

All 887 active loot-table JSONs parsed; zero errors. The 67 modified NBT templates were independently decoded and checked for **no changes except LootTable strings**. Four ZIP variants (RUST/JAVA, 1024/2032 Nether) passed CRC and diff checks. There are no forced `LootTableSeed` strings in structure templates.

**Structure difficulty** determines the reward tier; not Minecraft game setting (Peaceful/Easy/Normal/Hard). Java 26.3 has no standard `random_difficulty_chance` loot predicate (that predicate is Bedrock-specific). Existing biome-/dimension-specific `thalvoryn:chests/rc37/*` tables stay unchanged.

No in-game runtime, FPS/TPS, or generated-world placement testing was performed. Changing a datapack does not update loot already generated in the save.

Binary private ZIPs are delivered in the ChatGPT conversation, not this public repo, pending review of bundled third-party asset licenses.