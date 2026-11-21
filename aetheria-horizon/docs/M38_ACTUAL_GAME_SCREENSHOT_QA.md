# M38 real-game visual QA
On 2026-11-21 the user's PC was connected with explicit permission.
Installed official Horizon Nightly 2026-11-20; existing user mod was M32
v0.8.0, not M37. M37 .hmod 0.8.5 was downloaded from a SUCCESS GitHub
Actions artifact, SHA256:
5f364d39a3c270f8ffc88418899bb989437b3d1171b4c8d884c18fdf7d3026cc
It was tested via horizon-ctl --dev-mod in a fully separate QA
game-dir and QA world, without overwriting the user's real M32 install.
Real Vulkan Radeon RX580 GPU game screenshots were collected:
preview-5spells-wasm/target/m37-runtime-qa/baseline-m37.png
and M37-Ember-t0200/t0700/t1200/t2300.png. Filenames incorrectly say
Ember: user's active selected spell was WILD Thorn Dart (index 100).
The real images showed a pale green diffuse smoke pillar, vague flat
transparent surfaces. ART FAIL. User was actively giving game inputs,
so avoid competing with them over the camera/keyboard. User later exited.
The 75fps figure was observed while loaded in forest and is NOT a
controlled under-spell benchmark.

M38 response: dedicated sharp barbed projectile SDF material for
Thorn Dart, now 12th hero signature, plus whole original 3D
tapered stem with alternating side spikes for safe shader failure.
No unmasked proxy quads allowed. Fresh pinned Naga/WASM CI required,
followed by ACTUAL new in-game screenshot before any quality approval.
The remaining 88 of the first 100 do not yet have unique GPU hero SDFs.
Not 100 production-ready spells, no server-authoritative damage/physics.
Never substitute AI-generated concept art for gameplay screenshots.
M38 critical correction: the old M36 procedural wisps also drew a large green fog billboard over Thorn Dart. Added explicit suppression of generic M35/M36 particle overlays for all twelve signature spells, even if SDF shader fails; use name-true GPU mask or coherent 3D fallback alone. This addresses the actual original symptom rather than merely layering a spear on top of smoke.
