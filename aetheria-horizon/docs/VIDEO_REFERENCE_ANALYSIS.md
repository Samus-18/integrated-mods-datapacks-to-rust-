# Four user videos — direct frame sampling, 2026-11-18
All descriptions are visual observations. Aetheria reinterpretations are **fantasy concepts**, not tested game features.

| Video | Duration | Visible reference | Aetheria direction |
| --- | ---: | --- | --- |
| Download (3).mp4 | 27.4s | Green vertical sky column and hovering structure (~2–11s); luminous ring and concentrated violet/gold light (~19–27s) | Sky-sized runic ritual with layered magic beam; no orbital device |
| Download (4).mp4 | 58.0s | Sky split into radial cracks around a dark aperture (~4–12s), large dust plumes (~20–38s), crater (~42–56s) | Dimensional rift, geomantic rupture, temporary magical debris |
| Download (5).mp4 | 150.7s | Settings wheel, glowing strikes, terrain destruction, circular ritual (~90–105s), lightning/lava burst (~130–140s) | Fluid spell selector, cinematic spell phases, controlled magic impacts |
| Download (6).mp4 | 40.1s | Violet/teal crystals rise (~3–18s), radiant pillars and bright crystal field (~30–40s) | Glacial crystal cathedral and summoning sanctuary |

Methods to explore: readable 3D silhouettes, glow layers over correct HDR bloom pipeline, depth-aware refraction if API permits, telegraph→cast→impact→afterglow timing, local camera recoil and spatial audio. Staged VFX timing **data** now exists for twenty action intents in `src/vfx/timeline.rs`; there is not yet a real renderer or GPU test. Limit particle counts and protect terrain/player saves. No screenshots, models or audio taken from uploads were committed.
