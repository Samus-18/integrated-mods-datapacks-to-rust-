# Thalvoryn V72 – static QA and chunk-generation performance caps

**Status: PRIVATE TEST, NOT IN-GAME VERIFIED.** Built from V71.

## Changes
13 placed-feature count overrides were lowered from 256 to 80–112 attempts per invocation, notably glow lichen, copper ore, cave vegetation, dripstone and sculk. All 6,835 NBT templates, loot tables, structure definitions/sets, biome definitions and noise settings remain unchanged. This is a plausible performance optimization, **not measured TPS/FPS**.

## Independent local QA
- Four ZIPs pass CRC, registry, feature ordering, reference, pack-format and cross-variant scans: **4/4 PASS_STATIC_ONLY**.
- Deep resource scan: **6,835 NBT decompressed, 5,900 smaller templates parsed fully, 935 large templates scanned without materializing all voxels**.
- **34,507 NBT loot references** inspected and **0 missing reachable connector pools**.
- **140 invalid connector references exist in inactive source templates**; they are not reachable from the currently active start-pool graph. Those files must not be re-enabled without further repair.
- Preloaded `Items` tags remain in some structures, including intentional bookshelves and traps. The static QA does not prove loot diversity in game.
- External vanilla registries, actual terrain placement, cave connections and TPS/FPS **have not been runtime-tested**.

For reproducible automated validation, see the standalone **V72_AUTO_QA_TOOLKIT.zip** supplied in the associated private chat artifacts. Place its `audit.py`, `nbt_codec_v67.py`, and `RUN_QA_WINDOWS.cmd` in the directory containing the four private V72 ZIPs, then run the CMD using Python 3.10+.

The source third-party structure assets and binary packs are intentionally not copied into this public repository pending license review.