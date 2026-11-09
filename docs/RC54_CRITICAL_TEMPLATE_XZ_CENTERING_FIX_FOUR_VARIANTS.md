# RC54 — critical template-centering regression corrected in all four private candidates

**9 November 2026 • PRIVATE STATIC QA PASSED • NOT IN-GAME VERIFIED • NOT STABLE.**

The user explicitly requested a completely finished 100% version in the same chat. That standard **cannot honestly be met** without a running Horizon/Minecraft 26.3 installation, the required engine patches and the complete Photon shader port. Nevertheless, an actual new build was produced and a critical preexisting regression repaired in **all four** already assembled Java/Horizon-Rust and Nether1024/2032 variants.

## Newly discovered, source-confirmed regression

**Authoritative primary code:** [Horizon `TemplateFeature::place`](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-world/src/worldgen/structure/features.rs) explicitly shifts the placement origin **west by template.size[0]/2** and **north by template.size[2]/2**, rotated with the selected rotation, before placing the template. All nine affected custom template-configured features use fixed `rotations: ["none"]`. [Official Minecraft 26.2 technical release notes](https://feedback.minecraft.net/hc/en-us/articles/46690753273997-Minecraft-Java-Edition-26-2) likewise specify that `minecraft:template` places structures **centered around the origin by default**. Minecraft 26.3 release notes only add `processors` to this feature type, not an origin-rule change.

RC47 and RC48 accidentally assumed the NBT templates were placed at a positive-coordinate corner and **added an unnecessary horizontal offset**. Their `volume_match` predicates then passed for one region of the world while the actual template was generated somewhere else. This is more serious than a stylistic difference and could cause failed checks, clipping and bizarre island/cave placement. Earlier ZIP/JSON/NBT static QA had not inspected the engine implementation.

## Exactly what RC54 repairs

### Nether: six low/high variants of the 13×12×17 lava skiff
- Previous `minecraft:offset x=-6,y=1,z=-8` corrected to **`x=0,y=1,z=0`**. The feature itself centers the ship horizontally.
- Entire two-layer **13×17 lava footprint** predicate now covers centered `X=-6..6, Z=-8..8, Y=-2..-1`, **221 columns × two layers**. Deck clearance covers the same horizontal area Y0..2 and mast air Y3..11 on X−2..2, Z−3..3.
- Prior rarities, biome bindings, templates, NBT, and 32-step downward scan are unchanged. It is a valid geometrical correction, **not a live guarantee of actual spawn or cross-chunk exclusion**.

### End: one shared generator of six 15×12×15 sky-island NBT
- Previous unnecessary `offset x=-7,z=-7` removed.
- Air volume predicate centered at **X=-7..7, Z=-7..7, Y=0..11**, covering all 225 template footprint columns.
- Six different original shapes and the existing End dragon/structure sets retained.

### Overworld: two 13×39×13 descending cave entrances
- Existing Y offset remains `−16 −16 −3 = −35`; extra X/Z `−6` removed.
- Solid-place checks moved to centered `[-5,10,-5]` and `[-3,15,-3]` relative to the final anchor. This preserves the world-space check points and brings them into the actual centered 13×13 NBT area.
- Surface air probe preserved. Does not prove entrances always connect to an actual natural cave.

## Rebuilt private output

**Single combined archive:** `Thalvoryn_RC54_CTYRI_VARIANTY_OPRAVENE_PRIVATE_HANDOFF.zip`  
**SHA256:** `a5c6f81e499f74b2d87e85857fa6d19bede84182933c1d50c78fdb9bd194e7c4`  
**Size:** `42,222,319 bytes`, 15 files; private Library `/Horizon/Thalvoryn/RC54`.

| Separate candidate | SHA256 | Bytes |
| --- | --- | ---: |
| JAVA Nether1024 | `bb22b8ac8f61f1bc199e6ba51482f9511bfd8e1fd2a2c58a7e0a5a99709c5a9c` | 10,207,342 |
| JAVA Nether2032 | `d8d134eb9627f2076b9fcf23edc2c301d6bff0f35ab462a60435c3b5846a1dea` | 10,207,408 |
| RUST Nether1024 | `1084477604cfb077857829f149973bfd06a04457d68914604fb8289316e4616a` | 10,207,342 |
| RUST Nether2032 | `99169d5429d8340dec95ab10204f97fbb1e067f59e51ae380c0cd3708ee4b468` | 10,207,408 |

**Independently verified in each candidate:**
- exactly **10 modified files** vs its RC53 source (nine custom placed features plus `pack.mcmeta`), **zero** NBT changes.
- ZIP CRC PASS, **1707/1707 JSON + metadata parsed** with duplicate-key detection, **1378/1378 previous NBT byte-identical**.
- actual NBT dimensions decoded independently: 2 skiffs 13×12×17, 2 entrances 13×39×13 and 6 islets 15×12×15. All templates use `rotations:[none]` so centering formula and adjusted footprint are applicable.
- all 136 user namespace biome feature refs present; active custom structure IDs resolve; **full flattened 26.3 biome feature ordering** checked independently across biomes, 246 features / **334 edges with no cycle**. This improves older checks that inspected adjacent features within each layer only.
- Nether dimension type and noise settings match requested heights 1024/2032, both divisible by 16, pack format 121. Other game resources—including `pack.png`, all map NBT and their loot, original terrain, old structures, and previous changelog—remain untouched.
- Combined ZIP reopened and each inner ZIP content digest matched the manifest. Both deterministic builder and independent validator included.

**No real-game or compiler run happened.** The environment has Java 21 (Minecraft Java 26.3 server needs 25), no `cargo`/`rustc`, no Horizon Nightly binary/GPU, no `horizon-shader`/naga validator. None of the actual engine defects—LOD black/white, 1024-high Nether portal arrival, dynamic custom biome F3 registration, full cross-chunk site validation, actual FPS/TPS—can be solved solely by these datapack files. `Photon_Horizon_Native_WGSL_PREVIEW_NOT_FULL_PORT.hshader` is still only a minimal native shader preview, **not** the requested full Photon. Windows Converter v7.2 is a preserved prior build with no new Windows GUI test. Nether2032 remains experimental; Java/Rust differences are only human-readable metadata.

**Release policy:** Keep all NBT binaries **private** until external imported map rights are confirmed; publish only this technical status on GitHub. Do not claim “100% complete”, “Stable”, “tested on Nightly 2ebf00a”, or ask the exhausted user to test immediately. The current best test-first candidate, **when testing becomes possible**, remains RUST Nether1024 RC54 on a backup/new world. No earlier variant needs to be installed alongside RC54. All outstanding original user requirements remain in [the complete request register](RC44_SECOND_FULL_VIDEO_AND_USER_REQUIREMENTS_AUDIT.md) and issue #2.

**Scope progress:** the former ~48% was only a rough task-scope estimate and cannot be promoted to 100% by repackaging. Static QA counts are not gameplay completion percentages.