# Original-prompt Aetheria visual contract — faithful traceability index

**Source status — read this before changing spell graphics.** The user's complete verbatim first master prompt (2026-11-18) is not present in this repository and could not be recovered verbatim during the 2026-11-19 session. Do **not** describe this file or `PROJECT_SPECIFICATION.md` as the full original prompt. Exact available quotations are in `USER_LOG.md`; organized requirements derived from them are in `PROJECT_SPECIFICATION.md`. The user repeatedly requests to use the original prompt rather than improvise. Do not quietly invent details absent from those available sources.

## Verified verbatim fragments preserved in the repository

> "Design **10 distinct Arcane Classes**, each containing **20 completely unique spells** (total **200 spells**: 14 Standard Spells, 4 Signature Awakening Spells, 2 Cinematic Ultimate Spells per class)."

> "Strict Rule: No palette-swapped clones—every spell must have unique 3D geometry, physics behavior, and visual identity."

> "Maintain a `/docs/memory/` folder in the repository and update it after every milestone so zero context is ever lost"

> „Nepřidávej žádné sci-fi ani orbitální zbraně – všechno je čistá fantasy magie, odkaz na orbital-laser ber jen jako technickou inspiraci pro kód 3D paprsků.“

## Recovered visual requirements (structured summary, NOT direct quotation)

- Per-spell readable geometry, unique physics and animations: **real closed/spatial 3D mesh**, not just flat vanilla particles, overlapping billboards, or recolored shared templates.
- **WGSL** procedural material / **SDF** ornamental runes where supported by pinned Horizon's API; controlled **HDR/emissive bloom**, edge lighting and depth-aware refraction **only if actually available**. Do not claim advanced effects from a shader that only receives UV and input color.
- **Spatial volumetric-looking 3D rays**, organic branches, large ice crystal structures anchored to terrain, living lightning, layered magical circles, original fantasy skies and impact choreography. The source videos are *references for beauty, motion and physical clarity*, not instructions to copy scenes or sci-fi content.
- Individually timed **Telegraph → Cast → Impact → Afterglow** VFX, each with original **charge/cast/impact positional sound**, reactive player gestures and camera intents where safe; 20 cinematic ultimate designs, with robust motion-reduction options.
- Budget-conscious world rendering: prevent white overexposure and full-screen obscuration. Never promise native GPU instancing, world destruction or real multiplayer mechanics solely from procedural visuals.
- Ten classes × twenty = 200 **design** spell slots. A cast intent in the portable core is *not* proof of the full gameplay implementation; core spells need persisted/server-authoritative world interactions, acquisition, equipable spellbooks, GUI and audio to count as finished.

## User-owned visual sources

- Four original MP4s `Download (3).mp4`, `Download (4).mp4`, `Download (5).mp4`, `Download (6).mp4` were re-uploaded on 2026-11-19. Their **exact originals** are backed up in the user's ChatGPT Library `/Aetheria Horizon/Reference Videos/Original Uploads/`. SHA-256 checks and reliable content observations are in `../../references/original-videos/README.md`, with chronological frame analysis in `../VIDEO_REFERENCE_ANALYSIS.md`.
- **The binaries are not yet in GitHub**. Do not claim otherwise. The Library contains persistent original MP4s, whereas the repo retains the fidelity/traceability index.
- The first confirmed 5-spell gameplay recording is documented in `PLAYTEST_LOG.md` (heavily overbright white facets, oversized ice, overlapping effects). The user does not want additional intermediate manual playtests.

## Development validation rule

Before every VFX change: re-read `PROJECT_SPECIFICATION.md`, `USER_LOG.md`, `VIDEO_REFERENCE_ANALYSIS.md`, and this traceability index. If an exact original instruction is unavailable, preserve its uncertainty; the verified requirements take precedence over invented details. After changes, use automated Rust tests + pinned Horizon WASM CI, keep original geometry and audio files, and record what *is* integrated versus only designed. Do not send user-facing test packages until the user requests them or the accepted milestone is reached.

## M14 / 2026-11-19 — implemented portions

- Unique duration arrays for all **30** initially curated spells in `src/vfx/timeline.rs`.
- Actual `preview-5spells-wasm/src/cinematic_beams.rs` draws **30 temporal world-space profiles** using visible 5/6-sided beams, spatial rings, varied faceted ice, roots and lightning; onsets, impacts, and fade-out are visibly separate.
- CPU-regression tests compare complete **geometry coordinates without colors** for all 30 cast identities and assert finite bounded geometry at multiple life stages; each stage is synchronized with three positional SFX cues in the preview.
- The Horizon guest shader remains UV+color only, with time-varying material modulation; **no real GPU raymarching, screen-space refraction, HDR bloom or dynamic lighting** claimed or implemented here.
- Real server-side gameplay (damage/healing/world editing/persistence/network) is still a separate incomplete milestone.
