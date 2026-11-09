# RC58 — consolidated bulk batch: dynamic rewards, 40 land-site guards, caves and underground hamlet

**2026-11-09 | PRIVATE STATIC QA PASS | NOT a tested/stable release.**

Responding to the request to stop publishing tiny RC increments, this is **one significant update batch** built physically from all four RC57 variants. The user wants progress in **10-percentage-point milestones**, not a separate “done” percentage for each small file. The *prior scope estimate* was 51%; this batch makes substantive progress but no runtime testing, renderer fix or completed Photon should be inferred from a static pass.

## Four coordinated improvements in one build

### 1. True per-placement random reward rolls, not duplicated fixed Items

For **76 actually pre-filled reward chests/barrels across 12 real NBT templates**:
- Replace the stored `Items` list with a Minecraft-native **`LootTable`** string referring to an already existing, *physically present* Thalvoryn loot-table JSON.
- Delete fixed `LootTableSeed` if present. The Horizon engine at the **user's own pinned Nightly `2ebf00a`** derives the loot RNG seed from the block position when no nonzero seed is saved, so two copies placed at different coordinates do not reuse a single static inventory.
- The loot table is chosen by dimension and structure (End ship / ruin / shrine, Overworld ship / settlement / storage, Nether low-tier storage), and three Sky Caravan variants point at **different loot-table IDs**.
- **All previously empty decorative barrels are untouched.** No palette entries, block coordinates, building geometry, non-target block entities or structure entities were changed.

Source verification: [Horizon pinned `structure_blocks.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-client/src/structure_blocks.rs) explicitly reads `LootTable`, drops stale `Items`, and invokes `LootData.fill` with a world-position-derived fallback seed; [pinned `block_entity_nbt.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-client-core/src/block_entity_nbt.rs) recognizes block-entity loot table metadata. Minecraft's [official 26.3 technical notes](https://feedback.minecraft.net/hc/en-us/articles/48913133328013-Minecraft-Java-Edition-26-3) document the `modifier` fields used by the pack's existing loot-table JSON.

**Caution:** This is source and static metadata validation, **not an actual chest-opening test**. Loot tables and their entries were *not* executed by a live game. Previously generated chunks will keep their existing block entities; only new placements get new NBT.

### 2. 40 selected Rust land structures: terrain and top-fluid start checks

Only in **RUST** variants, `horizon:start_checks` was added to **40 existing *active* jigsaw structures** (nine Thalvoryn outposts and variant watches, 23 medium TREK houses/forts and 8 rare TREK large structures). Each is projected to `WORLD_SURFACE_WG` and originally lacked guards.

Each has `terrain_radius:1`, a curated `terrain_range` of 22–38 (based on category and mountain exposure), and `liquid:"top"` to reject a water/lava start column. These three keys are implemented in the pinned [Horizon `start_checks.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-world/src/worldgen/structure/start_checks.rs).

**This is NOT a full-footprint water/terrain/forest clearance check.** It samples chunk centres and one liquid column; it may reduce real starts, particularly on rough mountains. The **Java** variants retain the vanilla schema with no Horizon-specific fields.

### 3. Two formerly fully disabled vanilla cave dungeons restored at conservative rates

Previously `data/minecraft/worldgen/placed_feature/monster_room.json` and `monster_room_deep.json` had only `count:0`, so they **never attempted to generate** in any biome, notwithstanding their usage by cave biome feature lists.

The new two placed features attempt **one candidate, about once in 7 or 9 chunks respectively**, using `in_square`, bounded Y ranges, a minimum depth below `OCEAN_FLOOR_WG` and the biome check. Actual success rate is unknown and can be lower. The moderate cadence is intentional for TPS and to avoid dungeon spam.

### 4. Underground hamlet candidates increased, without changing the structure

`thalvoryn:underground/spiral_hamlet` changed only its random-spread parameters:
- spacing **52 → 44**,
- separation **20 → 17**,
- frequency **0.44 → 0.56**.

The simple candidate-grid estimate is **1.78×**, not a measured count of reachable underground villages. The structure model, height, biome restrictions, terrain and density functions remain unchanged.

## Independent verification against actual RC57

A separate verification program reopens the physical *source* RC57 ZIP and the physical *output* RC58 ZIP. It does not import the builder.

| Check | Result |
| --- | --- |
| Four separate candidates | **JAVA/RUST × Nether 1024/2032: PASS** |
| ZIP CRC and duplicate member names | **PASS** |
| JSON and pack metadata parsed | **1,707 per candidate, duplicate-key rejection** |
| Original NBT resources | **1,378** in each |
| Completely untouched NBT | **1,366 byte-identical** in each |
| Modified NBT | **12**; full binary decode before/after with **76 exact container conversions**, every other structure tag, block position/state/entity unchanged |
| Loot-table refs | **All 76 point to existing chest-type table JSON with populated pools and entries** |
| Rust site rules | **Exactly 40**, no unrequested Java extension |
| Vanilla dungeons | **2 restored**, with valid placed-feature modifiers |
| Hamlet random spread | **1.78× mathematical candidate rate**, unchanged other fields |
| Biome placed feature references | **246 named features**, custom refs resolve |
| Worldgen feature order | **334 edges, no detected cycle** |
| Nether dimension/noise heights | **1024 or 2032 paired correctly** |
| Pack art, all other structures, End/Nether terrain, earlier fixes | Unchanged |
| NBT content across all four variants | **Identical** |

- **Source RC57 SHA256**: `3ef867518dc5877102a28f7faf28d5120828ef6c794940324f6de4aa07ee0f20`.
- **Combined RC58 ZIP**: `Thalvoryn_RC58_BATCH_LOOT_STRUCTURES_CAVES_PRIVATE_STATIC_CANDIDATE.zip`.
- **SHA256**: `ee5146a99fa93fc82218ed74cdb4ff976a224ba88cd5090a6ff33abd7e42d7bb`.
- **Size**: **42,326,412 bytes**, 12 outer entries; four complete inner standalone datapack ZIPs, old Converter v7.2, distinct **Photon native WGSL PREVIEW NOT FULL PORT**, both build and independently written verify scripts, SHA manifest, Czech README.
- Independent QA report is kept as a separate file in **private ChatGPT Library `/Horizon/Thalvoryn/RC58_BULK`**, avoiding a circular archive-hash dependency.

**No proprietary imported map/structure NBT has been published on GitHub.** Public repo only receives documentation and progress, since upstream NBT rights and credits remain unresolved.

## Material blockers remain

- **No actual Minecraft Java 26.3 + Java25 or Horizon Rust binary** was executed, so there is no verified world load/loot roll, chunk generation, rarity, collision, LOD or portalling result.
- The full original demand includes engine native LOD black/white stripes, black screen beyond 16k LOD, Nether portal freeze/arrival, custom biome IDs/F3, FPS/TPS and complete Photon v1.4 port; **none of those were completed in this batch**.
- The 2,032-height Nether profile, 40 terrain start checks and new cave rates are statically reviewed but **need real generation/acceptance tests**.
- Windows EXE is the older recovered v7.2, not newly tested on Windows. Native Photon file is a **preview**, not the full requested port.
- No 100% or “stable / tested on Horizon Nightly 2ebf00a” claim. That pin was used for **source compatibility checking only**.

The next legitimate 10-point completion milestone cannot be certified just by relabeling ZIPs; runtime and engine/shader work are still substantial. [Issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) remains OPEN.
