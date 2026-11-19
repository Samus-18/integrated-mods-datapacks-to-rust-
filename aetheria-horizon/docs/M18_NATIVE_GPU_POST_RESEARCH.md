# M18 — Real GPU VFX architecture from original Rust effect source (2026-11-19)

## Discovery grounded in the user's four original videos

After rewatching all four original attachments and searching the actual Horizon Rust mods, identified a highly relevant OPEN-SOURCE technical reference:

- [Stink-O/orbital-laser](https://github.com/Stink-O/orbital-laser), **MIT**. Its repository includes a real Crystal Bloom effect, a broken-sky shader, a modular post-pass shader architecture, and documentation of coordinated impact and terrain effects. These directly resemble several observed features of the user's references. We did **not** copy its shader code, textures, models or science-fiction elements.
- [Orbital Laser implementation notes](https://github.com/Stink-O/orbital-laser/blob/main/docs/HOW-IT-WORKS.md) describe camera-aware fullscreen post drawing: rays reconstructed from camera matrices, opaque scene depth for occlusion, RGB color and floating-point intermediates, and individual mod-data per spell.
- [Horizon official code mod API](https://playhorizon.dev/docs/mod-code) supports hook-based Rust mods; native hooks expose more than guest WASM. Its current separate [shader API 0.2](https://playhorizon.dev/docs/api/shaders) can provide postprocessing, but that does **not** imply the pinned v0.5.3 guest WASM .hmod can directly read depth and draw postpasses.
- Stink-O's native mod/shader implementation is an independent project / fork with versioned native integration. **Do not import it or claim a simple binary load works on the user's Nightly 2026-11-18 dd67b76.** First verify the official, exact native API and build route.

## Why our earlier effects did not achieve the requested quality

The 30 client effects were mostly colored world-quad geometry with a modest per-face UV fragment. Even a giant 900-quad budget cannot create **depth-aware volumetric scattering**, temporal blend, HDR glow, large transparent atmospheric plume or world-space refraction from an ordinary local-only fragment without color/depth buffers. The 180-view CPU SVG snapshots prove geometry projection only, not a realistic game effect.

## Original experimental GPU prototype added in Aetheria

- Source: native-gpu-vfx/aurora_volume.wgsl.
- Purely ORIGINAL procedural algorithm: along one ray through the world, sample a bounded (6–24) series of positions, evaluate a twisted fantasy aurora density field around a given spell target, integrate colored radiance with front-to-back transmission, and output premultiplied radiance/alpha.
- The eventual native post caller must supply exact ray origin/direction reconstructed from the engine camera, target point, spell clock and a maximum ray distance *clamped to the actual scene-depth hit* to avoid magic shining through walls.
- The shader is independently checked with **Naga 30.0.1**, the version in the pinned Horizon dd67b source Cargo.lock, as part of the preview's native-host CI tests.
- This WGSL is **NOT** shipped or invoked by the current 30-spell WASM .hmod. It has **not** been rendered in Horizon and is not evidence of native GPU compatibility.

## Implementation gate before a beautiful native Aetheria release

1. Verify postpass hook and resource access on the exact Horizon version used. Old .hnmod source-mixins are not equivalent to current .hmod native code; use official tool's native mod template and pinned API.
2. Integrate a **unique spell-owned post uniform** (center, color phase, time, quality) from the Rust authoritative cast system. Never fake synchronization with sampled random UI.
3. GPU shader must read current scene color + scene depth; reconstruct world ray safely, clamp before opaque blocks, and compose in verified linear-light HDR intermediate. Fall back gracefully when unavailable.
4. Layer with **actual connected 3D geometry** (already prototyped in hero_vfx) and original positional sound. Distinct school-specific materials, not recolored clones.
5. Run Naga shader check, headless captured image if feasible, GPU timestamp/pacing on real RX 580 and actual Horizon Nightly. Add quality controls and photosensitivity reduction before multiplayer.

## Current honest status

One user-video-inspired 3D ice spell (#6 Shard Chorus) and its surface shader are in the internal WASM branch. A separate experimental raymarch GPU calculation source is Naga-testable. **The large cinematic source-video quality, true HDR and game integration are still unfinished.** Do not ask the user to playtest intermediate builds or claim 30 complete spells.
