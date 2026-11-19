# M24 — 20 more authored Standard spell silhouettes (2026-11-19)

**Target:** continue toward the user's hard rule of **200 different, beautiful, fully functioning fantasy spells; no generic stick, hollow ring, color-swap or unfinished release**. This milestone is another **art-directed Rust geometry draft**, not a finished game mod.

## Source-controlled additions

- `preview-5spells-wasm/src/standard_channelled.rs`: 10 independently scripted sixth spells (ID 006 Firebrand Sigil, 026 Hoarfrost Trail, 046 Cyclone Stride, 066 Granite Orbit, 086 Surge Fan, 106 Seedburst, 126 Flashstep, 146 Comet Pearl, 166 Echo Step, 186 Wraith Step).
- `preview-5spells-wasm/src/standard_instant.rs`: 10 independently scripted seventh spells (ID 007 Pyre Serpent, 027 Permafrost Grip, 047 Banshee Whistle, 067 Ore Sense, 087 Rainweaver, 107 Vine Swing, 127 Magnet Stone, 147 Astral Compass, 167 Aging Touch, 187 Carrion Swarm).
- `preview-5spells-wasm/src/lib.rs`: routes those 20 actual 200-slot IDs to the dedicated code before the previous `mass_forms` semantic fallback; no unsafe gameplay world actions are introduced.
- Each has source-name/mesh/mechanic-derived actual 3D spatial bodies, hand-built four-phase visual animation (telegraph/cast/impact/afterglow), color-independent mesh fingerprints and under-500-face budgets.

## Concrete original silhouette choices

| ID | Scene identity | ID | Scene identity |
|---:|---|---:|---|
| 006 | forged, floor-rooted flame seal, molten engravings | 007 | segmented coiled fire-serpent body and jaws |
| 026 | branching crystalline permafrost path | 027 | articulated five-finger ice grasp |
| 046 | diagonal path of connected lateral wind steps | 047 | three-dimensional pressure trumpet, broad membranes |
| 066 | eleven thick levitating defensive stone chunks | 067 | exposed ore-vein pillars among broken rock |
| 086 | layered sideward breaking water fan | 087 | rain cloud clusters and 24 falling droplets |
| 106 | sequenced opening physical seed pods and leaves | 107 | thick arched grapple vine and large leaves |
| 126 | four running silhouette afterimages and swept sheets | 127 | heavy levitating lodestone and ore fragments |
| 146 | large faceted comet, traveling stone fragments | 147 | 8-armed physical star compass |
| 166 | ghost footprints and floor time-folds | 167 | weathered palm with 12 hour markers and split crystal |
| 186 | four draped, hooded passing ghosts with distinct depth | 187 | fifteen independently fluttering spectral moths |

## The actual review and correction

Opened **the exact 20 Cast geometry SVGs** exported by pinned SDK CI, each from two independent oblique cameras, not an AI mockup or fake Minecraft screenshot. Several looked poor even though all tests had passed:

- #027 Permafrost Grip and #167 Aging Touch originally resembled **featureless slabs with spikes**; reworked fingers, wrist, palm and material relief.
- #067 Ore Sense originally looked like a **dark flat saucer with tiny features**; removed dominating plate and enlarged exposed rock/crystal mineral formations.
- #126 Flashstep and #186 Wraith Step resembled **lines of identical boxes**; reworked them into articulated figures and distinct cape-shaped spectral bodies.
- #186 originally used four costly 126-quad spherical heads; those exhausted the scene face budget, so replaced them with compact faceted skulls.
- Reopened the **revised contact sheet** from final CI: hand shapes and ghost silhouettes are more legible from both views; ore crystals actually stand out. However the surfaces and animation are still *low-poly diagnostic geometry*. Some forms are not yet cinematic; do NOT claim they match the video reference.

## Technical validation — pinned SDK and strict audit

Final VFX code commit SHA `74840513a8f2b522efcb1c65c7e7fefae4e97469`:
[GitHub Actions run 44614524466](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44614524466) — **SUCCESS**, **21/21 preview unit tests**, no failures. Build includes pinned WASM `.hmod`, SHA512, real school/stage 30 OGG audio cues, custom animated WGSL material and original 200-entry registry. The internal artifact `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal` emitted **860 CPU geometry projections**. An extra M24 gate checks another **80** two-view, two-stage geometry render observations across these 20 new per-ID scenes. Geometry coordinates are finite, color-insensitive meshes differ, no scenes hard-clip at 500, and viewed polygons have non-trivial area from both angles.

**This is not real GPU screenshot or in-game test.** Native depth-aware HDR, bloom, procedural physics/terrain, gameplay damage/healing, server world adapter, multiplayer authority, persistent skill progression and RX 580 GPU performance are *not* demonstrated.

## Exact progress accounting

- **200/200 authored registry + mana/cooldown + cast-intent/timelines.**
- **200/200 client-local basic visual drafts**, varying in maturity.
- **130/200 individual Rust-authored spell geometry layouts**: 30 curated first spells (3 per school), 20 earlier M23 Standards, **20 M24 Standards**, 40 Awakening, 20 Ultimate. These are NOT 130 finished cinematics.
- **70/200 remaining Standards** (local index 8–14 in each school) still use a general-purpose semantic 3D layout and require dedicated scene design.
- **170/200 extended actions remain DesignIntent**; authoritative adapter deliberately rejects these rather than pretend world effects exist. Even the first 30 typed actions are not yet applied to the real Horizon game.

## Next quality targets

The remaining 70 source designs should be rewritten as recognizably different spell silhouettes (begin with local Standard #8–9: Lava Bloom, Sleet Mirror, Earthbind, Barkskin, Weightless Field, etc.). A separate code review of their current M23 diagnostic SVGs found obvious cuboids, thin crystal racks and flat plates; none are artistically approved. At the same time, **actual native Horizon depth-aware GPU magic lighting/material integration** must be verified rather than ever saying that an extra 100 polygons will make it beautiful.

No unpolished player ZIP has been sent, and no additional manual in-game testing has been requested.
