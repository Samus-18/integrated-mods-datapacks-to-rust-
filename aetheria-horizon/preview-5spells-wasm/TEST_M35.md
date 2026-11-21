# Aetheria Horizon — M35 procedural-first VFX **INTERNAL QA ONLY** (0.8.3)

Nightly 2026-11-20 / Minecraft 26.3 / pinned SDK
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`.

## What changed

- All 200 selectable preview spell IDs now have an **original mathematical
  particle-emitter accent system** in addition to the existing draft bodies.
- Ten different school motion fields: thermal updrafts, icy spindrift, wind
  shear, falling rock dust, water spray, flora pollen, charged ion sparks,
  collapsing astral motes, staggered time echoes and spectral wisps.
- A **separate depth-tested optional WGSL material** generates four soft
  particle silhouettes from small camera-facing quads. Zero-opacity at
  sprite boundaries avoids rectangular particle cards.
- If the separate particle WGSL cannot compile or initialize, the effect
  deliberately omits those particle cards; solid spell geometry still draws.
  NO fake full-screen postprocessing, HDR/bloom or scene-depth sampling.

## Controls

Tap R to cycle the 10 schools. Hold R + mouse wheel to choose one of the
20 spells in that school. Z casts the chosen preview spell. G is not bound.

## Explicit limitations

- **This is not 200 polished spells.** The other 198 still have older,
  visually provisional base geometry and only gain a new common particle
  layer. Frost Needle (#021) and Rime Bridge (#022) retain their edited
  Blender-bodied scenes. Actual cinematic quality is *not* yet approved.
- The visually rendered Rime Bridge is NOT walkable or collidable. Neither
  spell deals real server-side damage or edits blocks.
- The module is a client-local preview with synthetic demo mana and audio;
  no real multiplayer authority, save or character progression.
- This is a CPU world-quad emitter with shader masks, **not GPU instancing**.
  It does not yet have HDR bloom, fullscreen distortion or camera blur.
- Pinned SDK compile and GitHub CI must pass for this EXACT revision;
  offline geometry screenshots and Rust unit tests are not in-game FPS proof.
- Do not publish or install as a finished mod. For future opt-in testing,
  use only a separate disposable local test world and record actual Horizon
  screenshots and GPU/FPS metrics before promoting quality status.

## Development rules

See `docs/M35_PROCEDURAL_VFX_DIRECTION.md` for verbatim user requests,
research, promises, limitations and strict acceptance gates. All changes
for this milestone were made via GitHub without using Remote Desktop
Commander or exhausting the user's limited PC connection.
