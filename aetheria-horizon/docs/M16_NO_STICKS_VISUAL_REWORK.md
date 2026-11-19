# M16 — Stop shipping stick-and-ring magic

**User acceptance correction, 2026-11-19.** The user examined the exact offscreen screenshot of **Blink Fold** generated from the v0.5.3 Rust geometry. Feedback: *"ani nevim co to je nerikej mi ze delas to co jsem rikal ze hlavne nemas delat a to tycky a kruhy obycejny jako drive ... vse jsi prost edelala jen z tycek"*. The screenshot was genuinely skeletal: two outline rings and multiple thin beams/spikes. This is a **confirmed artistic failure** despite 180 snapshot exports, 6 passing VFX tests and earlier projected-area increases.

## Acceptance rule

**Reject any spell whose dominant visible silhouette consists of disconnected thin tubes, rods, needles, empty hoops or generic rings.** Such primitives may only be minor decorative accents when the actual spell concept justifies them (e.g. natural lightning branches). A test asserting that two meshes differ is insufficient: unique bad wireframes are still bad.

Every new spell should be designed **from a distinctive coherent primary 3D mass first**: animated textured volume, substantial solid ice, flowing draped sheet or crest, connected flowering organic body, rocky mass, or rich spatial aperture. Build temporal motion and material layering *around the mass*. Do not solve the problem by merely increasing the count of thin tubes or scaling them up. Preserve each spell's unique identity; not all spells should become the same curtain/sphere.

## Immediate production correction — Blink Fold

- Replaced the main `(7, 2)` effect in `preview-5spells-wasm/src/cinematic_beams.rs` with **two filled, thick, spatially distorted dimensional veils** and a **connected wide folded-space river**. Each veil is tessellated 8 × 10 in front and back plus solid edge walls, with curved rippling depth; the interconnecting river is a multi-row, front/back mesh, rather than beams.
- Removed its fallback shared-school/portable sculpt from `preview-5spells-wasm/src/lib.rs`, which had been reintroducing the old circles and sticks over this specific spell.
- Added a specific geometry regression test requiring plentiful broad faces and nontrivial projected area from **two diagonal views**; the old general uniqueness test alone would never have found this issue.

## Honest status

- This is a *targeted correction*, not a claim that all 30 effects are beautiful yet.
- The other 29 have not automatically passed a professional art-direction review merely because their geometry is valid. The image that prompted this correction proves the prior quality gate was too weak.
- The diagnostics remain **CPU world-quad projections**, not Minecraft Horizon screenshots. They cannot prove real HDR bloom, refraction, occlusion, animated camera or true in-game visual fidelity. Do not ask the user to playtest until sufficiently polished.
- Future audit must distinguish **coherent filled mass**, recognizable silhouette and material/cinematic read from simple quad count and polygon-area statistics.


## Full four-phase correction and evidence

The next source correction (commit `377fd3e039870edeb4d9b1a7e23356518ef8e5cb`) eliminated a further defect discovered in the actual M16 side-by-side image: **the Impact stage still displayed an old circular ring**, while Telegraph and Afterglow used other circular primitive fallbacks.

Now the Blink Fold spell branches out **before** the generic stage-based effects; all four stages use the original thick distorted front/back tessellated sheets. The temporal growth/fade directly scales 3D X/Y/Z, not merely ring radii. The material has a restrained per-face turquoise-violet phase gradient. The underlying generic astral scaffold is disabled for this spell.

- Verified pinned WASM + 30 OGG pack CI success: [run 44579075320](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44579075320).
- Audited the latest real geometry snapshots of Blink Fold, both diagonal views, Cast/Impact/Afterglow: no circles and no wireframe at any of those stages; the primary silhouette is two thick, somewhat draped volumetric surfaces plus an intersecting dimensional flow.
- **Artistic review still NOT passed for release.** The updated projection looks like a pair of purple-blue folded sheets, which has much more 3D mass than the old outline but is still far from the richness and atmospheric cinematic lighting of the reference videos. It lacks native HDR/bloom/refraction and compelling material texture. Do not describe it as equivalent to God Wand, a finished VFX, or a proven in-game Horizon appearance.
- Other 29 spell visuals still require individual material and silhouette art-direction rework. Automated geometry uniqueness or projected-area increases cannot establish beauty.

## Additional five-spell surface-first replacement

Follow-up commits `a3c687f178e6139bb120c57f1709d6114bfc6e04`, `983af975db5791027ccf530e52460fe79af85214`, `f09faa76bf681edaebe42fe441d5f3f2bf1d0bf5` implement full multi-sided volumetric bodies in **four additional effects**: Gravity Thread, Second Hand, Moment Slip and Tick Pulse. Along with Blink Fold, all five now bypass the previous ring/tube-based shared-school sculpture. All four phases use their own coherent body-growth/fade and avoid the generic ring telegraph, impact halo and afterglow.

Unique silhouettes: **gravity** gets a faceted warped enclosed gravitational lens; **Second Hand** gets an hourglass/time vessel; **Moment Slip** gets staggered solid translucent shutter panes; **Tick Pulse** gets multiple expanding tessellated spherical shells. None rely on two thin outlined circles and connecting beam sticks as their central composition. Their complete source and two-angle diagnostic visual snapshots must be inspected before any artistic approval. Still not equivalent to native GPU bloom or game screenshots; the full reference-video aesthetic remains unfinished for all 30.

## Final M16 build and verified source-frame review

One more actual second-view appearance audit showed that `Moment Slip`'s first set of time shutter membranes still collapsed to line-like edges. Final corrective commit `994e9ee538d46e451545d9e8c4b162f157392c27` replaced them with **six rotated, closed, thick 3D prisms**; they now remain visibly solid from both diagonal viewpoints at Cast and Impact. This is the same principle as the user's explicit request for solid, recognizable geometry in every direction.

- Final internal source compiled: [GitHub Actions run 44581075496](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44581075496) — **success**.
- Preview Rust tests: **7 passed, 0 failed**, including the two-camera filled Blink Fold test and full 180-view projected geometry audit.
- `Aetheria-30Spells-Visual-Audit-v0.5.3` in that run contains exact projections of the current coded geometry, not an artist's recreation or a Horizon screenshot.
- Source changes were inspected from the rendered contact sheets. **Actual artistic result remains unfinished:** these five now have solid volumes instead of wireframes, but the surfaces still look like basic low-poly folded sheets, hourglass, time slabs and sphere because advanced shader/atmosphere interactions are not working in this WASM preview. No claim of achieving the quality in the user's reference videos.
- Remaining **25** of the current 30 demo spell identities are not yet certified against this stricter art-direction gate. Do not ship a "polished 30 spell" build just because compile/tests pass.
