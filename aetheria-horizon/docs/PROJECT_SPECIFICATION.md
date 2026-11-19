# AETHERIA HORIZON — persistent master specification and execution contract

**Scope:** High-fidelity original fantasy RPG magic mod for Minecraft 26.3 through Horizon's Rust mod SDK and available client rendering API. No Codex. No paid fal model dependencies. Work, notes and versions must remain in the user's GitHub under \`aetheria-horizon/\`.

This document is an **organized specification from the user's original master prompt**, not a byte-for-byte copy of the chat. \`docs/memory/USER_LOG.md\` preserves exact quotations where provided; refer to the original user message for the complete original phrasing.

## 1 — Architecture and release routes

- Rust gameplay core independent from the engine when appropriate, \`EngineBridge\` for verified engine capabilities.
- Native source-mod \`.hmod\` with supported Horizon \`Hooks\`, GPU interfaces only through documented Horizon APIs. Native builds are per pinned game revision and rely on Horizon Launcher. Do **not** invent unrestricted direct \`wgpu\` access.
- Precompiled Rust/WebAssembly \`.hmod\` fallback for visual testing via \`horizon-mods/\`, which the launcher loads without rebuilding native source. WASM still uses Horizon's actual rendering API; do not promise native wgpu handles in guest.
- Own WGSL SDF/material shaders when supported by Horizon's fragment pipeline, Naga validation, camera correctness, bloom pipeline only if supported. No Iris/OptiFine, no Vanilla 2D billboard shortcuts.
- All performance claims require real testing. Instancing/LOD/particle budget and low-FPS scalable presets; zero GC alone does not guarantee smooth rendering.
- Mod format/version/SDK pins must be tested, never guessed. Do not install WASM in \`horizon-native-mods/\`, or native source in \`horizon-mods/\`.

## 2 — Permanent GitHub memory and reproducibility

- \`docs/memory/USER_LOG.md\`: dated exact requirement quotes and changes. Do not falsely claim full verbatim archive if only snippets exist.
- \`docs/memory/AI_DEV_LOG.md\`: tech changes, hooks, build/check logs, tests, CI links, limitations.
- \`docs/memory/SPELL_REGISTRY.md\`: **200 planned spells** with ID, class, name, cast type, mana, cooldown, 3D mesh, WGSL/SFX, status.
- \`docs/memory/PLAYTEST_LOG.md\`: video timecodes, user feedback and in-game observations.
- Every milestone has GitHub commit SHA and enough docs/source to continue.
- Include exact build and deploy workflow commands. No unverified installable builds.

## 3 — 10 classes × 20 unique spells

Each class has **14 standard, 4 signature awakenings, 2 cinematic ultimates**; total **200**, each with distinct geometry, movement, physics and audio identity, **no recolor clones**:

1. Pyromancer: solar, magma, phoenix, meteors and supernovas.
2. Cryomancer: ice spikes, crystal cages, blizzard and stasis.
3. Aeromancer: tornado, windblade, vacuum, mobility.
4. Geomancer: tectonics, boulders, stone fists and crystal barrier.
5. Hydromancer: waterjets, wave floods and water-dragon summons.
6. Druid: organic roots, vines, giant carnivorous flora, spores and World Tree.
7. Electromancer: natural lightning storms, chain reactions, magnetism and storm beasts. **No orbital or ion gun weapon**. Orbital Laser reference is technical rendering technique only.
8. Void / Astral Mage: spatial rifts, gravity, starlight, teleport, dark implosions.
9. Chronomancer: local time slowing, projectile stasis, temporal rewind, future echoes.
10. Necromancer / Blood Mage: bones, soul drain, crimson ritual, undead summons.

## 4 — Core RPG gameplay

- Player-bound \`Mana\`, \`MaxMana\`, \`ManaRegen\`, \`Cooldowns\`, \`UnlockedNodes\`, \`SelectedClass\`, persisted safely and authoritative/synced for multiplayer.
- Cast types: Instant, Charged (0–1 held), Channeled, Combo chains (e.g. Wet + Shock).
- Books with 4–8 configurable spell slots; one-use scrolls and crafting; tier I–V \`Arcane Inscription Table\`.
- Structured world interactions with permissions and rollback safety: radial knockback, freeze/scorch, fluid interaction, timed temporary block edits, carefully limited crater effects.
- Custom textures/models for items and readable staff gestures/player animation; do not fake game action with visual-only output.

## 5 — Visual and audio identity

- Unique layered world-space 3D geometry for every spell, volumetric approximations, dynamic procedural shards/lightning/roots/flora where supported; SDF runic inscriptions used selectively, not the default look of every cast.
- Shader identities with HDR emissive **only in verified compatible render pipelines** and tempered exposure; fresnel, animated textures, trails, mist/heat and refraction when APIs permit.
- Dynamic lighting where Horizon exposes suitable hooks; scalable draw budgets, LOD and culling; no unconditional 100,000-particle guarantee.
- Three-phase spatial sounds: Charge → Cast → Impact, with range/reverb cues.
- 20 cinematic ultimates: cutscene framing, hitstop/local time dilation, camera shake with accessibility options, monumental **fantasy** sky/world phenomena; avoid global multiplayer tick freezes and unverified camera edits.

## 6 — Fluid RPG UI

- Non-boxy liquid mana indicator (SDF wave/slosh), radial spell selector with blur and 3D previews as supported.
- Constellation skill tree with pan/zoom, nodes, Bezier magical links, class selector.
- Settings for quality, VFX intensity, subtitles, photosensitivity, particles, camera shake, fluid UI; sensible defaults for older RX 580 hardware.

## 7 — Priorities and completion definitions

- M0–M3: portable Rust gameplay core, 200 designs, 20 logical cast intents, 39 unit tests verified in CI.
- M4: 5 procedural preview spells as native source; game launcher native build lock prevented local testing.
- M5: 5 precompiled Rust/WASM shaders and 3D geometry; CI validated, **user then confirmed video in game**.
- M6: improve shape/readability and add 3 distinct physical VFX patterns each for these 5 preview spells; prove CI and get user visual feedback.
- Later: connect portable gameplay core, mana and cooldowns, damage/effects, UI, animations, SFX, then productionize remaining 180 core intents and all 200 actual spells.
- \`Visual preview builds\` are **not** the 200-spell release. Acceptance means confirmed loading, enjoyable recognizably different casts, meaningful gameplay interactions and real GPU test, not only Cargo compilation.

## 8 — Visual references

- Four original TikTok clips detailed in \`docs/VIDEO_REFERENCE_ANALYSIS.md\`.
- User gameplay footage \`horizon-client_MnaFG4NG1R.mp4\` detailed in \`docs/memory/PLAYTEST_LOG.md\`.
- Orbital Laser, modded fantasy inspirations: reference only for large volumetric 3D compositing and world impacts, never for copied art or sci-fi orbital guns.

## 9 — Working repository layout

\`\`\`text
aetheria-horizon/
  Cargo.toml                    # portable RPG subsystem
  src/core/                     # progression, save, networking adapter
  src/magic/                    # logic, registries, casting
  src/vfx/                      # spatial planner, geometry, camera
  src/sfx/                      # acoustic layer
  src/ui/                       # fluid HUD, wheel, constellation
  assets/shaders/               # WGSL
  docs/PROJECT_SPECIFICATION.md
  docs/VIDEO_REFERENCE_ANALYSIS.md
  docs/memory/USER_LOG.md
  docs/memory/AI_DEV_LOG.md
  docs/memory/SPELL_REGISTRY.md
  docs/memory/PLAYTEST_LOG.md
  preview-5spells/             # native source test
  preview-5spells-wasm/        # precompiled playable visual test
\`\`\`
