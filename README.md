# Thalvoryn: Expanded Worlds

**A world-generation overhaul for Minecraft Java Edition 26.3 and Horizon Rust 26.3.**

Thalvoryn: Expanded Worlds expands Minecraft's three original dimensions with new structures, improved terrain, biome-themed ruins, enhanced caves, floating islands, mobs, treasure and more.

The project combines and expands **TREK** structures and **JJThunder** world generation, aiming to make exploration more rewarding without completely replacing the vanilla Minecraft experience.

## Features

### 🌿 Overworld
- Expanded JJThunder terrain with tall mountains and varied landscapes.
- TREK castles, villages, towers, ruins and other structures.
- New biome-themed watchtowers and outposts.
- Rare floating Sky Garden islands containing treasure.
- Improved caves, cave entrances, geological formations and vegetation.
- Structures designed to generate at different elevations, including mountainous terrain.
- Passive and hostile mobs in selected structures.
- Loot chests and hidden rewards.

### 🔥 Nether
- Expanded Nether terrain and more varied natural formations.
- Crimson, warped, basalt and soul-themed structure variants.
- Elevated bridges, ruins, watchtowers and viaducts.
- Additional glowstone, vegetation and basalt formations.
- Structures designed for multiple vertical levels.
- Biome-dependent loot and enemies.

### 💜 End
- Enhanced End landscapes and floating islands.
- Rare citadels, shrines, observatories and ancient ruins.
- Crystal gardens, purpur formations and obsidian spires.
- More environmental variety in the outer End.
- Reduced spawn frequency for major structures to keep exploration rewarding.
- Additional treasure locations and biome-specific structures.

### 🏰 Structures and exploration

Thalvoryn retains the extensive TREK structure collection while introducing additional custom structures and biome-specific variants.

The RC13 package contains, according to its static audit:

- **931 NBT structure templates**
- **128 loot tables**
- **13 additional biome-themed structure variants**
- Structure generation across suitable Overworld, Nether and outer End biomes.

*An NBT template may represent one piece of a larger structure rather than an entire independent building.*

### 🌍 Two Nether height options

| Variant | Nether height | Recommended use |
|---|---|---|
| Nether 256 | 256 blocks | Recommended; less demanding |
| Nether 2032 | 2,032 blocks | Experimental large-scale Nether |

Both options are available separately for Java Edition and Horizon Rust.

## Downloads

Download the appropriate archive from the GitHub Releases page:

- **Java Edition bundle** — contains both Java datapack variants.
- **Horizon Rust bundle** — contains both Rust datapack variants.

Each bundle contains two individual datapack ZIP files and the same custom pixel-art pack icon.

## Installation

1. Download the bundle for your Minecraft edition.
2. Extract the outer ZIP archive.
3. Choose **one** Nether height variant.
4. Place that inner ZIP in your world's `datapacks` folder.
5. Disable or remove older Thalvoryn datapacks.
6. Create a new world for the best world-generation experience.

**Do not install both Nether variants together. Do not mix Java and Rust datapacks.**

Existing generated chunks will not automatically regenerate.

## Compatibility

- **Minecraft Java Edition:** 26.3.
- **Horizon Rust:** Nightly **2026-11-07**, build/commit **`2ebf00a`** (Minecraft 26.3) — **tested in-game on this build**.
- **Other Horizon Rust Nightly builds:** You can try them, but compatibility has **not been tested or confirmed**. Changes in the game engine may affect datapack loading and world generation.

Testing on the listed Horizon build does not guarantee that every RC13 structure or feature has been verified. Java Edition and Horizon Rust use separate datapack builds because their world-generation implementations may differ.

## Development status

**Current release: RC13 — Release Candidate / Testing**

> **Known issue with Horizon Rust RC13:** Some generated structure chests and barrels may be empty. A Rust RC14 hotfix with prefilled NBT container inventories has been prepared, but **the GitHub Release still contains the old Rust RC13 asset until it is manually replaced**. Java RC13 is unchanged. Existing generated chunks are not fixed retroactively. The RC14 workaround uses pre-generated (non-rerolled) loot and requires in-game testing.

The datapacks have undergone static checks for JSON, NBT structures, loot references and world-generation ordering.

The Java RC11 baseline was confirmed to load in-game. RC13 includes additional world-generation content and requires further in-game verification.

Some content may differ between the Java and Rust implementations. The large Nether variant is experimental.

## Credits

- **TREK** — original structure collection and associated assets.
- **JJThunder / ThunderFusion** — original terrain-generation foundation.
- **Thalvoryn additions** — additional structures, integration, biome variants, loot, terrain adjustments and compatibility work.

Original authors and third-party assets must receive their required attribution. Redistribution rights and applicable licenses should be verified before a public release.

---

**Explore a more varied Minecraft world. Discover ancient ruins, floating islands, dangerous structures and hidden treasures across all three dimensions.**
