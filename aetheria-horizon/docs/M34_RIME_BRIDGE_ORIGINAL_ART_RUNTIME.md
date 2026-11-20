# M34 — Rime Bridge: original 3D glacial walkway art / Rust Horizon VFX

Date: 2026-11-21. Active development branch: `aetheria-m33-ci` (draft PR #3),
based on the latest `aetheria-30-spell-test`. M33 prior pinned WASM
[Actions run #44833699687](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44833699687)
completed **SUCCESS**; M33 separate Rust CI 5+78 tests and Rapier passed.
M34 has its own CI, do not copy M33's success as proof of M34.

## Goals, rejected approaches, real visual inspection

Earlier real-game Aetheria videos proved hundreds of colored rods, circles,
cards and thin spikes looked bad. M33 succeeded at connecting ONE original
Blender body to Horizon's actual client world-geometry draw path. M34 deliberately
chooses a **different shape and gameplay role** for original spell #022:
Rime Bridge, a geometrically substantial suspended ice walkway.

Blender 5.2 generated 7 independently closed eight-sided faceted slabs with
colored ice crown bevels. First two-view Blender renders were inspected and
**rejected as seven bland disconnected white blocks**. Revised the SAME source
to add TWO sweeping, thick, deeply curved and watertight support buttresses
beneath the walkway: **9 independent sculpted solids total**, without circles,
cylindrical rods, empty rings or crossed billboard planes. The second
three-quarter and side Blender renders were inspected. It now clearly looks
like a sculpted bridge, although it remains relatively low-poly and lacks
cinematic refraction/lighting. **Do not claim beauty user approval.**

## Concrete code and model

- `asset-pipeline/rime-bridge/rime_bridge.py`: authored Blender geometry,
  structured per-material faces and per-plate asymmetry; each entire source
  piece validated for closed edge manifold and **positive signed volume**
  *inside Blender before export*, preventing M33 inward-normal regression.
- Editable 9-solid `export/rime_bridge.blend`, 3D `.glb`, `.obj`, two
  genuine Blender Eevee images. Original art, no copyrighted external assets.
- `asset-pipeline/mesh_to_rust.py`: reusable deterministic OBJ→Horizon quad
  converter (next spells can reuse it). Validates every complete closed
  object, edge incidence exactly 2, positive signed volume, source SHA
  independent of Windows/Unix line endings, material categories and
  finite coordinates. Splits convex 6/8/10/12-sided caps into whole quads,
  never removes the top/bottom shell.
- Generated `src/rime_bridge_mesh_generated.rs` contains
  **398 generated vertices / 380 preserved 3D quads**,
  nine individually addressable atomic full-body component ranges
  (7 × 32 plate quads; 2 × 78 support quads).
- `src/rime_bridge.rs`: original **4-phase** visual: ice plates
  rise/grow sequentially outward (ages 4–44), long structural buttresses
  unfurl and hold (ages 27–80), both dissolve by 104 ticks. Local
  cast-facing horizontal orientation, stable if aiming vertically down,
  exact physical mesh source faces (not fake GLB loader), material per face,
  finite safeguards and all-or-nothing component LOD.
- In **actual** `src/lib.rs::render_spell`, original #022 routes into the
  new sculpt module *before* generic `volumetric_forms`, while #021 still
  uses M33 Frost Needle. Remaining **198** IDs keep prototype art.

**Important gameplay truth:** THIS IS A GHOST/VFX WALKWAY ONLY.
It creates **no physical world blocks, collision, inventory, item,
crafting, server-authoritative bridge, or traversable path**. Calling it
a working traversal spell would be incorrect and unsafe. Real gameplay
requires server-owned temporary block operations with proper expiry,
saved-world consistency, permission checks, client/remote sync and
all-world rollback, separately integrated to a verified Horizon native
server API.

## Research after 2026-11-20

- Current official Horizon native mod build manual:
  https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/how-to/native-mods.md
- Current Horizon shader pack API **0.2**, linear-light color and world
  color/depth post resources:
  https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md
- Horizon renderer accepts optional world geometry material pipelines;
  stock WASM guest's available `PipelineInput` does NOT automatically
  provide a full-screen mod-local post pass. New shader pack post stages
  are their **own separate** category. Do not fuse them through a nonexistent
  API or claim global HDR light.
- Native mod reference project:
  https://github.com/horizon-rust/horizon-native-mod-template
  (its source-mixin `.hnmod` route rebuilds an engine; **not** equivalent
  to our pinned `.hmod` guest and not copied into deliverables).
- `Stink-O/orbital-laser` advanced client+server native renderer
  study: https://github.com/Stink-O/orbital-laser/blob/main/docs/HOW-IT-WORKS.md.
  It references additional fork-specific APIs. No third-party source copy.
- `SpellEngine` 3D model FX + stage choreography:
  https://github.com/ZsoltMolnarrr/SpellEngine/blob/1.21.1/CHANGELOG.md.
  Java/Fabric/NeoForge packages incompatible with Horizon Rust.

## Verified / unverified

- Blender 2-camera real images: reviewed; first simple block-only sculpt
  rejected, second structural model accepted as a more substantial **source
  experiment**, NOT final art quality.
- `python asset-pipeline/rime-bridge/validate_bridge.py` passed locally:
  9 GLB meshes / 3 materials, exact source SHA, two Blender renders,
  all 9 closed outward normals, source Rust generator reproducible.
- Separate Rust CI compiles the **same** `src/rime_bridge.rs` against the
  source shape of `Scene` and tests component geometry, four stages,
  finite values, horizontal facing and hard quad budget. Pin to SDK
  Nightly 2026-11-20 remains required.
- Native actual screenshot / 60fps / GPU profiling / physical bridge:
  NOT TESTED. Do NOT issue finished ZIP, and do not merge PR until real
  Horizon compatibility and user-level visual quality pass.

### Delivery policy

Keep M34 in a draft branch, not a false completed mod. Preserve
`Skyfall-Lost-Islands` untouched (Roblox project stopped). M34 video and
code tests are source/CI, not proof of in-game aesthetic quality.
