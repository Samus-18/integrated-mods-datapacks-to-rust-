# Four user videos — direct frame sampling, 2026-11-18
All descriptions are visual observations. Aetheria reinterpretations are **fantasy concepts**, not tested game features.

| Video | Duration | Visible reference | Aetheria direction |
| --- | ---: | --- | --- |
| Download (3).mp4 | 27.4s | Green vertical sky column and hovering structure (~2–11s); luminous ring and concentrated violet/gold light (~19–27s) | Sky-sized runic ritual with layered magic beam; no orbital device |
| Download (4).mp4 | 58.0s | Sky split into radial cracks around a dark aperture (~4–12s), large dust plumes (~20–38s), crater (~42–56s) | Dimensional rift, geomantic rupture, temporary magical debris |
| Download (5).mp4 | 150.7s | Settings wheel, glowing strikes, terrain destruction, circular ritual (~90–105s), lightning/lava burst (~130–140s) | Fluid spell selector, cinematic spell phases, controlled magic impacts |
| Download (6).mp4 | 40.1s | Violet/teal crystals rise (~3–18s), radiant pillars and bright crystal field (~30–40s) | Glacial crystal cathedral and summoning sanctuary |

Methods to explore: readable 3D silhouettes, glow layers over correct HDR bloom pipeline, depth-aware refraction if API permits, telegraph→cast→impact→afterglow timing, local camera recoil and spatial audio. Staged VFX timing **data** now exists for twenty action intents in `src/vfx/timeline.rs`; there is not yet a real renderer or GPU test. Limit particle counts and protect terrain/player saves. No screenshots, models or audio taken from uploads were committed.

## 2026-11-19 — Follow-up direct sampling (M8)

Reopened all four actual uploaded videos for further frame references, sampled each around six evenly spaced moments, and compared the combined contact sheet. This remains **reference-only**: no source frames or artwork were copied into the mod.

- **Download (3), 27.37 s:** broad aerial terrain, concentrated colorful sky/ground illumination, dramatic dawn contrast; we need environment-scale fantasy events that remain readable across the landscape.
- **Download (4), 58.03 s:** foreshortened dark aperture with radial fractured sky and huge dust/crater landscape. Adapt only as asymmetrically broken *astral rupture* and controlled geomancy; no sci-fi object.
- **Download (5), 150.72 s:** actual destructive world-scale visuals with debris fields, surface damage, lightning-like emission and delay before full impact. Technical lesson: ground-coupled temporal phases, material opacity and layered audio matter more than simply high particle count.
- **Download (6), 40.10 s:** violet/blue crystalline structures appear as substantial masses against a dark world, then flare brightly. Translate to closed multi-sided ice/astral crystals with colored facet shading and staggered growth; avoid pure white overexposure.

All ten class sculpture prototypes and 20 camera-intent paths use this **art direction** but do not themselves prove fidelity to those clips. Actual in-game comparatives wait until M100 per user's requested testing gate.


## 2026-11-19 — Re-uploaded ORIGINAL videos examined again and archived

The user re-uploaded all **four original MP4s**: `Download (3).mp4`, `Download (4).mp4`, `Download (5).mp4`, `Download (6).mp4`. Every file was opened and eight time-spaced frames sampled. Full files' dimensions, durations, byte sizes and SHA-256 are recorded in [original-video reference archive index](../references/original-videos/README.md). **The four byte-exact files and a 40.5 MB ZIP were saved to ChatGPT Library** at `/Aetheria Horizon/Reference Videos/`. The GitHub repo presently stores this analysis and checksum manifest only; original video binaries are not in GitHub because the available connector cannot transfer them. Do not write that the actual videos are version-controlled until their git blobs exist.

New observations from the actual re-uploaded files (not inferred from their names):

| Source | Sampled moments | Visible evidence | Original Aetheria rendering target |
| --- | --- | --- | --- |
| Download (3) | 1.7, 5.1, 8.6, 12, 18.8, 22.2, 25.7s | Aerial green/gold concentrated vertical pillar above landscape, then luminous ritual circle and violet/gold blast | World-scaled fantasy ritual cylinder; visible solid-sided beam and tapered interior, ground-based ring, subdued pale core |
| Download (4) | 3.6, 10.9, 18.1, 25.4, 32.6, 39.9, 47.1s | Blue sky subdivided by radial cracks around a dark oval aperture, falling dark fragments, haze and large crater aftermath | Volumetric multi-ring rift rim, original crack branching and spatial rock fragments; no permanent terrain destruction in client VFX |
| Download (5) | 9.4, 28.3, 47.1, 65.9, 84.8, 103.6, 122.5, 141.3s | Expansive landscape strikes and debris, heavy dust, bright magenta/violet circular formation and later molten light | Choreography with strong anticipation, spatial hit pulse, sustained world-scale atmospheric veil and fiery afterglow |
| Download (6) | 2.5, 7.5, 12.5, 17.5, 22.6, 27.6, 32.6, 37.6s | Luminous tall teal/purple 3D crystal columns spreading from ground, then larger crystal field | Multi-sided solid crystals with stepped timing, non-uniform heights, surfaces shaded by orientation and low-overexposure glow |

These are references, not permission to bundle third-party footage in the shipped mod or clone copyrighted visuals. Continue with original code/meshes and the existing pinned Horizon mod API; no unsupported bloom/GPU claims. Explicit user preference: no more user-performed intermediate playtesting, keep developing and compiling in GitHub.
