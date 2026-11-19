# M15 — Reproducible geometry appearance audit (2026-11-19)

## Why this milestone

User requests that Aetheria's magic be beautiful, three-dimensional, *different for every spell*, and **visually checked every time** without asking for another intermediate Windows/Horizon playtest. This adds a source- and CI-based *appearance review* process instead of treating Cargo compilation alone as visual proof. The reference remains the four exact original MP4s preserved in the user's Library, with SHA-256 references in `references/original-videos/README.md`. The full literal original master prompt has not been recovered; use `docs/memory/ORIGINAL_PROMPT_VISUAL_REQUIREMENTS.md` and `docs/PROJECT_SPECIFICATION.md` with the documented uncertainty.

## What actually runs

`preview-5spells-wasm/src/visual_audit.rs` calls the **same `render_spell` CPU geometry function** as the client `.hmod` code (not a separate fake hand drawing) and captures 30 spells × Cast / Impact / Afterglow × 2 diagonal viewpoints = **180 reproducible SVG projections**.

The audit exports:
- `index.html`: 180 paired, image-based views with selected name, slot, phase, age in ticks, 3D world-span and quad count.
- `spell_XX_YY_viewZ.svg`: translucent faces sorted by approximate depth, demonstrating silhouettes from the front and oblique/back side.
- `geometry_report.tsv`: quads, absolute span in blocks, projected-coverage proxy, fraction of edge-on faces, and vertical extent.

The tests assert that all 180 views exist, geometry is finite, frame budgets ≤900 quads are obeyed, view coverage is nonzero, and geometry is not almost entirely edge-on in either viewpoint. The previous M14 test independently asserts that all thirty 3D mesh coordinate sequences differ **with their RGB values ignored**; this alone does not demonstrate artistic quality.

The `.github/workflows/aetheria-preview-wasm.yml` job sets `AETHERIA_VISUAL_AUDIT=1` for the native-host Rust tests, and archives the `target/visual-audit` directory separately from the internal .hmod build. A reviewer can download the artifact, open `index.html`, review each school and compare with the four original clips. Every VFX revision should keep this workflow and review its snapshots before considering a player-facing release.

## Explicit limits

These visual projections are **not** Horizon game screenshots, and do not recreate wgpu depth ordering, fragment WGSL rendering, postprocess bloom, shadows, terrain clipping, GPU timing, particle color grading or item/player animations. Auto-fit in each SVG is only for diagnostic framing; the **absolute span** is retained in the corner text and TSV so giant/clipped effects are not disguised. No automatic numeric aesthetic test can verify the requested cinematic quality or a GPU visual regression. Actual in-game appearance still needs a real Horizon client/offscreen engine environment, which this connector does not provide.

The next user-conducted test remains deferred until the user's required milestone. Continue with source visual inspection and automatic CI validation meantime.

## Actual visual inspection and two correction passes — 2026-11-19

I extracted GitHub Actions' **180 SVGs** and rendered contact sheets for all 30 spell identities at two angles. These are actual projections of the current procedural geometry, not imagined mockups and not rendered Minecraft screenshots. Inspection showed that several schools' motifs looked like *thin wire rigs* rather than the visual references' massive physical forms. The most problematic early examples were Cinder Fan, Rime Bridge, Static Halo, Thunder Needle, Star Shard, Blink Fold, Moss Mend and Flowstep.

I performed two focused source revisions in `cinematic_beams.rs`:
- **Cinder Fan**: thick 6-sided molten flame blades with substantial mid-span membranes and faceted cores.
- **Rime Bridge**: interconnected solid-faced ice platform plates, downwards icicles, base crystal supports.
- **Flowstep**: physical cresting wave membranes and twisting secondary water ribbons.
- **Moss Mend**: an expanding green leaf flower, multi-sided rising seed core and ground ritual.
- **Static Halo**: multi-axis electric crown, orbiting charged crystal spikes.
- **Thunder Needle**: widened prismatic hard spear, charging seal and spatial impact branches.
- **Star Shard**: a deep ten-petal shaded astral aperture with raised rims rather than an empty decorative ring.
- **Blink Fold**: two thick spatial portals connected by 3D strands and dimensional membranes.

### Measured before/after (from actual geometry_report.tsv)

The metric below is **mean projected polygon area summed across faces**, normalized to a fixed diagnostic viewport and averaged over **two different camera angles** during each spell's Cast phase. It measures how much 3D geometry is exposed to the view; it is **not** physical screen-pixel coverage, artistic quality, in-game brightness or a guarantee of GPU performance. Overlapping faces are counted repeatedly.

| Spell | Before M15 | After M15 | Relative change |
|---|---:|---:|---:|
| Cinder Fan | 0.156 | 0.339 | 2.17× |
| Rime Bridge | 0.109 | 0.394 | 3.61× |
| Flowstep | 0.116 | 0.203 | 1.75× |
| Moss Mend | 0.107 | 0.324 | 3.03× |
| Static Halo | 0.093 | 0.157 | 1.69× |
| Thunder Needle | 0.100 | 0.126 | 1.26× |
| Star Shard | 0.093 | 0.190 | 2.04× |
| Blink Fold | 0.112 | 0.320 | 2.86× |

The revised cards are visibly fuller in the diagnostic projections, yet certain motifs — especially Thunder Needle and other narrow beams — still lack the **true layered luminous material, dense atmospheric lighting and environmental interaction** visible in the original reference videos. Further improvements require verified Horizon renderer functionality, real engine-side shots and material/lighting work rather than blindly emitting more quads.

## Confirmed CI and reproducibility

- Last reviewed source commit: `c863c4b684d934dcc27f7c1c1a6709314f408cbd`.
- Pinned SDK full WASM build + shader, ZIP, SHA-512, and all 30 OGG files: **success**, [run 44570867156](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44570867156).
- Native-host unit tests for WASM preview: **6 passed, 0 failed**, including render of all 180 geometry projections, independent-angle coverage, finite coordinates, unique geometry regardless of RGB color, and bounded lifetime.
- Internal Actions artifact: `Aetheria-30Spells-Visual-Audit-v0.5.3` (index.html, TSV and 180 SVGs). Artifacts are downloadable from the run; GitHub Actions retention is finite. Source generator stays version-controlled for reproduction.
- **Largest measured emitted scene:** 898 quads; cap remains 900 across effects. The limit is a CPU geometry budget, **not** evidence that 60 FPS is assured on Windows.

**NOT claimed:** in-game Windows visual quality verification, native GPU bloom/refraction/HDR, completed world spell mechanics, actual 3D spatial audio impact fidelity, multiplayer, persistent save data or 100 finished spells. User's next manual in-game test remains deferred.
