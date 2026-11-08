# RC43 – high-altitude caves, cave entrances, lava-guarded ships and End ecology

**2026-11-08 · Horizon Rust / Minecraft Java 26.3 · PRIVATE EXPERIMENTAL TEST.** Based directly on RC42, which itself was repaired based on the user's actual uploaded New World save. **Not runtime-tested, not stable.**

## Changes actually present in the ZIP

- **Overworld giant cave mitigation:** original `thunderfusion:overworld_overworld_combined` density was byte-preserved as `thunderfusion:overworld_overworld_combined_rc42_caves`. The active function now uses `minecraft:lerp`, from 0% original surface contribution at Y320 to 78% at Y760. The deep caves stay unchanged below Y320. This is a proposed response to the user's giant high-elevation hollow cliffs, **not a confirmed visual fix**.
- **Cave diversity:** Lush and Dripstone climate weirdness intervals split across 0. Deep Dark given a limited more accessible depth/erosion window. Four existing subterranean growth features extend upwards to Y320–340 while retaining air, soil and below-surface filters. Nine cave underground-room features have a higher attempt rate, as do two buried Nether relic features. Increased attempts **are not guaranteed generated structures**.
- **Two authored spiral cave access NBT** (`mossy_forest_descent` and `highland_stone_descent`, 13×39×13, each 871 stored blocks) have 32 consecutive descending positions with a physical floor and 3-block clearance, ending in a small lighted chamber. Ground and air sampling is center-adjusted to the 26.3 template placement. **Connection to a naturally open underground cave is not guaranteed**.
- **Experimental small Nether lava skiffs:** activate the original 13×12×17 fireproof crimson/warped boats via Minecraft 26.3 `minecraft:template`. Both templates specify `rotations: ["none"]`. Placed features scan down to lava, then apply `minecraft:volume_match` over the **centered full 13×17 footprint** for **two layers of lava beneath the hull** and additional air/deck/mast clearance. Four feature variations include Y37–62 low sea scans and rarer Y170–880 elevated lava searches; invalid ground / shallow lava fails the placement predicate. **This depends on full 26.3 implementation of `template`, `environment_scan`, and `volume_match` inside Horizon Rust and is NOT gameplay-tested.** It does not prove all ship placement issues are solved.
- **Three original native End microhabitats:** crystal crown in highlands, obsidian spires in barrens, purpur thicket in midlands/small End islands. Each is an original 9×11×9 template with centered soil/air checks and no new biome ID. **End natural noise height still uses vanilla 128, not full 512 terrain.**
- **RC42 baked loot and original files:** every one of **1,367 original NBT templates, including physical filled Items inventories, is byte-identical to RC42**, as are all existing loot-table JSON and the pixel-art `pack.png`. No new custom biome ID was added.

## Failure findings and fixes during RC43 QA

Initial RC43 candidate **failed independent codec and path tests** and was not issued as final: two `minecraft:offset` y values were outside the official ±16 limit; two `environment_scan.max_steps` exceeded the 1–32 range; cave spiral had two non-contiguous turns; default random template rotation did not match the tested hull footprint. All were corrected in the final RC43 ZIP before final QA.

## Final ZIP + verification

**`Thalvoryn_Horizon_Rust_RC43_CAVES_LAVA_PLACEMENT_PRIVATE_TEST.zip`**, **10,121,065 bytes**, **SHA256 `501117cdba3247c616b42e0a0143c5afac47814b6d775c33b1adac724d6ce3cd`**.

| Test | Result |
|---|---|
| ZIP CRC | PASS |
| JSON/pack metadata | **1702/1702 PASS** |
| NBT independent full scanner | **1372/1372 PASS** |
| Fully validated NBT block records | **5,273,603 PASS** |
| Old NBT byte-identical to RC42 | **1367/1367 PASS** |
| New authored structure templates | **5/5 PASS** |
| Cave descent staircase continuity/headroom | **2/2 PASS** |
| Lava ship full-footprint/liquid codec checks | **4/4 PASS** |
| 26.3 codec numerical bounds: volume/offset/scan | **PASS** |
| Missing authored references / biome feature order cycles | **0 found** |
| Dimension heights | Overworld −64..2031; Nether 0..1023, **no ceiling**; End 0..511 |
| Icon, previously filled RC42 loot | **Byte-identical** |

QA source, validation JSON, full NBT report and user test notes are provided in a **separate private ZIP in the conversation**, not in the GitHub repository.

## Unresolved engine-only work – DO NOT CLOSE AS DONE

- **Horizon renderer LOD** (far synthetic terrain mismatch, white/dark distant patches / black screen at extreme render distance).
- **Safe Nether portal exit** (portal forcer and full-block platform clearance in the Rust engine).
- **Actual custom biome/cave IDs in F3** (Horizon does not register user-added biomes from a world datapack).
- **Full terrain/water footprint support for all imported large ocean ships and map structures.** The RC43 liquid test is specific to the two **small** fireproof Nether skiffs.
- **Live RC43 testing in Horizon Nightly** and performance, Nether/skiff success rate, End biome styles and high-Y cave/density visuals. All worldgen changes apply only in **new chunks**, so create a **new world containing only RC43**; old world region fixes are separate.
- **Rights**: imported third-party map assets' public redistribution licenses are unverified. **Do not publish the binary as a public GitHub Release.**

References: [Minecraft Java 26.3](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3) (adds `volume_match`, 26.3 updated `minecraft:template`); [Minecraft 26.2](https://feedback.minecraft.net/hc/en-us/articles/46690753273997-Minecraft-Java-Edition-26-2) (default centered, randomized template rotation); [Minecraft placed features](https://minecraft.wiki/w/Placed_feature) (`environment_scan.max_steps` 1–32); [Horizon worldgen](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md).

[Requirements log](USER_REQUIREMENTS_BY_PART.md) · [Open issues #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
