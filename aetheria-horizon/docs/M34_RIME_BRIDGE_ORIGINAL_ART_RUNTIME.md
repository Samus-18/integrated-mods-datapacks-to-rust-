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

Blender 5.2 generated eight independently closed eight-sided faceted slabs with
colored ice crown bevels. First two-view Blender renders were inspected and
**rejected as plain disconnected white blocks**. Revised the SAME source
to add TWO sweeping, thick, deeply curved and watertight support buttresses
beneath the walkway: **10 independent sculpted solids total**, without circles,
cylindrical rods, empty rings or crossed billboard planes. The second
three-quarter and side Blender renders were inspected. It now clearly looks
like a sculpted bridge, although it remains relatively low-poly and lacks
cinematic refraction/lighting. **Do not claim beauty user approval.**

## Concrete code and model

- `asset-pipeline/rime-bridge/rime_bridge.py`: authored Blender geometry,
  structured per-material faces and per-plate asymmetry; each entire source
  piece validated for closed edge manifold and **positive signed volume**
  *inside Blender before export*, preventing M33 inward-normal regression.
- Editable 10-solid `export/rime_bridge.blend`, 3D `.glb`, `.obj`, two
  genuine Blender Eevee images. Original art, no copyrighted external assets.
- `asset-pipeline/mesh_to_rust.py`: reusable deterministic OBJ→Horizon quad
  converter (next spells can reuse it). Validates every complete closed
  object, edge incidence exactly 2, positive signed volume, source SHA
  independent of Windows/Unix line endings, material categories and
  finite coordinates. Splits convex 6/8/10/12-sided caps into whole quads,
  never removes the top/bottom shell.
- Generated `src/rime_bridge_mesh_generated.rs` contains
  **456 generated vertices / 436 preserved 3D quads**,
  ten individually addressable atomic full-body component ranges
  (8 × 32 plate quads; 2 × 90 support quads).
- `src/rime_bridge.rs`: original **4-phase** visual: ice plates
  rise/grow sequentially outward (ages 4–49), long structural buttresses
  unfurl and hold (ages 29–80), both dissolve by 104 ticks. Local
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
  10 GLB meshes / 3 materials, exact source SHA, two Blender renders,
  all 10 closed outward normals, source Rust generator reproducible.
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

### M34.1 consistency and cross-platform reproducibility correction
First pinned CI [run 44835909385](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44835909385) **FAILED** on a reproducible source check on Linux (fresh converter result != committed .rs), before WASM compilation. Separate Rust 9/9 source renderer + 78/78 core tests and Rapier succeeded in [run 44835909286](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44835909286). This failure is not hidden or a shader pass. Generalized OBJ tool now derives vertices, signed volumes and ngon cap centers using **exact decimal arithmetic**, giving a deterministic fixed-eight-decimal Rust source independent of binary floating-point rounding differences. Fresh pinned CI still required to confirm.

A source/core consistency pass found the canonical server-side `SpellAction::RimeBridge` plans **8 plates**, while the initial M34 art draft had only 7. Regenerated the Blender sculpt as **8 original plates + 2 curved buttresses** (ten watertight components) aligned to the future action. New figures: **436 source OBJ vertices / 380 original faces, 456 Rust vertices / 436 full world quads, 10 atomic components** (8×32 / 2×90). Added a core test ensuring action `plates=8`, spacing and expiry remain consistent. Still no actual collision or world block edits; everything here is a visual and portable-rule QA change.

## M34.1 final verified CI and source-projection ART review — 2026-11-21

- Full pinned Horizon Nightly 2026-11-20 WASM pipeline **SUCCESS** [run 44836927188](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44836927188), head `dc6ea667edc991360751c3b0650e6a7b6e73ef9d`. **47/47 Rust preview tests** passed, zero failures; guest `.hmod` build, model source/digest/reproducibility check, 10 outward-closed source bodies, pinned Naga shader checks, 30 original OGG files, full ZIP integrity and WASM SHA512 verified. Visual audit artifact produced **866 files** including 860 offscreen geometry views. This proves the package builds, not that the user approves the VFX in Horizon.
- Portable core job [run 44836927249](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44836927249) **SUCCESS**: **9/9** exact Rust original ice renderer tests; **79/79** portable RPG/rule tests; independent Rapier3D physics job **SUCCESS**. The new spell 22 8-plate action consistency test is included.
- Downloaded actual `Aetheria-M34-Visual-Audit-v0.8.2-Internal` artifact (ID 14678573216); visually inspected a 2-row contact sheet of native Rust-renderer source-quad projections (NOT AI images and NOT Horizon screenshots): `spell_04_01_view1`, `04_02_view1`, `04_03_view2` for #021 Frost Needle; `spell_05_01_view1`, `05_02_view1`, `05_03_view2` for #022 Rime Bridge. Frost cast reads as filled blue irregular lance, impact as a busier exploded cluster, afterglow as two full independent shards. Rime cast/impact stages currently show **only 1–2 rectangular-looking ice slabs** at first, even though the afterglow shows a full long bridge with visibly independent plate seams and two supporting fins. Art verdict **NOT ACCEPTED YET**: early-stage isolated slab has too little material complexity and late bridge still needs luminous glacial growth, physical-world anchoring and more original surface texture. Existing OBJ shape correctness does NOT certify cinematic beauty.
- Internal QA .hmod artifact SHA-256: `5b32a6c2d7366eb7a35ee9b14324de56bfdc60ede88afe076ca26c0770703339` as reported by successful workflow. Do **not** ship as polished 200-spell gameplay mod; no Horizon runtime video/GPU/FPS or authoritative block collision yet.
