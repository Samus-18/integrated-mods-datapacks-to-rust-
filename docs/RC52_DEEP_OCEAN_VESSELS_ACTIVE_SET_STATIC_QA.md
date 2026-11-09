# RC52 — Deep-ocean vessel diversity and active structure-set correction

**Date:** 9 Nov 2026. **Status:** Private build with independent static QA PASS; **NO Horizon/MC runtime testing, NOT STABLE**. Target Horizon Rust / Minecraft 26.3 / Nether 1024.

## Source-level bug found in RC51

The RC51 patch tightened `trek:overworld/medium/ship_pillager` to deep-ocean biomes, but this structure **did not have any active structure-set reference** and therefore the change could not activate natural generation. In contrast, the huge `trek:overworld/medium/birch_trade` ship had an *active dedicated* set with spacing 20/frequency 0.85, and its NBT footprint measures **80×68×25** with over **127,000 literal air blocks**. That large model has a severe potential to overwrite coastal terrain; a single deep-ocean biome check cannot verify its full 80-block length.

The shared TREK `ecology_mix_rc36` random-spread set contained 86 weighted entries including two independent water ships (`dark_oak_trade` and `jungle_trade`), but did not contain `ship_pillager`. The original `cold_red_trade` and `small_red_trade` models also lacked a natural-generation structure-set reference.

## What was actually changed

1. **One new custom structure set** `thalvoryn:overworld/rc52_deep_ocean_vessels`: five differently shaped, pre-existing TREK ships, equal weight, `spacing=36`, `separation=16`, `frequency=0.72`, and an exclusion zone of 5 chunks against the existing `trek:overworld/ecology_mix_rc36` set. This reduces independent competing water-ship placement grids and provides three previously unspawned vessel types with a valid source: `cold_red_trade`, `small_red_trade`, `ship_pillager`.
2. Removed **two active old ship entries only** from `ecology_mix_rc36` (86→84); all its normal trees, ruins, villages and land structures are preserved.
3. Updated **four biome eligibility lists**:
   - `cold_red_trade`, `dark_oak_trade`: deep cold / deep default ocean;
   - `jungle_trade`, `small_red_trade`: deep lukewarm ocean;
   - `ship_pillager`: original deep non-frozen biome tag retained.
4. Deleted **only the old dedicated random structure-set JSON** for the 80×68×25 `birch_trade` mega-ship. Its NBT, jigsaw pool, structure JSON and all inventory contents remain available for manual placement and future robust whole-hull validation. No imported map source was deleted.
5. `pack.mcmeta` description updated to explicit RC52 private build; one README added. The update is **one replacement ZIP**, not an additive datapack.

### Five smaller ships (actual source NBT sizes)

| Template | X × Y × Z |
| --- | --- |
| cold_red_trade | 41 × 39 × 13 |
| dark_oak_trade | 41 × 39 × 19 |
| jungle_trade | 20 × 22 × 11 |
| small_red_trade | 21 × 15 × 11 |
| ship_pillager | 21 × 22 × 13 |

## Independent verification

- **Input RC51 SHA256:** `4206c1043d746b8126e661837917952ed1ebecaa7a14e96e6f16a75ea6c982c5`
- **RC52 SHA256:** `89d65425476ac31d2614732048f71660854e311ca2f2b3d4b0b4ed9233eb9c63`; **10,207,354 bytes; 3,156 ZIP entries**
- **Diff:** 6 existing files modified (four ship structure JSONs, ecology mix structure-set JSON, `pack.mcmeta`); 2 added (one new set + README), 1 removed (only obsolete giant-ship set JSON).
- Independent Python validator opened both source and output ZIP, ZIP CRC PASS, parsed **1,707/1,707 JSON and metadata files** with duplicate-key rejection; **1,378/1,378 pre-existing NBT byte-identical** and the icon `pack.png` unchanged.
- All 5 ship `start_pool` entries, referenced NBT templates, NBT local dimensions/positions, exact active-set references and deep-ocean biome eligibility independently verified. **36 structure sets** after the replacement. No exclusion graph cycles. The giant ship's NBT and pool remain intact but it is not naturally active.
- Dimension / terrain / density functions and unrelated loot / trees / caves unchanged. Existing data included from RC45–RC51 remains.
- All private deliverables, deterministic builder, **separately written independent validation script**, JSON QA and Czech README stored in ChatGPT Library **`/Horizon/Thalvoryn/RC52`**. No binary public release: rights/attribution of imported NBT map content remain unresolved.

## Honest limits

- **This does not prove full-hull support in water or safety near coastlines**: the jigsaw `biomes` list checks a start position, not every block of a rotating ship; the engine needs a footprint/water-depth test.
- `exclusion_zone` prevents near starts relative to one other structure set, **not all structures or foliage**. The choice of ships and biome eligibility may cause some candidates to fail and lower the effective actual density.
- No live Horizon, Java 26.3 server, renderer, LOD, FPS/TPS or portal test was executed. Old chunks remain unchanged.
- This is not completion of Overworld vegetation/fire/large structure collision control, Nether ship full support or the native WGSL Photon port. All old requirements remain tracked in [full audit](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md).

**Rough full-project scope estimate: ~48% complete / ~52% remaining.** This is an engineering estimate, **not** a game test pass metric. No user testing requested (the user is unable to test now).

**Next priority:** actual engine-side full-footprint validator for large water/ground/End structures; then realistic cave reachability and LOD/Nether portal, followed by full native WGSL Photon and final Java/Rust packaging. Keep [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) open.