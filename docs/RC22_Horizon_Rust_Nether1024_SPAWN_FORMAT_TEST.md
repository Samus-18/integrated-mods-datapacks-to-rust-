# Thalvoryn RC22 — Horizon Rust 26.3 / Nether 1024 / mob-spawn format fix

**Development-only test ZIP. Static validation PASSED; gameplay NOT tested. Not a public GitHub asset release.**

## Root cause and fix

The RC21 datapack contained **268** structure spawn entries using the removed pre-26.3 **`minCount`/`maxCount`** keys across **134** structure definitions. Minecraft Java 26.3 / Horizon expects a **`count` int provider** in structure `spawn_overrides`. These old records can prevent valid structure decoding, explaining why additional structures could fail to generate even though their JSON and NBT files existed.

RC22 converts all 268 entries without changing entity type, spawn category, weight, target structure or bounding-box scope:
- Equal minimum/maximum -> `{"type":"minecraft:constant","value":N}`.
- A range -> `{"type":"minecraft:uniform","min_inclusive":N,"max_inclusive":M}`.

Four older uniform provider objects in two other structures carried extraneous `value` keys; these were normalized as well. In total **136 structure definition JSON files** changed. No other structure geometry, frequency or world terrain data was changed.

## Static verification

- Output: `Thalvoryn_Horizon_Rust_RC22_Nether1024_SPAWN_FORMAT_TEST.zip`
- Size: **8,897,725 bytes**.
- SHA-256: `01e7824fc85c32608784ff96a5a1f0df67a15ced81e86ec3dfb442244df4568c`.
- ZIP testzip / CRC: PASS.
- JSON + pack metadata parsed: **1,581**.
- Structure definitions: **298**; spawn entries codec-checked: **312**.
- Deprecated spawn-count fields remaining: **0**; invalid count providers: **0**.
- Structure sets: **107**; start-pool references **295**; template references **991**; missing custom targets **0**.
- **All 1,102 NBT files byte-for-byte identical to RC21**, including their chests and barrels.
- Nether world height **1,024**, noise settings, biome configuration, End terrain, ores and structure spacing **unchanged**.

## Remaining work

This is a **codec-level** fix only. The user-reported Nether terrain cutoff, real End/Nether structure rarity, surface spawn safety, prefab terrain collisions, loot and mobs still need **a clean-world Horizon in-game test**. Photon is a separate unfinished native-shader task. New F3 biome IDs and safe arrival at Nether portals need engine support. Existing chunks will not regenerate; test in a new world with only RC22 enabled.

The binary contains third-party source-map assets whose redistribution rights have not been verified. It has **not** been uploaded as a public GitHub Release.

### Reference

Horizon datapack migration documentation: https://github.com/horizon-rust/horizon/blob/main/docs/datapack-upgrade.md
