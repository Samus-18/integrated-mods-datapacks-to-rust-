# Aetheria Horizon — M31 GPU volumetric VFX test v0.7.0 (Rust → WASM)

**Experimental, not a finished RPG release.** This builds on the previously game-verified 5-spell Horizon client renderer and expands the client selector to **200 draft identities** (20 from each of ten classes). The portable Rust gameplay core has **30 different action intents**, but the server-facing world adapter has not yet been implemented: spells do **not** damage mobs, heal players, place blocks, unlock from loot or persist progression. Effects/costs are client-local; do not confuse them with server-authoritative gameplay.

## M31 v0.7.0 — installable bounded shader-side volume integration (2026-11-20)

**This test pack is for the user to try, unlike earlier internal-only M29/M30 artifacts.** Use GitHub Actions artifact `Aetheria-Horizon-M31-GPU-Test-v0.7.0`, which contains a compiled `aetheria_preview.hmod`, `README-TEST.md` and `SHA256.txt`. Shard Chorus's three closed low-alpha curved atmosphere sheets are now shaded by a real *shader-side 3D density integration*, with each visible pixel integrating 10 close/6 far samples of folded fantasy vapor. The material remains anchored in the actual authored faces (UV + local depth); world occlusion comes from Horizon's standard tested depth at those faces, not independent scene depth reads. Material also gives crystal faces normal-sensitive rim shading. Requires native GPU shading, but is still packaged as a normal WASM `.hmod` because the guest shader is resource-loaded by Horizon's own GPU. **Not** the native post-process / HDR bloom from Orbital Laser: that requires a separate trusted native render bridge, which isn't in this test build. No claims of real light casts or gameplay spells. Refer to `TEST_INSTRUCTIONS_M31.md`.

## Install
1. Download the compiled **Aetheria-Horizon-M31-GPU-Test-v0.7.0** artifact from this branch's successful GitHub Actions WASM workflow, extract its `aetheria_preview.hmod`. **Never use source ZIP or a build whose CI failed**.
2. Remove any old `aetheria_preview.hmod` from `horizon-native-mods/` and `horizon-mods/`.
3. Put the compiled .hmod in the game instance's `horizon-mods/` and enable `aetheria_preview`.
4. Start **Play** normally, not Launcher Build. You need a Horizon Nightly compatible with the pinned SDK.

## Keys
- **Hold R + mouse wheel**: browse all 30 spells without changing the vanilla hotbar slot.
- **N / M**: previous / next spell (useful without a mouse wheel).
- **G**: cast selected spell (local test VFX, mana/cooldown and 3-stage sound).
- **K**: animated 10-class constellation overlay; N/M changes the viewed class; K closes.
- **Z / X / C / V / B**: direct preview casts in Fire / Ice / Druid / Lightning / Astral schools.
- Selected spell name, school palette, readiness, mana bar and 5-neighbour picker appear near the normal hotbar. Vanilla health/armor/food/air are retained.

## Spells
The first three official Aetheria registry spells of each class:
- Pyromancer: Ember Needle, Cinder Fan, Magma Lash
- Cryomancer: Frost Needle, Rime Bridge, Shard Chorus
- Aeromancer: Gale Dart, Razor Crescent, Updraft Well (see registry for authoritative spelling)
- Geomancer: Stone Fist, Crystal Traverse, Gravel Jaws
- Hydromancer: Water Jet, Flowstep, Pearl Shield
- Druid: Thorn Seed, Root Snare, Moss Mend
- Electromancer: Fork Lightning, Static Halo, Thunder Needle
- Astral Mage: Star Shard, Gravity Thread, Blink Fold
- Chronomancer: Projectile Delay, Moment Slip, Tick Pulse
- Necromancer: Bone Spike, Soul Lantern, Grave Whisper

Spell names and costs shown in game come from `src/magic/registry.rs`, **not** a separately copied table. Geometry uses ten independent volumetric 3D school sculpts plus three per-school signature topologies. It does **not** yet meet the final per-spell sculpt quality bar. Max 8 concurrent effects, 900 world quads/frame globally; geometry falls back if optional shader fails. Original 30 OGG files are 10 schools × charge/cast/impact stages, with spell-rank pitch variation.

## What CI proves
The GitHub Actions job checks the pinned SDK build, package SHA-512, WASM binary, shader, ZIP integrity and 30 original Vorbis sound files. The core job tests actions, mana/cooldowns, progression and Rapier physics independently. This does **not** prove visual quality, damage, multiplayer or progression inside Horizon. Only a gameplay video or test on the user's Windows build can establish those.

## Next integration gate
Server-side player ownership and persistence; world entity targeting/LOS and actual damage/healing/status effects; legitimate unlock drops/pages/trials; multiplayer sync; interactive spellbook rather than view-only constellation; per-spell authored meshes/cameras, client quality settings and accessibility. Keep vanilla vital bars until native health-layer redraw is validated.

Reference: [official mod docs](https://playhorizon.dev/docs/mod-code). All sounds are synthesized and original; no external voice samples or paid AI assets.


## v0.6.0 correction from real Nightly 2026-11-18 feedback
The user's gameplay video showed the HUD incorrectly layered over title/options/keybind screens, plus an oversized, hard-to-read gold-framed HUD and repetitive sculpt silhouettes. The client now checks **`host.playing()` + player presence + `host.observed_screen_id().is_none()`** in the render hook, in the drawing function and for casting shortcuts. A new compact dark-glass strip (262 × 33 GUI px) is shown only during normal gameplay. R+scroll shows five nearby slots briefly; K opens a small 10-school/3-spell detail panel, no opaque 20-node placeholder map. Five original hand-sculpted spell silhouettes are restored for legacy shortcut schools; the 30-spell backend remains.

This is a code and CI test of the menu visibility predicate, **not** yet a verified user in-game fix. The previous v0.5.0 client should be removed before trying the new verified artifact.


## M13 v0.6.0 — cinematic volume pass (internal build, no user playtest)

Original four MP4s were re-uploaded 2026-11-19, inspected as actual videos, saved to the user's persistent Library and indexed with SHA256 under `references/original-videos/` (the raw MP4 binaries are **not** on GitHub yet). New `src/cinematic_beams.rs` draws **30 different fantasy spatial motifs**: real multi-sided beam surfaces, closed runic rings, staggered ice columns, branched thunder, organic tendrils and fractured astral sky portals. Temporal onset, dissipation and overexposure guards apply. Original five visual spell silhouettes are retained as supporting geometry when the budget permits; the old shared rank-pattern overlays are bypassed in active curated casts. This **does not implement real GPU volumetric rays, true bloom, world collision, multiplayer damage or loot**. Final effect fidelity awaits actual game validation later; user explicitly declines more intermediate test requests.

## v0.6.0 — M14 individual phase/geometry pass (2026-11-19)

This is an internal build, **not** a request for a manual test. The first 30 registered cast intents now each have a distinct 4-stage timeline: **Telegraph, Cast, Impact, Afterglow**. Client procedural VFX samples each timeline proportionally over a safe 104-tick lifetime. Each phase emits intentional 3D geometry (unique per school and rank), rather than showing a fully formed large effect immediately and leaving it unchanged. The original five huge sculpts now only support the Cast/Impact phase and may take at most 34% of the post-signature remaining quad budget. An updated WGSL fragment provides gentle surface energy motion without claiming physically correct Fresnel, dynamic light, volumetric raymarching or bloom.

The preview adds CPU-side tests for 30 nonempty finite distinct geometry fingerprints **ignoring material colors**, temporal dynamics and out-of-lifetime suppression; the CI workflow must execute those tests as well as compile the wasm. The design baseline remains `../../docs/PROJECT_SPECIFICATION.md` with literally quoted fragments in `../../docs/memory/USER_LOG.md`. The exact untouched original master prompt is not fully archived; never claim the paraphrase is its full text. Four original video MP4s were saved to the user's ChatGPT Library, indexed under `../../references/original-videos/`, and remain absent from GitHub binary blobs until a binary-capable upload is possible.


## M19/M20 — all 200 spell identities now in portable casting & internal client (v0.6.0)

All **200** original hand-described spell entries (10 schools × 14 standards + 4 awakenings + 2 ultimates) already existed in the canonical registry. M19 now provides **200 validated portable cast intents, individual four-stage phase timing, mana costs and cooldowns**, preserving the first 30 explicitly typed original actions. The remaining 170 use *exact* authored descriptions, meshes, roles and casting modes in `SpellAction::DesignIntent` — these are **NOT server world effects**. The authoritative command processor refuses all `DesignIntent` before charging mana. It cannot damage, heal, summon, teleport or edit blocks in game yet.

M20 expands the internal Horizon **local-client-only** selector to all 200; J/L selects class, N/M selects spell, R+wheel scrolls, K opens a ten-school/20-spell spellbook, G locally triggers the cast; the original five quick keys still use their designated first spell of each corresponding class. Previously curated 30 retain their original VFX, including the improved ice Shard Chorus. The additional 170 have **first-draft, role-driven, registry-mesh-semantic coherent solid 3D forms** — differentiated by actual spell descriptor and distinct geometry rather than 170 recolored beam clones — but they **have not passed an artistic release gate**. They are tagged DRAFT; do NOT send this as a completed beautiful 200-spell mod.

The new automated audit adds 170 × 2 action phases × 2 oblique views = **680** more diagnostic SVG images alongside 180 existing curated views. A passing Rust test proves finite, bounded, distinct geometry from those views, not cinematic beauty, correct lighting or real-game playability. Real WGSL fullscreen lighting remains a separate experimental non-integrated M18 GPU shader; not part of the `.hmod`. The genuine art-development milestone remains **200 source-directed high-quality distinct VFX, effects, spells and multiplayer gameplay**, not just a passing build.
