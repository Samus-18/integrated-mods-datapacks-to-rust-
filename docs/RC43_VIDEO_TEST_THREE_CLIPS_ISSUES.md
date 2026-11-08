# RC43 – evidence-based review of all 3 user gameplay videos

**8 Nov 2026 | 3 videos inspected over their full 200.1 seconds, with scene interval and zoomed frame-by-frame inspection. This is a bug report, NOT a claim of fixes.** The clips themselves cannot authenticate which exact datapack zip was loaded; likely RC43, compared against the available local RC43 archive. Do not publicly upload the user's actual gameplay files without permission.

## Clip 1 – Overworld, `horizon-client_SBbUyuE8LT.mp4` (92.4 s)

- **0–19 s:** building interiors and chest with **several carrot stacks and other low-value supplies**. This is not an empty chest: physical NBT Items exist, but repeated, meager reward distribution is visible.
- **8–38 s:** dense trees/bamboo against housing and paths; some organic features are too close to buildings. The one-grid exclusion for grove templates does not guarantee vanilla foliage never grows into structures.
- **39–54 s:** repetitive dense woodland, then broader savanna. **~47–49 s: sharp bright/white strips** at a chunk/terrain boundary. Could be missing LOD/mesh or distant-world geometry; not diagnosed by screenshots alone.
- **56–64 s:** defended ruined enclosure with visible illager and **barrel containing only one iron ingot** at ~60 s. Barrel is physically filled, but seriously under-rewarded.
- **71–92 s:** critical *vast* unnatural cavity, near-vertical planar terrain walls, unsupported geological shapes and sky/white below. **Player altitude visible around Y~110–180 in the cavern.** Direct RC43 density function `data/thunderfusion/worldgen/density_function/overworld_overworld_combined.json` starts its intended dampening at **Y320, rises to Y760**. This earlier “fix” is therefore largely **outside the observed problematic altitude**. Deep density/carver interaction must be examined before increasing cave attempts again.

**Source-level loot cause:** `build_thalvoryn_rc42_world_feedback.py` bakes only **1–2 stacks per barrel** and **2–4 per chest**, item choices with replacement (`random.choices`), allowing three identical carrot stacks; only 7% baseline / 16% “rare”-named chest has a bonus. Difficulty/actual structure rarity is not properly rewarded. RC43 inherits these fixed Items from RC42.

## Clip 2 – The End, `horizon-client_NzmvzLFg7d.mp4` (35.03 s)

- **0–14 s & 24–35 s:** overwhelmingly uniform vanilla-coloured End Stone islands and too many near-identical small floating fragments across the high sky. Few distinct landmarks visible in this area; do not extrapolate count to whole End.
- **15–23 s:** large imported ship with substantial hull hanging over void over a narrow island. No visible whole-footprint support.
- **RC43 technical cause verified:** imported End ship structure JSON uses an **absolute start height Y145–190** (not actual ground) and no full-hull terrain fit. The active airship structure set has spacing **100** and frequency **0.20**. **Stop random spawn or add full-hull placement support** instead of just altering rarity.
- The same `thalvoryn:end/high_sky_islands` placed feature uses **one identical vanilla `minecraft:end_island` shape**, Y104–276, rarity 1/22 across multiple existing End biomes, accounting for a visually repetitive high-layer effect. End's dimension generator still uses `minecraft:end` vanilla noise and biome source: no new genuine F3 custom biome IDs exist.

## Clip 3 – Nether, `horizon-client_vTVgXbtTcR.mp4` (72.7 s)

- **0–12 s:** crimson terrain and **a very large fortress-like Nether brick structure** (~9 s). So this footage DOES show structures; saying “none spawn” would be false.
- **13–21 s:** huge lava field, small purple deck/platform with conspicuous glowing cross (~19 s). This is not proof that experimental RC43 lava skiffs work. The two actual `lava_skiff_*.nbt` files contain **no glowstone block**; other Nether templates may be responsible.
- **22–40 s:** a long dense crimson-forest stretch without other visible medium-sized points of interest.
- **41–56 s:** enormous, rather uniform warped-forest surface and stark crimson/warped boundary; visually limited regional distinction and possible distant-LOD influence.
- **57–73 s:** huge basalt/magma/blackstone/lava expanses with one visible high crimson structure (~69 s) but large gaps between points of interest.
- RC43 Nether has multiple camp/watch structure sets (common spacing ~27–28, frequency ~0.55–0.57), **but 2D grid frequency does not provide vertical coverage, valid Y support, or evenly distributed discoveries in a 1024-tall dimension**. Nether biome source uses the five vanilla biome IDs with native embellishments, not new F3 biome IDs.

## Priorities; no unverified “stable” claims

1. **P0 cave density at Y~110–320**: preserve traversable caves, remove vast perfectly flat curtain walls and open-to-void fractures; compare with LOD disabled and same-seed terrain.
2. **P0 End imported ships**: disable unsafe absolute-Y auto-spawn until full 2D island/water footing validation or meaningful alternative, preserving NBT templates.
3. **P0 improve baked loot** for danger, structure type, rarity and dimension; eliminate repeated carrot copies and reward defended buildings more generously without extreme OP loot.
4. **P1 End island variation**: fewer repeated vanilla floating fragments, more differentiated forms and biome-themed locations using supported registry. Never label new F3 IDs as finished.
5. **P1 trees and surface-site fit**: foliage/bamboo placement conflicts; restrict botanical features where structures cover more than just the spawn-point chunk.
6. **P1 Nether ecology / true 3D site coverage**: richer safe locations across elevations, avoid empty cavern bands and flying or submerged structures.
7. **Engine dependent:** Horizon distant LOD white bands, safe Nether portal search, authentic new F3 biome IDs, engine collision validators, FPS/TPS runtime measurement. These cannot be honestly solved by static datapack changes alone.

**No changes were made to the user's world or datapack in this video review.** No RC44 build should be labelled final until the observed issues can be measured and resolved in runtime. [Existing open issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) remains the tracker. Local, private evidence collection contains time-stamped frames from each video and the detailed Czech report.