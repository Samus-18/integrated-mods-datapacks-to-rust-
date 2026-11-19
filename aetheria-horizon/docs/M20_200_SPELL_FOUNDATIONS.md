# M19–M20 — 200 authored spells, safe portable intents and initial visual drafts

Date: 2026-11-19. Branch: `aetheria-30-spell-test`. Internal experiment version: `0.6.0`. **NOT a user-ready 200-spell RPG.**

## Truthful implementation matrix

| Feature | Current status | Real limitation |
| --- | --- | --- |
| Ten schools × 20 unique spell definitions (14 Standard, 4 Awakening, 2 Ultimate each) | **200/200 already authored** in canonical `src/magic/registry.rs`, preserving unique names, mechanics, mesh descriptions, cast modes, costs, cooldowns and sound IDs | A registry entry alone does not make a functioning game spell |
| Portable Rust cast intents, mana/cooldown enforcement | **200/200**: original 30 distinct typed actions; 170 explicit `SpellAction::DesignIntent` carrying the exact source authoring text and ID | **170 DesignIntent actions are not implemented world mechanics** |
| Authoritative command transaction | Safe **guard implemented**; `world_adapter_ready` refuses a DesignIntent before mana/cooldown deductions; server sequencing still prevents replay | No official Horizon server world adapter or multiplayer net ownership connected |
| Spell-specific four-phase VFX choreography | **200/200** bounded timelines; preserve hand-directed first 30; new 170 have cast-role/cast-mode-sensitive deterministic distinct phase signatures | 170 timings are design contracts, not cinematic art-direction approval |
| Client library and GUI | **200/200 slots**, ten schools × 20 per school, J/L class switch, N/M or R+scroll global selector, K spellbook, G cast in world only | Client-local training mana only, no persistent unlocks; user still must ultimately test native GUI |
| Client VFX content | Original 30 hand-profiled local world-VFX; 170 new full-body **semantic geometry drafts** derived from each original source `mesh` wording rather than a reused line or color clone | No artistic pass on all 170 yet; still far below reference-video cinematic quality; **do not ship** |
| GPU shading | UV-based WGSL material in preview and standalone Naga-validated raymarch source | Full depth-aware native shader post integration/true HDR bloom not implemented |
| SFX | 30 originally authored school/stage OGG cues with per-spell temporal offset and small pitch variation | Not 600 individual spell sound cues, no fully synchronized world sound server |
| Playtest and FPS | Static unit tests, projected SVG audit and pinned SDK CI | **No real 200-spell Horizon game test**, RX 580 frame pacing unknown |

## What changed in source

1. `src/magic/registry.rs` keeps canonical 200 original designs, now flags portable *intent availability*, not server feature readiness; `src/magic/casting.rs` retains exact prior 30 typed actions and adds `DesignIntent { spell_id, mechanic, form, cast_type, role, intensity }` for 170. Tests validate every ID, resource and no unauthorized duplication.
2. `src/core/authority.rs` ensures incomplete designs **cannot** consume server resources. It rejects them before the cast call.
3. `src/vfx/timeline.rs` provides four-stage bounds for all 200. Role/cast mode and per-class ID influence the new 170; first 30 authored durations preserved.
4. `preview-5spells-wasm/src/catalog200.rs` maps all 200 global slot IDs to registry; first three per class map losslessly to their **original 30 curated VFX IDs** (including M17 Shard Chorus).
5. `preview-5spells-wasm/src/mass_forms.rs` provides first-draft non-ring-only meshes for 170 using source phrases as **primary form** (glacier, beast, folded time, carved glyph, ribbon, architecture, wave, wing, solid armor, avatar, relic and others), with distinct per-ID dimensions/positions. Additional tests reject unclassified generic fallback and check finite, bounded and unique coordinates ignoring colors.
6. `preview-5spells-wasm/src/compact_ui.rs` shows all 20 spells of chosen school, selected costs, spell roles and 200 global slot identities while checking in-game-only rendering. K, J/L, N/M, R+scroll and G are handled without entering menu/options/title screens.
7. `preview-5spells-wasm/src/visual_audit.rs` retains 180 prior 30-spell views and additionally tests/exports **680** Cast/Impact oblique views for the 170 new geometric drafts. This is **visual QA tooling**, not subjective quality certification.

## No-slop acceptance gates

The user's explicit direction: **no sticks, empty outlines, a dozen recolored school-default effects or arbitrary placeholder geometry**. The first-pass 170 are **not yet approved**. They must be inspected individually from both views in `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal`, then improved using custom connected solid 3D forms, procedural movement, layered material and camera/depth-aware light techniques where engine supports them. Each major/ultimate spell needs its own authored silhouette and timing, not scaled-up Standard output. No fake claims of originality based solely on distinct coordinates or color IDs.

## Server and compatibility gate

Do **not** take the expanded internal `.hmod` and call it a multiplayer gameplay mod. No real damage, healing, teleport, world structure placement, spell-progression persistence, network authority or 200 distinct signature audio tracks. Real user-world validation for Horizon Nightly and GPU budget measurement are absent. **Do not send an incomplete test build to the user**, per their request. Record successful GitHub actions separately when complete.
