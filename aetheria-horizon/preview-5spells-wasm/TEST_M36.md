# Aetheria Horizon M36 (0.8.4) — 100 procedural spell VFX **ART REVIEW BUILD**

This artifact is **an internal development build, not 100 finished polished spells**.
Engine: Horizon Nightly 2026-11-20 (pinned commit
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`), MC 26.3.

### Current source truth

- 100 explicitly named spell VFX choreography profiles, ten from each of ten schools.
- 48 different mathematical motion/silhouette patterns, each with its own
  timings, dimensions, direction and spread. 12 shader material classes
  (flame, glint, droplet, wisp, smoke, shard, petal, plasma arc, ribbon,
  fractured sigil, temporal fracture, bone).
- These are procedural 3D particle world-quads submitted through a separate
  optional depth-tested WGSL material. **They are not imported Blender assets,
  but they are NOT full actual game damage/spell-body implementations either.**
- Existing solid M32 VFX remain under the new particle layers; original 3D
  Frost Needle (#021) and Rime Bridge (#022) are unchanged.

### How to browse in disposable QA world

Tap **R** to change school, hold **R** and scroll to select spell, **Z** to
cast. The new authored profiles cover ranks 1–10 in each school:
Pyromancer #001–010, Cryomancer #021–030, Aeromancer #041–050,
Geomancer #061–070, Hydromancer #081–090, Druid #101–110,
Electromancer #121–130, Astral #141–150,
Chronomancer #161–170, Necromancer #181–190.

The other half of each class uses the older M35 shared emitter. The Rune
effect is not evidence of server-generated world physics or collision.

### Exact visual QA images and limits

A real Rust standalone test compiles the *same* authored geometry Rust source
that Horizon uses. It generates ten SVG contact sheets from source-quads:
100 spell panels, each at ages T+31/T+62. These SVG images apply an SVG
approximation to shader masks to help inspect spatial movement. They are **not
GPU/WGSL output, Minecraft screenshots or AI-generated pictures**. The first
100 review demonstrated that simple dots/strokes were too small to meet the
user's reference visual quality; 12 new mask families and larger source
inspection scale are an experimental improvement, still not validated in game.

### Hard completion blockers

Do not distribute this version as complete. For 100 finished spells we still
need to replace generic solid core silhouettes, verify actual GPU postprocess
support, synchronize audio and individual visual phases, add authoritative
world/collision/damage and saves, measure user's RX580 FPS and visually approve
every real Horizon in-game screenshot/video. See
`docs/M36_100_AUTHORED_PROCEDURAL_SCENES.md`.

All code and source art are saved on GitHub; no Remote Desktop Commander
connection to the user's PC was used in this phase.
