# M32 / 2026-11-20: actual user-video failure report and 0.8.0 visual redesign

**Trigger:** User's original recording `horizon-client_HlzF8azN8R.mp4` (107.333 seconds at 1894×1036, 30 fps, uploaded in this ChatGPT turn). Binary source was analyzed within chat; not republished to GitHub.

## Verified failures visible in that recording
- ~4–8 s: Horizon key binding page shows excessive Aetheria bindings (11+ shortcuts) all at once.
- ~16 s: custom K constellation/skills page takes over most of screen with many compact text cards, no intuitive interaction.
- ~60–92 s: frequent thin orange rings, spikes, narrow green sticks and skeletal low-opacity white shards; effects show weak material/volume identity and repeated generator templates.
- The prior build passed Rust/Naga CI checks but did NOT meet the user's reference visual quality bar. A clean .hmod package proves nothing about cinematic fidelity.

## Implemented, testable code changes
- `preview-5spells-wasm/src/lib.rs`: only two registered keybinds: R (tap: school, hold + mouse wheel: spell) and G (cast). The old 5 direct-cast, N/M/J/L and K bindings are unregistered; no full-screen constellation is drawn.
- `src/compact_ui.rs`: compact three-choice menu above hotbar, with current mana and cooldown.
- `src/lib.rs`: all 200 visual preview slots now call `volumetric_forms::render` rather than the old stick/ring/beam generator dispatch. The legacy code remains in tree for migration/tests and is not the live path.
- `src/volumetric_forms.rs`: 18 semantic classes based on the existing human-authored `SpellSpec.mesh`, using broad continuous 3D surfaces and whole-component LOD. The first new generic sphere layout was REJECTED after actual rendering of 18 sampled Rust-projected SVGs. Second revision removes generic spheres for all forms except specific Artifact / Cloud / Beast mesh semantics, and constructs different geometry for material, flow, wings, rifts and armor.
- `assets/aetheria_preview/shaders/pipeline/ethereal.wgsl`: the existing limited 6/10-sample density integrator now gives atmospheric sheets torn, feathered silhouettes and density-based transparency, eliminating the previous positive minimum alpha that exposed rectangular surfaces. Still no real fullscreen scene-depth/color post or HDR bloom.

## Original independent reference study
- Orbital Laser source and HOW-IT-WORKS: https://github.com/Stink-O/orbital-laser — true postprocessing with scene depth and analytic 3D solids in its fork, not transferable as-is to stock Horizon guest material shaders.
- Official Horizon shader API: https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md
- Spell Engine UX/particle library: https://github.com/ZsoltMolnarrr/SpellEngine
- Iron's Spells 'n Spellbooks (RPG casting and school identity): https://github.com/iron431
- Magic Particles Lib 3D surfaces: https://www.curseforge.com/minecraft/mc-mods/magic-particles-lib
- GeckoLib complex animation keyframes: https://modrinth.com/mod/geckolib

## Actual quality gate, NOT CI bait
The M32 code compiles against the real signed Horizon Nightly 2026-11-20 SDK `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`. Geometry/CI tests and source SVG are necessary but not sufficient. M32 has NOT been shown to look as good as the original cinematic videos. The remaining block is a real GPU postprocessing path and **200 individually shaped and validated** effects, followed by in-game FPS validation on the user's Windows GPU. Do not call M32 final, do not claim all 200 visually approved, and do not distribute it as a finished mod solely because CI passed.
