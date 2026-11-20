# Aetheria Horizon — M34 original Blender ice spells (0.8.2) — Horizon Rust

**Experimental visual mod. NOT a finished 200-spell RPG.**

This M32 build is a direct response to the user's 107-second 2026-11-20 in-game video. That video conclusively showed too many key bindings, a giant confusing spell gallery and VFX that looked like sticks, thin rings, empty spokes or repetitive transparent surfaces. Source compiles, mock shaders and CPU projections are **NOT** enough to certify beauty. See the full reference index in `../references/original-videos/` and `../docs/VIDEO_REFERENCE_ANALYSIS.md`.

## Install on the pinned Nightly

- Horizon **Nightly 2026-11-20**, Minecraft 26.3.
- Signed official SDK commit: `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`.
- Download **Aetheria-Horizon-M32-ART-REBUILD-Nightly-2026-11-20-v0.8.0** from a *successful* GitHub Actions run on this branch, unzip to find **aetheria_preview.hmod**.
- Remove an old Aetheria `.hmod` from your instance's `horizon-mods/` and `horizon-native-mods/`, install the new file into `horizon-mods/`, enable it and restart.
- Confirm the in-game mod status is **Loaded**, not simply Ready in the launcher.
- If a run has failed or is still building, do not install its artifact. See `TEST_M32.md` for current controls and known limits.

## Exactly TWO key bindings

- **R tap**: next magic school (10 schools).
- **Hold R + mouse wheel**: cycle 20 spell IDs *within the selected school*, without hijacking the normal hotbar when R is up.
- **Z**: cast the selected client-local visual spell (G conflicts with the Horizon game control).
- No K fullscreen gallery, no N/M/J/L/Z/X/C/V/B direct shortcut keys. The small native picker appears briefly over the hotbar only when used.

## M32 visual architecture

The **actual live render function** in `src/lib.rs` dispatches all 200 visual slots to `src/volumetric_forms.rs`, retiring the visible old beams, rod-tubes, runic-ring scaffolds and spikes. The old renderer source remains for static reference and isolated tests, but it is not dispatched by the live client.

The new original Rust geometry is wide, fully filled, double-sided curved atmosphere sheets, irregular non-stick ellipsoid masses, and broad fractured solid plates. It uses world-space geometry with Horizon depth testing. Each spell's source mesh semantics feed its sculpt choice, with different profiles for ten schools and rank-dependent size, shape and animation. The actual packaged WGSL shaders integrate a bounded **6/10-sample local 3D density field** on low-alpha volume surfaces; opaque stone/ice bodies retain surface-aware lighting. The fair geometry scheduler selects whole panels only (900 world quads total/frame).

**Not yet:** true shadow-casting GPU light in the world, fullscreen depth/color post-processing, real global bloom/HDR, actual refractive portals, individually finished/approved art for 200 spells, or gameplay spell damage. The remaining 170 `DesignIntent` server actions are still disabled. Do not market this as equal to reference videos until a real in-game 2026-11-20 Nightly art pass confirms it.

## Why the earlier builds were misleading

M30 and M31 tests checked Naga WGSL source, Rust geometry bounds, 3D mathematical structure and WASM package. They did NOT show that the game output looked cinematic. M31.1 fixed the *API transport version*, but not the too-large GUI or unattractive artwork. M32 explicitly stops treating a passing compile as artwork approval.

## Original research (technical concepts only, not copied art)

- Horizon shaders and post-pass API: https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md
- Orbital Laser volumetric and real-world physical compositing: https://github.com/Stink-O/orbital-laser/blob/main/docs/HOW-IT-WORKS.md
- Spell Engine particles and spell selection: https://github.com/ZsoltMolnarrr/SpellEngine
- Magic Particles Lib 3D extrusion: https://www.curseforge.com/minecraft/mc-mods/magic-particles-lib
- GeckoLib animations and event keyframes: https://modrinth.com/mod/geckolib

All Aetheria code is Rust and WGSL. No Java port, C++ renderer, Iris or OptiFine.
