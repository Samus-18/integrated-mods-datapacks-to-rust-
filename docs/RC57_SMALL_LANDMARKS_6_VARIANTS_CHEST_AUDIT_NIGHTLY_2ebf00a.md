# RC57 – 6 active Overworld landmarks, terrain guards, independent QA

**9 November 2026. PRIVATE STATIC CANDIDATE, NOT a tested Horizon/Minecraft release.**

This continues the physical **RC56** archives, preserving their earlier loot repair and four distinct Java/Rust × Nether1024/2032 variants. The user presently cannot run in-game tests and does not need to send new logs.

## Two discoveries from the full RC56 audit

1. An independent read-only NBT scan of 570 templates containing chests (3,312,467 block entries) found **2,111 ordinary chest blocks**, of which **558 had neither Items nor LootTable**. All 558 were confined to **five large imported building templates** (Overworld 02, 03, 08, 12 and Nether 06), each of which **has no direct active structure-set entry** in RC56. They comprise large groups of decorative/technical chests, not 558 proven empty reward chests in naturally generated active structures. **No blanket filling of hundreds of decorative containers** was performed. Separately, all six small-landmark templates currently chosen have 14 already populated ordinary chests.
2. The active `thalvoryn:overworld/imported/small_landmarks_rc36` structure set had just three small buildings `20,26,29`, with spacing 75 chunks and frequency .37. Three other existing small imported buildings `14,17,23` had existing NBT, jigsaw pools and physically stocked chests, but were not listed in an active structure set.

## Actual RC57 edits

- **One existing structure set:** now **six equally weighted** entries `buildings_14,17,20,23,26,29`. Original `minecraft:villages` exclusion zone (10 chunks), salt and spread algorithm retained.
- Candidate starts adjusted `spacing:75 → 52`, `separation:28 → 20`, `frequency:0.37 → 0.54`. Rough mathematical **candidate attempt** multiplier: `(75/52)^2 × (0.54/0.37) = 3.036`. This is **not** proof of actual structure density (biome restrictions, exclusion zones, failed placements and terrain guards may reject many starts).
- **Three newly enabled structures** 14/17/23: `terrain_adaptation: none → beard_thin`, a cautious terrain blending option. All other Jigsaw fields unchanged.
- **Horizon Rust only:** on all six small landmark jigsaw JSONs add `horizon:start_checks: {terrain_radius:1, terrain_range:18}` to reject overly uneven nearby terrain. In Java ZIPs, this non-standard Horizon field is **absent**.
- Metadata descriptions identify the new private RC57 candidate. One small readme added to each inner ZIP.
- **No original NBT, block/loot, biome JSON, Nether dimension/noise, structure icon, existing mixed structure-set, shader or Windows converter were modified.**

Existing source NBT model dimensions and filled chests:

| Imported building ID | Size XYZ | Original chests with actual inventory |
| --- | --- | ---: |
| 14 (newly active) | 22×30×35 | 1/1 |
| 17 (newly active) | 27×29×20 | 6/6 |
| 20 (previously active) | 17×22×30 | 1/1 |
| 23 (newly active) | 24×25×19 | 1/1 |
| 26 (previously active) | 17×33×19 | 4/4 |
| 29 (previously active) | 13×32×11 | 1/1 |

## Compatibility check against the ACTUAL target Horizon Nightly

The **`2ebf00a`** revision used by the user was opened directly using the GitHub connector, and this specific file exists in that revision:

[Horizon Nightly 2ebf00a – `start_checks.rs`](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-world/src/worldgen/structure/start_checks.rs)

Git blob hash: `f182706a8e234a376cb07e7f90f140fcbb9aae64`. This pinned source contains the actual `horizon:start_checks` decode and `terrain_radius`/`terrain_range` keys. This proves **source-level availability**, not successful runtime world generation on the user's machine.

## Archive and independently recreated QA

- Input **RC56 SHA256** `3212174b04640e6103313251c6f06212b90a806a7db2f468b10633062231716c`.
- Output combined **RC57 SHA256** `3ef867518dc5877102a28f7faf28d5120828ef6c794940324f6de4aa07ee0f20`.
- `Thalvoryn_RC57_SMALL_LANDMARKS_PRIVATE_STATIC_CANDIDATE.zip` — **40,113,282 bytes, 21 outer members**.
- Four inner candidates, each full vanilla-data ZIP:
  - JAVA Nether1024 – **10,226,158 bytes**, 5 edited original files.
  - JAVA Nether2032 – **10,226,224 bytes**, 5 edited original files.
  - RUST Nether1024 – **10,226,537 bytes**, 8 edited original files.
  - RUST Nether2032 – **10,226,603 bytes**, 8 edited original files.
- **Independent QA, not importing builder**: all outer/inner CRCs PASS; **exact allowlist** 5 or 8 changed original files and 1 inner readme added; **1,707 JSON/meta parsed in every variant with duplicate-key rejection**; **1,378/1,378 legacy NBT physically byte-identical in every variant**; all six important template NBT fully decoded with block-position, palette and inventory checks (14/14 chests stocked), all six template pool references exist; `pack.png` unaffected; 246 placed-feature references and **334 dependency DAG edges without detected cycle**, all namespace structure-set IDs resolve; Nether type/noise heights match 1024 or 2032.
- Four content hashes and both source scripts, independent QA report, installation info and target Nightly source proof are included in the combined ZIP. The private ZIP and individual QA are also stored in **ChatGPT Library `/Horizon/Thalvoryn/RC57`**.
- Previous native Windows EXE v7.2 and **Photon WGSL PREVIEW (NOT FULL PHOTON PORT)** are preserved, not re-tested.

### Unfinished, avoid false 100% claims

**No actual Minecraft 26.3 Java25 server nor Horizon Nightly was run** (environment has Java21; direct downloads are unavailable). There is no measured spawn rate, large-structure full-footprint/cross-chunk validation, new true custom biome registry/F3, safe high Nether portal transfer, LOD black/white band fix, FPS/TPS benchmark, full Photon 1.4 native WGSL visual parity, GUI runtime test, or final 2k Nether gameplay validation. Existing generated chunks will not retroactively populate with new landmark spawns.

The overall **~51%** figure is a qualitative estimate of project work completed, not runtime test success. **Not stable.** Do not publish private imported maps and NBT to public GitHub until licenses/attribution are resolved. Only this documentation is public. Issue #2 remains OPEN.