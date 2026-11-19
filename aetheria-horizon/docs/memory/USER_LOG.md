# USER_LOG — project instructions / 2026-11-18

The user's complete original master prompt is in the conversation; this log preserves exact quotations and key decisions. It is not claimed to be a verbatim export of the whole chat.

> "Design **10 distinct Arcane Classes**, each containing **20 completely unique spells** (total **200 spells**: 14 Standard Spells, 4 Signature Awakening Spells, 2 Cinematic Ultimate Spells per class)."

> "Strict Rule: No palette-swapped clones—every spell must have unique 3D geometry, physics behavior, and visual identity."

> "Maintain a `/docs/memory/` folder in the repository and update it after every milestone so zero context is ever lost"

> „Nepřidávej žádné sci-fi ani orbitální zbraně – všechno je čistá fantasy magie, odkaz na orbital-laser ber jen jako technickou inspiraci pro kód 3D paprsků.“

> "ne codex ani se neptej zapamatuj si to"

Destination: https://github.com/Samus-18/integrated-mods-datapacks-to-rust- — new directory `aetheria-horizon/`.

Requirements: 10 classes, 200 spells, four ways to cast; mana, cooldown, player save, multiplayer sync; spellbooks, scrolls, inscription table, five tiers; 3D geometry, WGSL VFX, HDR, refraction, lighting, world physics, cinematic ultimates, sound; clean mana HUD, fluid radial spell wheel, constellation skill tree; maintain logs and registry in GitHub. No Codex. Four attached MP4 videos are visual references.


## 2026-11-19 — First successful in-game test and improvement request

Exact user feedback (verbatim as received):
> "hele cekal jsem horsi ale dobry akorat lepsi variace a  no vsak vis podle originalniho promtu mela bys to vse mit na githubu to bylo v tom promtu prvim at si to ulozis tam"

Evidence supplied by user: `horizon-client_MnaFG4NG1R.mp4` (approximately 100 seconds) — local Horizon 26.3 playtest of the precompiled Rust/WASM five-spell preview v0.2.0.

Requirements reaffirmed: visual variation must improve; retain all original master specification requirements in GitHub; keep history, decisions and spell registry updated, without Codex. Preserve fantasy only. Do not equate five early effects with 200 finished spells.

The original large master prompt's full unedited byte-for-byte transcript is **not** presently in this GitHub file; the baseline above summarizes it. See `docs/PROJECT_SPECIFICATION.md` for a durable structured feature and delivery contract. Do not misrepresent a paraphrase as a verbatim record.

## 2026-11-19 — Deferred manual testing until halfway, solid 3D shapes and real systems

Exact user message (verbatim):
> "testovat uz nebudu jen jsem chtel vydet jestli jsi schopna to udelat dobre az to bude v pulce hotove a hlavne i gui atak dobrej i ziskani tech kouzel a tak to pak navrhnes ale ty kouzla at jsou  3d jo ne jen svetelne paprsky trena glacier je ledove ostny ze zeme co si ne takze 3D vidis na obrazku a tak u kazdeho ve 3D ale i zive at to vypada dobry sfx a animation az do 100 pak otestuju"

Attached image: `image(7).png` showing large broad pale-blue triangular ice spikes protruding from a ground surface, foreground-to-background row/perspective and irregular heights (Ice Awakening screenshot). Use only as reference; do not copy copyrighted assets.

Interpreted gate: next **user-performed** game test only after 100 of 200 actual complete spells (not merely 100 registry lines or visual-only demos). CI/programmatic automated checks remain allowed. 10 classes × (7 standards + 2 awakenings + 1 ultimate) each, full playable loop including solid 3D effects, mana, inventory/unlocking, server-state integration, GUI, animation and original audio. No early ZIPs or requests for testing.
