# Aetheria Horizon — validation ledger

| Milestone | Verification | Evidence | Scope |
| --- | --- | --- | --- |
| M0 | 19 Rust unit tests passed | [GitHub Actions 44394835431](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44394835431) | Portable Rust logic only |
| M1 | 28 Rust unit tests passed | [GitHub Actions 44395652564](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44395652564) | Mana, casting, spellbook, reactions |
| M2 | **33 passed / 0 failed** | [GitHub Actions 44397904415](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44397904415), code commit `2016ec34013ed97970e1e3ef1984be441b25163d` | 20 portable action intents, timelines, invalid-input guards |


| M3 | **39 passed / 0 failed** | [GitHub Actions 44482049153](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44482049153), commit `cec3131b3d9b2a13de8985a49d6276c5d8aafe02` | 20 distinct 3D VFX instance silhouettes; fair per-frame budgets and guard tests |

Command reproduced in CI:

```sh
cd aetheria-horizon
cargo test --all-targets
```

Initial M2 run 44397753631 failed **one legacy unit test** checking that spell ID 2 was unavailable. ID 2 (Cinder Fan) became implemented in M2, so the test now asserts on ID 3 (still design-only); subsequent run passed all 33 tests.

## Validation **not** performed yet

- Native Horizon launcher SDK compilation, manifest validation or playable `.hmod` build
- WGSL validation with Naga or GPU render integration
- Multiplayer/server-authoritative target, world saves, rollback or networking
- Actual VFX/audio/UI in a Minecraft 26.3 Horizon world
- VRAM/performance budgets or visual equivalence to the four supplied video clips

The word **implemented** in project documentation means the standalone Rust **action intent**, not working in-game magic.


## M27–M28 internal pinned Horizon branch (2026-11-19)

This section supersedes the old validation ledger's broad 'not yet compiled' statements for the **precompiled WASM preview only**. It does NOT supersede their separate warnings about real server/world integration and native GPU HDR.

| Milestone | Verified | Evidence | Still unverified |
| --- | --- | --- | --- |
| M27 | 28/28 preview Rust tests, 200/200 distinct scene routes, 860 source 3D SVG diagnostics, pinned Horizon WASM .hmod/SHA512/30 OGG | [Actions 44629836762](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762) | User game appearance, depth effects, world damage/network, coherent GPU silhouettes |
| M28 | **31/31 preview Rust tests**, three new fair-quad-allocation tests, full source and 120 M27 2-view gates, valid pinned WASM .hmod/OGG/WGSL/ZIP/SHA512 | [Actions 44632013016](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013016) | Real FPS, visually coherent LOD under many casts, native GPU postpass, 170 actual world mechanics |
| M28 portable Rust + Rapier | Both jobs completed SUCCESS | [Actions 44632013002](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013002) | Collision physics wired to Horizon block/entity world |

**Counting policy:** 200/200 authored client scene identities are complete as renderer routes; 0/100 user-accepted fully integrated RPG spells are verified. Shader Naga compilation and 860 CPU SVG geometry snapshots are not Minecraft in-game screenshots or aesthetic sign-off. Final user-facing package is withheld.


## M29 — component-aware world-quads and real geometry-source QA (2026-11-20)

| Milestone | Verified | Evidence | Still unverified |
| --- | --- | --- | --- |
| M29 first atomic-geometry pass | **36/36 Rust WASM preview tests**, exact 8-spell concurrent 900-quad model-ownership test, closed shell high/low tessellation tests, 2-view combined stress SVG, pinned Horizon WASM `.hmod` build, 30 OGG, WGSL/Naga, ZIP/SHA512 | [Actions 44752358562](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44752358562) | Horizon Nightly in-world screenshots, actual HDR/occlusion/fps, visual artistic quality |
| M29 second bone-wall art pass | **36/36 Rust preview tests**, WASM package and integrity checks passed; downloaded actual two-camera before/after SVGs | [Actions 44753597607](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44753597607) | The 2nd draft introduced a large plain box: it **fails user visual quality** despite source geometry/tests passing |
| M29 third sculptural bone-wall revision | **37/37 preview Rust tests**, new two-sided relief anti-box regression, pinned .hmod/OGG/WGSL/Naga/ZIP/SHA512 passed. Actual Cast/Impact source SVG reviewed from two cameras and visibly improved over blank cuboid | [Actions 44754956222](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44754956222), source commit `5201c64d8171bcfb984fb059e494211ebebdcee6` | Faceted proxy shapes still lack cinematic GPU materials; no Horizon world/server game test, no 170 spell mechanics |

**Actual source image review is not a game screenshot:** M29 showed an 8-cast combined CPU projection (897/900 submitted quads, all atomic panels intact) that remained visually cluttered and source-art rough; the first Bone Palisade looked like separated fence posts; the second revision looked like an oversized featureless wall. Third revision needs fresh art review. The 900-world-quad scheduler and 200 route table do NOT implement actual GPU volumetric effects, 170 DesignIntent world spell mechanics, server saving, networking, or verified finished 100-spell RPG gameplay. **0/100 fully integrated/verified; no test ZIP.**


## M30 — live procedural surface materials (2026-11-20)

| Check | Verified | Evidence | Not yet validated |
| --- | --- | --- | --- |
| M30 Horizon WASM + shaders | **38/38 preview tests PASS**; the 5 animated material families validate with pinned Naga and actual `PipelineInput.position` mock; glacier volumetric-like haze has coherent 3-sheet LOD; pinned `.hmod` compiles, 30 OGG, ZIP/SHA512 valid | [CI 44758785121](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44758785121) | In-game GPU output, true HDR/volumetric scene depth, reference-video visual fidelity and RX580 frame-times |

The new shader runs through the mod's **existing actual Horizon WGSL PipelineBlend::Translucent + WorldDepth::Tested** world quad path, not the unattached native post prototype. Source: [M30 visual materials](M30_LIVE_SHADER_MATERIALS.md). Pinned Horizon doc confirms 3D camera-relative `input.position`, linear `input.color` and `frame.time`. This does not change the **0/100 end-to-end gameplay verified** count: 170 gameplay `DesignIntent` actions are still unimplemented in a real world adapter.


## M31 — direct downloadable Horizon GPU haze test v0.7.0 (2026-11-20)

**Playable test archive** [Aetheria-Horizon-M31-GPU-Test-v0.7.0 (GitHub Actions 44763204220)](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44763204220) was produced by a **SUCCESS** pinned-SDK WASM workflow from source SHA `fe4c235a7fecf6c16b17e765ad75230dafcb398b`.

- Actual compiled `aetheria_preview.hmod` v0.7.0 (3,197,329 bytes) independently downloaded from workflow artifact `14642372974`, ZIP and `mod.wasm` SHA512 validated again. Download artifact ZIP contains `README-TEST.md` and matching `SHA256.txt`. `.hmod` SHA256: `c7edaa1fa8eee91057aed80c2b88aeecd0f99fff008dad7baf0fc37dd5cb1efe`.
- Workflow: host-side Rust test suite, Naga WGSL source validation, package checks, 30 OGG VFX audio cues, and geometry projections pass. **38/38 tests PASS, 0 failed**, independently confirmed from exact GitHub Actions run 44763204220 job logs; shader material and standalone native prototype Naga parser tests passed.
- Real shader now calls `aetheria_integrate_haze` on low-alpha original 3D scene sheets. **6/10 samples per pixel** by camera-relative distance, with explicit max 10 iterations and coherent low-detail sheet geometry. World depth testing at submitted atmospheric surfaces is available. No engine scene-depth texture is read by this WASM shader.
- **Not verified:** actually running the game on the user's Windows GPU and Nightly, HDR bloom, genuine fullscreen depth-limited volume compositing, performance/FPS, 200 true gameplay mechanics. Do not call this the final 200-spell RPG release; 170 core mechanics remain `DesignIntent`.

### M33 source-authored Frost Needle / 2026-11-20
- Original v2 Blender source exported; closed 3-part mesh, 2 actual preview renders and GLB/OBJ integrity PASS. Initial look rejected, taper and solid shoulder silhouette improved. No model copied from internet.
- Offline exact OBJ SHA256 -> Rust 130-vertex / 124-quad generator PASS; no topology loss through source conversion.
- Actual WASM client `render_spell` routes canonical Cryomancer Frost Needle (#021) to 4-phase mesh renderer; atomic 124-facet 3D groups protect silhouette under fair 900-world-quad budget. All other 199 render paths still experimental draft scenes.
- `validate_source.py` PASS on user's machine. WASM pinned SDK CI result pending actual run verification, not reported as success in advance.
- Real Horizon in-game visual, GPU HDR/depth/refraction and world-game mechanics NOT VERIFIED. See docs/M33_SHARP_PROJECTILE_REAL_BLENDER_RUNTIME.md.

**M33 geometry normal regression (new):** independent signed-volume OBJ audit caught all 108/108 reversed faces on first Blender v2 pass (despite appearing normal in Blender). Regenerated from corrected winding; all 3 closed parts now positive signed volume and **108/108 exterior faces point outward**. Reproduced on fresh Git Windows checkout, source SHA canonicalized LF/CRLF, Python source test re-passed. The pinned CI job must additionally pass `validate_normals.py`. Does not certify in-game shader/backface behavior.

### M34 2026-11-21
- New original Blender 8-plate / 2-glacial-buttress Rime Bridge sculpt. Original reviewed first two camera views failed aesthetically as simple blocks; second two actual Blender angles show a substantial curving supported bridge. Not an in-game approved visual.
- OBJ/GLB 10 watertight outward 3D solids; SHA canonical LF; 456 3D vertices, 436 fully preserved material-marked quads; 10 atomic 32/90-quad parts. Reproducible generalized OBJ->Rust tool and independent `validate_bridge.py` source/GLB validator PASS on Windows.
- Real live spell router dispatches Cryomancer #022 `rime_bridge` before generic M32 renderer; #021 keeps M33 Frost Needle. Standalone Rust QA and pinned Horizon SDK CI result reported separately when known. No actual server bridge, collision/blocks or game FPS verified.

**M34.1 integrity regression:** first Linux pinned WASM CI 44835909385 stopped before compile because regenerated model Rust differed across hosts despite local pass. Revised model converter uses Decimal, deterministic cap coordinates and source bytes. Aligned Rime Bridge with existing action's 8 authored plates: 10 watertight components, 436 original vertices / 380 mesh faces, 456 generated vertices / **436 atomic world quads**. New portable cast rule test asserts 8 segments and expiry. Fresh pinned WASM CI must prove Linux reproducibility; earlier failure remains recorded. No game runtime or collidable platform verified.

### M34.1 final CI and actual-source-geometry views verified
- Head `dc6ea667`, pinned Horizon SDK `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`: WASM CI [44836927188](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44836927188) **SUCCESS**, 47 preview tests, .hmod verified against manifest SHA512, Naga WGSL, 30 audio OGG; source-model integrity and source-Rust reproducibility on Linux passed. Core/Rapier CI [44836927249](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44836927249) **SUCCESS**, 9 source-mesh tests + 79 core tests + Rapier. Exact 8 plate rule enforced.
- Retrieved 866-file M34 source visual audit and **actually opened** six original Rust geometry projections (Frost #021 and Rime #022 at cast/impact/afterglow, at 2 camera orientations). Projections are NOT in-game screenshots. Frost projectile identifiable but impact cluttered; Rime early cast only slab-like; bridge structure clearly present by 61 ticks but still low-poly. **No cinematic beauty approval**. Both client-only visual, no GPU timing or world effect.

### M35 procedural VFX QA (status: work in progress until CI)
- Binding art direction: docs/M35_PROCEDURAL_VFX_DIRECTION.md; real GPU material/emitters, no 200 Blender-model detour. Public references are analysis only and no source code was copied.
- Source: preview-5spells-wasm/src/procedural_fx.rs seeded flows in 10 schools / 200 slots; optional separate soft_particle.wgsl with four feathered masks; runtime dual-pipeline activation/fallback without plain unmasked cards.
- Tests added: 200 IDs x multiple ages determinism and quad bounds, 10 spatial signatures, facing orientation incl vertical camera, zero/invalid budget, LOD, Naga validation, package integrity. **Compilation/gameplay unconfirmed until new CI run finishes**.
- Explicit limitations: world geometry quads are not GPU instancing. This is not a true postprocessing hook or GPU bloom. No server effects, player saves, game FPS, or art approval. M34 original models preserved.

### M36 first batch of 100 individually named procedural spell FX
- Selected first 10 from each of 10 canonical schools; original `authored_100_data.rs` contains 100 named design profiles / 48 distinct motion families. Actual Rust `authored_100_fx.rs` routes them into optional soft-particle GPU pipeline; outside these IDs the M35 emitter stays active. **Not 100 approved hero spells**.
- [First passing core CI run 44906337416](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44906337416): **19/19** exact-source renderer tests plus **79/79** RPG rules tests, Rapier physics PASS; exported 10 exact Rust source-quad SVG school contact sheets for all 100 at two ages. Actual 100 previews were inspected; overly small dots and simple trails => ART FAIL, no misleading cinematic approval.
- New 12-category procedural material shaders added after first source visuals; pinned Horizon WASM/Naga latest result must be separately verified. No in-game screenshot, damage/collision, multiplayer or FPS claims.



## M36.2 review (2026-11-21) — four phases, 100 previews, verified optional glow

**Source and tests after the first M36 art FAIL:**
- Per-profile VFX now has four authored lifecycle phases: condensation, animated growth/travel, impact/sustain, dissipating afterglow. Structural shapes (bridges/cages/shields/pools) hold instead of exploding; projectile/impact shapes expand. The 100 profiles use 48 trajectory families and 12 soft WGSL silhouettes, plus larger but bounded source-geometry hero cores. Existing 198 draft solid art is **still visible** underneath as it was before; more work is necessary.
- Standalone GitHub core [run 44907311604](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907311604): **20/20** actual-source renderer tests and **79/79** portable RPG tests PASSED; Rapier physics passed. After new hero cores, [run 44907516076](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516076) **core-tests and physics PASSED**.
- Downloaded and inspected **latest v4 actual Rust-source material approximation**: ten school panels, 100 identified spell names, two phases each. This is not GPU shader output, Horizon frames, an AI image or a beauty acceptance. Despite larger hero quads and 12 material families, **v4 is still too visually basic** — flat orange swooshes, simple sparkling clusters and uniform strokes do not match God Wand/Photon-like cinematic spells. Our count of actually in-game-vetted polished spells remains **0/100**. The latest full source collage was created from the generated v4 PNGs locally without using the user's PC.
- A separate **optional** official Horizon shaderpack API **0.2** has been built in `shaderpacks/aetheria_spell_glow/`: 8-tap restrained chroma-aware global bloom poststage. [Pinned official `horizon-shader check` + `pack` run 44907197852](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907197852) **SUCCESS**, generated `.hshader` artifact. **This is NOT attached to or automatically enabled by .hmod.** Horizon can run only one own-format shaderpack at a time and installing it could replace an existing look. No user PC installation, no FPS data.
- Latest full **M36 `.hmod` pinned WASM** build [run 44907516110](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516110) was **still in progress** when checked; do not label that exact revision compiled until success reported.

**Future acceptance gates:** actual native Horizon 26.3 captures of all 100 cast phases, performance on RX580, gameplay-authoritative damage/status/collision/save, richer layered VFX hero silhouettes and true GPU shader materials, synchronization of 100 distinct sound/impact profiles. This source-geometry milestone must not be published as a visually completed 100-spell mod. **No Remote Desktop Commander connection used.**


### M36.2 pinned Nightly full QA success (2026-11-21)
Latest M36 WASM official pinned SDK [run 44907516110](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907516110) **SUCCESS 58/58 preview tests**, .hmod and 30 OGGs verified, shader/Naga, signed WASM SHA512 and ZIP OK; build SHA256 `7a592a197704f32f361f317b3b7ac102f03441218ef3066c99632d1312d2112f`. Optional standalone official Horizon 0.2 glow shaderpack [run 44907197852](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44907197852) **SUCCESS**. Both compiled outputs remain **experimental**, not in-game visual-quality accepted, 0/100 fully implemented playable/game-proven spells, no GPU screenshot or measured target RX580 FPS. No remote user's PC was used.

### M37 names-first shader signatures (not approved in game)
- Isolated M37 Git worktree based on verified M36 `35e298f`. No old builds overwritten and active Roblox was not interrupted.
- First two independent Rust source-quad image audits visually FAILED (blocky/small/fragmented), despite earlier 24/24, 25/25 tests +79 shared tests passing. Source screenshots are never Horizon GPU screenshots or AI generated art.
- New dedicated optional `signature_sdf.wgsl` implements 11 individual SDF/animated noise hero pixels; separate depth-tested `signature_pipeline`, one bounded proxy per cast, no raw rectangular fallback, prior 3D body only on shader failure. Additional exact source Rust tests and pinned Naga shader validation added. Do not mark PASS before current CI; no actual client FPS/screenshots or server spell physics yet.

### M40 live botanical 3D candidate (2026-11-21)
On the new `aetheria-m40-worldspace-thorn` branch, id-100 Thorn Dart was moved out of its rejected 2.5D SDF hero card into original, bounded, real 3D worldspace quads with four materially distinct age phases; optional dedicated tiny masked spores remain. Added tests for 3D volume, distinct phase geometry, directional travel and budget. CI and real Horizon GPU screenshots are required before claiming visual success; original M39 RX580 images are the negative baseline. World damage/authority and total 100-spell acceptance remain incomplete.
