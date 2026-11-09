# Thalvoryn datapack BULK62 — End overlap, caves, Nether ecology

**10 November 2026. Private datapack-only static candidate, not a runtime-tested release.** No Photon or engine additions.

## Actual source-to-output changes

Input: four *physical* RC61 ZIPs (Java/Rust × Nether1024/2032).

**Overworld caves**
- `thalvoryn:calcite_grotto`: removed six inappropriate outdoor feature attempts, including plains trees, flowers, tall grass, pumpkins and sugar cane. Preserved mineral seams, dripstone, fungi, underground structures and other cave features.
- `thalvoryn:glowroot_caverns`: removed three especially redundant green vegetation attempts. Preserved crystals, cave vines, roots and fungi.

**Nether**
- `thalvoryn:obsidian_cinders`: removed large basalt-column and basalt-blob feature attempts, keeping smaller basalt, blackstone, mining and cinder vegetation. The other two new Nether biome definitions and both Nether height variants are untouched.

**End: reduce repeated/overlapping buildings while keeping natural islands**
- `thalvoryn:end/landmarks`: frequency 0.44→0.38; exclusion radius 7 chunks from `rc19_diverse_islands`.
- `thalvoryn:end/rc21_highland_ruins`: 0.40→0.32; exclusion 8 chunks from End landmarks.
- `thalvoryn:end/rc35_micro_landmarks`: 0.35→0.30; exclusion 6 chunks from highland ruins.
- `thalvoryn:end/handmade/sky_caravan`: 0.32→0.27; exclusion 8 chunks from natural diverse islands.
- `thalvoryn:end/sky_sanctuary`: 0.32→0.27; exclusion 8 chunks from Sky Caravan.
- The standalone `rc19_diverse_islands` (11 variants) stays unchanged, as do `minecraft:the_end` biome source and dragon-spawn area. The directed exclusion graph is acyclic. Frequency changes alter candidate attempts, **not proven real spawn counts**.

## Independently verified

Per each of four candidate ZIPs:
- **9 exact JSON/meta edits**, no resource additions/deletions. **1,716** JSON and metadata parsed with duplicate-key rejection. ZIP CRC PASS.
- **1,378/1,378** existing NBT templates byte-identical to RC61, including all structure shapes, chest/barrel inventories, entities and loot. `pack.png`, terrain noise and dimensions also unchanged.
- **49 biome definitions**, with 7 previously introduced custom IDs still actually selected in the Overworld and Nether biome sources.
- **246 distinct placed features, 353 ordering edges, no cycle** across all biome feature sequences.
- **36 structure sets, 311 structure definitions, 572 template pools, 1,251 NBT pool references, 65 feature-pool references**: no unresolved custom IDs. Nested list-pool elements checked.
- End dimension is still `minecraft:the_end` and none of the active End structure sets lists the central `minecraft:the_end` biome. All five newly added exclusion targets exist and no directed exclusion cycle occurs.
- Nether dimension type and noise settings match 1024 or 2032 exactly. Four variants preserve Rust-only structure guards versus Java-standard data.
- Independent source/output validator does not import the builder.

**Single private combined file:** `Thalvoryn_DATAPACK_BULK62_JAVA_RUST_1K_2K_PRIVATE_STATIC.zip`, **40,859,247 bytes**, SHA256 `22be3562d09f70d73423e0d7ebcb5f63bbb6d4683ead464d8a5859f84998ecf9`. Contains the four complete datapack ZIPs and sources/QA/manifest/README. Stored privately in ChatGPT Library `/Horizon/Thalvoryn/DATAPACK_BULK62`. No imported map binary published to GitHub because rights/attribution remain unconfirmed.

## What is actually unfinished

**This is not 100% complete or stable.** No Minecraft 26.3 Java 25 server, Horizon Nightly client or full worldgen/loot/chunk generation was run. High-risk unknowns remain: actual runtime codec acceptance, gameplay/performance of 2032-height Nether, structure full-hull support/collision across chunks, verified cave room accessibility, per-placement loot opening, and custom **End** biome IDs (the pinned `minecraft:the_end` source hardcodes the five vanilla End biomes). We intentionally preserve the central End island instead of swapping the whole End source to an unsafe untested `multi_noise`. Horizon LOD/portal and the native Photon port are separate work **after the datapack**, as the user instructed.

Earlier task-scope percentage estimates are not code-coverage or runtime-passing percentages and should **not** be increased merely for ZIP rebuilds. See [full audit](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md) and open [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
