# M29 — coherent 3D component LOD under simultaneous spells

**Branch:** `aetheria-30-spell-test` (not `main`). **Date:** 2026-11-20.  
**Delivery:** INTERNAL SOURCE ONLY. The M100 fully playable milestone is still
0/100; no user-facing unpolished ZIP or new game-playtest request.

## Actual bug fixed

M28's water-fill budget split evenly sampled *individual world quads* across
all scenes. It preserved the first and last authored source elements, but cut
holes in solid 3D topology under 8 overlapping spells: for example a 6-face
boulder might render as 3 separated faces, a 126-panel orb as broken sparsely
visible stripes, and a 6-panel membrane as an open paper wing. The absolute
900-face ceiling was respected but the silhouette could be structurally
destroyed.

## Implementation

1. `preview-5spells-wasm/src/lib.rs`: `Scene` now tracks a sequential
   `group_ids` entry per emitted quad. `begin_component(faces)` reserves
   **the complete required component face budget** or emits nothing;
   `end_component` restores nested ownership. Source `quad` outside an
   atomic component still becomes a valid 1-face group. Original per-scene
   900 solo-quads remain unchanged.
2. `preview-5spells-wasm/src/fair_geometry.rs`: budget is water-filled as
   before, but output now selects only **whole authored components**.
   Endpoint-first recursive-bisection order reaches late sculptures without
   biasing to initial authoring order; a deterministic round-robin recovery
   pass reuses spare faces. Full source face ordering is restored at the
   submission boundary. Component limits take priority over filling the last
   free slot. No random face removal from a closed group.
3. `lib.rs` physical helpers: actual `shard`, `membrane`, `tube`
   and `solid_crystal` methods are now grouped atomically at 2, 6, 5–8,
   and 10–16 side faces respectively.
4. `ultimate_forms.rs`: boxes (6 faces), full terrain slabs (16),
   complete elliptical orbs (**126 faces high**, **48 low**) and thick
   billowing veils (**80 high**, **35 low**) are grouped. The
   low-tessellation **closed form is generated** when at least four spells
   are live or the per-cast available budget falls below 180, instead of
   destroying high-density shell panels afterward.
5. `mass_forms.rs`: full 6-face boulders, each 49-face curved sky-rift
   membrane, 70-face hourglass and 80-face artifact shell gain atomic
   source grouping.
6. `cinematic_beams.rs`: curated Blink Fold veils/bridges, Moment Slip
   shutters, time-vessel bodies, temporal dome shells and Astral Lens
   receive group-aware solid LOD; busy versions use fewer tessellations
   but remain spatially filled. Existing supporting crystal/debris groups
   can still be selected independently.
7. `visual_audit.rs`: new eight-actual-spell stress test renders seven
   Ultimates plus Bone Palisade at Cast/Impact age, confirms each cast
   remains present, no selected group loses any panel, and projects the
   submitted bounded scene from two camera angles.
8. `ultimate_forms.rs` and `fair_geometry.rs` tests explicitly guard
   6-face group integrity, first/last sculpture pieces, 900-face limit,
   single body over-budget refusal, and exact high/low orb tessellation.

## What this does NOT solve

- M29 is **component-aware world-quad scheduling**, not true GPU native
  instancing or mesh simplification. Source meshes are still CPU-built.
- Some older scene-specific raw `Scene::quad` loops not wrapped in groups
  intentionally remain single-panel selectable, and some long roots/body
  assemblages can still become disconnected across subcomponents. These
  must be audited and grouped by semantic structure in subsequent work.
- If an entire group is bigger than the real draw budget, the whole
  component is skipped rather than being punctured. Next improvement:
  coherent alternative *whole* lower-detail model for every such case,
  per-model priorities and distance-aware reduction, especially at
  very high 20–64 simultaneously active effects.
- High-poly polygon counts do not certify the source-video aesthetic
  standard. The two diagnostic SVG projections are **not real game
  screenshots**, cannot reveal GPU lighting, shader correctness or actual
  FPS on RX 580.
- A native Horizon depth-aware HDR/refraction pipeline, authentic world
  spell physics/damage/healing, networking, save, unlocks and the 170
  server-rejected `DesignIntent` effects are still incomplete. 30
  school-level original OGG cues are not 200 unique polished spell sounds.

## Validation and next steps

The M29 source commit series starts at `2a63cc2`; see active branch
history for later exact commits. GitHub CI must pass pinned Horizon
`cargo horizon-mod build --wasm`, 200 route and 2-camera geometry
tests, OGG/ZIP/WGSL/SHA512. **As of document creation: new M29 WASM
CI was still in progress; do not report it passed until actual logs
are checked.**

Next work: inspect all busy 3D projections and actual frame-time limits,
tighten semantic grouping in remaining per-ID curved surfaces, then
advance the true server/world adapter and playable action inventory.
Do not present a source-only visual test as a finished spell.
