# M23 — Twenty bespoke Standard-spell VFX upgrades (2026-11-19)

**This is a development milestone, not 200 polished, playable spells.** The canonical source already defines 200 individually named spells and the client already has 200 selectable visual draft slots. The user wants all 200 to be *beautiful* and rejects palette swaps, rods/circles, and slop. Therefore this pass replaces two generic Standard slots per magic school with **20 individually authored original VFX compositions**, generated from the exact matching registry's name, mesh and behavior. Every scene uses the actual scene-quad engine, bounded four-stage timing, 3D surfaces or solids, and dedicated geometry tests.

## Ten original Instant spell signatures (fourth spell of each school)
| ID | Spell | Independently authored visible body |
|---|---|---|
| 004 | Furnace Step | Alternating molten running footprints with layered fiery feather-flames |
| 024 | Glaze Prison | Large six-sided enclosed translucent crystal cage, material buttresses |
| 044 | Feather Drift | Sixteen asymmetrical wide 3D feathers suspended over a vortex of air |
| 064 | Faultline | Solid staggered fractured terrain blocks branching down a crack path |
| 084 | Riptide Snare | Multiple connected spiraling water sheets and strong water crests |
| 104 | Briar Loop | Heavy ring of actual physical briar crystal/vegetation and leaf panels |
| 124 | Arc Net | Filled layered triangular electrified containment sheet, solid center |
| 144 | Moonlit Veil | Spatial silver phase curtains and thick inner refractive-looking core |
| 164 | Sands of Delay | Distinct floating upper/lower hourglass chambers, falling sand fragments |
| 184 | Blood Thread | Four broad braided crimson organic ribbon membranes, solid magical core |

## Ten original Charged spell signatures (fifth spell of each school)
| ID | Spell | Independently authored visible body |
|---|---|---|
| 005 | Ashen Veil | Multiple dense flowing ash drapes, fractured hot coal bits |
| 025 | Hail Carousel | Twelve orbiting chunky transparent hail asteroids with varied height |
| 045 | Vacuum Pin | Convergent angled wind sheets focused into condensed solid air core |
| 065 | Shard Bulwark | Overlapping broad physical gemstone shield plates, different widths |
| 085 | Mistcloak | Multiple spatial diffuse fog curtains and round airborne water droplets |
| 105 | Firefly Veil | Leafy 3D ward and nineteen emissive fly-like tiny faceted masses |
| 125 | Storm Coil | Triple broad climbing charged braided ribbons and substantial light heart |
| 145 | Rift Slice | Jagged filled crescent portal cleave with nonzero depth |
| 165 | Temporal Mark | Sculpted carved hour-mark body with inset hour-glass markers |
| 185 | Rib Cage | Twelve interlocking physically thick arched bone ribs rooted to the ground |

### Code and tests

- `preview-5spells-wasm/src/standard_forms.rs` / `charged_standard_forms.rs` have two explicit per-ID scene arms for **each school**, not a shared school-colored generic.
- Actual 200-slot `render_spell` dispatches these IDs before the generic `mass_forms` fallback.
- Independent Rust tests compare full emitted mesh coordinates ignoring color and assert finite geometry, time staging and vertex budget for each group. The existing 860-view SVG generator covers them.
- All `SpellAction::DesignIntent` rules and authoritative server rejection for these 20 remain unchanged: no invented combat or save functionality, and no caster mana deducted by an unimplemented server action.

### Limitations / no false quality claims

This is a first art-directed *geometry* pass: 20 more variants now have custom scripted silhouettes beyond the 30 curated, 40 Awakening and 20 Ultimate scripts. That still leaves **90 Standard semantic draft layouts** requiring dedicated art direction, plus material/lighting refinement for all 200. A mesh with different coordinates isn't automatically beautiful. Real native HDR bloom, depth-aware fog/refraction, terrain reaction, GPU timing on the user's RX 580, multiplayer and game-world functional effects are still unimplemented.

**Do not distribute this as a finished 200-spell mod or ask for intermediate user testing.** Confirm pinned SDK CI run and inspect diagnostic images after compilation; a green build is only technical validation, not aesthetic approval.
