# RC46 — End high-sky silhouettes + guarded Nether detail (PRIVATE / static-only)

**9 Nov 2026**. This is another **genuine, reproducibly built** incremental update to the private Thalvoryn datapack. **NO Horizon runtime game test or native shader build has been executed. NOT a final release.**

## Source and scope

Base: `RC45_NETHER_SHIP_CLUSTER_PRIVATE_TEST.zip` SHA256 `e5b37baf9be48a4e0a4db21f260f79e242018f4183f8fc3d864b7c7920888a40`.

Private RC46 result: `Thalvoryn_Horizon_Rust_RC46_END_VARIETY_NETHER_DETAILS_PRIVATE_STATIC_TEST.zip`
- **SHA256:** `97241d3cd01932f15bcd97a4edd43c145aa2c376449245c6c1acd5c2807153e4`
- **Size:** 10,174,338 bytes
- **Files:** 3,151

## Actual changes

1. The extra `thalvoryn:end/high_sky_islands` placed feature (previously a single repeated vanilla `minecraft:end_island` shape with chance 1/36) now points to a weighted `minecraft:template` selector with **six independently modelled original NBT islets**: `crescent` (392 blocks), `terraced_mesa` (465), `twins` (469), `long_ridge` (438), `hollow_ring` (519) and `three_petals` (582). All shapes have distinct contiguous 2D footprints with native block palettes. Existing large End islands, the central dragon arena, six archipelago structures, original vegetation and End city rules remain untouched. This feature has rarity **1/48**, plus a 15×12×15 **centred air-volume** predicate (x/z −7..7 and y 0..11) to avoid obvious interference. As this is a placement filter, **not** an engine-wide collision solver, it may reject many candidates; actual placement is not proven.
2. Two existing Nether biome JSONs get rare native environment-guarded detail: Nether Wastes and Basalt Deltas receive sparse surface `shroomlight` only on allowed air/solid substrate; Nether Wastes also receives small `blackstone` seams replacing **only** netherrack (ore feature). No changes to Nether biome IDs, native mob rules, dimension 1024 height, original flora, lava skiffs, other structure sets or noise.
3. `pack.mcmeta` description corrected from stale RC44 to an explicit `RC46 PRIVATE STATIC QA` marker; `pack_format=121` retained. Private readme added.

### Strict pre/post diff

Exactly **4 original files modified** (pack.mcmeta, 2 Nether biome JSONs, one End high-sky placed feature), **11 added** (6 original NBT islets, 2 configured features, 2 placed features, 1 readme), **0 removed**.

- ZIP CRC PASS.
- **1,708 JSON/pack metadata parsed**, all `thalvoryn` custom biome references resolve; independent run tested **136 namespace references**.
- **1,372/1,372 previously supplied structure NBT byte-identical**, including loot and trees.
- All **six new compressed NBT** decoded with a separate generic binary reader: palettes, positions, no duplicate block locations, connected footprint, matching namespaced native block IDs, sizes (15×12×15).
- Pack PNG, original worldgen noise/density, structure sets, template pools, old loot, and dimension types byte-identical.
- Dimensions remain **Overworld −64..2031, Nether 0..1023, End 0..511**.
- Two independent source/test programs and their JSON reports are saved privately under **ChatGPT Library → /Horizon/Thalvoryn/RC46**, alongside the private ZIP. This is NOT a public binary release: user-supplied imported NBT map rights have not been established.

## Important honest limitations

**No native Horizon execution**: we do not know if the new `minecraft:template` + `volume_match` combination succeeds in real generation, and the 15×12×15 air guard may make islands very rare. No proof of all cave shapes, Nether 3D placement, FPS/TPS, safe portal arrival, real End 512 natural noise, or native F3 custom biome IDs. Already generated chunks never regenerate merely because a pack changes. No claim of stable. No request for user tests now (user explicitly cannot test at present).

### Photon and Converter reality check

- Private Library's `photon_v1.4-RUST.zip` has **the same SHA256** `da0929b38f5fd99befaaa8cd141cbf60e1fad05b92d916d0eeeee7c79f86c0fd` as `Photon_v1.4_ALL_EFFECTS.zip`; it has 645 entries, 468 legacy GLSL shaders and **zero native WGSL files**. Therefore it is **not** a true Horizon WGSL port, regardless of the filename.
- An **original native WGSL preview** has now been authored with 3 Horizon shader API 0.1 stages (sky, material, post), six options, low-cost bloom, warm sky, terrain/fluids and End post tint. It is **explicitly not full Photon v1.4 visual parity** and **has not passed the `horizon-shader check` (Rust/naga unavailable)**. The source is kept private under `/Horizon/Photon`; **do not label it a finished Photon shader**.
- A prior Windows executable **Horizon Converter Suite Native v7.2** was recovered in the user's private Library, with 28 automated tests reported PASS and cross-compiled Win64 PE, but **actual Windows GUI runtime not verified**. Do not claim this task is unstarted.

Reference: [Horizon-native shader docs](https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md) and [Mojang 26.3 features](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3).

## Still-open work (including the old 10-part requirements)

- Overworld mid-Y caves/ugly cut walls, proper surface entrances, accessible underground village and cave-biome diversity.
- Whole-footprint site checks for large boats/buildings across dimensions, vanilla tree/fire/lava clearance near wooden homes, natural water/Nether ocean ships actually safe, no locally clustered repeat structures.
- Large natural End biome/topography diversity and true 512-height terrain; Nether multi-altitude small/medium discoveries without frequent same-template clusters.
- Baked vs genuinely randomized loot per structure copy, location/rarity balancing, custom mob ID & vanilla highland structures.
- Native Horizon Rust engine LOD, portal forcer, biome registry/F3, performance and **true native Photon shader**, Java/Rust 1k/2k validated outputs. None of these is closed just because RC46 parses.

**Next when the user is unavailable:** work offline from the repo/archives and continue static/CI tests, without asking for screenshots, claiming autonomous background work, or replacing their saves.