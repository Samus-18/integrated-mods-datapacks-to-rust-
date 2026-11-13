# Thalvoryn V75 — engineering QA and targeted fixes

**Minecraft target:** 26.3 / datapack format 121.  
**Builds:** Horizon RUST + Minecraft JAVA, each Nether 1024 and 2032.  
**Status:** `PASS_STATIC_ONLY` for all four ZIPs. **Not an error-free or in-game-verified final release.**

## Fixes from V74

- Six large imported Overworld buildings still lacked terrain start checks despite previous rounds of fixes. **RUST** now uses `horizon:start_checks` with terrain radius/range and a `liquid: top` check:
  - `buildings_01`: width 112, radius 4 chunks, terrain range 36 blocks
  - `buildings_02` and `buildings_03`: width 96, radius 4, range 34
  - `medieval_01`: width 78, radius 3, range 32
  - `buildings_08`: width 54, radius 2, range 30
  - `buildings_09`: width 62, radius 3, range 30
- Fixed stale **V72** version text in `/function thalvoryn:check`, `help`, `version`, and `horizon/status`.
- Fixed incorrect Nether height shown by the V74 status command in 2032-high builds: it now reports Y 0–2031 rather than 0–1023.
- JAVA's generator rules stay identical to V74: Horizon-specific terrain checks are not available through the vanilla datapack system.

## Independent regressions and test evidence

- Four archives: **4/4 PASS_STATIC_ONLY**, 11,240 ZIP entries per archive.
- Against actual V74 ZIPs: exactly **11 modified entries in RUST**, **5 in JAVA**. All structure NBT, loot tables, dimension definitions and noise settings are byte-for-byte unchanged.
- **246 biome files:** no feature ordering cycles or duplicate feature placement.
- **25 structure sets:** no cyclic exclusion dependencies.
- Deep scan of RUST/1024: **6,835 NBT files decompressed**, **5,901 smaller fully parsed**, **934 larger compressed templates inspected via fast binary scans**. No errors in the scanner's supported checks.
- Checked dimensions of **37 ocean-group starting templates**. The existing 33 boat structures have size-appropriate RUST biome radius; no unjustified edits made.
- Four negative QA control tests prove rejection of a deliberately stale version, wrong Nether status height, feature-order cycle and structure-exclusion cycle.
- The packaged V75 standalone QA toolkit was extracted and executed separately: **4/4 PASS_STATIC_ONLY**.

The private ChatGPT conversation includes the four V75 ZIPs, reproducible build script, complete QA JSON, and Windows `RUN_V75_QA_WINDOWS.cmd` toolkit. Binary imported structures are not republished on this public branch until source licensing is confirmed.

## Important limitations

Static checks do **not** prove every placed structure rests on terrain, no structure overlaps, natural caves always connect to tunnel outlets, performance/TPS improves, or portals/loot work in a running client. Testing requires a **new world** with only one V75 ZIP active. Old explored chunks never regenerate when replacing the pack.

The local environment has Java **21**, while Minecraft Java 26.3 requires Java **25** and an actual game/Horizon runtime; we did not run gameplay.