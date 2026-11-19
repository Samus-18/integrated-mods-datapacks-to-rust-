# Aetheria Horizon — Precompiled five-spell WASM preview v0.2.0

**Why this build:** Horizon Launcher 0.2.81 can refuse native builds before compilation if its cache OS lock is busy. This edition compiles the same Rust rendering hooks to WebAssembly, so **no launcher Build / native compiler / trust permission is needed**.

## Install
1. Remove any previous `aetheria_preview.hmod` from **both** `horizon-mods/` and `horizon-native-mods/` in the target game instance.
2. Copy the **precompiled** `aetheria_preview.hmod` from this release to `horizon-mods/` (not `horizon-native-mods/`). Do not unzip it.
3. Turn the mod on in Horizon Launcher Mods, or ensure `mod.aetheria_preview:true` is in `horizon-mods.txt` beside `options.txt`.
4. **Do not press Build.** Launch the game normally; enter a world. Press `Z`, `X`, `C`, `V`, `B`.

Spells: Phoenix Garden (fire blossom), Glacier Cathedral (ice crystal cathedral), Living Worldroot (roots/leaves), Thunder Wyrm (storm serpent), Astral Sunder (space-fantasy rift). 3D quads from Rust, optional original WGSL fragment, HUD labels. No damage, mana, altered saves, block explosions, sci-fi or orbital weapons.

## Verification and limitations
- Source is based on the native visual preview's `src/lib.rs` and Shader; this branch uses the same Horizon client rendering API.
- Build command: `cargo horizon-mod build --wasm`, using SDK-pinned `cargo-horizon-mod` at `dd67b76...`.
- The `.hmod` must contain `horizon-mod.toml`, `mod.wasm`, `assets/aetheria_preview/shaders/pipeline/ethereal.wgsl`, and a valid SHA-512 in `[wasm]`.
- CPU-side geometry budget reduced to 900 quads per frame for the sandbox. Shader remains optional and falls back to vanilla world quads.
- This is **not native Rust execution**; it is Rust compiled to WebAssembly and drawn via Horizon render hooks.
- **Not yet physically tested in the user's Windows/Horizon gameplay**; shaders and key events may differ with game build or shader packs. Failure details appear on the game's Mods screen, not a launcher native-build log.
- Never install both variants simultaneously. They share mod id `aetheria_preview`.

Official docs: https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/start/first-wasm-mod.md
