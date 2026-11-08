# Thalvoryn RC21 — Horizon Rust 26.3, Nether 1024

**Status: experimental private test ZIP. Static checks only, NOT verified in a running Horizon game.**

RC21 continues the RC20 datapack and targets previously unreachable TREK structures, inefficient mixed-biome structure selection, and underfilled inventory data in large NBT templates.

## What actually changed

- **Overworld:** added two missing random-spread structure sets to make `trek:overworld/medium/birch_trade` and `trek:village/mushroom` eligible to generate. Their biome filters and original templates remain unchanged. Spacing/separation: 26/9 and 33/12 chunks respectively.
- **Nether:** split the nine-way `rc19_surface_sites` set into five broadly compatible surface sites and four independently placed biome-themed variants (Crimson, Warped, Basalt, Soul); themed sets each use spacing 26, separation 9. This prevents biome-incompatible selections from taking a generic surface-site slot.
- **End:** split the 14-way `rc19_diverse_islands` set into 11 broadly compatible islands and three highland-focused ruins; the new ruins set uses spacing 20, separation 7. The original island set remains spacing 14, separation 5.
- **Loot:** selected 103 pre-existing underfilled (or some empty) chest/barrel inventories in 15 previously skipped large NBT structure templates. Added 214 modest dimension/structure-themed item stacks while retaining existing items. Decorative empty storage remains mostly empty. **Vanilla Stronghold templates and their LootTable references were left unchanged.** This is deterministic, not biome-aware runtime loot generation.
- **Unchanged:** Nether height remains 1,024 blocks; RC20's density parameters, End terrain shapes, Sky Island height, ore generation and Photon shader were not altered.

## Static checks actually performed

- ZIP size: **8,894,886 bytes**, entries: **2,764**, CRC/integrity: **PASS**.
- SHA-256: `e2e9f73d9fadc686acdf0abf4b4e9d7e7c793d93b3643967dc07002d77bfeb1f`.
- **15 edited NBT templates** parsed and validated for palette indices, block-position bounds, and unique inventory slots in 0–26.
- **107** structure-set configurations checked for spacing/separation; **0 duplicate pack salts**.
- Custom structure-set → structure → template-pool → NBT references validated; **0 missing custom structure/pool/template targets**.
- JSON/pack metadata syntax parsed successfully.
- Two previously orphaned TREK structure definitions now have structure sets.

## Remaining limitations and warnings

- **No in-game Horizon Rust test has been performed.** A valid datapack does not prove actual structure frequency, terrain correctness, mob spawning, End island diversity, loot persistence, or collision avoidance.
- RC20 terrain and height fixes have not yet been confirmed to fix the user's screenshot-reported Nether cutoffs.
- New biome identifiers/F3 names and guaranteed safe surface arrival after Nether portal travel require Horizon engine support; this build does not implement those engine changes.
- Photon is not a native Horizon/WGSL shader port and was **not changed in RC21**.
- Not every map asset has been imported or fully migrated; older entities/rotation metadata may still require testing.
- The bundled map/schematic assets came from third parties and their public redistribution licenses have **not been verified**. The RC21 binary **has not been uploaded as a public GitHub Release**.

For gameplay testing: use a **new world**, enable **only RC21**, and disable older Thalvoryn packs. Existing chunks will not regenerate.

Main project documentation: https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md
