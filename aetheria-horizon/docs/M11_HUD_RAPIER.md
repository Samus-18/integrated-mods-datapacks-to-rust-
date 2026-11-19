# Aetheria Horizon — M11 internal HUD and Rapier physics integration

## Visual and UX requirement

The player requested **no more concept images** and an in-game compact spell-selection system inspired by the *alternate*, quick-switch method of Iron's Spells 'n Spellbooks, without copying icons. Previous giant always-on spell wheel covers combat; use only the **selected spell and a tiny mana bar** near the vanilla hotbar. Keep hearts, armor, hunger, air, hotbar and effects drawn by Horizon. The small spell-selection strip expands only while holding the picker or briefly after switching.

### Actual internal preview controls

| Action | Key | Behaviour |
|---|---|---|
| Quick picker | Hold **R** | Show compact 5-spell strip, game stays visible |
| Choose previous/next | **Mouse wheel** while holding R | Cycle spells; cancel vanilla hotbar scrolling only for this shortcut |
| Keyboard fallback | **N / M** | Previous / next spell |
| Cast selected | **G** | Cast the selected **client preview** spell |
| Constellation | **K** | Toggle animated starfield/skill constellation overlay |
| Old direct-cast preview | **Z / X / C / V / B** | Technical demonstration, retained for internal testing |

All controls use Horizon's registered key mappings in `preview-5spells-wasm/src/lib.rs`. `gui_render_post` paints the HUD through the supported client GUI batch. 20 handcrafted, standalone 9x9 pixel-run glyphs render for the sample class constellation. Slow parallax and shimmer are deterministic and run in the menu only; no pre-rendered/generated AI imagery.

For now the five client effect previews have client-local demo mana and cooldown (not saved, not verified by a server). **No actual first-person damage, loot, world modifications, spell unlocks, HUD server synchronization or final multiplayer gameplay in the preview**. The portable Rust engine already models real casting/progression independently; authenticated integration remains a major milestone.

### Health/armor/vanilla HUD

The overlay fetches **real client player's living health, max health, and armor attribute** when the quick picker is visible. The game's vanilla HUD is deliberately retained so hunger and oxygen are not accidentally removed and the real values remain accurate. A fully custom fantasy health/armor skin requires the versioned native `health_hud` hook to draw **all the health-layer rows** (food, air, armor and hearts) or a carefully registered GUI layer; do not cancel `minecraft:player_health` without replacing all of them.

### Rapier is about physics, not visual beauty

Original repository: https://github.com/dimforge/rapier (Apache-2.0); isolated dependency `rapier3d = "=0.36.1"`. Rapier supplies collision queries, rigid bodies, forces, gravity, continuous collision detection, joints and bounce. The visible beauty shown in the references is a *different* stack: custom authored 3D mesh silhouettes, lighting, volumetric WGSL, particle choreography, temporal cues, camera, SFX, and environment responses. Do **not** replace these with small default spheres from a physics library.

Isolated Rust adapter: `rapier-sim/src/lib.rs` and `rapier-sim/src/impacts.rs`.
- **10 distinct secondary impact debris patterns**, one for each class, not the complete spells themselves.
- Static demonstration floor and dynamic ice, stone, ember and wind-carried fragments.
- Bounded to 96 bodies, explicit input guards, deterministic seeds, Rapier spatial raycast and no trusted client data.
- Tests verify falling/bouncing trajectories, collision query, body count, reproducibility and school variation.
- Real Minecraft block collision meshes, entity hit detection, per-world tick budgeting and applying server-authoritative damage/knockback remain unwired.

## Milestone gate

- **Core test suite:** [72/72 passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44530777587).
- **Actual Rapier3D suite:** [7/7 passed](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44530777587).
- **Actual Horizon WASM preview compile:** [successful integration](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44531527124) before adding mouse-wheel shortcut and improved icon batching. Check newer CI run for those additions; do not imply they passed until verified.
- **Next user manual game test:** only when 100 truly finished spells are present with VFX, animations, mana/cooldowns, server effects, GUI, loot and progression. This remains 0/100 verified completed, not an overall code-percentage metric.
