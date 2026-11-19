# Aetheria Horizon — 100-spell half-project gate (M100)

**User acceptance rule (2026-11-19):** No additional manual gameplay tests or early test ZIPs. Offer the next user-facing game test only after **100 actually working spells** with real 3D geometry, dynamic animations, meaningful gameplay, acquisition, usable GUI and spatial SFX. CI may run continuously while developing.

## Exactly what counts as one finished spell

- Distinct recognizably 3D **solid** mesh or volumetric form (not simply two intersecting transparent planes; not a colored clone), grows/moves in world space.
- Functional casting, cooldown, mana, game-state interaction, damage/heal/control consistent with class and server-authoritative rules.
- Original multi-stage Charge/Cast/Impact sound played from world position, plus fitting player/spell animations and clear hit/telegraph feedback.
- Obtainable and equipable through spellbook / Arcane Inscription Table / constellation skill tree in a fully working GUI; saved, network-safe where applicable.
- Successfully CI-built, packaged for a pinned Horizon 26.3 SDK, passes regression tests and budget checks; integrated in the running game, not only a Rust enum.
- At M100, obtain an actual Windows/Horizon in-game visual and mechanics test (user performs this **only when all 100 are ready**). Until then the status remains **not ready for user test**.

## M100 class mix

Each class contributes **7 Standard** (positions 1–7), **2 Awakening** (positions 15–16), **1 Ultimate** (position 19). This delivers 10×(7+2+1)=**100**, including ten real fantasy cinematic ultimates; remaining 100 designs are reserved for the full project.

**Important:** Five preview spells (Phoenix Garden, Glacier Cathedral, Living Worldroot, Thunder Wyrm, Astral Sunder) and their VFX variants are **visual technology demonstrators**, not 5 automatically completed registry spells. These are not counted as playable finished spells.

| ID | Class | Spell | Tier | Planned 3D form | Game logic | Solid GPU geometry | Real SFX | Obtainable/UI | Fully integrated |
|---:|---|---|---|---|---|---|---|---|---|
| 001 | Pyromancer | Ember Needle | Standard | twisting ember thorn | Portable core only | Planned | Planned | Planned | No |
| 002 | Pyromancer | Cinder Fan | Standard | seven curved coal blades | Portable core only | Planned | Planned | Planned | No |
| 003 | Pyromancer | Magma Lash | Standard | segmented molten whip | Design | Planned | Planned | Planned | No |
| 004 | Pyromancer | Furnace Step | Standard | molten footstep ribbon | Design | Planned | Planned | Planned | No |
| 005 | Pyromancer | Ashen Veil | Standard | rolling ash cloak | Design | Planned | Planned | Planned | No |
| 006 | Pyromancer | Firebrand Sigil | Standard | engraved fire seal | Design | Planned | Planned | Planned | No |
| 007 | Pyromancer | Pyre Serpent | Standard | coiled fire serpent | Design | Planned | Planned | Planned | No |
| 015 | Pyromancer | Phoenix Mantle | Awakening | ember feather wings | Design | Planned | Planned | Planned | No |
| 016 | Pyromancer | Magma Crown | Awakening | seven basalt magma spires | Design | Planned | Planned | Planned | No |
| 019 | Pyromancer | Cataclysmic Caldera | Ultimate | tiered erupting caldera | Design | Planned | Planned | Planned | No |
| 021 | Cryomancer | Frost Needle | Standard | thin hexagonal ice dart | Portable core only | Planned | Planned | Planned | No |
| 022 | Cryomancer | Rime Bridge | Standard | interlocking suspended ice plates | Portable core only | Planned | Planned | Planned | No |
| 023 | Cryomancer | Shard Chorus | Standard | asymmetric crystal fan | Design | Planned | Planned | Planned | No |
| 024 | Cryomancer | Glaze Prison | Standard | faceted six-sided cage | Design | Planned | Planned | Planned | No |
| 025 | Cryomancer | Hail Carousel | Standard | transparent hail asteroids | Design | Planned | Planned | Planned | No |
| 026 | Cryomancer | Hoarfrost Trail | Standard | branching frost floor vines | Design | Planned | Planned | Planned | No |
| 027 | Cryomancer | Permafrost Grip | Standard | clawed crystalline hand | Design | Planned | Planned | Planned | No |
| 035 | Cryomancer | Avalanche Choir | Awakening | tessellated rolling avalanche | Design | Planned | Planned | Planned | No |
| 036 | Cryomancer | Diamondheart | Awakening | interlocked clear armor shells | Design | Planned | Planned | Planned | No |
| 039 | Cryomancer | White Silence | Ultimate | colossal snowflake vault | Design | Planned | Planned | Planned | No |
| 041 | Aeromancer | Gale Palm | Standard | spiral air pressure palm | Portable core only | Planned | Planned | Planned | No |
| 042 | Aeromancer | Razor Crescent | Standard | translucent wind crescent | Portable core only | Planned | Planned | Planned | No |
| 043 | Aeromancer | Updraft Well | Standard | stacked rising air rings | Design | Planned | Planned | Planned | No |
| 044 | Aeromancer | Feather Drift | Standard | floating feathers and vortex | Design | Planned | Planned | Planned | No |
| 045 | Aeromancer | Vacuum Pin | Standard | compressed transparent needle | Design | Planned | Planned | Planned | No |
| 046 | Aeromancer | Cyclone Stride | Standard | ribbon slipstream footsteps | Design | Planned | Planned | Planned | No |
| 047 | Aeromancer | Banshee Whistle | Standard | concentric vibration helix | Design | Planned | Planned | Planned | No |
| 055 | Aeromancer | Tempest Staircase | Awakening | floating air plate helix | Design | Planned | Planned | Planned | No |
| 056 | Aeromancer | Zephyr Doppel | Awakening | hollow flowing silhouette | Design | Planned | Planned | Planned | No |
| 059 | Aeromancer | Worldbreath Spiral | Ultimate | colossal vortex column | Design | Planned | Planned | Planned | No |
| 061 | Geomancer | Stone Knuckle | Standard | articulated stone fist | Portable core only | Planned | Planned | Planned | No |
| 062 | Geomancer | Crystal Traverse | Standard | floating prismatic stairs | Portable core only | Planned | Planned | Planned | No |
| 063 | Geomancer | Gravel Jaws | Standard | interlocking jagged jaws | Design | Planned | Planned | Planned | No |
| 064 | Geomancer | Faultline | Standard | branching earth seam | Design | Planned | Planned | Planned | No |
| 065 | Geomancer | Shard Bulwark | Standard | overlapping crystal shield petals | Design | Planned | Planned | Planned | No |
| 066 | Geomancer | Granite Orbit | Standard | polygonal boulder orbit | Design | Planned | Planned | Planned | No |
| 067 | Geomancer | Ore Sense | Standard | subsurface crystal resonance lattice | Design | Planned | Planned | Planned | No |
| 075 | Geomancer | Colossus Gauntlet | Awakening | articulated cliff hands | Design | Planned | Planned | Planned | No |
| 076 | Geomancer | Gemheart Armor | Awakening | layered gem-plated mantle | Design | Planned | Planned | Planned | No |
| 079 | Geomancer | Earthshaker Throne | Ultimate | tiered suspended throne | Design | Planned | Planned | Planned | No |
| 081 | Hydromancer | Tide Needle | Standard | braided liquid spear | Portable core only | Planned | Planned | Planned | No |
| 082 | Hydromancer | Flowstep | Standard | rolling ribbon surfboard | Portable core only | Planned | Planned | Planned | No |
| 083 | Hydromancer | Pearl Shield | Standard | layered water pearl | Design | Planned | Planned | Planned | No |
| 084 | Hydromancer | Riptide Snare | Standard | coiled liquid tether | Design | Planned | Planned | Planned | No |
| 085 | Hydromancer | Mistcloak | Standard | billowing droplet cloak | Design | Planned | Planned | Planned | No |
| 086 | Hydromancer | Surge Fan | Standard | scalloped water fan | Design | Planned | Planned | Planned | No |
| 087 | Hydromancer | Rainweaver | Standard | suspended rain lattice | Design | Planned | Planned | Planned | No |
| 095 | Hydromancer | Ocean Guardian | Awakening | armored water turtle | Design | Planned | Planned | Planned | No |
| 096 | Hydromancer | Monsoon Choir | Awakening | spiraling water fountains | Design | Planned | Planned | Planned | No |
| 099 | Hydromancer | King Tide | Ultimate | stepped tidal mountain | Design | Planned | Planned | Planned | No |
| 101 | Druid | Thorn Dart | Standard | barbed spinning seed | Portable core only | Planned | Planned | Planned | No |
| 102 | Druid | Root Snare | Standard | knotted branching roots | Portable core only | Planned | Planned | Planned | No |
| 103 | Druid | Moss Mend | Standard | living moss weave | Design | Planned | Planned | Planned | No |
| 104 | Druid | Briar Loop | Standard | braided thorn wreath | Design | Planned | Planned | Planned | No |
| 105 | Druid | Firefly Veil | Standard | light insect veil | Design | Planned | Planned | Planned | No |
| 106 | Druid | Seedburst | Standard | radial seed pod cluster | Design | Planned | Planned | Planned | No |
| 107 | Druid | Vine Swing | Standard | growing jointed vine rope | Design | Planned | Planned | Planned | No |
| 115 | Druid | Ancient Ent | Awakening | gnarled articulated tree | Design | Planned | Planned | Planned | No |
| 116 | Druid | Emerald Canopy | Awakening | layered suspended forest canopy | Design | Planned | Planned | Planned | No |
| 119 | Druid | World Tree Awakening | Ultimate | colossal rooted world tree | Design | Planned | Planned | Planned | No |
| 121 | Electromancer | Spark Fork | Standard | jagged spark fork | Portable core only | Planned | Planned | Planned | No |
| 122 | Electromancer | Static Halo | Standard | charged spinning crown | Portable core only | Planned | Planned | Planned | No |
| 123 | Electromancer | Thunder Needle | Standard | electric crystal needle | Design | Planned | Planned | Planned | No |
| 124 | Electromancer | Arc Net | Standard | triangular lightning lattice | Design | Planned | Planned | Planned | No |
| 125 | Electromancer | Storm Coil | Standard | triple helical arc torus | Design | Planned | Planned | Planned | No |
| 126 | Electromancer | Flashstep | Standard | stretched light silhouette | Design | Planned | Planned | Planned | No |
| 127 | Electromancer | Magnet Stone | Standard | runic floating lodestone | Design | Planned | Planned | Planned | No |
| 135 | Electromancer | Living Thunder | Awakening | winged arc elemental | Design | Planned | Planned | Planned | No |
| 136 | Electromancer | Thunder Drum | Awakening | rune drum rings | Design | Planned | Planned | Planned | No |
| 139 | Electromancer | Judgement of Storms | Ultimate | sky-spanning lightning canopy | Design | Planned | Planned | Planned | No |
| 141 | Astral Mage | Star Shard | Standard | seven pointed spectral star | Portable core only | Planned | Planned | Planned | No |
| 142 | Astral Mage | Gravity Thread | Standard | taut gravity filament | Portable core only | Planned | Planned | Planned | No |
| 143 | Astral Mage | Blink Fold | Standard | folded rectangular spatial ribbons | Design | Planned | Planned | Planned | No |
| 144 | Astral Mage | Moonlit Veil | Standard | curved silver phase sheets | Design | Planned | Planned | Planned | No |
| 145 | Astral Mage | Rift Slice | Standard | broken crescent rift | Design | Planned | Planned | Planned | No |
| 146 | Astral Mage | Comet Pearl | Standard | many faceted comet sphere | Design | Planned | Planned | Planned | No |
| 147 | Astral Mage | Astral Compass | Standard | rotating star compass | Design | Planned | Planned | Planned | No |
| 155 | Astral Mage | Nebula Mantle | Awakening | transparent star filled mantle | Design | Planned | Planned | Planned | No |
| 156 | Astral Mage | Gravity Crown | Awakening | tilted gravity halos | Design | Planned | Planned | Planned | No |
| 159 | Astral Mage | Starfall Liturgy | Ultimate | multi-ring starfall constellation | Design | Planned | Planned | Planned | No |
| 161 | Chronomancer | Second Hand | Standard | rotating brass clock hand | Portable core only | Planned | Planned | Planned | No |
| 162 | Chronomancer | Moment Slip | Standard | curving faded movement trail | Portable core only | Planned | Planned | Planned | No |
| 163 | Chronomancer | Tick Pulse | Standard | nested translucent watch gears | Design | Planned | Planned | Planned | No |
| 164 | Chronomancer | Sands of Delay | Standard | hovering inverted sandglass | Design | Planned | Planned | Planned | No |
| 165 | Chronomancer | Temporal Mark | Standard | hourglass target rune | Design | Planned | Planned | Planned | No |
| 166 | Chronomancer | Echo Step | Standard | offset footstep echoes | Design | Planned | Planned | Planned | No |
| 167 | Chronomancer | Aging Touch | Standard | cracked sundial palm | Design | Planned | Planned | Planned | No |
| 175 | Chronomancer | Hour Guardian | Awakening | articulated floating timekeeper | Design | Planned | Planned | Planned | No |
| 176 | Chronomancer | Stasis Lens | Awakening | layered stopped-time lens | Design | Planned | Planned | Planned | No |
| 179 | Chronomancer | Thirteenth Hour | Ultimate | colossal cathedral clock | Design | Planned | Planned | Planned | No |
| 181 | Necromancer | Bone Thorn | Standard | twisted vertebral spear | Portable core only | Planned | Planned | Planned | No |
| 182 | Necromancer | Soul Lantern | Standard | floating skull lantern | Portable core only | Planned | Planned | Planned | No |
| 183 | Necromancer | Grave Whisper | Standard | hollow skull wind glyph | Design | Planned | Planned | Planned | No |
| 184 | Necromancer | Blood Thread | Standard | braided crimson cord | Design | Planned | Planned | Planned | No |
| 185 | Necromancer | Rib Cage | Standard | interlocking giant ribs | Design | Planned | Planned | Planned | No |
| 186 | Necromancer | Wraith Step | Standard | ghost footstep trail | Design | Planned | Planned | Planned | No |
| 187 | Necromancer | Carrion Swarm | Standard | spirit moth swarm | Design | Planned | Planned | Planned | No |
| 195 | Necromancer | Grim Shepherd | Awakening | armored skeleton guardian | Design | Planned | Planned | Planned | No |
| 196 | Necromancer | Soul Exchange | Awakening | connected soul vessels | Design | Planned | Planned | Planned | No |
| 199 | Necromancer | Kingdom of Ash | Ultimate | colossal shadow cemetery arches | Design | Planned | Planned | Planned | No |

## Completion snapshot

**0 / 100 fully integrated spells are verified.** 20 / 100 have portable core *intent* prototypes, but no end-to-end server-authoritative gameplay. 5 independent visual showcase effects have game video evidence in v0.2.0. Do not interpret 100 planned entries as 100 playable abilities.

## Development sequence (without user testing)

1. Solid/faceted/crystalline/organic 3D geometry engine, spatial attachment/ground collision, time-staged animation, original audio, LOD and draw fairness. Verify in CI.
2. Move the portable mana/cooldown/damage/heal/control core onto the real Horizon server/client boundary; implement safe game-state and save/network syncing.
3. Make the complete GUI (mana orb, spell wheel, constellation tree, class selection, collection and crafting) and server-authoritative obtaining/unlocking loop.
4. Build 100 individually authored and mechanically distinct spells for the exact IDs in the table, including 10 awakening sequences and 10 cinematic ultimates; balance and automated integration checks.
5. Only then package the M100 build for the user's first postponed manual test. Fix any reported issues before moving toward 200.

## Quality requirements for the example image

Reference `image(7).png`, user-supplied Ice Awakening screenshot on 2026-11-19: blue sharp giant wedge/spike masses emerging from the *ground*, deep perspective and serrated heights. Reproduce the **concept** with original geometric ice meshes, multiple opaque differently shaded side facets, ground anchoring and sequential growth. Do not copy the external game's UI/assets/models.
