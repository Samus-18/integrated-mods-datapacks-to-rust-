# RC47 — corrected template/footprint placement and ocean vessel safety

**Date:** 9 Nov 2026  
**Status:** Private static QA PASS, not a tested or stable Horizon Rust release.  
**Target:** MC 26.3, Horizon Nightly; no in-game or renderer test.

## Discovery in RC46

The previous custom `minecraft:template` placed-feature guards were written as though the NBT template's local origin was at the *centre* of the generated feature. But the actual compressed NBT files store their blocks in a **non-negative local coordinate box**:

- Nether lava-skiff: **13×12×17**, local X=0..12, Z=0..16. The old centred footprint check X=-6..6 and Z=-8..8 covered only **63/221 columns** of that local box if templates are origin-placed.
- New RC46 End sky islets: **15×12×15**, local X=0..14, Z=0..14. The old centered air-guard X/Z=-7..7 covered only **64/225 local columns**, assuming origin placement.

This mismatch was missed by older audits that checked only predicate dimensions, not their *alignment* to actual template coordinates.

## RC47 changes

1. **Six lava-skiff placement definitions** (crimson/warped, low/high and RC45 mixed low/high): shift local template origin with the `minecraft:offset` `x=-6, y=1, z=-8` after downward lava search. Footprint tests now check **all local X=0..12 and Z=0..16**, with **two lava layers below the hull**; the deck and mast air checks are equally shifted to the correct local coordinate frame. Rarity, 3D height ranges, scan count, template models and biome targeting are unchanged.
2. **RC46 End high-sky islets:** shift template origin `x=-7,y=0,z=-7`, and check an **entire 15×12×15 air volume in local X/Z=0..14, Y=0..11**. The six distinct shapes, their NBT data and the chance 1/48 are unmodified.
3. **Unsafe large imported ocean ships:** stop only the `thalvoryn:overworld/imported/maritime_rc23` **random structure set**. It selected 8 enormous imported ships at `start_height:absolute 68` with no `project_start_to_heightmap` and no whole-water-hull checks. The current Overworld noise settings specify sea level **63**, so placement rules were not convincingly anchored to the sea surface. **All eight structure definitions, start pools and NBT remain intact, manual-only for now**, rather than risking ships on shore or in air. This is a conservative disabling, **NOT a completed water-ship fix**. No other structure set is changed.
4. `pack.mcmeta` description updated to **RC47 PRIVATE STATIC QA**. A short warning README added.

## Exact pre/post verification

| Check | Verified |
| --- | --- |
| Base RC46 SHA256 | `97241d3cd01932f15bcd97a4edd43c145aa2c376449245c6c1acd5c2807153e4` |
| RC47 SHA256 | `39145a1534c10f126397b5b0c7425e6c249937905bb62f92dae8618dfac0d278` |
| RC47 file size | **10,174,514 bytes** |
| ZIP members | **3,151** |
| JSON / metadata parsed independently | **1,707** |
| Prior NBT byte-identical | **1,378 / 1,378** |
| Changed existing files | 8 (6 lava skiff JSON + 1 End island JSON + pack.mcmeta) |
| Removed | 1 unsafe ocean ship structure-set JSON |
| Added | 1 private README text file |
| Cross-check | ZIP CRC PASS, all custom biome refs and active structure refs present, all 8 imported ocean ship definitions and pools retained |
| Immutable sections | All dimension/height, biome, terrain/noise/density functions, feature sets unrelated to these placements, old loot/NBT, pack icon |

Two separate Python source/validator files and their reports have been retained with the private RC47 build, **ChatGPT Library → /Horizon/Thalvoryn/RC47**. The source validator decodes actual NBT sizes independently from the writing code.

## Crucial uncertainty and remaining work

The improved local-coordinate geometry is confirmed **statically**, but the exact *runtime origin-placement semantics* of `minecraft:template`, actual `volume_match` behavior in the Horizon Nightly codec, and cross-chunk checks are **not confirmed by running the game**. The old and new versions both remain experimental. In particular, this does not prove that every ocean/Nether/End structure is safely placed.

Do not ship the **private ZIP** on public GitHub until licensing of user-supplied imported NBT maps is resolved.

**Next offline targets:** Inspect and repair cave-entrance placed-feature coordinate/codec rules (notably 35-block offset probe), guard small End surface decorations, draft an engine-level ground/water/lava full-footprint site validator rather than spoofing a datapack-only implementation. Then review Overworld vegetation and wood-vs-lava safety buffers, cave/End height/noise, runtime LOD and portals, and the actual WGSL Photon port. No need to ask the user to test yet.
