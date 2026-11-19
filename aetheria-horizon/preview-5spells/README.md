# Aetheria Horizon — 5-spell native visual preview (0.1.2)

**Early visual tech demo, not a full magic gameplay release.** This build contains **five unique procedural 3D effects** drawn with Horizon's `submit_custom_geometry` hook, plus an optional custom WGSL world-fragment pipeline. All geometry is assembled in Rust; no AI-generated meshes, paid services or vanilla beam/circle assets.

## Spells (Controls → Aetheria magic)

| Key | Spell | Shape |
|---|---|---|
| Z | Phoenix Garden | Opening fire blossom, moving 3D petals and feather tails |
| X | Glacier Cathedral | Procedurally rising crystalline vault, floating splinters |
| C | Living Worldroot | Branching wooden roots, growing leaves and floating spores |
| V | Thunder Wyrm | Coiling segmented lightning serpent with jagged fins |
| B | Astral Sunder | Torn angular astral plane with peeling membranes and shards |

Each effect is drawn **7 blocks forward from the camera** at the moment of casting. Move away to see the shape. Effects last about 5 seconds and fade; press again to retrigger. GPU load is bounded to at most 1,600 extra world quads per frame, 8 active spells and 115-block draw distance. Existing world's shared renderer capacity always takes precedence; no world save changes or destructive effects.

## Install/build for Horizon 26.3 (launcher 0.2.81)

This is a **native source `.hmod`**, not a regular content mod. It is pinned to Horizon SDK `dd67b76ff23e28c6d8e21859df600d31572f81a0`. Installation is different from the `horizon-mods/` folder.

1. Keep exactly one `aetheria_preview.hmod` in the **selected game's folder** under `horizon-native-mods/`. Do **not** keep a second copy in `horizon-mods/`.
2. In Horizon Launcher > Mods, make sure `aetheria_preview` is listed as **Native** and enabled.
3. Choose **Build** (or **Build and play**). The launcher compiles the native source against the selected game's prebuilt SDK and asks for native-code trust as appropriate.
4. After successful build enter a world and try Z / X / C / V / B. Rendering and game FPS have not been verified on the user's PC.

Launcher documentation: https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/start/installing.md

### Build blocked before log: `A native build or cache cleanup is running`

This indicates that a **different process holds the native build cache OS lock**. In `launcher/src/native_mods.rs` the busy return occurs *before* `clear_log`, so a new compile log is not produced. The lock is acquired at `<launcher-data>/native/.build.lock` by `launcher/src/native_cache.rs`. A leftover lock file alone does not mean the lock is held: it is an **OS file lock**, released when its owner exits, including a crash.

- Check whether any actual Horizon native build, Cargo compilation, or storage cleanup is in progress; do not interrupt a legitimate running build.
- If none is running, close all launcher and Horizon processes completely and restart Windows to release their OS locks; reopen a **single** launcher and retry Build **once**.
- Do **not** delete `.build.lock`, native caches, game folders, or worlds as a first step. Deleting the file is not a safe unlock and can compromise concurrent build protection.
- If still blocked immediately after a clean restart, investigate which process owns the lock (Windows Resource Monitor / Process Explorer) and report the launcher version, process list and full current message.

Exact source: https://github.com/horizon-rust/horizon/blob/main/launcher/src/native_cache.rs

## Validation / transparency

- Source package is structurally audited for ZIP safety and required file layout.
- GitHub Actions on Linux successfully ran `cargo horizon-mod build` and validated the produced native package. A native Rust host check also compiled; a cross-check workflow initially missed a wasm32 target. **No Windows 10/11 in-game render/behavior test is verified**. Do not equate package validation with playable verification.
- The optional WGSL pipeline falls back to normal 3D quads if it cannot resolve.
- **Not implemented:** world damage, enemy knockback, mana/cooldown sync, sound, dynamic lighting cast onto terrain, HDR bloom guarantee or multiplayer spell replication. These require a second, server-authoritative gameplay phase.

## Visual style

Based on 4 supplied videos for spatial scale and layered motion; sci-fi/orbital weapons excluded. Shapes are 3D faceted surfaces and articulated structures, rather than isolated beams or flat magic rings. Performance has been bounded for the RX 580 class of GPU, but real FPS measurements are still pending.
