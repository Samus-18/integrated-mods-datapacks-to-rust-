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

## 2026-11-19 — Keep working, preserve original videos and original specification

Exact user message (verbatim):
> "pokracuj pohraj si v tom ale drz se ty krase jak v tych videich a originalniho promtu"

Confirmed interpretation: continue implementation within this chat and their linked GitHub, no Codex/no paid asset generators, adhere to all original Aetheria requirements (200 spells, 10 classes, native Rust/Horizon when supported, solid unique 3D shapes, elaborate SFX/animations/VFX, fantasy-only, mana/progression/GUI, original videos as artistic references). Do not send test ZIP before 100 fully playable spells (user directive from preceding turn).

## 2026-11-19 — Iron-style minimal HUD, animated background, actual Rapier physics (exact instruction)

> nemusis mi delat obrazek tych animaci jen kdyz jses porste v tom menu pozadi se hejbe krasne a tak ui ikonky ale to herni menu dej styl iron spells wheel vyberu nebo jak to tam je ten druhy zpusob a ukazatek na manu a hp i armor to muzes tam dat upravit vanilla hud aat to udelas nemusis delat dalsi obrazek jen to udelej implementuj to a pak pokracuj v kouzlech a nasel jsem duvod prej proc jsou ty kouzla tak krasne prej tenhle github to dela[https://github.com/dimforge/rapier](https://github.com/dimforge/rapier)

Interpretation: no more requested UI images; draw living menu backdrop + icons in-game. Keep gameplay view clear and HUD tiny; R+mousewheel quick spell selection inspired by Iron's Spells' alternate selection control, use no copied copyrighted UI artwork. Health and armor should come from actual vanilla player stats; do not fabricate or hide food/air. Rust/Rapier can improve physics of 3D debris and projectiles; it does **not** create attractive shaders, 3D meshes, bloom or audio by itself. Continue implementing distinctive, physical-looking 3D spells. User still declines manual testing until 100 actually integrated spells.


## 2026-11-19 — User corrected missing archival originals and re-uploaded all four MP4s

Exact feedback:
> "aha sorry napsala ze tam nejsou i kdyz v originalnim promtu bylo vse co pisu i ty a i prilohy at si vzdy ulozis do githubu vzdy vlastni slozky wtf no tady je mas znovu a uloz si je tam"

Attached `Download (3).mp4`, `Download (4).mp4`, `Download (5).mp4`, `Download (6).mp4`. The original project instruction is to archive every user-supplied prompt and attachment in distinct GitHub folders, not only paraphrases and analyses. We had previously saved only textual video descriptions, which was incomplete.

**2026-11-19 preservation:** the exact four MP4s and a combined ZIP have been saved into the user's persistent ChatGPT Library (`/Aetheria Horizon/Reference Videos/`), including their original bytes, while a GitHub `references/original-videos/` folder contains checksum manifest and direct frame analysis. GitHub originals were not copied because the currently available GitHub connector cannot read and upload the multi-MB local binary files. Do not misreport that they're now in GitHub. Next time binary upload is supported, sync the originals and verify SHA256. No manual game testing requested.

## 2026-11-19 — Explicit rejection of stick-and-circle VFX after actual geometry screenshot

Exact user message:
> "ani nevim co to je nerikej mi ze delas to co jsem rikal ze hlavne nemas delat a to tycky a kruhy  obycejny jako drive to si nepamatujes ale vse jsi prost edelala jen z tycek"

User saw exact generated offscreen geometry picture of Blink Fold (slot 24). The geometry had two empty circular frames, narrow beams/spikes and little volume. This is **unacceptable** even though unit tests passed and polygons were technically 3D. The correction is to prioritize substantial non-generic volumetric **filled forms** with detailed materials, kinetic shape, silhouette and stage continuity. Never assert that a numeric uniqueness/area test proves it looks beautiful. The user does not want to test another half-built mod.


## 2026-11-19 — User demands original-video replay and current fantasy spell mod research

Exact feedback:
> "PROSTE ZNOVU  se koukni na ty videa vyhledej si i dalsi vytvori rust kouzel a uvidis sama  a klidne i normalni kouzla ale ty novy ted vychazeji uplny krasny diky claude a tak , tak to muzes taky udelat ne"

Read as: compare the DIRECT source footage with more recent Minecraft magic VFX examples, including non-Rust mods as art direction; stop claiming that thin rods/rings match the reference's visual quality. User spec retains **original fantasy not sci-fi**, no copies of licensed content, no repeated user playtests before a meaningful polished version. AI tools are allowed, but authors' AI usage is not verified by the cited pages. Exact user video filenames and SHA256 remain in references/original-videos/README.md (original binaries archived in the user's Library, not falsely claimed to be git blobs). Concrete M17 code and citations documented in docs/M17_VIDEO_RESEARCH_AND_MATERIAL_PASS.md.
