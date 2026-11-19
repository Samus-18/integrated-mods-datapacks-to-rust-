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

## Actual 20-spell contact-sheet inspection and visual correction (2026-11-19)

The first full 200-spell WASM CI for the new 20 bespoke Standard compositions succeeded: [run 44609749293](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44609749293), **18/18 preview tests passed, 0 failed**, with the source-authored 860-view projection ZIP saved from artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal`. We **opened the actual 20 Cast SVG images** from this artifact and assembled a contact sheet. The quality check found clear remaining failures despite green CI: #025 Hail Carousel appeared as only four blank spheres because each orb consumed enough faces to exhaust the 500-quad allowance before the intended 12 hail chunks; #165 Temporal Mark appeared as a flat unreadable slab; #164 Sands of Delay looked like two smooth globes; #044 Feather Drift looked like disconnected thin sticks; #104 Briar Loop looked like a sparse cage of thorns rather than an organically interwoven structure. Some other designs are still basic geometry drafts.

Manual source reworks from these observed images:
- Hail Carousel: re-author twelve varied angular **faceted floating hail asteroid masses** from closed crystals (no blank spheres), reducing face cost.
- Temporal Mark: an actual **open time dial** with 12 hour stones and two triangular floating sandglass wings, rather than an opaque rectangular monolith.
- Sands of Delay: an **eight-sided two-chamber tapered sandglass body**, engraved top/bottom supports and streaming crystal sand instead of two ovals.
- Feather Drift: wide curved solid 3D **feather membrane bodies**, not a few thin kite-shaped spikes.
- Briar Loop: two interwoven rows of substantial living thorn roots, side leaves and a solid ground base rather than disconnected skinny spikes.
- Ashen Veil, Vacuum Pin and Mistcloak: reduced excessive full-body curtain or sphere quads; these previously silently clipped at the exact **500 emitted-face cap**, hiding intended last detail. Reduced the overlapping curtain count, replaced water droplets with cheaper individual crystals, and strengthened the Charged test to reject exact-cap saturation.

**Do not mark these artistically approved** until the latest CI artifact is reopened. This audit is real Rust CPU geometry rendered to SVG from two camera projections, not the Horizon shader's output. It cannot prove actual depth, HDR material, glowing fog, GPU performance or visual equivalence to the original four videos.


## Final source-geometry audit and verified build

Latest code commit `88096ac7279a898e402c2680218f8864bdd8fd5e` passed the full pinned Horizon SDK workflow [run 44611449822](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44611449822): **18/18 native-host preview tests passed**, all 860 source-geometry diagnostic views exported, full `.hmod` package, SHA512, 30 OGG and WGSL integrity verified. The test now rejects a **500-face clipped** scene for each of the 20 new Standard signatures.

Opened the **final 20 Cast SVGs** from run artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal` and compared with the initial source snapshots. Actual observed changes:
- #025 Hail Carousel now visibly shows **numerous angular floating ice chunks**, not four smooth spheres; emission **288** vs **500 clipped**.
- #165 Temporal Mark now reads as an open 12-marker **time dial and sandglass**, not an opaque square plaque; 132 faces.
- #164 Sands of Delay is now a recognizably tapered **glass hourglass**, not two round blobs; 388 faces.
- #044 Feather Drift has broad leaf/feather faces, although no proper glowing downwash yet.
- #104 Briar Loop is more dense and organic, now **436 faces** under the 500 limit rather than truncating second-layer vegetation.
- #005 Ashen Veil now **428** faces, #045 Vacuum Pin **446**, #085 Mistcloak **396**; no truncated last effects.

The final screenshot audit still shows BASIC source geometry, not the 2026 reference-video-quality light/motion that the user wants. #124 Arc Net remains too much like stacked triangle cards, some storm ribbons are plank-like, and #004 Furnace Step looks more like standing fire blades than glowing footprints. These and the other **90** uncustomized Standard silhouettes need targeted further art direction and, importantly, a real depth-aware native GPU shading bridge. No user manual test requested; no release claimed.
