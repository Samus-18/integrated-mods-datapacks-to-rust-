# Aetheria Horizon

Fantasy-only Minecraft 26.3 / Horizon Rust magic RPG. Ten classes, 200 distinct **designed** spells (14 standard, four awakenings, two ultimates per class). References use orbital-laser **only for rendering techniques**, no sci-fi or orbital weapons.

**Current milestone M1: 200-spell design registry and portable Rust RPG gameplay core; NOT a playable Horizon mod.** Ten entry spells have core action intents with state and cooldowns; other 190 are designs. Additional core features: 4–8-slot spellbooks, tiers I–V, charge/channel controls and status reactions (including water + lightning). 3D and WGSL are experiments pending native engine integration; no in-game validation is claimed. See `docs/memory` and `docs/ARCHITECTURE.md`.

**To test core:** `cd aetheria-horizon && cargo test` (Rust 2024 toolchain). Do not attempt to install the source as `.hmod` yet: the verified SDK bridge is pending.

Owner license decision: **TBD**. No TikTok audio/models or other copyrighted assets included.

## Verified tests
GitHub Actions run [44395652564](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44395652564): `cargo test --all-targets` → **28 passed, 0 failed**. This does not validate native Horizon packaging, WGSL, FPS, multiplayer or in-game effects.
