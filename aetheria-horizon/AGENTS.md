# AGENTS.md — Aetheria Horizon (Codex development instructions)

## Project identity / goals

Aetheria Horizon is an ORIGINAL fantasy RPG magic mod for Minecraft 26.3
through Horizon Rust. Its full design has **ten classes × 20 spells = 200**:
per class 14 Standard, 4 Awakening, 2 cinematic Ultimates. All 200
must eventually have their own actual mechanics, solid/evolving 3D visual
form, stage choreography, original spatial audio, balanced mana/cooldowns,
discoverable equipment/unlocks and working multiplayer-safe persistence.
No science fiction, orbital weaponry, clones of third-party assets, palette
swap filler, or low-effort rods/empty rings disguised as finished VFX.

Read `docs/CODEX_HANDOFF_2026-11-20.md` first for a current handoff and the
precise state of the code. Then the following canonical existing sources:

- `docs/PROJECT_SPECIFICATION.md`
- `docs/memory/ORIGINAL_PROMPT_VISUAL_REQUIREMENTS.md`
- `docs/memory/USER_LOG.md` and `docs/memory/AI_DEV_LOG.md`
- `docs/memory/SPELL_REGISTRY.md` (canonical 200 names and mechanics)
- `docs/VIDEO_REFERENCE_ANALYSIS.md` and
  `references/original-videos/README.md`
- `docs/M27_COMPLETE_200_ROUTING_AND_VISUAL_QA.md`
- `docs/M28_RENDER_FAIRNESS_AND_AUDIT.md`
- `docs/VALIDATION.md`
- `docs/M100_IMPLEMENTATION_BOARD.md`

The literal complete original prompt is NOT stored verbatim on GitHub.
Do not claim otherwise. Four original reference videos are backed up in
the user's ChatGPT Library; GitHub currently contains written analysis and
checksums, **not their MP4 bytes**. Ask for assets only when actually
necessary for a particular implementation or visual-comparison task.

## Branch / workflow / truth in reporting

- Active code branch: **`aetheria-30-spell-test`**; older `main` ends at M11.
- Treat current source, tests and CI as the source of truth; never overwrite
  code with an old memory/README claim.
- Work in small coherent batches. For each batch: inspect code, implement
  original capability, add regression tests, run real tests, review
  representative geometry renders from at least two camera angles, then
  update `docs/memory/AI_DEV_LOG.md`, `docs/VALIDATION.md`, and the
  appropriate milestone note with observed results, known gaps and commit.
- Keep unrelated repository projects untouched.
- Brief Czech user updates are preferred. Do real implementation rather
  than produce only plans, screenshots, mockups, or reports.
- Do not promise asynchronous delivery. Do not invent engine APIs,
  physical performance metrics, tested versions, a working multiplayer loop
  or polished visual art solely because Cargo tests passed.
- Do not use copyrighted third-party animations/meshes/sounds, nor assume
  Rapier3D gives shader effects. The orbital-laser source is reference for
  rendering architecture **only**, never a sci-fi feature request.

## Exact current baseline

At M27, the 200-slot client demo had **200/200 explicit independently
selected scene render paths**: 30 curated, 110 Standard, 40 Awakening, 20
Ultimate. This is **NOT** 200 completed gameplay spells or individually
approved cinematic VFX. Many remain low-poly or have visual issues.

At M28, `preview-5spells-wasm/src/fair_geometry.rs` adds fair VFX
world-quad scheduling with source-shape sampling; the bone wall spell #194
has budget headroom. Pinned Horizon WASM CI 44632013016 reported **31/31
tests passed**, validated .hmod, SHA512, WGSL, 30 original OGG audio events,
and generated 860 CPU geometry projection images. This does **not** prove
in-game screenshot fidelity or FPS on Windows. Core/isolated Rapier tests
pass separately. See the handoff for IDs/URLs and date.

**CRITICAL GAPS:** 170 extended action intents are still `DesignIntent`,
server-guarded; actual Horizon multiplayer damage/heal/control, block
effects, unlocking/crafting, world save, network sync are incomplete.
Real depth-aware HDR/refraction/volumetric native GPU postprocess is
a standalone WGSL research file not wired into Horizon. Current audio is
30 school-level charge/cast/impact clips, not 200 unique spell soundsets.
The GUI/compact picker is a client-local demonstration, not a complete
server-backed item/skill interface. M100 user acceptance still
**0/100 fully integrated and game-verified spells**.

## Hard visual requirements

- Real world-space substantial, filled 3D models and staged movement.
  Ice must emerge from terrain as wide faceted grounded spikes, not
  white billboards. Different spells need distinct *physical* silhouettes.
- No generic sticks, intersecting planes, circles, empty hoops, same-effect
  recolors, giant featureless blocks or blown-out full-screen white.
- Compare art with original-user reference notes; multiple camera angles
  and cast/impact stages are necessary, but geometry tests cannot certify
  beauty. Always report artistic uncertainty.
- Avoid front-of-camera obstruction, lighting glare, expensive uncontrolled
  draw loops or holes from overzealous polygon subsampling. Protect older
  GPUs (including Radeon RX 580) through quality/distance LOD and caps.
- Use real verified Horizon APIs and SDK pin; do not silently substitute
  Vanilla/Forge/Iris/OptiFine render integration. Do not promise HDR
  unless the actual scene color/depth postpass and compositing are verified.

## RPG and UI requirements

- Class-specific actions and targets, spell acquisition and unlock loop,
  4–8 slot spellbooks, inscription tiers I–V, mana, cooldowns, progress,
  saves/network authoritative validation and permissions.
- Compact in-game HUD, keep vanilla health/armor/food/air visible unless
  a *complete* safe replacement is built. R + mouse wheel quick selector
  (N/M fallback), G cast, K animated constellation UI, J/L change school.
  Living fantasy menu backgrounds; small custom pixel icons; no intrusive
  giant permanent wheel or extra AI-generated UI pictures.
- 20 cinematic Ultimates; distinct timeline, positional SFX, safer camera
  shake and reduced-motion/photosensitivity settings.

## Validation / delivery

Portable core: `cd aetheria-horizon && cargo test --all-targets`.
Isolated physics: `cd aetheria-horizon/rapier-sim && cargo test --all-targets`.
Preview: `cd aetheria-horizon/preview-5spells-wasm &&
cargo test --lib`. The full pinned Horizon WASM CI lives at
`../.github/workflows/aetheria-preview-wasm.yml` relative to repository
root (actual repo path `.github/workflows/aetheria-preview-wasm.yml`);
it runs original SFX generation, `cargo horizon-mod build --wasm`,
Naga and asset-integrity checks plus geometry projections.

Do not deliver an unpolished download ZIP or ask user to repeatedly
playtest incomplete builds. Fully integrated spell milestones require
verified game-state effects and real Horizon runtime validation, not
only a 200-scene route table. User accepts automated CI while working.
