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
