# Thalvoryn RC17 — Horizon Rust 26.3 (Nether 1024, imported legacy map builds)

**Status:** private local experimental test ZIP. **In-game Horizon runtime has NOT been tested.** This is not a public release.

RC17 retains the RC16 Nether terrain/biome, Overworld sky, End and loot adjustments and imports *25 additional* individually bounded structures extracted from user-supplied maps:

- **12** vessels and structures from `ship-collection.zip` (legacy 1.11 format), distributed across Overworld, Nether and End.
- **1** large medieval assembly from `medievalbuildings.zip` (legacy 1.12 format), assigned to Overworld.
- **12** additional builds from `Buildings collection.rar` (modern Anvil world, DataVersion 4189), distributed across the three dimensions.

The old maps are not mods, and no additional game mod is required merely to read their source data. Import conversion translates a conservative set of legacy numeric block IDs and metadata into named modern states, including stair orientation, wooden variants, logs, doors, fence/pane neighbors, and storage blocks. Rejects unsupported IDs rather than inventing mappings. **Map entities, display entities, signs and some complex tile/entity properties are not fully migrated.** Some placements and physics may still require manual correction after in-game tests.

**Loot:** 57 selected storage blocks across 25 structures have 4–7 deterministic dimension-themed item stacks; for maps without original chests, a chest was added only when an empty spot over a solid floor was detected. Decorative storage is mostly left untouched.

## Static validation

- Archive ZIP CRC: PASS
- New NBT structure files: 25 parsed, block bounds and palette indices validated
- New worldgen JSON: 75 (three each: `structure_set`, `structure`, `template_pool`)
- New files: 101 (including README); existing RC16 files retained byte-for-byte except `pack.mcmeta`.
- 1,413 JSON and metadata files parse; 0 syntax errors.
- Total files: 2,514.
- Local test ZIP: `Thalvoryn_Horizon_Rust_RC17_Nether1024_LEGACY_MAPS_TEST.zip`
- SHA-256: `1492b9a80e1a8624b4399feaeb2a3222d1ecadc581daa153cf87fdab9e1fa58d`
- ZIP size: 8,120,993 bytes

## Not guaranteed

- **No Horizon in-game test**: structures might fail to spawn or have decorative/orientation or terrain-intersection issues. In particular, very large modern-map templates may stress generation.
- **Not all source-map builds imported.** Only 25 spatially bounded components selected; the archives contain many more constructions and pieces.
- Original legacy-world item-frame, mob, sign and tile-entity data are not preserved wholesale.
- Existing world chunks will not regenerate. Disable older Thalvoryn variants and test in a new world.
- **No public GitHub ZIP upload**: supplied third-party build redistribution rights remain unverified, and the connected GitHub interface has no binary release-asset upload action. Request original author permission before redistributing.

The imported structures are additional private test assets and do not imply a compatibility guarantee, confirmed rarity, or complete conversion of every user-provided asset.
