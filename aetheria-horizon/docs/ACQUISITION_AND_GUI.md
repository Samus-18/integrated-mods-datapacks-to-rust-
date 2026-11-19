# Aetheria — Spell obtaining + GUI architecture (M100)

**Design contract**, not a claim of wired in-game menus yet. All inventories must be held and validated on the authoritative server; client previews do not mint unlocks.

## Player progression and learning loop
1. Begin by choosing a **class** at the Constellation Altar; obtain a starter spellbook (4 slots) and first beginner spell. Class choice must be persisted; respec requires a controlled ritual, not a free client-side switch.
2. Exploration of overworld, cave/Nether/End magical structures and quests yields **Arcane Pages** (for a precise spell ID), **Runic Shards**, **Awakening Sigils** and **Primal Essence**. Loot rarity balances with tier and dimension; procedural ruins / enemy encounters must not flood or duplicate rewards.
3. The **Arcane Inscription Table** combines the page and shards to learn a spell. It upgrades the book through tiers I–V and unlocks additional book slots (4→8). Transactions spend resources only when all conditions hold; double-click/multiplayer requests must be idempotent.
4. Awakening spells require a class-specific trial; ultimate spells additionally require a boss/ritual challenge. The 100-spell midpoint includes 7 standard, 2 awakenings and 1 ultimate per class; 10 ultimates must have actual fantasy world events.
5. A player equips learned spells into 4–8 slots, selects with a fluid radial wheel, sees mana/cooldown on a borderless fluid HUD and may reassign spellbook slots without losing unlocks.
6. Save progress, class, mana, unlocked nodes, page inventory and trials in a versioned world-safe format; only server may approve loot, learning, equip or cast. Client receives sync and visual VFX events.

## Fluid GUI principles
- High-contrast, clean glass + liquid fantasy magic, **no blocky vanilla inventory imitation**, comfortable at 720p–4K.
- Top-right settings, adjustable shader intensity, colors and font scale; keyboard/controller accessibility.
- Mana orb with fluid slosh, radial spell cooldown progress, 3D spell preview in center of radial wheel.
- Constellation screen: draggable star chart with 10 class constellations; node lines use curved SDF energy; zoom and clear unlocked/available/locked states, prices, and tooltip for prerequisites.
- Table screen: two straightforward panels (**Spell Pages** and **Inscription**) with recipe/progression feedback, confirm button disabled for missing resources and server error explanation.
- Avoid bloated button count, unnecessary multiple menus, unlabeled icons, flashy blur obscuring controls. Respect photosensitivity with reduced flashes/screen shake presets.

## Implementation status
- Portable Rust: `src/magic/progression.rs` has transactionally checked `PlayerProgression`, `ArcaneSupply`, tier/trial/level gates and 10-class learning costs, tested by GitHub Actions.
- Portable UI data: `src/ui/constellation_menu.rs` exposes 20 node statuses per class and mana fill.
- Existing `src/magic/spellbook.rs` and `src/ui/spell_wheel.rs` offer 4–8 slot and wheel calculations.
- Real Horizon rendering, items, world loot, menu interaction, save/network bindings: **not yet implemented**.
