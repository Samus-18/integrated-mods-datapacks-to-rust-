# Aetheria Horizon — 5-spell native visual preview (0.1.0)

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

## Install/build for Horizon 26.3

The project is a **source native `.hmod` preview**, pinned to Horizon SDK `dd67b76ff23e28c6d8e21859df600d31572f81a0` (released 18 Nov 2026). It will not function as a regular resource pack or on arbitrary Nightlies. Native Horizon mods compile against the **matching installed build's SDK**.

1. From the Horizon launcher, install/select the matching Horizon build. Native code requires explicit trust: confirm only if you trust the source.
2. Install the modding tool if missing:
   `cargo install --locked --git https://github.com/horizon-rust/horizon cargo-horizon-mod`
3. Unpack the source `.hmod` into a folder (or use this source folder directly). Run in that folder:
   `cargo horizon-mod build`
4. Put `target/horizon-mod/aetheria_preview.hmod` into your instance's `horizon-mods/` and add `mod.aetheria_preview:true` to `horizon-mods.txt`. Open the world and press Z/X/C/V/B.

The launcher-native builder may require **Prepare & play** (e.g. `horizon-launcher native-build . --instance "Your Instance" --trust-native-code`) instead of the generic `cargo horizon-mod build` depending on launcher version. Install the selected Nightly first.

## Validation / transparency

- Source package is structurally audited for ZIP safety and required file layout.
- Standalone VFX geometry **has not yet been compiled against the native SDK or tested inside Minecraft** in this environment (no Horizon executable, SDK or Rust compiler available). This artifact is for build/testing, **not verified playable**.
- The optional WGSL pipeline falls back to normal 3D quads if it cannot resolve.
- **Not implemented:** world damage, enemy knockback, mana/cooldown sync, sound, dynamic lighting cast onto terrain, HDR bloom guarantee or multiplayer spell replication. These require a second, server-authoritative gameplay phase.

## Visual style

Based on 4 supplied videos for spatial scale and layered motion; sci-fi/orbital weapons excluded. Shapes are 3D faceted surfaces and articulated structures, rather than isolated beams or flat magic rings. Performance has been bounded for the RX 580 class of GPU, but real FPS measurements are still pending.
