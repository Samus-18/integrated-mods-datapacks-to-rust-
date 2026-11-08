# Thalvoryn RC26 — Horizon Rust 26.3, Nether 1024

**Status: private experimental test build. Not verified in Horizon runtime and not publicly released.** Source: RC25; user-reported problems are preserved in [GitHub issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

## RC25 → RC26: actual changes
- **Five genuinely redesigned original End NBT island shapes** replace `thalvoryn:end/archipelago/asymmetric_1..5`: crescent, branching double-lobe, dark volcano-like plateau, split ravine and fragmented cluster. Five distinct materials/palettes. New geometries: **12,587 / 18,656 / 15,438 / 22,496 / 15,895 blocks**.
- Island selection favors those five alternatives over older repetitive islands (weight 5 vs 1), but lowers the overall archipelago frequency to 0.44 on a 58/20 chunk grid. Sky airships now choose eight ships on a much rarer 115/43 grid at 0.19; small variant obelisks and groves are rarer.
- **Overworld maritime structures:** 16 retained smaller imported ship templates are eligible only in non-frozen deep-ocean biomes (deep ocean, deep cold, deep lukewarm) with start Y65, shared grid spacing 104, separation 45, frequency 0.27. This **cannot guarantee water under the entire ship**; it is a conservative biome approximation.
- **Imported buildings** in Overworld and End: two natural-spawn sets removed rather than allowing ungrounded structures. Their NBT, structure definitions and pools are retained for a future full-footprint slope check.
- **Nether:** Crimson camp, Warped garden, Basalt outpost and Soul waystation now each have the matching biome constraint and share a weighted group with their biome's existing watchtower. Four redundant independently scheduled site groups were removed. Other large Nether ruins are sparser. Several excessive blackstone/basalt surface accents were reduced to more selective noise thresholds.
- **Not changed:** RC25 chest/barrel inventories, mob spawn overrides, dimension heights, world noise generation apart from the specified Nether material rule, Sky Garden altitude, native engine, Photon shader.

## Verification
- ZIP CRC passed, **2,712 files** (including **1,524** JSON/pack metadata, **1,102** gzip NBT templates, **298** structure definitions, **50** active structure sets, **563** template pools).
- All five generated NBT templates parsed, with **85,072 total unique block coordinates**, legal palette indices, dimensions and block states, no out-of-bounds positions.
- All 1,097 other NBT templates retained **byte-identically** from RC25.
- Static self-reference traversal: no missing *owned* structure / pool / NBT targets. Built-in Minecraft resources may be inherited from game registries.
- Dimension JSON: Overworld `min_y=-64, height=2096` (maximum build Y2031); Nether `min_y=0,height=1024`.
- ZIP size: **8,902,693 B**; SHA-256: `118b29e9869c93d343663a609169604a18b2659d107e238dee88bfa709839ca4`.

## Unresolved: do not claim done
- **No Horizon in-game validation**. Existing world chunks will not regenerate; make a fresh world and enable only RC26.
- True boat placement requires engine/native-mod predicates on depth and water/lava throughout a hull's full bounding box. Nether imported lava ships remain disabled rather than spawning on terrain.
- True building grounding requires multi-point terrain slope and footprint validation; risky imported buildings were deliberately suspended.
- Portal safe exit is engine behavior; datapack does not change it.
- The observed apparent vanilla world height needs the **actual loaded world dimension type** checked. A separate read-only Python script `Horizon_World_Height_Diagnostic_RC26.py` is available in the user's chat to inspect ZIP + world `level.dat` + `datapacks` + `latest.log`. Pack height numbers alone are not evidence that the active game loaded them.
- F3 custom biome IDs, native Photon port, and full ecological End terrain/vegetation generation remain incomplete. RC26 only replaces five End NBT islands; it does not claim to replace native End noise.
- Third-party map and structure redistribution rights remain unverified. **Binary ZIP is a private ChatGPT test artifact, not a public GitHub Release.**

Horizon references: [world generation](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md), [world height](https://github.com/horizon-rust/horizon/blob/main/docs/world-height.md), [mods](https://github.com/horizon-rust/horizon/blob/main/docs/mods.md).
