# Thalvoryn V74 — reproducible private test and regression QA

**Status:** PASS_STATIC_ONLY (all 4 builds). **NOT** Minecraft/Horizon in-game verified; not a final error-free release.

## Build ancestry / reproducibility
The previous V73 *ZIP archives* were unavailable in this execution session. V74 is therefore rebuilt from the archived V72 ZIPs, reapplying the demonstrated V73 container repair, plus the changes below. The original source archives remain intact.

**Four private variants**: RUST 1024, RUST 2032, JAVA 1024, JAVA 2032, all on Minecraft 26.3 pack format 121. They are distributed in the related private ChatGPT conversation; third-party structure binaries are **not** publicly uploaded pending license review.

## Changes compared to V72
- Recreated the prefab container fix in **47 NBT files**: converted **83 filled chests/trapped chests** to thematically appropriate vanilla loot; converted **60 barrels** to four sparse randomized loot tables; cleared **234 decorative barrels**. Preserved **9 functional dispenser/brewing inventories**.
- Reduced the compound `pointed_dripstone` placed-feature attempt maximum from **96 × 5 = 480** to **96 × 2 = 192** per invocation. **Not a measured TPS improvement**.
- Added limited collision-proximity guards for RUST groups `ow_small`, `ow_settlements`, `ow_surface_dungeons`, `nether_minor`, and `end_minor`. JAVA gets a vanilla-compatible `ow_small` exclusion against settlements. These constrain candidate start chunks, **not complete structure bounding boxes**.
- Updated `pack.mcmeta` and `/function thalvoryn:version` to **V74**.

## Automated validation
- Independent pack audit: **4/4 PASS_STATIC_ONLY**, ZIP CRC, JSON, registry references, placement ranges and jigsaw start pools.
- **6,835 NBT templates** decompressed in the deep audit; **0 missing active jigsaw connector pools**, **0 missing referenced own loot tables**.
- Independent per-template comparison: **47 modified NBT** have unchanged geometry, block states, palettes, positions and non-storage mechanisms.
- Cross-variant verification: NBT and loot are identical across all four; Overworld noise/density and the selected End generator resources are identical; the Nether height matches each build. Four Nether density functions appropriately differ between 1024 and 2032.
- Regression QA: a *deliberately corrupted copy* of V74 with one hard-coded filled barrel is detected and **FAILS**; the four intended builds pass.
- **140 missing connector pool names remain in inactive source templates**. They were not fabricated as empty replacements; these assets must stay inactive.
- Custom ships in End / large structure-to-slope placement / FPS/TPS / natural cave intersection **not proven in game**.

## Repeatable offline QA
The private toolkit includes Python 3.10+ standard-library files: `audit.py`, `nbt_codec_v67.py`, `qa_gate_v74.py`, `RUN_QA_WINDOWS.cmd`, and a README. Copy the toolkit next to the four ZIPs and run the Windows command; it writes `QA_GATE_V74.json`.

This repository note deliberately contains no unlicensed copied structure NBT. Public redistribution requires evaluating source licenses.

**Known blocker to a final guarantee:** There was no running Minecraft 26.3/Horizon instance available for loading worlds, observing structure collisions, opening loot containers, measuring TPS/FPS or testing portals. No static check can establish zero in-game errors.