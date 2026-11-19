# M22 — Forty independently scripted Awakening spell scene drafts

Date: 2026-11-19. Source: canonical `aetheria-horizon/src/magic/registry.rs`. Branch: `aetheria-30-spell-test`, internal version 0.6.0. **Not a user-facing finished release.**

The user asks for all 200 spell identities and *no slop*: each must ultimately have a stunning, distinct, recognizable 3D fantasy look. The 200 registry/cast/phase foundations and 170 semantic VFX drafts from M20 were not artistically acceptable. In M21 all 20 Ultimates received individual compositions. In M22 the **40 Awakening** entries (`(id-1)%20=14..17`) get an **explicit individually scripted layout** in `preview-5spells-wasm/src/awakening_forms.rs`, rather than using the original generic semantic form for all.

## Author-specific design correspondence (40 existing spells retained)

| Class | Awakening #1 | Awakening #2 | Awakening #3 | Awakening #4 |
| --- | --- | --- | --- | --- |
| Pyromancer | #15 Phoenix Mantle: dense feathered wing-cloak | #16 Magma Crown: staggered basalt towers | #17 Solar Crucible: heavy luminous furnace | #18 Dragonfire Procession: multiple moving fire-dragon bodies |
| Cryomancer | #35 Avalanche Choir: rolling segmented snow wall | #36 Diamondheart: layered crystal armor | #37 Cryostasis Vault: ice-dome architecture | #38 Glacial Leviathan: coiled cold-body creature |
| Aeromancer | #55 Tempest Staircase: substantial helical hovering plates | #56 Zephyr Doppel: split transparent air figures | #57 Storm Eye: sculptural rotating wind curtains and still center | #58 Hurricane Harp: broad harp-frame and wind ribbons |
| Geomancer | #75 Colossus Gauntlet: giant carved stone fists | #76 Gemheart Armor: layered protective gem-body | #77 Tectonic Crown: leaning earth columns | #78 Monolith March: physical marching monolith sentinels |
| Hydromancer | #95 Ocean Guardian: bulky turtle-like water guardian | #96 Monsoon Choir: multiple fluid restorative fountains | #97 Abyssal Dome: layered nested pressure shells | #98 Leviathan Spiral: coiling fluid beast |
| Druid | #115 Ancient Ent: physical walking tree guardian | #116 Emerald Canopy: multiple overlapping leafy panels | #117 Verdant Communion: blooming rooted ground petals | #118 Thornheart Swarm: densely connected bramble groves |
| Electromancer | #135 Living Thunder: thick winged charged elemental | #136 Thunder Drum: tangible charged drum vessel | #137 Tempest Conductor: broad forged lightning conductor and wings | #138 Heavens Rattle: fractured airborne stone/thunder seals |
| Astral Mage | #155 Nebula Mantle: layered starry spectral cloth | #156 Gravity Crown: heavy suspended orbital rock geometry | #157 Celestial Echo: mirrored paired astral figures | #158 Lunar Eclipse: two overlapping deep lunar occluders |
| Chronomancer | #175 Hour Guardian: floating solid timekeeper | #176 Stasis Lens: layered physical temporal shells | #177 Entropy Ribbon: broad unraveling flowing fabric | #178 Reprise Chorus: grouped suspended time faceted dials |
| Necromancer | #195 Grim Shepherd: armored spirit guardian + ceremonial scythe | #196 Soul Exchange: twin energy vessels and interwoven spirit tide | #197 Crimson Aegis: layered protective rune shield | #198 Eidolon Procession: ordered ghost-knight group |

They use original closed/multi-sided fantasy geometry helpers shared with the Ultimate source. Most motion is stage-driven and bounded under 900 scene quads. These are *individual authored first-draft compositions*, **NOT 40 fully polished shader/lighting/camera effects**. Some shapes may still be generic or visually too rough, and the user's no-slop art gate must reject them until reviewed in the **same 860-source-geometry-view audit**, from two diagonal cameras at Cast/Impact.

All 200 **cast-intents and timeline definitions** exist in portable Rust. Only the first 30 have explicitly typed behavior prototypes, and even those are not wired to the real Horizon gameplay world. For the 170 DesignIntent actions, server-authoritative code rejects casting before spending mana. All Awakening client abilities remain visual previews with no real summoned mobs, block collision, healing, combat or persistent save sync.

Future hard requirements: non-generic silhouettes, per-spell bespoke material variations, genuine GPU scene/depth integration for richer light/atmosphere, verified native Horizon gameplay, stable 60 FPS on RX 580 and real user-world art review when requested. The current pinned SDK/WASM CI and new 40-spell test should be validated before claiming compilation success.
