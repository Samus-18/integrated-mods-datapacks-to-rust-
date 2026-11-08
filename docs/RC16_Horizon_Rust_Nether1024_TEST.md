# Thalvoryn RC16 — Horizon Rust 26.3, Nether 1024

**Status: Experimental local test ZIP; static checks completed, not tested in a running Horizon world.**

RC16 is an update to the user-supplied RC15 build. It responds to a documented terrain issue at Y 588: RC15 applied a -1000 density gradient between Y575 and Y735, causing an unnaturally flat terrain cutoff. RC16 restores the original terrain density foundation, adds broad/fine horizontal and volumetric noise, and a gradual high-altitude fade instead of the rigid cutoff.

## Worldgen changes

- **Nether:** height 1024, no bedrock ceiling; variable terrain designed to taper through upper heights. 49 climate targets that map **only to the five existing vanilla Nether biomes**, with warped/crimson forests favoured over basalt deltas/soul sand valleys. Five surface material accents. Increased natural Nether quartz, gold, glowstone and fungi.
- **Ancient debris:** two 26.3 vanilla placed-feature overrides. High-altitude option uses 1-in-16 chunk **placement attempts** across Y8–820; additional low-level option uses three placement attempts over Y6–480. This is **not** an exact 6% per-block chance and **not** a strict 3–4-block limit.
- **Structures:** increased frequency across 17 Nether and 19 End structure sets, populated seven previously empty TREK structure-set files, and projected nine authored Nether ground structures onto the terrain heightmap. Some ruins/bridges remain aerial.
- **Overworld:** Sky Garden and four other aerial structures raised **+45 blocks relative to RC15**, with increased Sky Garden frequency and another bounded map structure.
- **End:** five additional asymmetric island silhouettes, another bounded map structure, and more frequent End landmarks. Vanilla End biome names remain in use.
- **Loot:** 640 storage containers in 257 templates revised to carry 3–8 stacks selected according to dimension and structure category. The biggest old TREK templates retain their original RC15 inventories. Pre-filled NBT does not adapt to biome at runtime.

## Verified locally

- ZIP integrity and CRC: PASS
- JSON / pack metadata: 1,332 parsed
- NBT structures: 264 changed/new parsed and positions/palette indices validated; 735 other NBT files inherited byte-identically from RC15
- Self-referenced structure/pool/template targets missing: **0**
- ZIP: 6,747,176 bytes
- SHA-256: `b5615e4eeedc459af81281c02fa5596ee03618af2a10a8dd9d7188da642f4e80`

## Not verified or not possible via this datapack

- **In-game Horizon worldgen test not run.** Actual terrain forms, generated chest persistence, collision-free structures, high-altitude generation and observed spawn chances remain unverified.
- The current Horizon worldgen docs say newly introduced datapack biome identifiers are ignored until they have native IDs/visual properties. Therefore **custom biome names in F3 cannot be guaranteed** without a Horizon engine change. Biome distribution instead adjusts the existing Nether biome IDs.
- Nether portal arrival position is handled by the game engine; the datapack cannot guarantee a surface spawn.
- Additional legacy 1.11/1.12 user-provided map archives are not imported without block-state and orientation migrations; not all supplied assets are included.
- Previously generated chunks are unchanged; start a NEW test world and disable old Thalvoryn ZIPs.

## Distribution

This experimental test build contains user-supplied third-party map/schematic content. The archive **has not been publicly uploaded to GitHub**, because its redistribution rights are unverified and the connected GitHub actions do not provide release-asset binary upload. Confirm author permission before publishing the ZIP.

Technical source references: Horizon's [world-generation documentation](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md) and [Minecraft Java 26.3 notes](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3).
