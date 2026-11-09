# RC56 — physical loot restored to 36 overlooked storage blocks

**Date:** 9 November 2026.  
**Status:** PRIVATE / STATIC QA PASSED / no Minecraft or Horizon runtime tests / NOT STABLE.

## Why RC55's sampling had missed the issue

The previous RC55 QA fully parsed 17 selected NBT templates and showed 81 filled containers, but it did **not** decode inventories for all 1,378 templates. The user's long-standing reports of empty chests/barrels justified a broader, differentiated scan.

A new independent read-only binary-NBT audit decoded **all 185 NBT templates whose palettes include barrels** (1,884,975 blocks, 5,392 containers of any type) and **all 570 NBT templates whose palettes include ordinary or trapped chests** (3,312,467 blocks, 6,133 containers of any type; these sets overlap). It detected:
- Two empty reward barrels in an otherwise stocked Overworld building.
- Six empty ordinary chests in an Overworld Air Vessel.
- Six empty ordinary chests in each of three End Sky Caravan NBT variants.
- Eight empty ordinary chests interleaved with four physically stocked chests in a Nether building.
- Two empty ordinary chests beside four stocked chests in another Overworld building.
- Hundreds of *deliberately decorative* empty barrels in large vessel walls. Those must not be indiscriminately filled.

This is a **targeted fix**, not an assertion that every empty container in every unused/imported structure should hold loot. It does not alter decorative storage walls or add duplicate OP drops to every barrel.

## Actual changes compared with the RC55 ZIPs

Exactly **7 existing NBT templates** altered, adding `nbt: {id, Items}` to exactly **36 previously NBT-less storage blocks**, without changing any block state, position, palette, entity, dimension/biome/structure definition, or already present inventory:

| Source template | Fixed containers |
| --- | ---: |
| `thalvoryn:end/handmade/sky_caravan` | 6 chests |
| `thalvoryn:rc42_variants/thalvoryn/end/handmade/sky_caravan_loot_1` | 6 chests |
| `thalvoryn:rc42_variants/thalvoryn/end/handmade/sky_caravan_loot_2` | 6 chests |
| `thalvoryn:overworld/handmade/air_vessel` | 6 chests |
| `thalvoryn:overworld/imported/buildings_01` | 2 barrels |
| `thalvoryn:overworld/imported/buildings_17` | 2 chests |
| `thalvoryn:nether/imported/buildings_24` | 8 chests |

End rewards mix pearls, chorus fruit, occasional diamonds, amethyst, a rare shulker shell and limited consumables, with **different placements across the three variant templates**. Overworld carries common food/iron/materials and occasional mid-tier rewards; Nether uses restrained quartz, magma, nether wart, ingots and fire charges. Each item-stack follows the already existing MC 26.3 template format `Items: [{Slot: TAG_Byte, id: TAG_String, count: TAG_Int}, ...]`. **The inventory is physically embedded and deterministic per NBT template, NOT re-randomized by the game on every placement.**

Only a metadata description was changed in addition to those seven NBT files. All four independent RC55 candidate variants were patched in the same way and were not stacked together.

## Independent comparison and validation

- Original combined RC55 SHA256: `d1f9702afb2ff0760c1ef13967a880fc6fdc838b08b2e79f2a1dc95bc6d0ed81`.
- **New RC56 single private archive SHA256: `3212174b04640e6103313251c6f06212b90a806a7db2f468b10633062231716c`.**
- Size **42,320,502 bytes**, 16 entries; saved privately to **ChatGPT Library `/Horizon/Thalvoryn/RC56`**.
- Inside: Java + Rust builds, each with Nether 1,024 and 2,032 height; a carried-forward Windows Converter Suite v7.2 (no new Windows runtime test), and a strictly labelled **WGSL shader PREVIEW_NOT_FULL_PORT**.
- Two separate programs (builder and independent reader) opened the actual source+target archives and checked **CRC**, exact path lists, exact edit whitelist (**7 NBT + `pack.mcmeta`**), **1,371 untouched NBT byte-for-byte**, **all 36 inserted tile NBTs** with 2–4 nonempty valid item stacks, no slot duplicates, stable unchanged top-level NBT data/positions/palette/entities, and cross-variant identical patched NBT bytes.
- **1,707 JSON/meta parsed with duplicate-key detection per variant**. Complete biome feature dependency graph **246 nodes / 334 edges / no detected cycle**, all custom referenced features and active structure sets resolve. Height of Nether dimension type and noise settings consistent with 1,024/2,032 in all four builds. `pack.png` and all other datapack data byte-identical to their respective RC55 predecessors.

### Per-variant SHA256

| Candidate | SHA256 |
| --- | --- |
| Java Nether1024 | `70f11961c254c368eca62f5866ecc53da21586e9f4b6fe299ff5d95243598e23` |
| Java Nether2032 | `4affa743a703c020909f5f82645d14cace2fa6c43ca9e5fce04f0e52cc4390cf` |
| Rust Nether1024 | `032cd31b58435f49e353b05628d25777b2014983ab6b94a39914793311c17a4f` |
| Rust Nether2032 | `9776d56e6aa17dc4f406bc2ee97204b63b5d24059ea92efad9291829348c0460` |

## Remaining blockers and release policy

No actual MC 26.3 Java/Horizon instance or GPU was run. No in-game chest-opening results, actual sky-island navigation, 2k Nether noise regeneration, native LOD/portal fixes, full-footprint site collision engine changes, complete WGSL Photon v1.4 port, Windows EXE runtime smoke test, or performance benchmark was done. The current Nightly's support for Horizon-only start checks remains unproven. Therefore **NOT 100%, NOT stable**. Existing previously generated chunks won't get new NBT loot just from installing the pack. The user cannot test right now.

The ~**50%** global progress estimate is a rough work-scope judgement, NOT a percent of test cases or validated code. The underlying extensive feature and renderer requirements remain tracked in [full requirements](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md). Keep [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) open.

**No binary publication on public GitHub** pending rights for imported third-party NBT assets; only document the work. The full ZIP and reports are in the private Library folder mentioned above.
