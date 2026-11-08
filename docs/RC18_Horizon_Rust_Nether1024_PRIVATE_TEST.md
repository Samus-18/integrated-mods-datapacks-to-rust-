# Thalvoryn RC18 — Horizon Rust 26.3 / Nether 1024

**Experimental private test build; static validation only. Not a public release, and not run in Horizon.**

## Changes since RC17

- **78 imported intact bounded structures** from user-provided archives: **61 ships** and **17 modern-map buildings**; **27 Overworld**, **26 Nether**, **25 End**. Three clipped or tiny building-map components rejected.
- Three new weighted structure sets group these additions by dimension rather than creating 78 unrelated high-frequency spawn grids.
- **134 structure-specific monster spawn override profiles**, restricted to structure piece bounds; these are rules for natural mobs, not guaranteed pre-placed entities. Mobs include Overworld skeletons/zombies, Nether piglins, End endermen/endermites and rare shulker profiles.
- **191 more TREK storage inventories** rebalanced in **32 templates**, preserving pre-existing larger custom inventories where possible.
- **5** End imported structures projected onto the surface; **4** Nether surface-site sets made more frequent.
- Nether dimension height **1024**, density and ore configurations, existing Nether biome remapping, End island generation and Overworld sky height settings inherited **unchanged** from RC16/RC17.

## Verification

- ZIP CRC and integrity: pass.
- JSON and pack metadata: parsed.
- Added files: 238 (78 NBT, 159 JSON, 1 README); changed old files: 93; removed: 0.
- Changed/new NBT parsed with valid palette indexes and block position bounds: **110**; unchanged NBT byte-inherited from RC17.
- Owned structure set / structure / pool / NBT reference links: no missing targets found.
- ZIP: `Thalvoryn_Horizon_Rust_RC18_Nether1024_PRIVATE_TEST.zip`
- Size: **8,886,583 bytes**.
- SHA-256: `6a5ba20dc6c715dec8aaf4ec54dd93fd828f2bee6d3f25d620d1a5b1a028d18f`

## Unresolved and not claimed

- **No game runtime or Windows native compile test.** Structure rarity, biome behavior, geometry intersecting terrain/vegetation, loot persistence and actual mob spawns require play testing.
- Custom biome F3 names and guaranteed surface-safe Nether portal arrival cannot be fully guaranteed by this datapack; they require Horizon engine support.
- Photon shader: separate task, **not a verified native Horizon/WGSL conversion**. No claims of all-effects parity.
- Existing RC16 Nether terrain cutoff change is preserved but the user screenshot's reported issue has not been verified resolved in-game.
- Older map entities, signs, item frames, mobs and complex tile data are not all converted.
- The sources include third-party maps with **unverified redistribution permissions**. RC18 ZIP **was not uploaded publicly to GitHub**. Available only as the private test attachment in the conversation.

Use a **fresh world** with **only RC18** enabled, not alongside RC17 and earlier. Existing generated chunks retain their prior terrain and structures.
