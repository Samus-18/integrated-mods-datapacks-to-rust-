# Thalvoryn consolidated engine + native WGSL source batch (9 November 2026)

**State: private, static-QA only. Not compiled, not an in-game-tested release, not a complete Photon 1.4 port.**

The user rightly pointed out that repeated minor datapack RC ZIPs did not finish the large outstanding work. This consolidated batch makes actual **source-level engine and shader progress** without relabeling the existing worldgen ZIPs as a finished release.

## Source-level Horizon height fixes (actual code in this repository)

- [Backup-first source patch script](../tools/thalvoryn/patch_horizon_tall_worlds.py)
- [16 automated Python tests](../tools/thalvoryn/test_tall_worlds.py)
- Target Nightly commit: **horizon-rust/horizon `2ebf00a`**, with exact old-source occurrence counts verified by directly fetching the pinned files:
  - `crates/horizon-client/src/game/nether_portal.rs`: two hardcoded Nether/Overworld search section bounds `0..15`/`-4..19`, one hardcoded portal creation logical height `128`/`384`
  - `crates/horizon-client/src/game/raids.rs`: one hardcoded surface-height ceiling `127`/`319` in `surface_height`
- The patch replaces these with tall-dimension ranges sourced from actual `world.bottom_y()` and `world.top_y()`, keeping the two original **vanilla** dimension profiles unchanged. Its logical-height calculation is tailored to Thalvoryn dimension types (their `logical_height` equals their physical height).
- Tests cover vanilla Nether, vanilla Overworld, Nether1024, Nether2032 and tall Overworld2096 plus mismatches, backup-first source writes, dry runs and repeat protection. **16/16 PASS for Python source/text/arithmetic tests. NO Rust `cargo` build and NO portal runtime test.**

These source files are not a compiled EXE and are not an engine installation. To apply they require a checkout of Horizon and a subsequent rebuild. The patcher refuses unknown source changes and does not overwrite a backup.

## Original native Horizon WGSL experiment, expanded beyond old preview

A newly-authored experimental Horizon API 0.1 shader contains **five** native WGSL stages: `sky`, `material`, `weather`, `post_bloom` and `post_grade`. It declares **14 selectable options** with shadow toggling/resolution and light, night, water, rain, exposure, color/vignette and End controls. The material code references Horizon's built-in dynamic shadow depth map, optional water reflection and LabPBR maps.

**This is original code, not the full port of source Photon 1.4's 468 GLSL files.** Features such as original Photon cloud algorithm, all volumetric lighting, SSR, TAA, original PBR/shadow parity and shaders across all visual pipelines are still missing. It was validated only with a TOML parser, stage signature checks, resource names matched against pinned `docs/api/shaders-reference.md`, and basic WGSL structural checks. **The actual `horizon-shader check`/Naga and GPU runtime were unavailable and NOT claimed PASS.**

Do not conflate this shader candidate with the original `photon_v1.4-RUST.zip`, which was a byte-identical GLSL pack and **not native WGSL**.

## Preserved worldgen and independently repeated QA

The private combined archive includes the unchanged **four RC59 datapacks**:
- JAVA Nether1024 and Nether2032
- RUST Nether1024 and Nether2032

From a physical RC60 handoff, their ZIP CRC, duplicate filename checks, **1707 JSON/meta**, **1378 source NBT** per build, JSON duplicate keys, `pack_format=121`, Nether height/noise match for 1024/2032, Overworld 2096 and byte-identical NBT content across all variants were rechecked.

The older native Windows Converter v7.2 is included unchanged. The safe LOD-setting script from RC60 is carried forward, **not** a fix to Horizon's black-screen or white stripe renderer defect.

## Deliverable, limitations and next milestone

**Private combined file:** `Thalvoryn_HORIZON_BULK_ENGINE_NATIVE_SHADER_SOURCE_AUDIT.zip`  
**SHA256:** `ce00103736d54783f726011c5b42e94de81d462a41fbee0f505f01aae322d0fb`  
Size approx. **42.2 MB**, archived in private Library `/Horizon/Thalvoryn/ENGINE_SHADER_BULK_SOURCE`; contains the four unchanged RC59 candidates, all scripts/tests, independent QA, native WGSL sources/package, older Converter and LOD safety tool. Proprietary imported-map NBT assets have **not** been put on public GitHub.

**Not solved:** actual Horizon compilation, Nether portal freeze/load black screen, LOD artifact and GPU performance, complete native Photon 1.4 parity, safe large-structure site placement, custom biome registry/F3 and gameplay/runtime testing of two height variants. Thus there is **no defensible 60% or 100% claim**. User requested updates in 10% completion milestones; the prior 50% estimate is still the last stated milestone, not a formal engineering score.

**No user testing required now.** Issue #2 stays open pending compiled client and actual acceptance tests.