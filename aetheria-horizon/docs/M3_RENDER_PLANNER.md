# M3 — 3D VFX instance planning (portable Rust)

This milestone adds the first **frame-level spatial VFX planner** for the 20 M2 gameplay intents.

- \`src/vfx/scene.rs\`: 20 distinct fantasy 3D spatial silhouettes, 4 phase-aware instance settings, local-space-to-world transform, stable 10-class color palette.
- Stable deterministic scheduling sorted by cast tick and spell id; fair, round-robin allocation for simultaneous effects.
- Global ceiling: 1,200 instances/frame; at most 64 active effects planned per frame. Distance LOD, adjustable quality, charged intensity ceiling, invalid float/direction guards and despawn after the timeline.
- Samples carry typed motif, stage, 3D position, scale, tint, emissive and alpha for later upload to a renderer.
- Unit tests verify distinct profiles, fairness, deterministic order, timing, culling and heavy-load limits.

**Scope:** This is a portable Rust rendering-intent planner. It does not draw on screen, compile WGSL, guarantee bloom, attach to a camera or mutate blocks. Official Horizon's current native \`.hmod\` packaging uses a pinned SDK. The existing \`horizon-mod.toml\` is a draft; do not install as a finished mod.

Next: pin the exact supported client SDK and build a real \`#[mod_api::horizon_mod]\` entry; connect cast events and client-side rendering using **verified** available hooks; validate shader layout, Naga and actual game performance before publishing.
