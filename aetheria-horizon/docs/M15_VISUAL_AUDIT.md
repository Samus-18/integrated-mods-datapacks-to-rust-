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
