# Thalvoryn RC25 — Horizon Rust 26.3, Nether 1024

**Status: private test build; complete static NBT loot scan and a conservative loot edit; NOT run in Horizon.**

RC25 is based on the user's RC24 placement-safety ZIP and retains all RC24 structure-set changes, including reduced End duplicate grids, safe/ocean-only selection of smaller Overworld ships, and suspension of unsafe imported Nether ships/buildings.

## Full scan

Using a Big-Endian NBT decoder and six batches (to avoid per-call timeouts), **all 1,102 NBT templates were parsed without error**, including the largest TREK castles and map templates. Results before RC25 editing:

- 1,267 chests, 1,568 barrels and 10 dispensers with NBT block entities in scanned templates
- 687 observed containers with 0–2 stored stacks; some are vanilla loot-table references and are **not** broken or really empty. Among them, 316 had no explicit LootTable field, primarily decorative two-stack barrels in large buildings.
- All code-based resource and structure references remain part of RC24; game runtime outcomes cannot be concluded from counts.

## Controlled changes

- 29 edited NBT templates
- 593 storage entities changed, including **533 formerly filled containers cleared** because repeated two-item decorative barrel patterns were excessive (deterministic thinning of dense groups; not a proof of creative intent)
- **60 sparse container inventories** enriched with **80 moderate themed item stacks**, preserving previous valuable items and choosing additional food, materials, supplies according to the structure category and dimension
- All reference `LootTable` NBT fields remain untouched. Vanilla Minecraft templates are untouched.
- All **1,102** templates remain present; **1,073** are byte-identical to RC24.
- Modified NBT files were decoded, re-encoded and compared to their RC24 versions: identical dimensions, exact block palette/states and positions, unchanged entities and unrelated block NBT, unique inventory slots [0,26], valid item counts [1,64].

## Static verification

- ZIP: `Thalvoryn_Horizon_Rust_RC25_Nether1024_LOOT_PLACEMENT_TEST.zip`
- Size: **8,797,885 bytes**
- SHA-256: `01c81b4c98c08f55008cad2ec03c2229c1f26a51a97d33f70f332fc8540ce7f0`
- ZIP CRC: pass.
- Inherited RC24: 56 active structure sets, 237 unique structures, 0 missing internal structure → pool → template links and no duplicate custom Nether/End structure-set refs, with Overworld -64..2032 / Nether 0..1024 only **in JSON**.

## Remains UNSOLVED

- **Not tested in Horizon runtime.** Even valid NBT does not prove loot persistence or that structures load.
- Decorative-barrel classification is an inference; in-game spot review is essential.
- There is no full-water/lava-footprint or full-slope predicate for jigsaw structures. Some ships and imported buildings are still inactive for this reason; remaining small ships cannot be guaranteed correctly in water from a biome filter alone.
- Nether portal safe spawn and custom F3 biome IDs need game/engine support; the present datapack does not implement those features.
- True End terrain variation, custom tree/structure collision and the Photon native shader port are separate open issues.
- User's observed default -64..256 world remains unverified against the **loaded** world type and actual new-world logs.
- Third-party map redistribution permissions unverified. **Not a public GitHub binary release.**

## Use

Start a **new world** with only RC25, disable RC24 and older variants. Existing chunks will not regenerate. Report remaining weird placements and chest cases at [Issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). The original user's 16 screenshots + video were indexed in that issue, but not uploaded as media attachments by the connector.
