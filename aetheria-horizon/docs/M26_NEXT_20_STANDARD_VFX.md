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
