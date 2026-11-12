# Thalvoryn V73 — targeted loot repair and new full binary NBT QA

**Status: PRIVATE TEST, STATIC QA PASSED. No game/Horizon runtime testing has been performed.**

Based on V72. Four test archives: RUST/JAVA × Nether heights 1024/2032. ZIPs are delivered privately in the original ChatGPT conversation; third-party bundled assets are not republished here.

## Actual code/data changes

- Found **377 nonempty prefabricated containers** under `data/minecraft/structure` previously missed by the V72 audit (modified End City, pillager outposts, stronghold, woodland mansion).
- Rewrote block-entity metadata in **47 existing NBT templates**, with every original block ID, coordinate, palette and entity preserved.
- **83 chests/trapped chests** now refer to Minecraft 26.3 family-appropriate loot tables.
- Of **294 prefilled barrels**, **60** now use one of 4 sparse randomized tables and **234** have been cleared as decorative storage instead of identical duplicated items.
- **9 existing nonchest functional inventories** (dispensers/brewing stand) remain untouched.
- Four new loot tables: `thalvoryn:chests/v73_end_supplies`, `.../v73_raider_supplies`, `.../v73_stronghold_supplies`, `.../v73_mansion_supplies`.
- No other worldgen or structure-set edits from V72; no Stellarity included.

## Independent static tests

- **4/4 ZIPs PASS_STATIC_ONLY**, including CRC, all JSON, resources/loot references, pack metadata, jigsaw rules.
- NBT differential test proved 47 affected templates have **unchanged geometry, block positions, palettes, entities, and functional inventory blocks**.
- New fast binary scanner visited **all 6,835 NBT templates** and **14,888,746 block entries**: no palette-index violation, no block position outside its template.
- Regression control: four **V72** ZIPs correctly **FAIL** on 377 fixed containers apiece; **V73** passes.
- No claim of measured loot randomness inside a running game.

## Known limitations

In-game correctness still not established: TPS/FPS, terrain blending, overlapping structures, cave connections to procedurally generated cavities, Java-only restrictions, and Nether/End performance. The 140 broken connector names in **inactive** source templates remain isolated; do not re-enable them unchecked. Existing chunks do not regenerate. The imported source assets require license review before public redistribution.

A local Python 3.10+ QA toolkit (`v73_qa_gate.py`, `audit.py`, `nbt_codec_v67.py`, `RUN_QA_WINDOWS.cmd`) and full private build/test manifest accompany the four ZIP artifacts in ChatGPT.