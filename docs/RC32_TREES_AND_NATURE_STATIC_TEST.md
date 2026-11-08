# RC32 – Stromy a příroda / Trees and Nature

**Private Horizon Rust 26.3 test build · 2026-11-08. Not tested in the actual game.**

Implements user requirements, part 2:

> 🌲 Stromy a příroda
>
> Využít všechny vhodné vlastní stromy z tvých schémat, nové druhy lesů, přirozené rozmístění vegetace
>
> samozřejmě že vždy kontrola před a po jestli se něco nerozbilo

Input: `Thalvoryn_Horizon_Rust_RC31_OVERWORLD_SKY_CAVES_ICON_PRIVATE_TEST.zip`; preserves its Sky Islands, high-mountain and cave changes, original `pack.png`, and all dimension settings.

## Finding before modification

The previous pack **already contained 48 individual NBT tree templates**, extracted from the supplied `50-autumn-trees-ptdrv (1).schem`; they were assigned exactly once across four grove template pools (12 per pool). All 48 NBT templates contained native Minecraft block IDs and valid block coordinates. **However**, every group's tree selection used nearly the same oak trunk / cherry / azalea foliage palette even in taiga and savanna. The original large schematic also contains custom `red_poplar_leaves` block IDs that cannot simply be used as native vanilla/Horizon blocks without registration. No additional unvetted schematic components were injected.

## RC32 changes

- **All 48 original tree NBT remain unchanged**, referenced in their original groups at low weight; cloned **48 biome-fit versions** with the same dimensions, block positions, palette indices, entity data and other NBT, altering only native wood/foliage block IDs for each group.
- **12 additional darkwood-tree variants** use the geometry of 12 original birch-group trees with native dark oak wood/leaves. Total **60 new NBT**.
- Five mutually disjoint themed grove groups, with native biome ID restrictions and shared spacing *within each group*:
  - Birch woodland: birch forest and old-growth birch forest
  - High spruce forest: taiga / old-growth taigas / windswept forest / grove
  - Meadow woodland: plains / forest / flower forest / meadow / cherry grove
  - Acacia savanna: savanna / savanna plateau / windswept savanna
  - Darkwood: dark forest
- Each original group changed from spacing **26**, separation **9**, frequency **0.32** to spacing **24**, separation **9**, frequency **0.43** (rare single tree site per lattice cell, not high-density structure spam). Darkwood uses 28/11/frequency 0.36. These are attempts, not guaranteed tree counts.
- **7 native-block vegetation patches**: leaf litter, ferns, moss carpet, cornflower, short grass, pink petals, azalea. Each patch is a 26.3 inline `worldgen/feature/minecraft:simple_block` with `block_predicate_filter`: source block must be air, and grass/dirt/moss/podzol/etc. directly beneath.
- **3 additional native placed trees** for old-growth spruce, savanna acacia, meadow birch at low rates using existing vanilla feature IDs.
- Vegetation placements added in consistent order to **16 existing overridden vanilla biomes** (one biome `minecraft:grove` is inherited directly from the game and was not overridden). Small biome color changes in birch/dark/spruce/savanna variants. **No custom biome ID was added**; names in F3 remain vanilla biome names.
- Original pixel-art icon `pack.png` from RC31 retained byte-for-byte.

## Before and after validation

| Check | RC31 before | RC32 after |
|---|---:|---:|
| ZIP members | 2,732 | 2,813 |
| JSON + pack metadata | 1,535 | 1,555 |
| NBT structure templates | 1,106 | 1,166 |
| Original valid imported tree templates | 48 | 48, byte-identical |
| Biome-fit NBT copies | 0 | 48 |
| Darkwood NBT copies | 0 | 12 |
| Autumn grove groups | 4 | 5 |
| New natural ground patch features | 0 | 7 |
| New restrained native tree features | 0 | 3 |
| Updated biome overrides | – | 16 |

**Automated tests PASS:**
- Source ZIP and result ZIP CRC
- All result JSON parse
- All 60 new tree NBT complete independent big-endian decode
- Every copied tree's dimension, complete block list, states, position indices and entities identical to its source
- All 1,106 original NBT files preserved byte-identically
- Owned structure-set → structure → pool → NBT refs exist
- Valid grove spacing/separation and disjoint grove biome scopes
- New forest patches require matching air and native soil under the block
- Feature-decoration ordering graph across overridden biomes remains acyclic
- Nether, End, noise/density functions unchanged; **Overworld Y−64…2031, Nether Y0…1023, End Y0…511** retained
- `pack.png` byte-identical to RC31

Output: `Thalvoryn_Horizon_Rust_RC32_TREES_NATURE_Nether1024_PRIVATE_TEST.zip`  
Size: **9,003,896 bytes**  
SHA-256: `2f5b90a5afc577e1862521a1d908f174eca6bb0248d330a654875d8e63b7dbf3`

## Honest limitations

**No Horizon runtime test.** This does not prove vegetation will place in every target biome, that native placed feature ID names exist in the installed Nightly, or that jigsaw trees won't intersect other structures. Biome/heightmap filters are not a whole-footprint collision detector. Full biome registry identifiers in F3, improved distant LOD, and native Photon are engine-side future work. Do not combine RC32 with RC31 in one new world. Existing chunks won't regenerate.

The user-provided larger schematic contains unknown third-party creations; redistribution rights are not automatically granted by possession or conversion. The complete ZIP also includes other map assets with unverified licensing. Keep this build a private attachment, not a public GitHub release.

Track further user-provided parts at [USER_REQUIREMENTS_BY_PART.md](USER_REQUIREMENTS_BY_PART.md) and earlier failures at [Issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
