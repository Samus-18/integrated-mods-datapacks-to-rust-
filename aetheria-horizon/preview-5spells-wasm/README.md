# Aetheria Horizon — 30-spell test build v0.5.0 (Rust → WASM)

**Experimental, not a finished RPG release.** This builds on the previously game-verified 5-spell Horizon client renderer and expands the client selector to **30 distinct registry IDs** (three from each of ten classes). The portable Rust gameplay core has **30 different action intents**, but the server-facing world adapter has not yet been implemented: spells do **not** damage mobs, heal players, place blocks, unlock from loot or persist progression. Effects/costs are client-local; do not confuse them with server-authoritative gameplay.

## Install
1. Download the compiled **Aetheria-Horizon-30Spells-WASM-v0.5.0-Internal** artifact from this branch's successful GitHub Actions WASM workflow, extract its `aetheria_preview.hmod`. **Never use source ZIP or a build whose CI failed**.
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
