# RC59 — 268 additional dynamic reward chests, 29 land guards, Nether bridges and cave entrances

**9 November 2026 | PRIVATE STATIC QA PASS | NOT a tested or stable release.**

This is one substantial batch that builds directly upon the physical RC58 archive. The user wants updates at **10-percentage-point completion milestones**; do not increment an arbitrary percentage after each new ZIP. The prior 50% milestone is the last one communicated, and **60% is not yet demonstrated** because engine/runtime work remains.

## Real changes in RC59

1. **268 previously fixed-item chests, across 156 pool-linked RC42 NBT templates, now use native `LootTable` tags** instead of repeating pre-baked `Items`. All 156 templates were enumerated from **190 actual `template_pool` NBT location references** present in RC58. Every one of the 78 paired `_loot_1` / `_loot_2` template families uses **two distinct, existing loot tables**; **19 distinct `thalvoryn:chests/rc37/*` table IDs** across End/Nether/Overworld were selected by family/biome role. Each of their JSON definitions exists and has nonempty loot pools. No new potentially OP loot table was invented.
   - **200 previously filled barrels in those RC42 templates, plus their decorative empty barrels, were deliberately not changed.** No empty chest was filled without evidence it was a reward container.
   - Actual origin NBT block positions, geometry, palettes, entities, non-container metadata, unrelated block entities and all non-target Items remain byte/semantically unchanged.
   - Per-spawn random loot is supported at source level by Horizon Nightly `2ebf00a`: the block entity `LootTable` without seed uses a deterministic position-derived roll, as implemented in [pinned `structure_blocks.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-client/src/structure_blocks.rs). **No in-game chest-opening test was performed.**

2. **29 additional selected, active conventional land jigsaw structures receive Horizon-only start guards**, in Rust variants only. `terrain_radius:1`, `terrain_range:30` or `36` and `liquid:"top"` restrict starting on shore/steep terrain. The already-guarded 40 large land structures and five ocean vessels from earlier releases are unchanged. This source feature exists at pinned [Nightly `2ebf00a` `start_checks.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-world/src/worldgen/structure/start_checks.rs).
   - Applied to safe conventional houses, towers, camps, cottages and trader stops, not water reservoirs, stilt structures, treehouses, natural land formations, void islands or underground structures.
   - The **Java candidates do not contain any newly added Horizon-only start-check fields**.
   - Start checks are **NOT** per-block full-footprint validation and may reject some previously possible starts.

3. **Nether `skybridge_ruin` and `cinder_viaduct`**: random-spread spacing **108→78**, separation **45→30**, frequency **0.12→0.18**. Rough candidate-attempt multiplier: **2.876×** for each, not a measured actual spawn factor. An additional *one-way* `exclusion_zone` keeps cinder viaduct starts at least 10 chunks away from the independent skybridge set; no exclusion graph cycle was found. Their models, height/biome filters and loot remain untouched.

4. **Overworld forest and highland stair cave entrances**: placement rarity `1/22→1/15` and `1/30→1/20` (about **1.47×** and **1.50×** candidate attempts respectively). The existing volume/terrain/height checks and templates remain unchanged. Runtime accessibility still needs validation.

## Independent binary regression and exact output provenance

Input: `Thalvoryn_RC58_BATCH_LOOT_STRUCTURES_CAVES_PRIVATE_STATIC_CANDIDATE.zip` SHA256 `ee5146a99fa93fc82218ed74cdb4ff976a224ba88cd5090a6ff33abd7e42d7bb`.

Output: **`Thalvoryn_RC59_BULK_268_CHEST_LOOT_SITE_SAFETY_NETHER_CAVES_PRIVATE_STATIC.zip`**, size **40,011,526 bytes**, SHA256 **`94830048cc14bc2fc9791a44710713201318b18527a7235154160df1c1b69335`**, 19 outer members.

| Variant | SHA256 |
| --- | --- |
| JAVA Nether1024 | `1031aed50e0786f62778559d1130a483ccd53c905420902c377de50518c8991b` |
| JAVA Nether2032 | `e0329b4951dd8d20718fed28cff1356ac1d8b443b3a8709d13e99fccc5588ca8` |
| RUST Nether1024 | `1965122f4beca60aa60f5c03f7e21a40932b2605a35df5880f3e59b2d4103caf` |
| RUST Nether2032 | `7b968de403595f310d0d5fd82155a4ed924d523e7de5951547196160e6985d79` |

The **independent verifier does not import the RC59 builder**; it reopens old/new archives, decodes the changed NBTs, checks all item removal and placement-preserving changes, compares the common NBT bytes across four variants, and validates all 19 referenced loot tables. The first attempt at full ×4 binary parsing exceeded the execution time budget, so the completed verifier performs full independent semantic decoding **once for common NBT data** and checks exact source/output byte identities against each of the other three ZIPs—an equivalent but more efficient regression strategy.

**PASS:**
- 4 inner ZIP CRC tests and 1 outer CRC; no duplicate filenames.
- **1,707 JSON/metadata documents parsed per candidate with duplicate-key checks**, archive format 121.
- **1,378 original NBT**: **156 targeted and fully parsed before/after**, **1,222 untouched byte-identical** in *each* of the four candidates. Every output NBT file byte-identical across Java/Rust and 1k/2k.
- **268 precise chest conversions**, all 78 paired variants with differing actual loot-table IDs, no palette/position/entity changes.
- Exact file-change whitelist: **161 existing files** per Java candidate (156 NBT, 2 Nether structure sets, 2 cave entry placed features, 1 pack metadata); **190** per Rust (same +29 structure JSONs).
- Full biome placed-feature order: **246 unique nodes / 334 directed edges / no cycle**. All custom biome placed-feature refs and active structure-set refs resolve, including the new Nether exclusion target.
- Existing Nether dimension and noise heights match the respective **1024 or experimental 2032** profile. Pack art and all unrelated resources unchanged.

**Stored privately:** ChatGPT Library **`/Horizon/Thalvoryn/RC59_BULK`**, including the complete ZIP, readable Czech start/diagnostic commands and the independently generated QA JSON. Builder and separate verifier are both inside the ZIP; the RC58 source and audit are retained as history. **No user NBT/map binaries published on public GitHub**, because imported third-party assets' licenses/credits remain unresolved.

## Crucial unsolved requirements (do not claim 100%)

No Minecraft 26.3 + Java25 or Horizon executable was run, no seed-generated world/chest interaction, no GPU render benchmark. The user's reported LOD white/black artifacts, 16k LOD crash, Nether portal freeze/arrival, FPS/TPS, native biome registration/F3, complete full-footprint collision guards for all large structures, full Photon 1.4 native WGSL port and real 2k Nether appearance are **not fixed by this datapack batch**. The archived native WGSL shader remains only a preview; the Windows Converter is still the previously produced v7.2 without new Windows GUI runtime tests. Do not advertise these as stable or tested on 2ebf00a; that commit was **source-checked only**.

This batch is substantial progress toward the **next 60% milestone**, but no completion percentage should be increased solely because the package was rebuilt. Keep [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) OPEN.
