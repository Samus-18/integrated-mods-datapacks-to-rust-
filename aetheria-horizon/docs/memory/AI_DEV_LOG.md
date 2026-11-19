# AI_DEV_LOG

## M0 / 2026-11-18 — Baseline architecture and code
- Reviewed 4 user MP4 uploads for VFX motifs. Extracted notes in docs/VIDEO_REFERENCE_ANALYSIS.md.
- Compared current Horizon docs, the mixin native template and orbital-laser's fork-specific SDK. Avoided invented Hooks render signatures.
- Stored 200 unique fantasy spell design entries (10 classes × 14 standard + 4 awakening + 2 ultimate); 190 remain designs only.
- Added standalone Rust `PlayerMagic` mana, regen, unlocks, cooldowns; a v1 round-trip save codec; a deterministic casting engine with 10 distinct `SpellAction` intents (first spell per class); and defensive validation tests.
- Added `EngineBridge` trait stub for server-authoritative Horizon integration, two procedural 3D mesh generators, particle draw budget and camera cue model.
- Added simplified spell wheel, HUD, constellation coordinates, audio cue descriptions, and four **standalone WGSL experiments** (runes, beam, refraction, liquid HUD).
- GitHub Actions workflow for `cargo test --all-targets`. Local toolchain unavailable; **do not claim tests passed until CI proves it**.
- The drafted `horizon-mod.toml` pins upstream Horizon `43a7e395836c0ab6c8538862e31ede8fccf494ba`, mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0` as reference metadata ONLY; no SDK bridge or final mod packaging.

## Required next work
1. Verify CI test result and fix failures.
2. Generate exact SDK for the target build and implement the real `#[mod_api::horizon_mod]` entry point, world save and networking.
3. Validate WGSL in Naga and bind supported render targets/matrices; add real lighting/VFX, meshes and audio assets.
4. Implement all remaining spell behaviors (no palette swaps), craftable books, inscription table, UI and skill tree.
5. Cinematic ultimates with local-only camera slow effects; performance and in-game validation against Horizon 26.3.

**No claim of playable mod, passed in-game tests, 200 working spells or 100k particle guarantee.**

## M0 validation result — GitHub Actions
- Run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44394835431
- Workflow: Aetheria Rust core / job core-tests, commit `5bfea4367debd2416e1a13c6d2f38c747e2c7ef0`.
- `cargo test --all-targets`: **19 passed, 0 failed**, GitHub Actions conclusion **success**.
- This establishes only portable Rust core compilation/unit testing. No Horizon engine, shader compiler, native packaging, or in-game GPU tests have passed.

## M1 / 2026-11-18 — RPG systems phase
- Added class-validated 4–8 slot `Spellbook`, duplicate prevention and selectable spells.
- Added `InscriptionTable` with safe tier upgrades I–V using scroll/crystal material budgets (engine integration pending).
- Added four target-specific elemental reactions, including Wet + Lightning → Conductive Surge, with expiration and consumption.
- Added charged cast timing (hold/release) and channeled mana drain with protection against double-charging on the same tick.
- Corrected the first prototype spell's `cast_type` to Instant (10 entries), matching their actual behavior.
- Unit tests written; test result pending GitHub Actions.

## M1 validation — GitHub Actions
- Run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44395652564
- Commit `a1a10be5b35848d5378495b14c42060885a1f5ea`
- `cargo test --all-targets`: **28 tests passed, 0 failed**; workflow completed successfully.
- Does not include Horizon native SDK build, Naga WGSL check, GPU rendering, real world/save or gameplay integration.

## M2 / 2026-11-18 — Expanded portable gameplay core
- Added second unique gameplay action to all 10 classes (Cinder Fan, Rime Bridge, Razor Crescent, Crystal Traverse, Flowstep, Root Snare, Static Halo, Gravity Thread, Moment Slip, Soul Lantern): **20 distinct core intents, 180 design-only spells**.
- Implemented ID-based action dispatch without disturbing original 10 class-specific prototypes. Unimplemented actions never spend mana. NaN/infinite charge returns `InvalidInput` before mana/cooldown changes.
- Added 20 separate four-stage VFX sequences, per-spell timing and particle ceilings; metadata only, **not yet wgpu rendering**.
- Added Rust tests for 20 unique action variants, safe invalid inputs, temporal request and VFX stage boundaries.
- Confirmed from official Horizon modding book: native mods are `.hmod` code archives compiled by pinned launcher SDK; source-mixin `.hnmod` workflow is separate. Do not claim compatibility based on Orbital Laser's fork-specific SDK.
- Rechecked all four attached videos by direct frame samples. Replaced reference notes with observed timings and fantasy-only adaptations.
- M2 CI run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44397904415, source commit `2016ec34013ed97970e1e3ef1984be441b25163d`: **33 tests passed, 0 failed** via `cargo test --all-targets`.
- First M2 CI run failed because an older test expected spell 2 to be unimplemented. This was correct to update after enabling Cinder Fan; fixed test now targets spell 3. No other tests failed. These are core tests, **not** a playable mod verification.

## M3 / 2026-11-19 — Deterministic 3D effect instance planning
- Added `src/vfx/scene.rs`, registered via `src/vfx/mod.rs`: portable CPU-side, per-frame 3D instance planning for the 20 M2 spell intents, each with a unique spatial silhouette (not merely a color swap).
- Added phase-specific scale, alpha and emissive, 10-class palette, local-to-world transforms, finite-input and direction checks, quality control and distance falloff.
- Added a global 1,200 instances/frame limit, 64-active-effect scheduling limit and fair round-robin distribution among simultaneous casts. Limits protect the intent planner only, not yet actual GPU frame times.
- Added six Rust tests: unique motifs/paths, fairness/cap, determinism, phase lifetime, bad-input culling, burst limits.
- GitHub Actions run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44482049153
- Code commit `cec3131b3d9b2a13de8985a49d6276c5d8aafe02`: **39 passed, 0 failed** with `cargo test --all-targets`.
- Updated compatibility research against current official Horizon: a native code mod uses a flat `.hmod`, pinned prebuilt SDK, `#[mod_api::horizon_mod]`, and `Hooks`/typed Host interfaces. Existing `horizon-mod.toml` remains a draft, not a tested installable mod; no real engine hooks or GPU backend have been wired.
- Technical reference only: Orbital Laser historically used fork-specific WASM APIs; do not copy unsupported renderer hooks or sci-fi content.

## M4 / 2026-11-19 — First five-spell native VISUAL test package
- Added separate `preview-5spells/` source-native mod. Five distinct procedural 3D fantasy silhouettes rather than only circles/beams: Phoenix Garden, Glacier Cathedral, Living Worldroot, Thunder Wyrm, Astral Sunder.
- Verified available Horizon APIs: `Registrar::register_key_mapping`, `ClientHost::consume_click`, `ClientHost::camera`, `Hooks::submit_custom_geometry`, `WorldQuad`, optional render pipeline WGSL fragment.
- SDK pin `dd67b76ff23e28c6d8e21859df600d31572f81a0` has an official release; pinned mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0` is checked against official Horizon source.
- Preview triggers: Z/X/C/V/B, 8 active spell limit and 1,600 extra quad limit. No world edit/entity damage, no direct GLSL/Iris/OptiFine API or Sci-Fi weapons.
- Source native hmod archive verified locally as safe ZIP; SDK compile and Minecraft 26.3 in-game test NOT performed at this milestone. A dedicated native CI workflow attempts to prove compatibility and emit an artifact.

## M5 / 2026-11-19 — Launcher lock workaround: precompiled Rust/WASM visual preview
- User's Horizon Launcher 0.2.81 could recognize Aetheria native 0.1.2 but its native builder refused with "A native build or cache cleanup is running" and no useful compiler log. Official launcher sources show its native cache OS lock can fail before any mod compilation; repeatedly repackaging the native mod cannot resolve that environmental failure.
- Added separate `aetheria-horizon/preview-5spells-wasm/` with same 5 Rust client-render hooks and original procedural mesh/WGSL logic, but no `[native]` manifest. The SDK source pin moves to Cargo metadata and the official tool generates `[wasm]`.
- Lowered visual world-quad planning cap to 900 for WASM prototype. The sandbox still calls the Horizon renderer, so native builder lock is no longer a prerequisite to test visual spells.
- Added `.github/workflows/aetheria-preview-wasm.yml`, using official pinned `cargo-horizon-mod` at Horizon `dd67b76ff23e28c6d8e21859df600d31572f81a0`. Job compiles `cargo horizon-mod build --wasm` and checks ZIP layout, `mod.wasm`, manifest pin and SHA-512 integrity.
- Run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44492554390 — **SUCCESS**, completed code compilation, package SHA check and artifact upload.
- Verified downloaded `aetheria_preview.hmod` with independent ZIP integrity checks, wasm magic/version and `SHA512(mod.wasm)==manifest.wasm.sha512`. Package size 2,667,720 bytes, SHA256 `f557746c4c6ccb535afadc5ae4b18cc50d4a0e48600026318c36cef1736c4094`.
- Installation: remove any native copy from `horizon-native-mods/`, put **only the compiled WASM .hmod** into `horizon-mods/`, enable and launch **without Build**. Same mod id means simultaneous native/wasm copies are unsupported.
- Important: `cargo horizon-mod build --wasm` succeeds on Linux CI, **not equivalent to verified in-game Windows Horizon rendering**. Damage, mana and multiplayer integration remain missing. The user is to test in-game first.

## M6 / 2026-11-19 — First confirmed in-game video + visual variation pass
- Reviewed user-uploaded \`horizon-client_MnaFG4NG1R.mp4\` (100.13 sec, 1906×1036, 30 fps). User says appearance exceeded low expectations but requests **better variations**. Captured HUD FPS reads about 75 in sampled frames; not a hardware performance guarantee.
- At ~14 s Phoenix bloom is heavily overbright; ~27 s glacial shards dominate almost the entire screen; ~34–38 s multiple spells share sharp white facets; ~90–94 s Astral effect shows violet membranes and shards. Observations and caveats written to \`docs/memory/PLAYTEST_LOG.md\`.
- Created \`docs/PROJECT_SPECIFICATION.md\` as a durable organized version of all original master requirements (10×20 spells, no sci-fi, fantasy cinematic VFX, effects/physics/audio, spellbooks, persistent mana and cooldowns, fluid UI, full GitHub logs and registry). Not misrepresented as byte-for-byte copy of original.
- Updated \`docs/memory/USER_LOG.md\` with verbatim user feedback and the exact source video name. Updated \`docs/memory/SPELL_REGISTRY.md\` to distinguish the five in-game visual demos from the 200 planned RPG spells.
- \`preview-5spells-wasm\` v0.3.0: ten new geometry-specific alternatives (total **15 unique 3D visual arrangements**) cycling per key with every repeat. Added Phoenix Wings / Ember Cyclone, Glacier Arches / Frost Surge, Living Cage / Verdant Canopy, Storm Antlers / Storm Manta, Void Blossom / Fractured Stair.
- Optional shader now uses translucent rather than additive blending; lower emitted brightness and less alpha coverage to preserve detail. VFX 900-quads-per-frame budget is fairly shared over active spells rather than first draw consuming the pool. No paid assets or orbital weapons.
- Build status: **CI pending**. Do not claim v0.3.0 rendered correctly in the user's game until the user tries it. No new damage/mana/sound features in this phase.
