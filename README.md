# Thalvoryn: Expanded Worlds

**A world-generation overhaul for Minecraft Java Edition 26.3 and Horizon Rust 26.3.**

Thalvoryn: Expanded Worlds expands Minecraft's three original dimensions with new structures, improved terrain, biome-themed ruins, enhanced caves, floating islands, mobs, treasure and more.

The project combines and expands **TREK** structures and **JJThunder** world generation, aiming to make exploration more rewarding without completely replacing the vanilla Minecraft experience.

## Experimental Horizon Rust RC15 build (Nether 1024)

An **experimental RC15 test build** was assembled for Horizon Rust 26.3, with a 1,024-block roofless Nether, terrain-relative and more frequent Sky Gardens, more Nether/End structures, a reduced chorus density, adjusted loot, 48 schematic-derived tree templates, and three builds extracted from a user-provided map. The archive has undergone **static ZIP, JSON, NBT and reference checks**, **not an in-game Horizon Rust test**. Older RC13 downloads remain the published baseline. **RC15 ZIP files have not been uploaded to GitHub:** public redistribution rights for supplied third-party builds are unverified.

See [RC15 development notes](docs/RC15_Horizon_Rust_Nether1024.md) for feature details, limitations, tests, compatibility and licensing.

## Experimental Horizon Rust RC16 update (Nether 1024)

A new **RC16 private test ZIP** has been built from RC15, addressing the screenshot-reported hard Nether terrain cutoff, missing / rare Nether and End structures, Sky Island altitudes, Nether biome distribution and loot variety. It uses a 1,024-block roofless Nether, remixes *existing* Nether biome IDs (so no custom F3 biome names), makes structures more common, raises Overworld sky structures by 45 blocks relative to RC15, adds five End island designs and changes 640 prefilled containers. **Only static NBT/JSON/ZIP/reference checks passed; Horizon runtime testing has NOT happened.** Third-party map redistribution rights remain unverified, so **the RC16 ZIP is not publicly hosted on GitHub**. The earlier RC13 public-release guidance below is historical.

[RC16 technical details, audit and limits](docs/RC16_Horizon_Rust_Nether1024_TEST.md)

## Experimental RC17 Horizon Rust — imported map structures

An experimental **private RC17 Nether-1024 test ZIP** builds on RC16 and adds **25 bounded structures** from the previously supplied 1.11/1.12 ship/medieval and modern Buildings Collection worlds. The test archive has 25 new NBT templates and 75 new worldgen JSON files. Static ZIP, JSON, NBT palette/index and resource-reference checks passed; **Horizon in-game generation has not been tested**. This is **not a public GitHub ZIP release** because rights to redistribute the supplied third-party builds remain unverified. [RC17 import audit and caveats](docs/RC17_Horizon_Rust_Nether1024_LEGACY_MAPS_TEST.md).

## Experimental Horizon Rust RC18 — additional map structures

A **private RC18 Nether 1024 test ZIP** was produced from RC17. It adds **78** more bounded map structures (**61** ships, **17** modern buildings), grouped into weighted Overworld/Nether/End placement sets; **134** authored structure mob-spawn profiles; **191** additional TREK loot-container repairs; and several surface-placement tweaks. All new/changed NBT and worldgen JSON passed **static checks only**, not a Horizon in-game runtime test. **Public ZIP upload is withheld** until the third-party map redistribution rights are established. This is a development status update, not a verified public release.

[RC18 feature details, checksum and unresolved engine issues](docs/RC18_Horizon_Rust_Nether1024_PRIVATE_TEST.md).

## Experimental Horizon Rust RC19 (Nether 1024) — terrain & End corrections

An **RC19 experimental private test archive** improves RC18 configuration: converts residual 2032-high Nether density coordinates to 1024-high rules, moves the unreachable floating Nether band into Y340–705, uses weighted placement of 9 existing Nether surface sites and 14 End island/ruin types, increases relevant structure frequency and reduces monotonous End pillars/chorus. Ancient-debris attempts now favor upper layers rather than lower. **Static validation only (2,755 ZIP entries, 1,574 JSON documents, 1,102 unchanged NBT structures); Horizon in-game test NOT run.** No public ZIP released because supplied third-party world assets have unverified distribution licenses.

[RC19 technical changelog, checksum, tests and limitations](docs/RC19_Horizon_Rust_Nether1024_TERRAIN_END_TEST.md).

## RC20 Horizon Rust — Nether 1024 / End terrain-and-structure experiment

The latest **private experimental test ZIP** is RC20, built from RC19. It reorders the Nether bedrock-floor material rule, tunes Nether density-noise variation and increases End island/ruin and vanilla End-city placement attempts. **All 1,102 prior NBT templates remain unchanged**; the attempted small-inventory scan did not result in any NBT loot changes. ZIP CRC and 1,574 JSON/metadata records passed static verification. **Not tested in-game**; results and spawn rates may differ in Horizon Rust. Third-party map asset redistribution rights have not been established, so **the ZIP has not been published to GitHub Releases**.

[RC20 change log, SHA-256 and test limitations](docs/RC20_Horizon_Rust_Nether1024_TERRAIN_END_TEST.md).

## Experimental RC21 for Horizon Rust (Nether 1024)

The newest private **RC21 TEST** ZIP repairs structure-set discovery and chosen large-structure loot inventories in RC20. It enables two TREK structures that had no registered placement sets (`birch_trade` and the mushroom village), separates four Nether biome-specialized surface watches from generic structures, separates three highland-only End ruins from an 11-way all-biome island set, and enriches 103 underfilled chests/barrels across 15 large NBT templates with 214 moderate additional item stacks. **Static checks passed, but no in-game Horizon runtime test was performed.** The 1,024-block Nether terrain, ore parameters, End terrain and Photon renderer were **not** changed in this revision.

[RC21 details, hash and verification limitations](docs/RC21_Horizon_Rust_Nether1024_STRUCTURES_LOOT_TEST.md). Because third-party map redistribution permissions are not verified, this experimental binary **has not been uploaded publicly to GitHub Releases**.

## RC22 Horizon Rust — structure spawning format repair (Nether 1024)

An experimental **RC22 private TEST** archive corrects **268** pre-26.3 `spawn_overrides` `minCount/maxCount` entries across **134** structures, converting them to the `count` int-provider format required by Minecraft Java 26.3/Horizon. Four extra uniform-count fields in two additional definitions were normalized, for **136 corrected structure definitions in total**. This may resolve structures failing to decode and appear. **Static JSON/ZIP/reference checks passed, but an in-game Horizon test has NOT been performed**; Nether/End terrain and NBT geometry were not altered. Public binary release is withheld until third-party build rights are confirmed.

[RC22 changelog, test results and SHA-256](docs/RC22_Horizon_Rust_Nether1024_SPAWN_FORMAT_TEST.md).

## RC24 Horizon Rust — conservative structure placement experiment

**Latest private test archive:** RC24 Nether 1024. Based on the user's screenshots/video in [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2), this version removes duplicate End/Nether structure grids, limits overworld imported ships to ocean biomes and smaller hulls, pauses imported Nether buildings that cannot be safely grounded, and lowers density of cloned-looking or overlapping builds. The **1,102 original NBT templates are retained byte-identically**. ZIP/JSON/static reference checks passed; **in-game Horizon testing, safe portal arrival, full liquid/slope footprint validation and the Photon port remain undone.** No public binary release due to unverified third-party asset permissions.

[RC24 technical audit and limitations](docs/RC24_HORIZON_RUST_PLACEMENT_HARDENING.md).

## RC25 Horizon Rust — loot consistency update (private test)

**Latest private experimental ZIP is RC25**, built on RC24. It carries forward the safer structure placement and deduplicated Nether/End spawn sets, and adds a full static inventory scan of all **1,102 NBT templates**. An exact-geometry NBT rewrite conservatively thinned **533 overly stocked/decorative containers** and enriched **60 sparse inventories** with 80 contextual item stacks across 29 templates. Existing Minecraft loot tables, entity data, palette indices and block positions were preserved. **Static tests passed; in-game Horizon generation is NOT verified.** Full water/lava footprint validation, world-height runtime confirmation, Nether portal safety, custom F3 biomes and Photon remain open. [RC25 audit](docs/RC25_HORIZON_RUST_LOOT_AUDIT.md) and [user issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). The test ZIP is not publicly released because map-asset redistribution rights remain unverified.

## RC26 — Horizon Rust 26.3: redesigned End islands and safer placement

The latest **private experimental RC26** (Nether 1024) builds on RC25. Five previously repetitive End NBT island templates have been remodelled into distinct geometries; their weighted placement is more varied and End airship fleets are much rarer. Overworld ships are restricted to deep-ocean biome starts (not a full water-footprint check), imported ground structures with unverified slopes are paused, and matching Nether camps/watches are grouped to reduce overlapping starts. Nether material accent patches were made less extreme. The original RC25 loot remains unchanged. **Static ZIP, NBT and JSON tests passed; Horizon runtime tests were NOT run.** Native portal safety, structure footprint predicates, custom F3 biome identifiers, and the Photon native port remain open in [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). The binary is not a public release until third-party asset redistribution rights are verified.

[RC26 technical changelog and SHA-256](docs/RC26_HORIZON_RUST_ISLANDS_PLACEMENT_TEST.md).

## RC27 — Overworld ship spam and ocean-island placement mitigation (private test)

After the user's **RC26 in-game Overworld screenshots** ([issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2)), RC27 removes independent TREK ship-like starts, pauses seven badly anchored coastal/farm-island starts and the independent air vessel, and reduces the remaining 16 imported deep-ocean ships to one group at **spacing 144, separation 62, frequency 0.14**. All 1,102 NBT templates, Nether/End settings and Photon remain unchanged. **ZIP/JSON/resource-link tests passed; no RC27 Horizon gameplay test has been run.** The observed incorrect/missing distant LOD structures and biome appearance are **not fixed by this datapack**: Horizon's [graphics documentation](https://github.com/horizon-rust/horizon/blob/main/docs/graphics.md) explains its coarse distant generation. [RC27 findings, original screenshot coordinate manifest and test guide](docs/RC27_OCEAN_STRUCTURE_SPACING_AND_LOD_REPORT.md). The binary is a private ChatGPT test attachment, not a public GitHub Release.

## RC28 (Horizon Rust 26.3) — JJThunder full-height codec repair

**Latest private test build: RC28 Nether 1024.** The RC27 data still used obsolete pre-26.3 dimension-type keys in **Overworld and Nether**, even though the numerical heights and the JJThunder density range were correct. RC28 moves these two dimension types to the Minecraft 26.3 **environment `attributes` schema**, retaining **Overworld Y −64…2031** (height 2096) and **roofless Nether Y 0…1023** (height 1024). This is a plausible fix for the observed vanilla world-height fallback, but **no in-game Horizon validation has been run**. All terrain noise, structures, biomes, loot, and all **1,102 NBT** files remain identical to RC27. [Detailed RC28 audit and SHA-256](docs/RC28_JJThunder_2032_Height_26_3_Codec_Fix.md) · [open user issues](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

Original JJThunder To The Max documents terrain from Y −64 to 2032; its [Modrinth page](https://modrinth.com/datapack/jjthunder-to-the-max) lists **All Rights Reserved**. Our private modified ZIP is **not publicly uploaded** because supplied third-party map assets also require verified permission.

## RC29 — Nether stays at 1024; Overworld unchanged; End build range 512 (private test)

The latest private **RC29 TEST** honors the clarified dimensions: **Overworld −64…2031** (JJThunder/ThunderFusion worldgen unchanged), **roofless Nether 0…1023** (**183 Nether-related files byte-identical to RC28; no height increase**), and **End 0…511** (End build height expanded from 256 to 512). Five authored End islands use distinct low/mid/high placement bands without increasing their structure-set frequency. **Note:** the natural End noise generator is still vanilla `minecraft:end` with no new natural high-altitude terrain above its native noise extent. All **1,102 NBT** templates are unchanged, and static ZIP/JSON/reference checks passed; in-game Horizon verification is still outstanding. [RC29 audit and exact limits](docs/RC29_END512_NETHER1024_UNCHANGED.md). The private binary is not a public release pending third-party redistribution permission.

## RC30 — End high sky islands (private test, Nether 1024 unchanged)

The newest **RC30 experimental ZIP** moves the existing small natural End sky-island feature from Y68–132 to **Y104–276**, reduces its rarity-filter attempt rate (24 → 36), and retains the **End 512** build height. It deliberately **does not change** JJThunder Overworld (-64…2031), Nether (0…1023) or any of the 1,102 NBT structures. This is a targeted JSON worldgen improvement, **not** a finished natural End terrain overhaul or Horizon runtime test. See [RC30 audit](docs/RC30_END_HIGH_ISLANDS_TEST.md) and [still-open problems](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). The ZIP remains a private test attachment until third-party map redistribution permissions are verified.

## Features

### 🌿 Overworld
- Expanded JJThunder terrain with tall mountains and varied landscapes.
- TREK castles, villages, towers, ruins and other structures.
- New biome-themed watchtowers and outposts.
- Rare floating Sky Garden islands containing treasure.
- Improved caves, cave entrances, geological formations and vegetation.
- Structures designed to generate at different elevations, including mountainous terrain.
- Passive and hostile mobs in selected structures.
- Loot chests and hidden rewards.

### 🔥 Nether
- Expanded Nether terrain and more varied natural formations.
- Crimson, warped, basalt and soul-themed structure variants.
- Elevated bridges, ruins, watchtowers and viaducts.
- Additional glowstone, vegetation and basalt formations.
- Structures designed for multiple vertical levels.
- Biome-dependent loot and enemies.

### 💜 End
- Enhanced End landscapes and floating islands.
- Rare citadels, shrines, observatories and ancient ruins.
- Crystal gardens, purpur formations and obsidian spires.
- More environmental variety in the outer End.
- Reduced spawn frequency for major structures to keep exploration rewarding.
- Additional treasure locations and biome-specific structures.

### 🏰 Structures and exploration

Thalvoryn retains the extensive TREK structure collection while introducing additional custom structures and biome-specific variants.

The RC13 package contains, according to its static audit:

- **931 NBT structure templates**
- **128 loot tables**
- **13 additional biome-themed structure variants**
- Structure generation across suitable Overworld, Nether and outer End biomes.

*An NBT template may represent one piece of a larger structure rather than an entire independent building.*

### 🌍 Two Nether height options

| Variant | Nether height | Recommended use |
|---|---|---|
| Nether 256 | 256 blocks | Recommended; less demanding |
| Nether 2032 | 2,032 blocks | Experimental large-scale Nether |

Both options are available separately for Java Edition and Horizon Rust.

## Downloads

Download the appropriate archive from the GitHub Releases page:

- **Java Edition bundle** — contains both Java datapack variants.
- **Horizon Rust bundle** — contains both Rust datapack variants.

Each bundle contains two individual datapack ZIP files and the same custom pixel-art pack icon.

## Installation

1. Download the bundle for your Minecraft edition.
2. Extract the outer ZIP archive.
3. Choose **one** Nether height variant.
4. Place that inner ZIP in your world's `datapacks` folder.
5. Disable or remove older Thalvoryn datapacks.
6. Create a new world for the best world-generation experience.

**Do not install both Nether variants together. Do not mix Java and Rust datapacks.**

Existing generated chunks will not automatically regenerate.

## Compatibility

- **Minecraft Java Edition:** 26.3.
- **Horizon Rust:** Nightly **2026-11-07**, build/commit **`2ebf00a`** (Minecraft 26.3) — **tested in-game on this build**.
- **Other Horizon Rust Nightly builds:** You can try them, but compatibility has **not been tested or confirmed**. Changes in the game engine may affect datapack loading and world generation.

Testing on the listed Horizon build does not guarantee that every RC13 structure or feature has been verified. Java Edition and Horizon Rust use separate datapack builds because their world-generation implementations may differ.

## Development status

**Current release: RC13 — Release Candidate / Testing**

> **Known issue with Horizon Rust RC13:** Some generated structure chests and barrels may be empty. A Rust RC14 hotfix with prefilled NBT container inventories has been prepared, but **the GitHub Release still contains the old Rust RC13 asset until it is manually replaced**. Java RC13 is unchanged. Existing generated chunks are not fixed retroactively. The RC14 workaround uses pre-generated (non-rerolled) loot and requires in-game testing.

The datapacks have undergone static checks for JSON, NBT structures, loot references and world-generation ordering.

The Java RC11 baseline was confirmed to load in-game. RC13 includes additional world-generation content and requires further in-game verification.

Some content may differ between the Java and Rust implementations. The large Nether variant is experimental.

## Credits

- **TREK** — original structure collection and associated assets.
- **JJThunder / ThunderFusion** — original terrain-generation foundation.
- **Thalvoryn additions** — additional structures, integration, biome variants, loot, terrain adjustments and compatibility work.

Original authors and third-party assets must receive their required attribution. Redistribution rights and applicable licenses should be verified before a public release.

---

**Explore a more varied Minecraft world. Discover ancient ruins, floating islands, dangerous structures and hidden treasures across all three dimensions.**
