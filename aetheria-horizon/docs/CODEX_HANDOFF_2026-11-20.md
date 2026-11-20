# Aetheria Horizon — Codex full-context handoff (2026-11-20)

**Purpose:** enable a new Codex session to continue the user's actual Rust magic
mod from its precise existing GitHub state, without relying on this ChatGPT
conversation or accidentally starting from obsolete `main`.
**This document preserves a trustworthy summary, NOT a byte-for-byte export of
every conversation turn or all original reference MP4 bytes.**

## 0. Repository and branch selection

- Repository: `Samus-18/integrated-mods-datapacks-to-rust-`
- **Use branch `aetheria-30-spell-test`.** It contains Aetheria through M28.
- Default `main` only has older M11 Aetheria. Do **not** check out `main`
  and overwrite any M12–M28 files, and do not merge/rebase without review.
- Working directory: `aetheria-horizon/`.
- Other folders and branches for `Thalvoryn`, `JJThunder`, `TREK`,
  `Photon` are unrelated projects; do not modify them for this handoff.
- Read repository root `AGENTS.md` plus local `aetheria-horizon/AGENTS.md`
  and this handoff **before editing code**.
- Stable M28 handoff head immediately prior to Codex documents:
  `48c71e40190b72275514d5c5d9a2ab4084b3704f`.
  Handoff commits will advance this SHA; use current live HEAD afterward.

## 1. Vision (from original user requirements)

An original, modern, **beautiful** fantasy RPG magic mod for Minecraft
26.3 / Horizon Rust, not simply a particle or keybind demonstration.

- 10 Arcane Classes × 20 spells = **200** (14 Standard, 4 Awakening,
  2 Cinematic Ultimate per class).
- Ten identities: Pyromancer, Cryomancer, Aeromancer, Geomancer,
  Hydromancer, Druid, Electromancer, Void/Astral, Chronomancer,
  Necromancer/Blood Mage.
- Every ability must eventually have a meaningful distinct game mechanic,
  non-cloned 3D appearance, movement, spatial audio, mana/cooldown,
  progression/unlocking and real-world/server-safe behavior. The user
  **strongly rejects** generic thin rods, empty rings, intersecting flat
  planes, color-swapped copies, overexposed white blobs and boring
  featureless primitives. Do not optimize solely for passing unit tests.
- Strong *environment-scale* fantasy choreography: broad opaque ice wedges
  erupting from terrain, living roots, sculpted fire/air/water volumes,
  lightning animals/cloudbanks, space-time fractures and monumental
  cinematic ultimates; staged telegraph → cast → impact → afterglow.
- GPU cinematic ambitions include original WGSL materials, accurate HDR
  composition where possible, glowing emission, depth-aware fog and
  occlusion/refraction, safe camera effects and performance tiers. These
  only count as implemented when the exact Horizon API actually exposes
  the needed resources.
- 200 unique spell behaviors: damage, healing, freeze, water+electric
  reactions, push/knockback, mobility, temporary block/world edits,
  summons, time/space effects; permissions, rollback-safe boundaries,
  server authority, and safe multiplayer save/network synchronization.
- Crafting/acquisition: scrolls, Arcane Inscription Table tiers I–V,
  spellbooks with 4–8 slots, skill progression and Constellation menu,
  unlocking/obtaining 200 spells through actual gameplay.
- HUD: very compact mana + chosen-spell tile; hold **R** and turn mouse
  wheel to select (N/M fallback), **G** casts, **K** opens animated
  constellation; **J/L** change school. Keep vanilla HP/armor/food/air
  intact unless all are safely redrawn. Dynamic original pixel icons.
  No extra concept art/mockup needed in place of actual coding.
- Genuine 3D animations, distinct sound stages, 20 cinematic ultimates,
  optional reduced-motion/photosensitivity settings and LOD/fair budgets
  for older PCs (user has Radeon RX 580 class of hardware).
- Rust on Horizon ONLY; **no Iris/OptiFine/Forge workaround**. Science
  fiction/orbital weapons are **prohibited**, despite using the
  open-source Orbital Laser project as a *technical renderer study*.
- Preserve original project intent, test rigorously, update GitHub
  memory after milestones, do not repeatedly ask user to test half-builds.

## 2. Canonical requirements and historical provenance

Read in this order:

1. `docs/PROJECT_SPECIFICATION.md` — full structured project contract.
2. `docs/memory/ORIGINAL_PROMPT_VISUAL_REQUIREMENTS.md` —
   recovered exact fragments, missing-original caveat, source-video notes.
3. `docs/memory/USER_LOG.md` — verbatim available user corrections.
4. `docs/memory/SPELL_REGISTRY.md` — all 200 IDs/names/roles/mesh concepts.
5. `docs/VIDEO_REFERENCE_ANALYSIS.md`, `docs/M16_NO_STICKS_VISUAL_REWORK.md`,
   `docs/M17_VIDEO_RESEARCH_AND_MATERIAL_PASS.md`,
   `docs/M18_NATIVE_GPU_POST_RESEARCH.md` — VFX standards.
6. `references/original-videos/README.md` — original videos, known hashes
   and source location. The four actual MP4s named `Download (3).mp4`
   through `Download (6).mp4` are **not in GitHub**: originals archived
   in the user's separate ChatGPT Library. Another original recording
   `horizon-client_MnaFG4NG1R.mp4` showed that early five visual effects
   loaded in-game but had visual overexposure and weak differentiation.
   Do not present CPU SVG renders or generated mockups as actual screenshots.
7. `docs/memory/AI_DEV_LOG.md` — chronological M0–M28 changes and CI evidence.
8. `docs/VALIDATION.md` and
   `docs/M27_COMPLETE_200_ROUTING_AND_VISUAL_QA.md` and
   `docs/M28_RENDER_FAIRNESS_AND_AUDIT.md` — verified technical baseline.
9. `docs/M100_IMPLEMENTATION_BOARD.md` — 100-spell half-project
   acceptance gate; the preexisting board is a planning ledger, not a
   statement that 100 spells work.
10. `docs/ACQUISITION_AND_GUI.md`, `docs/M10_STATE_AND_UI.md`,
    `docs/M11_HUD_RAPIER.md`, `docs/SFX_PIPELINE.md` — RPG/UI/audio design
    and prototype implementation.

The full exact very first message and all its attachments are **NOT**
present byte-for-byte in the repository. Do not claim Codex magically has
this entire chat. The documents are the most faithful persistent summary
currently available.

## 3. Code map

| Path | Reality today |
| --- | --- |
| `src/magic/registry.rs` | Canonical 200 SpellSpec entries and design data |
| `src/magic/casting.rs` | Portable action intents and mana/cooldown casting |
| `src/magic/progression.rs` | Deterministic learn/unlock cost rules |
| `src/core/authority.rs` | Server-intent validation model, **not hooked to actual Horizon server events** |
| `src/core/snapshot.rs` | Portable v2 save codec, **not Horizon save/world binding** |
| `src/ui/minimal_hud.rs`, `fluid_magic.rs` | UI state/layout foundations |
| `src/vfx/arcane_forms.rs`, `cinematic.rs`, `geometry.rs`, `scene.rs` | Portable solid-mesh and camera-intent prototypes |
| `rapier-sim/` | Independent Rapier3D collision/debris proof-of-concept; not integrated with world effects |
| `preview-5spells-wasm/src/lib.rs` | Real Horizon guest WASM Hooks; **client-local demo state and world-quad rendering** |
| `preview-5spells-wasm/src/catalog200.rs` | 200 slots and class/rank mapping |
| `preview-5spells-wasm/src/explicit_coverage.rs` | Verifies exact 200 individual scene dispatch routes |
| `preview-5spells-wasm/src/cinematic_beams.rs`, `hero_vfx.rs` | First curated unique spell forms |
| `preview-5spells-wasm/src/standard_*.rs`, `charged_standard_forms.rs` | Distinct Standard spell visual compositions |
| `preview-5spells-wasm/src/awakening_forms.rs`, `ultimate_forms.rs` | 40 Awakening and 20 Ultimate visual first passes |
| `preview-5spells-wasm/src/visual_audit.rs` | Two-camera, cast/impact CPU geometry SVG diagnostics |
| `preview-5spells-wasm/src/fair_geometry.rs` | M28 dynamic fair quad budget and source sampling |
| `preview-5spells-wasm/src/compact_ui.rs` | Tiny client-only picker/constellation demo |
| `preview-5spells-wasm/assets/.../ethereal.wgsl` | Limited UV/color material fragment; not depth-aware HDR post |
| `native-gpu-vfx/aurora_volume.wgsl` | Naga-checkable standalone aurora raymarch **not connected to Horizon renderer** |
| `preview-5spells-wasm/tools/generate_sound_assets.py` | Synthesizes 30 class/stage original OGG cues, not 200 spell-specific sound sets |
| `.github/workflows/aetheria-preview-wasm.yml` | Pinned Horizon WASM compilation, audio, Naga/geometry and package checks |
| `.github/workflows/aetheria-core.yml` | Portable Rust core and separate Rapier physics tests |

(The workflows live at repository **root** `.github/workflows/`,
not inside `aetheria-horizon`.)

## 4. Grounded progress — do not confuse visual identity with playable

| Category | Verified | NOT verified |
| --- | --- | --- |
| Spell registry | 200/200 distinct canonical design entries | 200 world effects |
| Client local VFX dispatch | **200/200** individually authored renderer paths at M27 | Aesthetic approval of all 200, real native GPU instancing/post |
| Cast/rules intent | 30 curated typed gameplay intents; additional 170 `DesignIntent` identities | The 170 `DesignIntent` effects **are rejected by server guard**, not actual world actions |
| Preview UI | WASM key controls, HUD and animated local constellation prototype | Full authentic inventory/inscription/progression multiplayer GUI |
| Audio | 30 generated school-level charge/cast/impact OGG events | Unique 200-spell finished original SFX |
| World physics | Isolated Rapier collision/ricochet/debris crate tests pass | Actual Horizon block/entity collision hook, world authority |
| Native GPU | Standalone aurora volume WGSL validated with pinned Naga | Actual depth buffer binding, HDR postpass and measured GPU results |
| Acceptance | M28 source CI green | **0/100 fully integrated-and-game-verified playable spells** |

Important history: the user's early Windows game video confirmed that the
**earlier 5-spell WASM prototype could render**. It did NOT confirm the
later 200-scene visual quality, or all spells being playable.

## 5. Most recent validated M27–M28

M27: `docs/M27_COMPLETE_200_ROUTING_AND_VISUAL_QA.md` —
30 last individually authored Standard scenes; 200/200 routed.
[GitHub run 44629836762](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762):
**28/28** preview unit tests, 860 CPU SVG projected source views, 30 OGG
assets, pinned WASM .hmod integrity.

M28: `docs/M28_RENDER_FAIRNESS_AND_AUDIT.md` —
fixed unfair per-spell quad truncation under concurrent effects:
`fair_geometry::sample_fair` water-fills available quads, samples
the complete bounded source instead of only its beginning; retains
900-face solo budget. Bone Palisade #194 had 486/500 faces, now uses
lighter tapered bone rib tubes; test requires ≤400 faces for its Cast
sample. Potential issue: sampling sparse individual faces can expose
holes and will need coherent geometry LOD.

[**Pinned Horizon run 44632013016**](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013016):
**SUCCESS, 31/31 Rust preview tests**, WASM .hmod compilation, WGSL/Naga,
30 OGG, ZIP/SHA512, 860 CPU SVG projection checks.
[Core+Rapier run 44632013002](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013002):
SUCCESS. No newer Windows actual in-game VFX benchmark.

## 6. Immediate Codex plan (implement, don't only describe)

**Phase A — engineering visual fidelity**
1. Inspect actual rendered M28 geometry, reproduce busy multi-cast
   scenarios and ensure all whole 3D bodies remain visible. Replace
   naive per-face sparse downsampling with **component-aware/coherent
   mesh LOD**, prevent holes, keep fair scheduling, 900 max submitted quads,
   8 visible effects, and deterministic regression tests.
2. Review weak silhouettes (#194 and all remaining clearly simplistic
   shapes) at multiple camera angles; re-author substantial closed
   models instead of adding rod/ring decorations. Visually inspect
   actual generated CPU projections (not AI generated pictures).
3. Research and validate **exact pinned Horizon Rust native renderer
   capabilities** before connecting the existing original aurora
   raymarch native WGSL. Add depth-aware occlusion, correct HDR
   compositing and fallback only when the real API allows; do not
   invent hooks or claim visually tested until measured.
4. Add performance-oriented coherent LOD, culling, graceful settings
   and regression tests; avoid white-overexposure and all-screen
   obstruction.

**Phase B — actual RPG gameplay, parallel with art**
1. Inspect and port each gameplay action to a *verified* real Horizon
   server/world adapter. Protect targets, permissions and block changes,
   implement save/load and network validation, and test hostile inputs.
2. Replace `SpellAction::DesignIntent` only after the specific
   server effect truly exists, with unique behavior per spell and
   automated world-gameplay tests. Keep a truthful completion ledger.
3. Connect spell acquisition, inventory, inscription tiers I–V,
   book equipment, unlocks and GUI to authenticated state.
4. Integrate original class/spell-specific SFX and appropriate 3D
   physics, player gestures and ultimate choreography. Avoid naive
   camera takeover or global server tick freezes.

**Phase C — verification/release gate**
- Run portable tests and pinned WASM CI regularly. Validate source and
  package SHA/manifest/asset checks. Use actual Horizon client runtime,
  GPU screenshots and tests **where technically possible**, and record
  when unavailable.
- Do not automatically deliver incomplete installable ZIPs or ask the
  user for repeated manual tests. Offer first major user playtest only
  after meaningful fully integrated accepted milestones; current
  half-project gate tracks **100 complete spells** rather than
  100 drawn diagrams.
- Preserve all 200 individually named spells and their classes.
- Update `docs/memory/AI_DEV_LOG.md`, `docs/VALIDATION.md`,
  `docs/memory/SPELL_REGISTRY.md` status fields and a milestone file
  whenever work actually changes, distinguishing tests, art and
  playable world mechanics.

## 7. Build commands and branch guard

From checkout root (Linux runner / Codex cloud):

```bash
git status --short --branch
git branch --show-current
# MUST inspect that current source contains M28 files before changing anything.
test -f aetheria-horizon/preview-5spells-wasm/src/fair_geometry.rs
cd aetheria-horizon
cargo test --all-targets
cd rapier-sim && cargo test --all-targets
cd ../preview-5spells-wasm && cargo test --lib
```

The Horizon guest SDK is installed by the official pinned CI workflow
from revision `dd67b76ff23e28c6d8e21859df600d31572f81a0` with
`cargo-horizon-mod`, then `cargo horizon-mod build --wasm`; running
these locally without generated `.horizon/sdk` may fail and is NOT a
reason to invent replacement APIs. The workflow also synthesizes
sounds, checks 200 routes/geometry, Naga, WASM and SHA512.

For new Codex cloud tasks, use an environment that includes this repo and
select the actual branch `aetheria-30-spell-test` as the starting revision.
A new Codex cloud task does **not** inherit another task's uncommitted files
or this ChatGPT conversation.

## 8. Owner working style / constraints

- Respond and summarize briefly **in Czech**, with facts/results and
  actual changed files. The user prefers progress, not repeated
  speculative estimates.
- Work directly on source and automated tests, compare visual source
  frames rather than generate fictional in-game screenshots.
- User does not want meaningless UI concept pictures or early broken
  archives. Preserve custom original assets, avoid copyrighted copies.
- If original MP4 binaries are missing in Codex's workspace, do not
  claim to have rewatched them: cite existing checksums/analysis, and
  request original media only if necessary for exact verification.
- Do not write to unrelated repo projects and do not mutate `main`
  without explicit authorization.
- Be candid about execution limitations: a passing Rust unit test
  proves neither world gameplay nor professional visual fidelity.

## 9. Ready-to-paste initial Codex task (Czech)

> Pokračuj ve vývoji Aetheria Horizon v repozitáři
> `Samus-18/integrated-mods-datapacks-to-rust-`,
> **větev `aetheria-30-spell-test`**, složka `aetheria-horizon/`.
> Nejdřív si přečti kořenový a projektový `AGENTS.md`,
> `docs/CODEX_HANDOFF_2026-11-20.md`, `docs/PROJECT_SPECIFICATION.md`,
> dokumenty v `docs/memory/` a M27/M28. Ověř reálný aktuální kód,
> aktuální testy a stav CI. Nevycházej ze starého `main`.
> Pak navazuj skutečnou implementací: oprav souvislé 3D LOD při více
> současných efektech, zachovej kompletní modely a 900-quad rozpočet,
> přidej cílené regresní testy a ověř vzhled reálnými projekcemi
> z kódu; následně pokračuj skutečným Horizon world/server gameplay
> propojením, bez domýšlení nepodporovaných API. Všech 200 kouzel musí
> mít nakonec skutečnou jedinečnou mechaniku, krásné 3D/VFX, SFX,
> animace a získávání přes GUI. Nevyráběj tenké tyčky, prázdné kruhy,
> generické přebarvené efekty ani sci-fi zbraně. Neposílej
> nedokončený testovací ZIP. Změny testuj, ukládej do vývojové větve,
> aktualizuj GitHub dokumentaci a řekni mi stručně česky,
> co jsi opravdu udělal a co ještě chybí.
