# Aetheria Horizon — first confirmed user gameplay playtest

- Date: 2026-11-19
- Video supplied: \`horizon-client_MnaFG4NG1R.mp4\`
- Duration: approximately **100.13 s** at 30 fps; display 1906×1036
- Build being demonstrated: precompiled Rust/WebAssembly \`aetheria_preview\` v0.2.0
- Environment: Minecraft 26.3 / Horizon; mesa/badlands biome visible. UI overlays and F3-like diagnostics.
- Qualitative outcome: user says effects look **better than expected** but requests better variations. No claim of external playtest or full-system gameplay completeness.

## Frame-by-frame observations

| Time range | Visible observation | Action |
| --- | --- | --- |
| ~14 s | Phoenix Garden appears as a huge fire-petal burst with many white-hot facets, very large in view | Lower brightness and avoid full-screen additive white |
| ~27–30 s | Glacier effect forms long cyan/white crystal points filling screen when nearby | Preserve icy shape and translucent interiors, reduce glare and allow distance to read |
| ~34–38 s | Further intense pale cyan, purple and white shard patterns, hard to distinguish at a glance | Introduce distinct wing, organic canopy, arcing lightning and folded astral forms |
| ~42 s | Some effects recede in distance; geometry remains discernible | Keep world-space placement and clipping checks |
| ~90–94 s | Astral Sunder shows lilac angular material and detached fragments; temporary settings popup overlaps at ~90 s | Make rift depth, folded surface forms and silhouettes less similar to Glacier |
| ~2–98 s | Debug FPS label shows around 75 fps in sampled scenes; moving camera over mesa landscape | Not a controlled benchmark. Keep load limits and avoid assuming constant FPS or zero stutters |

**Evidence limits:** not all five spell variants were exercised identically and no damage, mana, networking or world-state behavior is established by this footage. Performance cannot be extrapolated beyond the shown settings/scene.

## Next visual release acceptance criteria

1. 5 playable casting keys Z X C V B, 3 visually different geometry outcomes per key, not palette swaps.
2. Keep at most 8 active effects and 900 extra quad submissions/frame, **fairly distributed** so one large cast doesn't starve other casts.
3. Prevent massive overbright white buildup by avoiding additive blending on the optional custom fragment shader.
4. Do not insert fake 3D character models, orbital cannons, 2D billboard beams or sci-fi weapon aesthetics.
5. Retest on real user setup; CI compile/package success is not equal to verified in-game aesthetics.
6. Later phases: player mana, class identity, cooldown, 4–8 spellbook slots, crafting, scrolls, reactions, spatial audio, world impact, skill tree and cinematic 20 ultimates.
