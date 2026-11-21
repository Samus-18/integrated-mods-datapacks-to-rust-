# M37 Internal — Pixel-true signature spell visual QA, NOT final

Engine: Horizon Rust pinned Nightly 2026-11-20 / Minecraft 26.3,
SDK 0a875da91dbb0fdcb7ba8c42c11341aa229af51a.
This M37 build is for development only and MUST NOT be marketed as 100
finished God-Wand-quality gameplay spells.

Key setup (in a separate actual game test world): tap R to cycle schools;
hold R and mouse-scroll to choose the spell in the current school;
Z casts locally. The 11 new optional GPU SDF heroes are Ember Needle
(#1), Pyre Serpent (#7), Snowblind (#30), Twister Seed (#48),
Faultline (#64), Rainweaver (#87), Root Snare (#102), Arc Net (#124),
Rift Slice (#145), Moment Slip (#162), Carrion Swarm (#187).
Frost Needle (#21) and Rime Bridge (#22) keep their prior separate art.

When optional `aetheria_preview:signature_sdf` pipeline fails, old
M37 source meshes should appear instead. No unmasked rectangles.
Check OpenGL/Vulkan shader compile errors, GPU backend compatibility,
world-space placement and camera-facing culling in Output/log.
Capture an **ACTUAL Horizon in-game screenshot/video** of each major
spell at charge, mid-formation, impact and afterglow. Also test multiple
simultaneous casts and 100-200 IDs. Do not substitute AI art or offscreen
Rust SVG geometry for actual client screenshots.

The 11 signature WGSL materials are pixel procedural 2.5D VFX. They
are NOT server damage, not physics collisions, not true traversable
ice, not authentic saving/progression or a real fullscreen postpass.
No FPS benchmark has yet been verified on RX580. Visual quality is
**UNKNOWN until actual client view is reviewed**.

Source record: `docs/M37_SHADER_TRUE_SILHOUETTES.md`.
