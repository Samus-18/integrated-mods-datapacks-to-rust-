# M26 — Twenty more source-authored Standard spell scenes (2026-11-19)

Internal-only development branch: `aetheria-30-spell-test`. The user requested continued 200-spell improvement without ring/stick/clone slop. The canonical **200/200 spells already existed**. Previously M23–M25 upgraded four additional Standard positions per school, leaving 50 semantic draft Standard layouts. This milestone adds actual per-ID Rust world-space compositions for positions **10 and 11 per school**, for **20 more** non-generic designs.

## New 20 explicit designs

| IDs | Original named spells | Individually scripted primary forms |
|---|---|---|
| 010, 011 | Searing Javelin; Kindle Chain | Heavy flaming javelin with broad fins; six multi-part linked coal hooks |
| 030, 031 | Snowblind; Frozen Tether | Three overlapping solid-centered blizzard curtains and drifting crystals; ten offset 3D ice links |
| 050, 051 | Thunderhead Draft; Suction Lattice | Dense cloud bank with rain crystals; nine triangular contracting pressure sheets with heavy focus |
| 070, 071 | Obsidian Needle; Tremor Echo | Tall dark chipped obsidian monolith with lower stone bracing; three staggered rows of real grounded quake blocks |
| 090, 091 | Whirlpool Ring; Brine Mark | Five sculpted rotating water crests; salt rosette growing from a broken wet plinth |
| 110, 111 | Mushroom Lantern; Sapling Chain | Articulated fungal stalk and wide lantern cap; five rooted miniature tree nodes interconnected by broad vine sheets |
| 130, 131 | Bolt Weaver; Conductive Seal | Wide stitched ground-energy geometry, thunder protrusions; carved heavy copper plinth with central conductor |
| 150, 151 | Orbiting Moons; Constellation Snare | Three substantial asymmetric crescent/crater bodies; seven 3D solid star nodes coupled by translucent capture planes |
| 170, 171 | Rewind Stitch; Timeglass Snare | Five shifting wide golden time ribbons; tapering time funnel with spatially suspended time-frozen arrow shards |
| 190, 191 | Crimson Pool; Spirit Bell | Filled low ritual reservoir with submerged shards; tapered enclosed spectral bell body, raised crown and internal clapper |

Code: `preview-5spells-wasm/src/standard_m26.rs`; actual client routing in `src/lib.rs`, before semantic fallback. Four-stage timeline growth and afterglow fade come from the established shared Rust contract. New test rejects identical geometry with color removed, non-finite coordinates and hard 500-quad truncation. Existing GitHub Actions preview audit should sample all 170 extended spells in Cast/Impact from two angles.

**Current meaning of completion:** 30 curated + 40 Awakening + 20 Ultimate + 60 dedicated additional Standard scenes = **150 explicitly scripted source-named identities**, leaving **30 additional Standard semantic visual drafts**, not 200 professional visually approved spells. The 170 extended cast mechanics still are `SpellAction::DesignIntent` and correctly rejected by server authority; there is no claim of damage, healing, AI summoning, real teleport, save or multiplayer functionality in Horizon.

## No-slop art gate

The authored shapes are deliberately not generic color swaps. Even so, their visual quality must be manually assessed from the next actual 860-view SVG artifact (Cast and Impact, both camera angles). A polygon-count or uniqueness unit test cannot certify beauty, lighting, translucency or visual similarity to source videos. The pinned client shader still cannot substitute true HDR, depth-aware post and native GPU integration. Review weak silhouettes and repair them before claiming acceptable quality. Do not distribute unpolished user builds.


## Actual initial 20-view art audit and first repair pass

Pinned WASM build and geometry CI for M26 pre-review source passed [run 44620581505](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44620581505): **24/24 preview unit tests, 0 failed**, generated 860 diagnostic SVG views and verified the WASM .hmod ZIP, original 30 OGG sounds and WGSL. The audit artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal` was downloaded and the actual 20 relevant Cast projections were decoded and inspected in a 5×4 contact sheet. Every one had nonempty spatial geometry, but several **FAILED the no-slop artistic gate**:

- **#150 Orbiting Moons** was just three smooth purple balls, unrelated to the referenced fractured moon look. Replaced with three thick sculpted chipped *crescent* moon segments, individually varied.
- **#171 Timeglass Snare** was a visually meaningless pile of intersecting thin crystals. Replaced with a connected, enclosed and visibly tapered double glass chamber and suspended arrow shards.
- **#110 Mushroom Lantern** was a boxy green trunk under a flat roof, not a lantern mushroom. Replaced with a faceted radial mushroom dome, broad shaded gills and luminous pointed under-hanging caps.

This is a *source-driven correction after seeing real diagnostic geometry*, not a generated concept image. A second full CI must pass and two-camera views of these new shapes should be reopened. **The 200-spell content is not artist-approved and does not have real engine HDR/bloom/depth effects.**


## Final verified build and second real SVG re-review

Following initial failure triage, source commit `811b44691998de5e9775c7887eb85abac5c76b2f` repaired #110 fungal dome, #150 chipped moon crescents, and the first version of #171 tapered time chambers. A second actual SVG review showed that **#171 still looked like a handful of crossing sticks** because edge crystals dominated transparent faces. Final fix `ef08d2668d85ef3e96d54fef41379029ff96ff8c` removed the thin ribs and strengthened solid chamber material/opaque caps. The **final source-generated Cast SVG** was opened: it now clearly reads as a substantial two-chamber hourglass, with a narrow middle and broad top/bottom; it is no longer a bundle of empty rods. The actual source image is **not an in-game screenshot**.

**Successful final pinned Horizon WASM CI**: [run 44622062765](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44622062765) — **24/24 Rust preview tests**, 0 failures; all **860** source geometry diagnostic SVG snapshots generated; .hmod package, WGSL, original 30 OGG SFX files, ZIP/SHA512 checks passed. No claim of true HDR, actual GPU illumination, real world spell mechanics or playtested compatibility on Windows. The current original 200-spell registry has **150 explicitly scripted per-ID visual compositions** and **30 remaining semantic first-pass standard drafts**; none of this guarantees visually polished cinematic spell quality for all 200.
