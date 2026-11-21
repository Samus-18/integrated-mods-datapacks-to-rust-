# Aetheria M40 v0.8.8 — INTERNAL REAL-HORIZON QA (not final)
Minecraft 26.3, Horizon Nightly 2026-11-20 (SDK 0a875da).
Thorn Dart (WILD 01/20, zero-based id 100):
- M39 transparent flat 2.5D green arrow was rejected in actual Radeon RX 580 captures.
- M40 routes Thorn Dart **exclusively to world-space solid 3D bark and
  sculpted leaf geometry**, even when optional signature_sdf shader works.
- Stage progression: growing seed (0-8 ticks), forward thorn flight
  (8-36), 3D branching burst (36-67), shrinking lingering leaf/spore
  after-effect (67-85). Not just four identical still images.
- A handful of small masked pollen sprites is optional; never add the
  generic green fog rejected during M37/M38.
- Preserve all user game installs. Use isolated --game-dir and --dev-mod.
- This is not actual server damage, collision, hit confirmation, crafting,
  lighting/HDR postprocess, production art approval, or 100 finished spells.
- Only claim game quality after inspecting **real Horizon Vulkan screenshots**.
