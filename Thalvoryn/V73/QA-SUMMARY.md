# Thalvoryn V73 — independent QA results (November 12, 2026)

## Reproducible findings
- Source: 4 × V72 archived packs, MC/Horizon target 26.3.
- The previous audit missed **377 nonempty prefab chests, trapped chests and barrels per variant**, all under `data/minecraft/structure`.
- Changed **47** matching NBT templates per variant (79 chests + 4 trapped chests -> family loot; 60 barrels -> sparse themed loot; 234 formerly filled decorative barrels -> empty).
- **9 preloaded functional inventories** (dispensers and brewing stand) are retained.
- NBT geometry, palettes, block states, coordinates, entity lists and unrelated files match their V72 baseline.
- ZIP static QA: **4/4 PASS_STATIC_ONLY**. 4 new loot table JSONs.
- Whole-template binary pass: **6,835 NBT, 14,888,746 block positions**, zero index/out-of-bounds failures. Two valid empty technical templates are not errors.
- Palette schema pass: **6,835 NBT, zero missing block IDs**. Minecraft 26.3 uses lowercase `id`/`properties`; the older uppercase `Name`/`Properties` is also supported. Both are valid. There was an initial false positive in a draft of the checker, corrected by reference to Horizon source `crates/horizon-world/src/worldgen/structure/template.rs`.
- Optional jigsaw diagnostic: **88,354 connector blocks**, 628 unmatched target names among actively reachable templates. Every such target pool has an empty fallback; some may intentionally terminate or be the already-used parent connector. This is **not proof of 628 broken connections**. We have not automatically edited any of them.
- Regression-control test: **V72 fails** because the 377 fixed inventories remained, **V73 passes**.

## Still not verified
No actual Minecraft/Horizon runtime, FPS/TPS measurement, biome/terrain geometry render, random natural cave connection, or real container opening. The static checker cannot guarantee these. Generated chunks do not update when replacing the datapack.

A private, copyable Python 3.10+ one-command Windows toolkit and the four private build ZIPs are attached to the original ChatGPT conversation. Source third-party structure assets aren't publicly redistributed here pending license review.