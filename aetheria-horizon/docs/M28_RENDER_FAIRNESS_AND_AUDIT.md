# M28 — fair multi-spell geometry, bone-wall headroom, and repository audit

**Branch:** `aetheria-30-spell-test` — internal development only, not merged into `main`.  
**Date:** 2026-11-19. **Release gate:** NOT MET; no user ZIP or manual playtest.

## What was actually inspected

Reviewed the live GitHub branch tree (208 entries), its latest M27 commits,
specification and original-prompt traceability index, 200-slot catalog,
200-route dispatch, all relevant renderer phases, the 500-facet shape cap,
M27 two-camera SVG visual regression, WGSL material, native GPU research
prototype, Audio/GUI/acquisition/core architecture, workflow paths and
current GitHub Actions status. This is a source/CI audit, not an in-engine
Windows test or proof that 200 spells look visually beautiful.

## Confirmed architectural limits

- `main` ends at older M11. The M12–M28 spell visual work is on
  `aetheria-30-spell-test`, **not** on `main` yet.
- M27 gives 200/200 individually routed **client VFX identities** (30 curated,
  110 Standard, 40 Awakening, 20 Ultimate), not 200 server-world abilities.
- All 170 non-curated gameplay intents retain the `DesignIntent` server
  rejection guard. No real 200-spell damage/healing/summon/world editing,
  network-safe acquisition or persistent Horizon save loop is verified.
- Original GPU raymarch is **source-only**, not connected to an actual scene
  color/depth native postpass; guest world-quad shader remains a limited
  animated per-face material. No HDR, verified GPU lights, occlusion/refraction.
- The existing 30 OGG clips are school/stage combinations, **not 200 distinct
  spell-specific original SFX sets**. Rapier3D lives in a separate simulation
  crate and does not control in-game spell collisions yet.
- The M100 gate counts **0 / 100 fully integrated**. Static geometry tests
  and 200 router IDs must never inflate this number.

## M28 changes

### Real world render-code correction

Prior `submit_custom_geometry` evenly split the frame budget and constructed
each `Scene` with that small capacity. Once the capacity was reached,
every subsequent face was simply skipped. For eight visible casts and 900
world-quads, this could render the first 112 faces and entirely omit the
later impact/roots/armor/body components, even when the complete authored
source was clearly readable.

`fair_geometry.rs` now works in two stages:

1. Build a capped **source draft** of up to 500 faces per effect for multiple
   simultaneous casts (retaining up to 900 for a solo spell, which was the
   old solo limit).
2. Use water-fill allocation across every active source draft, returning
   unused face capacity from small spells to detailed ones. Sample evenly
   across the **entire source** (including its first and last faces), instead
   of clipping the source tail, under a global maximum 900 submitted quads.

Three regression tests check spare-budget redistribution, an eight-cast
900-quad limit with deterministic source-edge inclusion, and zero/tiny
capacity behavior.

**Visual limitation:** selecting separated surface faces can expose holes
and incomplete facets when many large spells overlap. This fixes the
missing-end-of-spell bug, **not** professional solid-mesh LOD. Future work
must use coherent component/mesh LOD rather than sparse face samples. CPU
source drafts also consume more CPU time than truncated short scenes, and
need actual frame-time profiling on Horizon/RX 580.

### Bone Palisade (#194)

Previous authored source used 486/500 faces even before overlap. Both
outer ribs per bone column were 12-face pointed crystals. Replaced these
with five-sided 3D curved-looking *rounded tapered bone tubes* connected
to the existing thick ossified plates and vertical structure. Expected
authoring-cap headroom is now more than 100 faces. A regression assertion
requires no more than 400 faces during its Cast sample. Actual two-angle
source-image and Windows runtime quality must be checked before approval.

## Validation status

- Initial M27 internal pinned Horizon WASM: [44629836762](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762), **28 passed**, 860 diagnostic views, ZIP/WGSL/30 OGG/SHA512 checks.
- M28 introduced code commits `50e7a1f`, `141fae5`, `aa06755`, `b3c9d42`.
- **M28 pinned WASM run:** [44632013016](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013016) — **SUCCESS: 31/31 Rust preview tests, 0 failures**, including three M28 fairness tests, the Bone Palisade face-headroom assertion, all M27 camera/shape and 200-route coverage gates; pinned Horizon WASM `.hmod` compiled, 860 CPU geometry audit projections exported, 30 generated OGG events, Naga WGSL checks, ZIP and SHA512 integrity verified. These checks do NOT establish Windows in-game visual quality or multiplayer gameplay.
- No manual in-game testing or final release, by explicit user request.

## Highest-priority next steps

1. Obtain and inspect updated geometry artifacts with both camera angles for
   #194 and overlapping casts. Replace scattered face sampling with coherent
   per-component LOD; test perceptual completeness, not only face counts.
2. Research and verify the actual native Horizon scene-depth/post processing
   interface; connect only after supported engine hooks are proven, Naga
   validation and fallback preserved. The existing native WGSL prototype is
   not yet a rendered shader.
3. Replace `DesignIntent` server rejections incrementally with distinct
   permission-safe world gameplay actions. Connect real sound, item
   acquisition/inscription and class progression to server-authoritative
   player saves/network before counting spells as completed.
4. Continue manual internal 200-spell art review vs source videos and reject
   remaining thin/default low-poly silhouettes. No test ZIP until the
   user's stated acceptance milestone.
