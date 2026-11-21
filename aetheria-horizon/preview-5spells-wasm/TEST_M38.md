# Aetheria M38 (0.8.6) — real Horizon GPU visual QA, internal

This is a **development-only** client VFX preview, NOT a release claiming
100 finished or gameplay-verified magic spells.

Target Horizon Nightly 2026-11-20, Minecraft 26.3, pinned SDK.
The new GPU signature **Thorn Dart (WILD rank 1 / spell #101)** is built as
a sharply tapered green/wooden thorn with seven alternated barbs and
animated glowing veins. It replaces the previous generic green smoke
pillar seen in a REAL M37 Vulkan GPU screenshot. If the optional
signature SDF shader fails to load, the mod uses actual coherent source
3D thorn geometry instead of a shaderless rectangle.

Tests MUST include:
- The WHOLE Thorn Dart sequence at t+0.2 / t+0.7 / t+1.2 / t+2.3 seconds,
  captured using official Horizon screenshot commands from actual GPU.
- Confirm spell name is **Thorn Dart** in UI. Source photo names from
  previous M37 test misleadingly said Ember although the user selected
  Thorn Dart; correct that in new test captions.
- Verify spearhead and seven barbs are visually readable; reject any
  small green dot, flat card, vertical green gas, disconnected tube.
- Then select **Ember Needle** and **Pyre Serpent** and check signature
  shader works, is visible and looks intentional in actual game view.
- Test occlusion/depth, shader compilation errors, simultaneous casts
  and LOD. Record FPS on user's RX580 and avoid user/agent shared
  keyboard interference.

Testing without touching the user's installed M32:
Use the local Nightly executable's horizon-ctl launch --dev-mod PATH
--game-dir SEPARATE_QA_DIR --world SEPARATE_QA_WORLD --creative.
Use horizon-ctl screenshot to capture actual GPU frames. Never pass
off AI-generated images or offline SVGs as gameplay.

This release does not have 100 independently finished hero GPU shaders;
only 12 named shader heroes, 100 M36 motion profiles and 200 demo IDs.
Real server damage, physical world edits, multiplayer, saves and full
screen-space HDR integration remain unimplemented.
See docs/M38_ACTUAL_GAME_SCREENSHOT_QA.md and prior M37 docs.