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
- Reviewed user-uploaded `horizon-client_MnaFG4NG1R.mp4` (100.13 sec, 1906×1036, 30 fps). User says appearance exceeded low expectations but requests **better variations**. Captured HUD FPS reads about 75 in sampled frames; not a hardware performance guarantee.
- At ~14 s Phoenix bloom is heavily overbright; ~27 s glacial shards dominate almost the entire screen; ~34–38 s multiple spells share sharp white facets; ~90–94 s Astral effect shows violet membranes and shards. Observations and caveats written to `docs/memory/PLAYTEST_LOG.md`.
- Created `docs/PROJECT_SPECIFICATION.md` as a durable organized version of all original master requirements (10×20 spells, no sci-fi, fantasy cinematic VFX, effects/physics/audio, spellbooks, persistent mana and cooldowns, fluid UI, full GitHub logs and registry). Not misrepresented as byte-for-byte copy of original.
- Updated `docs/memory/USER_LOG.md` with verbatim user feedback and the exact source video name. Updated `docs/memory/SPELL_REGISTRY.md` to distinguish the five in-game visual demos from the 200 planned RPG spells.
- `preview-5spells-wasm` v0.3.0: ten new geometry-specific alternatives (total **15 unique 3D visual arrangements**) cycling per key with every repeat. Added Phoenix Wings / Ember Cyclone, Glacier Arches / Frost Surge, Living Cage / Verdant Canopy, Storm Antlers / Storm Manta, Void Blossom / Fractured Stair.
- Optional shader now uses translucent rather than additive blending; lower emitted brightness and less alpha coverage to preserve detail. VFX 900-quads-per-frame budget is fairly shared over active spells rather than first draw consuming the pool. No paid assets or orbital weapons.
- Build status: **CI pending**. Do not claim v0.3.0 rendered correctly in the user's game until the user tries it. No new damage/mana/sound features in this phase.

## M6 validation — verified 0.3.0 WASM artifact
- CI run: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44496889719 — **SUCCESS**. Pinned Horizon Rust/WASM compilation, archive validation, manifest v0.3.0 and SHA-512 integrity all passed.
- Verified downloaded artifact independently in container: `aetheria_preview.hmod` 2,676,063 bytes, SHA256 `24d1f1c8e9ae35e88f52cb230a161919bfdda1bffabfa409ed1126b06ba0e3bd`. Contains `mod.wasm` (WASM v1 magic), shader, source, `uses.json`; verifies SHA512 against `[wasm]`.
- Release includes the latest variant labels in the in-game HUD. The user has **not** yet tested visual quality or FPS of 0.3.0; only 0.2.0 has user-provided in-game footage. Do not call five visual demos the completed 200 spell RPG.

## M7 / 2026-11-19 — Solid ice geometry + progression & original audio foundation; next user test deferred until 100
- User supplied an Ice Awakening screenshot showing large wide pale-blue 3D spikes emerging from ground, and explicitly requests **no more manual test builds until the mod is halfway complete at 100 of 200 playable spells**. User wants genuine 3D, living animation, original SFX, polished GUI, spell acquisition and full RPG mechanics, not beams/flat quads.
- Preview source `preview-5spells-wasm/src/lib.rs`: after Horizon API verification, read `ClientHost::crosshair_target()` for ground block hits on Glacier and Druid visuals, add `Scene::solid_crystal` that emits 5–8 radial faceted 3D wall surfaces (2 vertical layers per side, different face shading) and stage ice shards erupting in ground rows, arches and waves. No world/block editing is claimed.
- Portable RPG `src/magic/progression.rs`: deterministic learn costs for all 200 entries, Arcane Pages, Shards, Awakening Sigils, Primal Essence, class/level/trial locks, transactional unlock with no loss of materials on failed attempt, XP progression. `src/ui/constellation_menu.rs`: view model for 20 nodes per class, mana progress, known/available/locked states. **These are not yet linked to Horizon world saves, server or GUI rendering.**
- Original audio generator `preview-5spells-wasm/tools/generate_sound_assets.py`: synthesizes 5 class families × 3 original charge/cast/impact stages = 15 OGG files and `sounds.json` during CI; hooks queue `SoundPlay::at` locally in three stages. No paid services, commercial recordings or vanilla audio assets. Music/audio and quality still need game listening checks at M100.
- Source changed to internal preview v0.4.0. CI pipeline now generates OGG assets, validates all OggS signatures, package integrity and `mod.wasm` SHA-512; **results pending** at time of this writing. No new release should be sent to the user.
- Added `docs/M100_IMPLEMENTATION_BOARD.md` for exactly 100 priority IDs (7 Standard + 2 Awakening + 1 Ultimate per class), `docs/ACQUISITION_AND_GUI.md`, `docs/SFX_PIPELINE.md`, and exact latest instruction in `docs/memory/USER_LOG.md`. Existing 200-spell registry remains a design inventory, not 200 abilities.
- Tests: Core Rust CI run [44501008373](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44501008373) **passed** with the new progression/UI modules. Windows user has **not tested** any v0.4.0 appearance or audio. Avoid claiming it has.


## M7 follow-up — proper 3D primitives and CI quality gates
- Portable shared mesh generator \`src/vfx/geometry.rs\`: new \`faceted_crystal\` returns a closed cap/body/tip 3D indexed triangular surface, 5–8 sides, lean and twist; \`crystal_rising\` handles delayed per-crystal growth. Unit tests prove every undirected surface edge has exactly two adjacent triangles, valid indices and non-zero spatial extent.
- [GitHub Actions 44502832542](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44502832542): **47 Rust tests passed, 0 failures**. These check the portable core, **not** Windows game integration.
- Horizon WASM preview \`Scene::membrane\` now extrudes actual front/back/edge surfaces, shared across phoenix wings, verdant canopies, storm manta and folded astral effects. Added 5–8-sided tube surfaces for roots, branching tendrils and thunder wyrm bodies instead of flat crossed quads. Maintained 900-quads-per-frame budget.
- Audio first CI attempt [44501787980](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44501787980) **failed** during OGG conversion because ubuntu runner lacked \`ffmpeg\`. Fixed by apt installing FFmpeg before deterministic sound synthesis. Latest internal pipeline run is [GitHub Actions workflow](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/workflows/aetheria-preview-wasm.yml); **do not claim it passed until the run finishes**.
- Localized 15 original stage sound subtitles in \`en_us.json\` and \`cs_cz.json\` for Horizon accessibility, alongside the original generated \`sounds.json\`.
- Added pinned tool caching and action-level concurrency to prevent repeated CI builds of expensive toolchain when several GitHub commits occur in a row.
- **No new public test build is released**. Halfway gate remains **0 / 100 fully integrated spells**; all this work is internal development infrastructure and 3D/progression/audio prototypes, not 100 finished spells.

## M8 / 2026-11-19 — Ten distinct volumetric school sculptures & 20 cinematic camera plans

- User directive (exact quote preserved in USER_LOG): "pokracuj pohraj si v tom ale drz se ty krase jak v tych videich a originalniho promtu". The four original MP4s were reopened and sampled directly: scale, clear silhouettes, evolving world-space mass, deep shadows/reflective surfaces and staged impact make them compelling; not simple beams or bloom. Space cannons / orbitals are strictly excluded from fantasy Aetheria.
- Added `src/vfx/arcane_forms.rs`: **ten individually authored closed 3D mesh scenes** for all ten magic schools. Examples include two volumetric feather wings with living tails, wave-delayed ground-origin ice wedges, triple braided wind columns, an articulated stone hand, a swimming water leviathan, branching root/leaf canopy, a 3D storm wyvern, staggered spatial fracture solids, a time hourglass, and curved skeletal ribcage. These are original geometric forms, not copied assets or palette swaps.
- New indexed-mesh primitives: manifold (closed) ellipsoids, continuously swept tube limbs with end caps, solid feather/leaf membranes with front/back and edge walls, and existing faceted crystals. Checks validate polygon incidence, bounded triangle counts, finite 3D extents and time growth. Renderer draw cost is still budget-limited.
- First CI attempt found a missing `Hash` derive; subsequent geometry tests identified water dragon and hourglass early-stage spatial growth defects. All three issues were fixed, not bypassed. **[GitHub Actions 44509592898](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44509592898) passed 53 portable Rust tests (0 failures)** including new volumetric mesh tests and future ultimate choreography.
- Added `src/vfx/cinematic.rs`: 20 distinct, tweened cubic-Bezier **camera intents** for the ten classes' two ultimate slots each; safe local VFX slowdown, bounded shake and FOV, opt-out for reduced motion. This code does not detach an in-game camera, freeze world ticks or constitute completed ultimates.
- `preview-5spells-wasm/src/lib.rs` now imports the shared core as a Cargo path dependency. Its **second visual variant** per each of the five preview keys is generated from actual closed 3D mesh triangles, converted conservatively to the verified Horizon quad hook with per-face shading and a per-frame hard budget. This is an internal integration test of renderer capability; it is not yet user-tested, GPU-instanced rendering, or full RPG gameplay.
- Internal WASM CI build for this integration: [Actions 44510001331](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44510001331), **pending at this log entry**. Do not claim complete WASM packaging until it passes.
- Ongoing hard gate: no next manual playtest or user-facing ZIP until **100 completely implemented gameplay spells**, with mana/cooldowns, acquisition/inscription, spellbook/UI, server-authoritative effects, 3D geometry, spatial SFX, animations and actual game-build checks. Ten new sculptures and 20 camera intents do **not** raise the current 0/100 complete status.

## M9 / 2026-11-19 — Make 3D sculpts draw through Horizon and unify learning→equipping→casting

- Added \`aetheria-horizon-core\` as a Rust dependency of \`preview-5spells-wasm\` and connected the **second visual style** of each of its five keys to \`vfx::arcane_forms::sculpt_school\`. Its 3D mesh triangles are transformed into world-space Horizon \`WorldQuad\`/ \`PipelineQuad\` submissions with face-dependent shading and triangle decimation to respect the 900-quads-per-frame limit. Distinct materials are mapped to readable per-school colors. This is *not* actual GPU instancing, and its new render path has not been visually tested on Windows.
- Added \`src/core/authority.rs\`, an authenticated-server-intent \`AuthoritativePlayer\` model combining class progress, 4-slot spellbook, spell learning, equipping, selecting, and first 20 portable cast actions (mana/cooldown calculations). Includes monotonically sequenced request validation to avoid processing duplicate cast/unlock commands, and rejects out-of-order/oversized initial sequences. This **does not yet receive network events or write to Horizon world state**.
- [GitHub Actions 44510503132](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44510503132) — **58 portable Rust tests passed / 0 failures** with the new authority model. Follow-up initial-sequence hardening test is re-running in CI.
- [GitHub Actions 44510001331](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44510001331) is the precompiled wasm/asset integration pipeline. Status pending at log time; do not represent a portable unit test as a successful full Horizon WASM render build.
- The 10 original 3D school models and 20 camera paths are *engineering foundations*, not 30 finished abilities. The **M100 user testing gate remains 0/100 completely implemented spells** until gameplay, server effects, save/network, true GUI, audio/animation and CI/Windows runtime are verified. No ZIP was released or user test requested.

## M10 / 2026-11-19 — full-state v2 codec, fluid UI model, book upgrades
- Added `src/core/snapshot.rs`: versioned full-state snapshot, all 10 classes, relative cooldowns, corruption detection, v1 mana/unlock migration, strict duplicate/class/NaN checks, bounded payload. FNV-1a checksum is **not** a cryptographic signature and cannot secure untrusted client payloads.
- Added `src/ui/fluid_magic.rs`: shader-ready data for 10 palette variants, wave-sloshed mana surface with reduced-motion option, 4–8 radial slots, real cooldown fractions and castability.
- Spellbook Tier I–V upgrades now extend a normal book from **4 to 8 usable slots**, not just changing a numeric tier. Resource spending remains transactional.
- GitHub Actions run [44520727940](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44520727940) passed **64/64 Rust tests** after full-state snapshot introduction. [44521086420](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44521086420) passed **68/68 tests** after the fluid HUD data model.
- An interim CI [44521283590](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44521283590) **failed** one snapshot test after changing the six-slot book test fixture; the faulty hard-coded 4-slot replacement never altered the snapshot. Corrected by equipping the starter spell, changing the six-slot fixture, and validating duplicate slots **with a recomputed matching checksum**, at commit `fa887ef4b591bc8362dfe545b0e48ca24efdca3e`. Final result: [CI workflow](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/workflows/aetheria-core.yml); keep its actual status accurate until finished.
- Saved implementation notes in `docs/M10_STATE_AND_UI.md`.
- **No game runtime claim**: neither Horizon world save/load nor actual on-screen GUI is wired. 0/100 end-to-end completed, user will only test again at 100; no ZIP delivered.
