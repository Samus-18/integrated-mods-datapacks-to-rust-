# RC55 — complete static re-audit + native Horizon ocean-ship start guards

**9 November 2026. Status: private static candidate; NOT fully implemented, NOT tested in-game, NOT stable.**

Previous RC54 contained four candidates (Java and Horizon Rust × Nether 1024/2032). The user asked to re-check everything and finish as far as feasible. Reinspection against the **current upstream Horizon Rust source** revealed an existing engine feature useful for safer structure placement:

- [Horizon `StartChecks` implementation](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-world/src/worldgen/structure/start_checks.rs): a jigsaw's optional **`horizon:start_checks`** can check the surrounding biomes and local heightmap range BEFORE assembling the structure. It is an existing **Horizon extension**, not part of vanilla Java.
- [Jigsaw placement code](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-world/src/worldgen/structure/jigsaw.rs) explicitly computes `position[1] + context.base_height` when a start projects to a heightmap. This source-level check supports the previously corrected relative Sky Islands placement in RC51.
- [Template feature centering](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-world/src/worldgen/structure/features.rs) confirms RC54's removal of double X/Z centering.

## Actual change from RC54

**Only RUST builds** have received the following JSON property in precisely five pre-existing active `trek:overworld/medium` jigsaw ship structures:

```json
"horizon:start_checks": {
  "biome_radius": 1,
  "terrain_radius": 1,
  "terrain_range": 6
}
```

Applies to `cold_red_trade`, `dark_oak_trade`, `jungle_trade`, `small_red_trade` and `ship_pillager`. It requires the 3×3 surrounding chunk centres to be within the ship's declared deep-ocean biome set and their water-surface `WORLD_SURFACE_WG` heights not to differ by more than six blocks. It is intended to reject coastal starts and severely uneven terrain. **It does not inspect water under every block of the rotated hull** and may make some types rarer, especially around biome borders. No use of `liquid:top`, because that would reject a ship on an ocean surface.

**JAVA builds** have **zero** Horizon-specific keys and retain the standard Java 26.3 data. Their only change versus RC54 is a version-description string in `pack.mcmeta`. This respects the difference between vanilla codec support and proprietary Horizon extensions.

**Important version caveat:** implementation is verified in *current upstream Horizon source*, **not proven present in the user's tested Horizon Nightly 2026-11-07 (2ebf00a)**. If that build rejects the extension, use RC54 or an updated engine build. No runtime claim.

## Independent, fresh QA across all four variants

Each of the four RC55 ZIPs was independently reopened and compared against its corresponding **physical** RC54 predecessor. The independent validator does NOT import the builder:

- ZIP CRC PASS, 3,156 members, **1,707 JSON/pack metadata successfully parsed with duplicate-key rejection**.
- **1,378 original NBT files all byte-identical** to the corresponding RC54 source; independently verified the gzip streams of all 1,378 per candidate and calculated the exact same per-archive NBT-content hash.
- Independent, complete NBT semantic parsing of **17 selected meaningful structures** (five ocean ships, two lava boats, two cave entrances, six End sky islands, stronghold and smithy): **125,970 block positions**, every palette state, no invalid coordinates or duplicated positions, **81 selected container blocks with actual `Items`**. This **does not claim** that every other NBT inventory was fully parsed.
- Explicit diff whitelist: RUST variants modified **five ship structure JSON + pack metadata**; JAVA variants modified **only pack metadata**. No biome, terrain density/noise, 1,378 NBT, loot, package icon, structure-set rules, other structures or datapack data altered.
- 246 distinct biome placed-feature IDs and **334 order edges, no ordering cycle**; 136 custom biome→placed-feature refs and all active structure-set custom refs resolved.
- Nether dimension type and noise heights agree exactly for **1024** or **2032** (the 2k experimental profile). All four packs have `pack_format=121` from the prior 26.3 target.
- Five **synthetic** Python site-check examples pass: flat deep ocean accepted; adjacent non-deep biome, sharp vertical edge and larger-than-six-block slope rejected. These are logic illustrations, NOT a game runtime test.
- All 4 inner ZIPs and final outer ZIP reopened; exact SHA256 hashes in the included `SHA256_MANIFEST.json`.

### Four variant hashes

| ZIP | SHA256 |
| --- | --- |
| JAVA Nether1024 | `2e558666aa81821cffb244e1ef6a81d932bda18dec280bf9c14c477375c87dce` |
| JAVA Nether2032 | `28d0c0a06c18cb1e8ad9ae6956de0783b692a7d1de13df26cdb4f2477e97602a` |
| RUST Nether1024 | `f7e83da5447de3de589d51bc7d6eea7848e1b8566aee521b906891d088137d7f` |
| RUST Nether2032 | `f9d0b8c873314797e2a05aa602c0e12506a0e8f3446794c47a1110b8296e5838` |

**Single private RC55 handoff archive**: `Thalvoryn_RC55_CTYRI_VARIANTY_PROVERENE_PRIVATE.zip`; **40,024,863 bytes**, **SHA256 `d1f9702afb2ff0760c1ef13967a880fc6fdc838b08b2e79f2a1dc95bc6d0ed81`**. Contains the four complete static candidates, builder, independent QA Python and JSON, Czech installation notes, limitations, old Converter Suite v7.2 and a clearly labelled **native WGSL shader preview that is NOT a full Photon v1.4 port**.

Private ChatGPT Library: **`/Horizon/Thalvoryn/RC55`**, saved and listing independently confirmed. Public GitHub receives **this documentation only**; import map NBT licensing/attribution remains unresolved.

## Unfinished requirements — important

No live Minecraft 26.3 Java server or Horizon Rust binary was available in the container; Java is version 21 instead of the required 25, `cargo`/`rustc` and GPU are absent. There was no chunk generation, portal transfer, screenshot comparison, chest-opening, F3 biome registry check, shader `naga` compilation, or FPS/TPS measurement.

This RC55 **does not** solve:
1. Runtime **black/white LOD seams, excessive 16k LOD, safe Nether-1024 portal location**, or engine performance.
2. New native registered cave/End/Nether biome IDs and their F3 renderer.
3. General full-footprint collision detection over all large imported structures, ships, ground/buffer/vegetation/fluid and neighboring chunks. This patch covers only **start checks** of five Rust ocean ship definitions, no water under entire hull.
4. Fully dynamic loot on every copied structure, actual dungeon accessibility, balanced mobs, End/Nether worldgen visual quality and 2k Nether verified behavior.
5. A **complete Photon v1.4 Rust-native WGSL port**. The bundled preview is **not** visual parity, and was not checked by `horizon-shader check`.
6. Runtime testing of the previously created Windows EXE Converter v7.2 and final user acceptance.

**Overall scope progress is an approximate ~49% (51% open).** It is a human scope estimate, not a formal pass rate. Completion at 100% cannot be responsibly claimed from static files alone. The user cannot test right now and was not asked to. Open [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) remains the tracking item.