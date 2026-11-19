# M14 — Individual visual rhythm, true 3D silhouettes, conservative shading

Date: 2026-11-19. Development branch: `aetheria-30-spell-test`.
This implements a **visual test subsystem**, not a fully working 30-spell multiplayer RPG.

## Source fidelity

The user's original master-prompt wording is not available in full in this repository. Before coding, we re-read `docs/PROJECT_SPECIFICATION.md`, `docs/memory/USER_LOG.md`, `docs/VIDEO_REFERENCE_ANALYSIS.md`, plus the archived original-video metadata in `references/original-videos/README.md`. A new `docs/memory/ORIGINAL_PROMPT_VISUAL_REQUIREMENTS.md` tracks exactly which quotes survive and separates them from summaries. Do not silently fill missing original text. Do not copy other mods' assets.

Visual references from the four user MP4s:
- (3) — layered fantasy ground ritual and sky-scale luminous pillar;
- (4) — deep dimensional fracture, world-space branching, controlled debris;
- (5) — gradual buildup, full-body burst and delayed world-scale afterglow;
- (6) — stepped growth of substantial ground-tethered 3D ice crystals.

## What changed

| Component | Implementation | Verified boundary |
| --- | --- | --- |
| `src/vfx/timeline.rs` | 30 distinct per-ID four-stage `Timeline` definitions (Telegraph, Cast, Impact, Afterglow) including 10 formerly missing third spells. | Independent Rust tests validate phase ownership, unique raw lengths and finite bounded durations |
| `preview-5spells-wasm/src/cinematic_beams.rs` | 30 individual world-space 3D compositions using six-sided crystals, pentagonal curved volume rays, solid ring arcs, root growth and lightning branches. `visual_phase` maps each core four-stage time ratio onto a 104-tick client-visible lifetime. | Stage-aware CPU tests check finite, bounded geometry and that full mesh coordinates differ ignoring all material colors |
| `preview-5spells-wasm/src/lib.rs` | Cast/Impact 3D background sculpts only, rather than overexposed background present during every phase; legacy sculpts cap at 34% of remaining capacity. Client SFX charge/cast/impact offsets now follow the unique per-spell timeline. | Old 900-quad global per-frame budget and 8 active effects remain |
| `assets/aetheria_preview/shaders/pipeline/ethereal.wgsl` | UV-frequency traveling weave, a restrained radial center/glint and alpha fading on real mesh faces. | Does *not* implement GPU volumetric raymarching, true Fresnel normals, actual HDR, screen refraction, screen-space bloom or Minecraft dynamic light |
| `.github/workflows/aetheria-preview-wasm.yml` | Pinned Horizon SDK compiles and packages v0.5.3 with an added `cargo test --lib` on the native host to execute VFX unit tests. | A passing build verifies syntax and CPU logic, not real in-game visual quality |

## Performance/safety

- **900 world quads/frame cap across the entire preview**, split fairly between up to 8 simultaneous effects.
- No unstable shader light feedback loops, negative radii, unbounded noise/particle streams, server tick freezes, actual block deletion or server gameplay claims.
- Every spell should show a silhouette in 3D from above and oblique camera angles.
- Avoid pure-white overexposure: glow is color-preserving, with deliberate phase timing and fading afterimage.
- The user's RX 580 and other configurations still require real measured GPU timing. Quads/frame are a CPU emission cap, not a measured frame-rate guarantee.

## Honest completion status

- The visual architecture has **30** distinct 3D spell motifs and individualized phases, not 200 completed spells.
- 200 registry designs remain a long-term plan. The original user's intended halfway milestone is **100 fully gameplay-integrated spells** with usable GUI, acquisition, correct 3D, SFX, persistence and safe multiplayer. None of the 30 VFX previews alone count toward that milestone.
- **No manual user test requested**. The next user-facing preview is gated by the user's latest intent and genuine system readiness.

### M14 automated validation — 2026-11-19

- **WASM pinned SDK build and `cargo test --lib`: passed** — [GitHub Actions run 44565012790](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44565012790).
- Newly enforced preview `cinematic_beams` geometry tests: **5 passed, 0 failed**, including unique full 3D meshes ignoring all RGB colors, phase signatures, finite coordinates and bounded lifetimes.
- Validated compiled internal `Aetheria-Horizon-30Spells-WASM-v0.5.3-Internal` package, SHA-512, 30 OGG cues, WGSL and ZIP integrity. Core Rust tests also passed on commit `135b207d5c2db665469b4b0d784c38af7ccea782`: [core run 44565012649](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44565012649).
- This does **not** prove the actual aesthetic result on the user's Horizon Nightly nor server effects, persisted world state or 100 integrated spells. No user-performed interim playtest requested.
