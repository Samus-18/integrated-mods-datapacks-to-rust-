# Aetheria — Master spell registry

200 unique spell **design entries** across ten classes. First 3 spells of each class (**30 total**) now have distinct portable Rust casting intents, bounded multi-sided world-space **3D VFX**, original spatial sound events and their own four-phase telegraph/cast/impact/afterglow timelines in the **client-only 0.5.3 WASM preview**. This is not gameplay completion: entity damage/healing, persistent unlocks, world modification, multiplayer authority and game save sync remain unimplemented. The remaining 170 spells are design-only. Five large prototypes (Phoenix Garden, Glacier Cathedral, Living Worldroot, Thunder Wyrm, Astral Sunder) are additional visual motifs, not separately completed spell IDs. A successful pinned SDK CI is recorded in `docs/M14_TEMPORAL_VFX.md`; it does not establish user-tested visual fidelity. Master original prompt exact full text remains unavailable, with known exact quotations and source details preserved in `docs/memory/ORIGINAL_PROMPT_VISUAL_REQUIREMENTS.md`.

| ID | Class | Spell | Role | Cast | Mana | Cooldown (ticks) | 3D mesh concept | Physics/behavior concept | WGSL profile | Sound cue | Status |
|---|---|---|---|---|---:|---:|---|---|---|---|---|
| 001 | Pyromancer | Ember Needle | Standard | Instant | 10 | 15 | twisting ember thorn | piercing ignite dart | spell_ember_needle | aetheria:ember_needle | Core cast intent + 3D client VFX/SFX; no world effect |
| 002 | Pyromancer | Cinder Fan | Standard | Instant | 14 | 23 | seven curved coal blades | widening multi target cinder attack | spell_cinder_fan | aetheria:cinder_fan | Core cast intent + 3D client VFX/SFX; no world effect |
| 003 | Pyromancer | Magma Lash | Standard | Instant | 18 | 31 | segmented molten whip | whip that curls around nearby cover | spell_magma_lash | aetheria:magma_lash | Core cast intent + 3D client VFX/SFX; no world effect |
| 004 | Pyromancer | Furnace Step | Standard | Instant | 22 | 39 | molten footstep ribbon | dash leaves delayed burning footsteps | spell_furnace_step | aetheria:furnace_step | Design only |
| 005 | Pyromancer | Ashen Veil | Standard | Charged | 26 | 47 | rolling ash cloak | concealing cloud deflects weak missiles | spell_ashen_veil | aetheria:ashen_veil | Design only |
| 006 | Pyromancer | Firebrand Sigil | Standard | Channeled | 30 | 55 | engraved fire seal | timed floor ignition and area marking | spell_firebrand_sigil | aetheria:firebrand_sigil | Design only |
| 007 | Pyromancer | Pyre Serpent | Standard | Instant | 34 | 63 | coiled fire serpent | heat seeking winding flame pursuit | spell_pyre_serpent | aetheria:pyre_serpent | Design only |
| 008 | Pyromancer | Lava Bloom | Standard | Combo | 10 | 71 | opening magma flower | radial petals explode sequentially | spell_lava_bloom | aetheria:lava_bloom | Design only |
| 009 | Pyromancer | Coalstorm | Standard | Charged | 14 | 79 | swirling charcoal stones | debris orbit damages nearby enemies | spell_coalstorm | aetheria:coalstorm | Design only |
| 010 | Pyromancer | Searing Javelin | Standard | Instant | 18 | 15 | barbed flame lance | piercing line attack with burn trail | spell_searing_javelin | aetheria:searing_javelin | Design only |
| 011 | Pyromancer | Kindle Chain | Standard | Channeled | 22 | 23 | linked ember hooks | propagates flame between ignited enemies | spell_kindle_chain | aetheria:kindle_chain | Design only |
| 012 | Pyromancer | Basalt Burst | Standard | Instant | 26 | 31 | angular volcanic fan | shatters heated rock in radial spray | spell_basalt_burst | aetheria:basalt_burst | Design only |
| 013 | Pyromancer | Heat Mirage | Standard | Charged | 30 | 39 | rippling refractive veil | distorts incoming aiming trajectories | spell_heat_mirage | aetheria:heat_mirage | Design only |
| 014 | Pyromancer | Infernal Anvil | Standard | Instant | 34 | 47 | rune-forged molten anvil | summoned heavy strike and spark shower | spell_infernal_anvil | aetheria:infernal_anvil | Design only |
| 015 | Pyromancer | Phoenix Mantle | Awakening | Combo | 55 | 300 | ember feather wings | glide on thermal lift and restore vigor | spell_phoenix_mantle | aetheria:phoenix_mantle | Design only |
| 016 | Pyromancer | Magma Crown | Awakening | Channeled | 55 | 300 | seven basalt magma spires | raise staggered defensive lava columns | spell_magma_crown | aetheria:magma_crown | Design only |
| 017 | Pyromancer | Solar Crucible | Awakening | Charged | 55 | 300 | golden multi-ring furnace | compress blast power inside a timed sphere | spell_solar_crucible | aetheria:solar_crucible | Design only |
| 018 | Pyromancer | Dragonfire Procession | Awakening | Instant | 55 | 300 | braided fire dragon heads | alternating draconic breath paths | spell_dragonfire_procession | aetheria:dragonfire_procession | Design only |
| 019 | Pyromancer | Cataclysmic Caldera | Ultimate | Instant | 95 | 1200 | tiered erupting caldera | raise erupting volcanic dome then collapse | spell_cataclysmic_caldera | aetheria:cataclysmic_caldera | Design only |
| 020 | Pyromancer | Sunheart Rebirth | Ultimate | Instant | 95 | 1200 | unfolding flaming phoenix | phoenix-healing sanctuary followed by shock | spell_sunheart_rebirth | aetheria:sunheart_rebirth | Design only |
| 021 | Cryomancer | Frost Needle | Standard | Instant | 10 | 15 | thin hexagonal ice dart | single target freezing precision lance | spell_frost_needle | aetheria:frost_needle | Core cast intent + 3D client VFX/SFX; no world effect |
| 022 | Cryomancer | Rime Bridge | Standard | Instant | 14 | 23 | interlocking suspended ice plates | freeze temporary traversable bridge | spell_rime_bridge | aetheria:rime_bridge | Core cast intent + 3D client VFX/SFX; no world effect |
| 023 | Cryomancer | Shard Chorus | Standard | Instant | 18 | 31 | asymmetric crystal fan | distribute spikes along divergent angles | spell_shard_chorus | aetheria:shard_chorus | Core cast intent + 3D client VFX/SFX; no world effect |
| 024 | Cryomancer | Glaze Prison | Standard | Instant | 22 | 39 | faceted six-sided cage | enclose hostile in brittle ice shell | spell_glaze_prison | aetheria:glaze_prison | Design only |
| 025 | Cryomancer | Hail Carousel | Standard | Charged | 26 | 47 | transparent hail asteroids | orbiting hail protects while damaging | spell_hail_carousel | aetheria:hail_carousel | Design only |
| 026 | Cryomancer | Hoarfrost Trail | Standard | Channeled | 30 | 55 | branching frost floor vines | freeze a path where caster walks | spell_hoarfrost_trail | aetheria:hoarfrost_trail | Design only |
| 027 | Cryomancer | Permafrost Grip | Standard | Instant | 34 | 63 | clawed crystalline hand | ice hand binds grounded enemy | spell_permafrost_grip | aetheria:permafrost_grip | Design only |
| 028 | Cryomancer | Sleet Mirror | Standard | Combo | 10 | 71 | three slanted ice mirrors | reflect directional projectiles using slabs | spell_sleet_mirror | aetheria:sleet_mirror | Design only |
| 029 | Cryomancer | Glacier Lance | Standard | Charged | 14 | 79 | twisted glacier spear | charging spear penetrates thick armor | spell_glacier_lance | aetheria:glacier_lance | Design only |
| 030 | Cryomancer | Snowblind | Standard | Instant | 18 | 15 | layered snow funnel | blizzard blocks view and slows entrants | spell_snowblind | aetheria:snowblind | Design only |
| 031 | Cryomancer | Frozen Tether | Standard | Channeled | 22 | 23 | circular icicle chain | connect chilled targets and share slow | spell_frozen_tether | aetheria:frozen_tether | Design only |
| 032 | Cryomancer | Icicle Rain | Standard | Instant | 26 | 31 | suspended ice chandelier | mark ground for falling icicle needles | spell_icicle_rain | aetheria:icicle_rain | Design only |
| 033 | Cryomancer | Crystal Fault | Standard | Charged | 30 | 39 | zigzag frozen fault ridge | ice ridge tears through terrain direction | spell_crystal_fault | aetheria:crystal_fault | Design only |
| 034 | Cryomancer | Winter Breath | Standard | Instant | 34 | 47 | conical crystal breath | cast widening mist cone and frost | spell_winter_breath | aetheria:winter_breath | Design only |
| 035 | Cryomancer | Avalanche Choir | Awakening | Combo | 55 | 300 | tessellated rolling avalanche | snow wall accelerates downhill | spell_avalanche_choir | aetheria:avalanche_choir | Design only |
| 036 | Cryomancer | Diamondheart | Awakening | Channeled | 55 | 300 | interlocked clear armor shells | reflective armor breaks into attacking shards | spell_diamondheart | aetheria:diamondheart | Design only |
| 037 | Cryomancer | Cryostasis Vault | Awakening | Charged | 55 | 300 | domed frost hourglass | halt enemy arrows inside ice dome | spell_cryostasis_vault | aetheria:cryostasis_vault | Design only |
| 038 | Cryomancer | Glacial Leviathan | Awakening | Instant | 55 | 300 | serpentine glacier skeleton | sweep enemies with coiling frost beast | spell_glacial_leviathan | aetheria:glacial_leviathan | Design only |
| 039 | Cryomancer | White Silence | Ultimate | Instant | 95 | 1200 | colossal snowflake vault | halt motion briefly throughout snow sanctuary | spell_white_silence | aetheria:white_silence | Design only |
| 040 | Cryomancer | Absolute Zero Cathedral | Ultimate | Instant | 95 | 1200 | towering ice cathedral ribs | erect huge cryogenic crystal sanctuary | spell_absolute_zero_cathedral | aetheria:absolute_zero_cathedral | Design only |
| 041 | Aeromancer | Gale Palm | Standard | Instant | 10 | 15 | spiral air pressure palm | narrow cone pushes foes away | spell_gale_palm | aetheria:gale_palm | Core cast intent + 3D client VFX/SFX; no world effect |
| 042 | Aeromancer | Razor Crescent | Standard | Instant | 14 | 23 | translucent wind crescent | sweeping curved air blade cuts targets | spell_razor_crescent | aetheria:razor_crescent | Core cast intent + 3D client VFX/SFX; no world effect |
| 043 | Aeromancer | Updraft Well | Standard | Instant | 18 | 31 | stacked rising air rings | vertical airflow lifts caster | spell_updraft_well | aetheria:updraft_well | Core cast intent + 3D client VFX/SFX; no world effect |
| 044 | Aeromancer | Feather Drift | Standard | Instant | 22 | 39 | floating feathers and vortex | reduce nearby friendly fall velocity | spell_feather_drift | aetheria:feather_drift | Design only |
| 045 | Aeromancer | Vacuum Pin | Standard | Charged | 26 | 47 | compressed transparent needle | pull nearby item or enemy to focus | spell_vacuum_pin | aetheria:vacuum_pin | Design only |
| 046 | Aeromancer | Cyclone Stride | Standard | Channeled | 30 | 55 | ribbon slipstream footsteps | dash follows steerable curved flow | spell_cyclone_stride | aetheria:cyclone_stride | Design only |
| 047 | Aeromancer | Banshee Whistle | Standard | Instant | 34 | 63 | concentric vibration helix | short sonic stunning pressure blast | spell_banshee_whistle | aetheria:banshee_whistle | Design only |
| 048 | Aeromancer | Twister Seed | Standard | Combo | 10 | 71 | narrow wind funnel | grows a wandering miniature tornado | spell_twister_seed | aetheria:twister_seed | Design only |
| 049 | Aeromancer | Aerial Mantle | Standard | Charged | 14 | 79 | flowing air wings | control movement during fall | spell_aerial_mantle | aetheria:aerial_mantle | Design only |
| 050 | Aeromancer | Thunderhead Draft | Standard | Instant | 18 | 15 | rotating vapor canopy | push floating storm cloud along path | spell_thunderhead_draft | aetheria:thunderhead_draft | Design only |
| 051 | Aeromancer | Suction Lattice | Standard | Channeled | 22 | 23 | triangular braided pressure strands | several anchors form a pull field | spell_suction_lattice | aetheria:suction_lattice | Design only |
| 052 | Aeromancer | Skyward Hook | Standard | Instant | 26 | 31 | coiled air tether | grapple toward selected upper ledge | spell_skyward_hook | aetheria:skyward_hook | Design only |
| 053 | Aeromancer | Soundbreak | Standard | Charged | 30 | 39 | double layered sonic cone | dash ends with sharp directional shockwave | spell_soundbreak | aetheria:soundbreak | Design only |
| 054 | Aeromancer | Windwall Gallery | Standard | Instant | 34 | 47 | arched translucent air ribs | curve incoming arrows away | spell_windwall_gallery | aetheria:windwall_gallery | Design only |
| 055 | Aeromancer | Tempest Staircase | Awakening | Combo | 55 | 300 | floating air plate helix | create sequential hovering wind steps | spell_tempest_staircase | aetheria:tempest_staircase | Design only |
| 056 | Aeromancer | Zephyr Doppel | Awakening | Channeled | 55 | 300 | hollow flowing silhouette | mislead hostiles with air double | spell_zephyr_doppel | aetheria:zephyr_doppel | Design only |
| 057 | Aeromancer | Storm Eye | Awakening | Charged | 55 | 300 | counter-rotating wind torus | still center surrounded by hurtling wind | spell_storm_eye | aetheria:storm_eye | Design only |
| 058 | Aeromancer | Hurricane Harp | Awakening | Instant | 55 | 300 | strung harp wind arcs | alternate push and pull on timed beats | spell_hurricane_harp | aetheria:hurricane_harp | Design only |
| 059 | Aeromancer | Worldbreath Spiral | Ultimate | Instant | 95 | 1200 | colossal vortex column | giant wind helix launches enemies | spell_worldbreath_spiral | aetheria:worldbreath_spiral | Design only |
| 060 | Aeromancer | Vacuum Requiem | Ultimate | Instant | 95 | 1200 | inverted pressure globe | implode area then release pressure outward | spell_vacuum_requiem | aetheria:vacuum_requiem | Design only |
| 061 | Geomancer | Stone Knuckle | Standard | Instant | 10 | 15 | articulated stone fist | stone fist uppercuts a hostile | spell_stone_knuckle | aetheria:stone_knuckle | Core cast intent + 3D client VFX/SFX; no world effect |
| 062 | Geomancer | Crystal Traverse | Standard | Instant | 14 | 23 | floating prismatic stairs | raise temporary rising stairway | spell_crystal_traverse | aetheria:crystal_traverse | Core cast intent + 3D client VFX/SFX; no world effect |
| 063 | Geomancer | Gravel Jaws | Standard | Instant | 18 | 31 | interlocking jagged jaws | close rocky plates on target | spell_gravel_jaws | aetheria:gravel_jaws | Core cast intent + 3D client VFX/SFX; no world effect |
| 064 | Geomancer | Faultline | Standard | Instant | 22 | 39 | branching earth seam | crack ripples through ground ahead | spell_faultline | aetheria:faultline | Design only |
| 065 | Geomancer | Shard Bulwark | Standard | Charged | 26 | 47 | overlapping crystal shield petals | instant temporary crystal barrier | spell_shard_bulwark | aetheria:shard_bulwark | Design only |
| 066 | Geomancer | Granite Orbit | Standard | Channeled | 30 | 55 | polygonal boulder orbit | defensive orbiting stones intercept | spell_granite_orbit | aetheria:granite_orbit | Design only |
| 067 | Geomancer | Ore Sense | Standard | Instant | 34 | 63 | subsurface crystal resonance lattice | brief reveal buried ore veins | spell_ore_sense | aetheria:ore_sense | Design only |
| 068 | Geomancer | Earthbind | Standard | Combo | 10 | 71 | angular stone cuffs | pin target feet with mineral shackles | spell_earthbind | aetheria:earthbind | Design only |
| 069 | Geomancer | Pebble Barrage | Standard | Charged | 14 | 79 | helical stone sling | fire repeated spinning stone shots | spell_pebble_barrage | aetheria:pebble_barrage | Design only |
| 070 | Geomancer | Obsidian Needle | Standard | Instant | 18 | 15 | thin black monolith | drive dark brittle spear upward | spell_obsidian_needle | aetheria:obsidian_needle | Design only |
| 071 | Geomancer | Tremor Echo | Standard | Channeled | 22 | 23 | nested fissure waves | repeat previous quake from placed marks | spell_tremor_echo | aetheria:tremor_echo | Design only |
| 072 | Geomancer | Quartz Garden | Standard | Instant | 26 | 31 | branching quartz sculptures | grow protective quartz maze | spell_quartz_garden | aetheria:quartz_garden | Design only |
| 073 | Geomancer | Mountain Shoulder | Standard | Charged | 30 | 39 | sliding cliff mantle | move a massive barricade sideways | spell_mountain_shoulder | aetheria:mountain_shoulder | Design only |
| 074 | Geomancer | Sandglass Trap | Standard | Instant | 34 | 47 | granular vortex pit | sink targets into unstable sand | spell_sandglass_trap | aetheria:sandglass_trap | Design only |
| 075 | Geomancer | Colossus Gauntlet | Awakening | Combo | 55 | 300 | articulated cliff hands | summon twin giant stone fists | spell_colossus_gauntlet | aetheria:colossus_gauntlet | Design only |
| 076 | Geomancer | Gemheart Armor | Awakening | Channeled | 55 | 300 | layered gem-plated mantle | reactive armor sheds cutting gems | spell_gemheart_armor | aetheria:gemheart_armor | Design only |
| 077 | Geomancer | Tectonic Crown | Awakening | Charged | 55 | 300 | tilted earth column crown | raise towers in delayed circle | spell_tectonic_crown | aetheria:tectonic_crown | Design only |
| 078 | Geomancer | Monolith March | Awakening | Instant | 55 | 300 | walking carved monoliths | march stone pillars to shield allies | spell_monolith_march | aetheria:monolith_march | Design only |
| 079 | Geomancer | Earthshaker Throne | Ultimate | Instant | 95 | 1200 | tiered suspended throne | lift platform and throw boulder rain | spell_earthshaker_throne | aetheria:earthshaker_throne | Design only |
| 080 | Geomancer | Cathedral of Roots | Ultimate | Instant | 95 | 1200 | interwoven rock arches | build broad stone ribbed defensive cavern | spell_cathedral_of_roots | aetheria:cathedral_of_roots | Design only |
| 081 | Hydromancer | Tide Needle | Standard | Instant | 10 | 15 | braided liquid spear | focused water jet pierces and wets | spell_tide_needle | aetheria:tide_needle | Core cast intent + 3D client VFX/SFX; no world effect |
| 082 | Hydromancer | Flowstep | Standard | Instant | 14 | 23 | rolling ribbon surfboard | slide rapidly on water ribbon | spell_flowstep | aetheria:flowstep | Core cast intent + 3D client VFX/SFX; no world effect |
| 083 | Hydromancer | Pearl Shield | Standard | Instant | 18 | 31 | layered water pearl | shell absorbs one inbound hit | spell_pearl_shield | aetheria:pearl_shield | Core cast intent + 3D client VFX/SFX; no world effect |
| 084 | Hydromancer | Riptide Snare | Standard | Instant | 22 | 39 | coiled liquid tether | current curls enemies inward | spell_riptide_snare | aetheria:riptide_snare | Design only |
| 085 | Hydromancer | Mistcloak | Standard | Charged | 26 | 47 | billowing droplet cloak | fog obscures friendly positions | spell_mistcloak | aetheria:mistcloak | Design only |
| 086 | Hydromancer | Surge Fan | Standard | Channeled | 30 | 55 | scalloped water fan | push foes with sideways water sheet | spell_surge_fan | aetheria:surge_fan | Design only |
| 087 | Hydromancer | Rainweaver | Standard | Instant | 34 | 63 | suspended rain lattice | bring controlled local rain patch | spell_rainweaver | aetheria:rainweaver | Design only |
| 088 | Hydromancer | Foam Cascade | Standard | Combo | 10 | 71 | stacked foaming breakers | several successive short pushing waves | spell_foam_cascade | aetheria:foam_cascade | Design only |
| 089 | Hydromancer | Abyss Jet | Standard | Charged | 14 | 79 | cavitating liquid lance | concentrated fluid beam penetrates armor | spell_abyss_jet | aetheria:abyss_jet | Design only |
| 090 | Hydromancer | Whirlpool Ring | Standard | Instant | 18 | 15 | nested rotating liquid toroids | orbiting current pulls enemies | spell_whirlpool_ring | aetheria:whirlpool_ring | Design only |
| 091 | Hydromancer | Brine Mark | Standard | Channeled | 22 | 23 | crystalline salt halo | wet target becomes vulnerable to lightning | spell_brine_mark | aetheria:brine_mark | Design only |
| 092 | Hydromancer | River Serpent | Standard | Instant | 26 | 31 | segmented aquatic serpent | water serpent bends around cover | spell_river_serpent | aetheria:river_serpent | Design only |
| 093 | Hydromancer | Tidal Mirror | Standard | Charged | 30 | 39 | curved transparent mirrors | project false reflections on water | spell_tidal_mirror | aetheria:tidal_mirror | Design only |
| 094 | Hydromancer | Undertow Hand | Standard | Instant | 34 | 47 | long fingered fluid palm | submerged water grip immobilizes | spell_undertow_hand | aetheria:undertow_hand | Design only |
| 095 | Hydromancer | Ocean Guardian | Awakening | Combo | 55 | 300 | armored water turtle | water spirit guards caster | spell_ocean_guardian | aetheria:ocean_guardian | Design only |
| 096 | Hydromancer | Monsoon Choir | Awakening | Channeled | 55 | 300 | spiraling water fountains | linked restorative fountains | spell_monsoon_choir | aetheria:monsoon_choir | Design only |
| 097 | Hydromancer | Abyssal Dome | Awakening | Charged | 55 | 300 | layered pressure globe | pressure dome slows inbound projectiles | spell_abyssal_dome | aetheria:abyssal_dome | Design only |
| 098 | Hydromancer | Leviathan Spiral | Awakening | Instant | 55 | 300 | helical fluid dragon | water dragon winds through hostiles | spell_leviathan_spiral | aetheria:leviathan_spiral | Design only |
| 099 | Hydromancer | King Tide | Ultimate | Instant | 95 | 1200 | stepped tidal mountain | gigantic traveling tsunami wave | spell_king_tide | aetheria:king_tide | Design only |
| 100 | Hydromancer | Deepheart Communion | Ultimate | Instant | 95 | 1200 | colossal glowing coral vault | luminous coral healing sanctuary | spell_deepheart_communion | aetheria:deepheart_communion | Design only |
| 101 | Druid | Thorn Dart | Standard | Instant | 10 | 15 | barbed spinning seed | thorn seed implants creeping root | spell_thorn_dart | aetheria:thorn_dart | Core cast intent + 3D client VFX/SFX; no world effect |
| 102 | Druid | Root Snare | Standard | Instant | 14 | 23 | knotted branching roots | roots grip several nearby feet | spell_root_snare | aetheria:root_snare | Core cast intent + 3D client VFX/SFX; no world effect |
| 103 | Druid | Moss Mend | Standard | Instant | 18 | 31 | living moss weave | moss continuously restores friendly health | spell_moss_mend | aetheria:moss_mend | Core cast intent + 3D client VFX/SFX; no world effect |
| 104 | Druid | Briar Loop | Standard | Instant | 22 | 39 | braided thorn wreath | rotating briars block advance | spell_briar_loop | aetheria:briar_loop | Design only |
| 105 | Druid | Firefly Veil | Standard | Charged | 26 | 47 | light insect veil | luminous insects conceal allies | spell_firefly_veil | aetheria:firefly_veil | Design only |
| 106 | Druid | Seedburst | Standard | Channeled | 30 | 55 | radial seed pod cluster | pods planted then explode in sequence | spell_seedburst | aetheria:seedburst | Design only |
| 107 | Druid | Vine Swing | Standard | Instant | 34 | 63 | growing jointed vine rope | grappling climbing vine line | spell_vine_swing | aetheria:vine_swing | Design only |
| 108 | Druid | Barkskin | Standard | Combo | 10 | 71 | interlocking bark plating | layered bark armor wears off | spell_barkskin | aetheria:barkskin | Design only |
| 109 | Druid | Bloomstep | Standard | Charged | 14 | 79 | unfurling flower path | flowers grant speed boosts | spell_bloomstep | aetheria:bloomstep | Design only |
| 110 | Druid | Mushroom Lantern | Standard | Instant | 18 | 15 | bioluminescent fungus canopy | light fungi create healing field | spell_mushroom_lantern | aetheria:mushroom_lantern | Design only |
| 111 | Druid | Sapling Chain | Standard | Channeled | 22 | 23 | branching root cables | root conduit connects summoned allies | spell_sapling_chain | aetheria:sapling_chain | Design only |
| 112 | Druid | Pollen Cyclone | Standard | Instant | 26 | 31 | floral dust funnel | slowing pollen storm drifts | spell_pollen_cyclone | aetheria:pollen_cyclone | Design only |
| 113 | Druid | Carnivore Bloom | Standard | Charged | 30 | 39 | giant carnivorous petals | jawed flower bites trapped target | spell_carnivore_bloom | aetheria:carnivore_bloom | Design only |
| 114 | Druid | Spore Burst | Standard | Instant | 34 | 47 | clustered spore sacks | glowing spores briefly disorient | spell_spore_burst | aetheria:spore_burst | Design only |
| 115 | Druid | Ancient Ent | Awakening | Combo | 55 | 300 | gnarled articulated tree | walking tree guard fights foes | spell_ancient_ent | aetheria:ancient_ent | Design only |
| 116 | Druid | Emerald Canopy | Awakening | Channeled | 55 | 300 | layered suspended forest canopy | leaf canopy shields and heals | spell_emerald_canopy | aetheria:emerald_canopy | Design only |
| 117 | Druid | Verdant Communion | Awakening | Charged | 55 | 300 | circular blooming root glyph | convert hostile roots into blooms | spell_verdant_communion | aetheria:verdant_communion | Design only |
| 118 | Druid | Thornheart Swarm | Awakening | Instant | 55 | 300 | interconnected vine forest | strangling vines jump between trapped enemies | spell_thornheart_swarm | aetheria:thornheart_swarm | Design only |
| 119 | Druid | World Tree Awakening | Ultimate | Instant | 95 | 1200 | colossal rooted world tree | grow enormous sacred defensive tree | spell_world_tree_awakening | aetheria:world_tree_awakening | Design only |
| 120 | Druid | Gaia Embrace | Ultimate | Instant | 95 | 1200 | ancient layered forest spirit | wide temporary restorative woodland | spell_gaia_embrace | aetheria:gaia_embrace | Design only |
| 121 | Electromancer | Spark Fork | Standard | Instant | 10 | 15 | jagged spark fork | two branching lightning arcs | spell_spark_fork | aetheria:spark_fork | Core cast intent + 3D client VFX/SFX; no world effect |
| 122 | Electromancer | Static Halo | Standard | Instant | 14 | 23 | charged spinning crown | close enemies receive sparks | spell_static_halo | aetheria:static_halo | Core cast intent + 3D client VFX/SFX; no world effect |
| 123 | Electromancer | Thunder Needle | Standard | Instant | 18 | 31 | electric crystal needle | accelerating precision spark spear | spell_thunder_needle | aetheria:thunder_needle | Core cast intent + 3D client VFX/SFX; no world effect |
| 124 | Electromancer | Arc Net | Standard | Instant | 22 | 39 | triangular lightning lattice | electrified mesh binds group | spell_arc_net | aetheria:arc_net | Design only |
| 125 | Electromancer | Storm Coil | Standard | Charged | 26 | 47 | triple helical arc torus | orbiting coils shock threats | spell_storm_coil | aetheria:storm_coil | Design only |
| 126 | Electromancer | Flashstep | Standard | Channeled | 30 | 55 | stretched light silhouette | instant short flicker teleport | spell_flashstep | aetheria:flashstep | Design only |
| 127 | Electromancer | Magnet Stone | Standard | Instant | 34 | 63 | runic floating lodestone | draw metal items into spiral | spell_magnet_stone | aetheria:magnet_stone | Design only |
| 128 | Electromancer | Chain Arc | Standard | Combo | 10 | 71 | segmented lightning tree | electric discharge hops across wet hostiles | spell_chain_arc | aetheria:chain_arc | Design only |
| 129 | Electromancer | Surge Palm | Standard | Charged | 14 | 79 | electric hand glyph | concentric short range discharge | spell_surge_palm | aetheria:surge_palm | Design only |
| 130 | Electromancer | Bolt Weaver | Standard | Instant | 18 | 15 | stitched ground filaments | repeat lightning zigzags across ground | spell_bolt_weaver | aetheria:bolt_weaver | Design only |
| 131 | Electromancer | Conductive Seal | Standard | Channeled | 22 | 23 | copper thunder rune | mark stone for subsequent lightning | spell_conductive_seal | aetheria:conductive_seal | Design only |
| 132 | Electromancer | Skyfire Crown | Standard | Instant | 26 | 31 | sky-rooted thunder trunk | storm bolt splits from cloud | spell_skyfire_crown | aetheria:skyfire_crown | Design only |
| 133 | Electromancer | Pulse Cage | Standard | Charged | 30 | 39 | latticed electric globe | paralyzing alternating arc sphere | spell_pulse_cage | aetheria:pulse_cage | Design only |
| 134 | Electromancer | Storm Feathers | Standard | Instant | 34 | 47 | blade-shaped spark feathers | fast curved charged feathers | spell_storm_feathers | aetheria:storm_feathers | Design only |
| 135 | Electromancer | Living Thunder | Awakening | Combo | 55 | 300 | winged arc elemental | floating element pursues enemies | spell_living_thunder | aetheria:living_thunder | Design only |
| 136 | Electromancer | Thunder Drum | Awakening | Channeled | 55 | 300 | rune drum rings | timed percussion shock waves | spell_thunder_drum | aetheria:thunder_drum | Design only |
| 137 | Electromancer | Tempest Conductor | Awakening | Charged | 55 | 300 | forked conductor wand | steer parallel bolts via pointer | spell_tempest_conductor | aetheria:tempest_conductor | Design only |
| 138 | Electromancer | Heavens Rattle | Awakening | Instant | 55 | 300 | giant moving thunder glyph | linked thunder pulse detonations | spell_heavens_rattle | aetheria:heavens_rattle | Design only |
| 139 | Electromancer | Judgement of Storms | Ultimate | Instant | 95 | 1200 | sky-spanning lightning canopy | colossal branching thunder tree | spell_judgement_of_storms | aetheria:judgement_of_storms | Design only |
| 140 | Electromancer | Heart of Thunder | Ultimate | Instant | 95 | 1200 | rotating electrified cloud heart | storm ward periodically discharges | spell_heart_of_thunder | aetheria:heart_of_thunder | Design only |
| 141 | Astral Mage | Star Shard | Standard | Instant | 10 | 15 | seven pointed spectral star | throw piercing star crystal | spell_star_shard | aetheria:star_shard | Core cast intent + 3D client VFX/SFX; no world effect |
| 142 | Astral Mage | Gravity Thread | Standard | Instant | 14 | 23 | taut gravity filament | pull small target toward focus | spell_gravity_thread | aetheria:gravity_thread | Core cast intent + 3D client VFX/SFX; no world effect |
| 143 | Astral Mage | Blink Fold | Standard | Instant | 18 | 31 | folded rectangular spatial ribbons | blink through folded local space | spell_blink_fold | aetheria:blink_fold | Core cast intent + 3D client VFX/SFX; no world effect |
| 144 | Astral Mage | Moonlit Veil | Standard | Instant | 22 | 39 | curved silver phase sheets | bend light to hide caster | spell_moonlit_veil | aetheria:moonlit_veil | Design only |
| 145 | Astral Mage | Rift Slice | Standard | Charged | 26 | 47 | broken crescent rift | short jagged dimensional slash | spell_rift_slice | aetheria:rift_slice | Design only |
| 146 | Astral Mage | Comet Pearl | Standard | Channeled | 30 | 55 | many faceted comet sphere | falling weighted star orb | spell_comet_pearl | aetheria:comet_pearl | Design only |
| 147 | Astral Mage | Astral Compass | Standard | Instant | 34 | 63 | rotating star compass | mark position visible through walls | spell_astral_compass | aetheria:astral_compass | Design only |
| 148 | Astral Mage | Weightless Field | Standard | Combo | 10 | 71 | spherical floating feather field | reduce gravity in fixed area | spell_weightless_field | aetheria:weightless_field | Design only |
| 149 | Astral Mage | Nightglass Mirror | Standard | Charged | 14 | 79 | black mirror prism | reflect one ranged attack | spell_nightglass_mirror | aetheria:nightglass_mirror | Design only |
| 150 | Astral Mage | Orbiting Moons | Standard | Instant | 18 | 15 | stone orbiting moonlets | three miniature moons protect | spell_orbiting_moons | aetheria:orbiting_moons | Design only |
| 151 | Astral Mage | Constellation Snare | Standard | Channeled | 22 | 23 | multi-node constellation chart | link enemy positions into star cage | spell_constellation_snare | aetheria:constellation_snare | Design only |
| 152 | Astral Mage | Starlight Gate | Standard | Instant | 26 | 31 | shimmering dimensional arch | temporary short range portal | spell_starlight_gate | aetheria:starlight_gate | Design only |
| 153 | Astral Mage | Void Spiral | Standard | Charged | 30 | 39 | dark ribbon vortex | capture projectiles in small gravity well | spell_void_spiral | aetheria:void_spiral | Design only |
| 154 | Astral Mage | Falling Constellation | Standard | Instant | 34 | 47 | luminous falling star tessellation | scatter stellar fragments over area | spell_falling_constellation | aetheria:falling_constellation | Design only |
| 155 | Astral Mage | Nebula Mantle | Awakening | Combo | 55 | 300 | transparent star filled mantle | nebula strengthens allies | spell_nebula_mantle | aetheria:nebula_mantle | Design only |
| 156 | Astral Mage | Gravity Crown | Awakening | Channeled | 55 | 300 | tilted gravity halos | force enemies to orbit center | spell_gravity_crown | aetheria:gravity_crown | Design only |
| 157 | Astral Mage | Celestial Echo | Awakening | Charged | 55 | 300 | floating spectral mirror glyph | copy one cast from positioned astral echo | spell_celestial_echo | aetheria:celestial_echo | Design only |
| 158 | Astral Mage | Lunar Eclipse | Awakening | Instant | 55 | 300 | occluding silver moon | dim zone and amplify selected magic | spell_lunar_eclipse | aetheria:lunar_eclipse | Design only |
| 159 | Astral Mage | Starfall Liturgy | Ultimate | Instant | 95 | 1200 | multi-ring starfall constellation | sequence of giant falling stars | spell_starfall_liturgy | aetheria:starfall_liturgy | Design only |
| 160 | Astral Mage | Hollow Sky | Ultimate | Instant | 95 | 1200 | giant broken moon gateway | vast dark portal pulls hostiles | spell_hollow_sky | aetheria:hollow_sky | Design only |
| 161 | Chronomancer | Second Hand | Standard | Instant | 10 | 15 | rotating brass clock hand | pause a projectile briefly | spell_second_hand | aetheria:second_hand | Core cast intent + 3D client VFX/SFX; no world effect |
| 162 | Chronomancer | Moment Slip | Standard | Instant | 14 | 23 | curving faded movement trail | shift back to prior safe position | spell_moment_slip | aetheria:moment_slip | Core cast intent + 3D client VFX/SFX; no world effect |
| 163 | Chronomancer | Tick Pulse | Standard | Instant | 18 | 31 | nested translucent watch gears | slow enemy motion within circle | spell_tick_pulse | aetheria:tick_pulse | Core cast intent + 3D client VFX/SFX; no world effect |
| 164 | Chronomancer | Sands of Delay | Standard | Instant | 22 | 39 | hovering inverted sandglass | sand cloud slows enemies | spell_sands_of_delay | aetheria:sands_of_delay | Design only |
| 165 | Chronomancer | Temporal Mark | Standard | Charged | 26 | 47 | hourglass target rune | record and repeat damage after delay | spell_temporal_mark | aetheria:temporal_mark | Design only |
| 166 | Chronomancer | Echo Step | Standard | Channeled | 30 | 55 | offset footstep echoes | replay last dash at reduced range | spell_echo_step | aetheria:echo_step | Design only |
| 167 | Chronomancer | Aging Touch | Standard | Instant | 34 | 63 | cracked sundial palm | apply weakening degeneration | spell_aging_touch | aetheria:aging_touch | Design only |
| 168 | Chronomancer | Stolen Breath | Standard | Combo | 10 | 71 | fractured stopped second ring | interrupt hostile action | spell_stolen_breath | aetheria:stolen_breath | Design only |
| 169 | Chronomancer | Clockwork Ward | Standard | Charged | 14 | 79 | gearwork shield | timed projectile deflection | spell_clockwork_ward | aetheria:clockwork_ward | Design only |
| 170 | Chronomancer | Rewind Stitch | Standard | Instant | 18 | 15 | backward flowing gold threads | restore recent minor damage | spell_rewind_stitch | aetheria:rewind_stitch | Design only |
| 171 | Chronomancer | Timeglass Snare | Standard | Channeled | 22 | 23 | suspended hourglass shards | freeze arrows within cone | spell_timeglass_snare | aetheria:timeglass_snare | Design only |
| 172 | Chronomancer | Future Flicker | Standard | Instant | 26 | 31 | translucent offset silhouette | summon decoy at future position | spell_future_flicker | aetheria:future_flicker | Design only |
| 173 | Chronomancer | Pendulum Edge | Standard | Charged | 30 | 39 | paired swinging pendulum blades | slash twice at different moments | spell_pendulum_edge | aetheria:pendulum_edge | Design only |
| 174 | Chronomancer | Chronal Bloom | Standard | Instant | 34 | 47 | concentric clock petals | cycle shield through phases | spell_chronal_bloom | aetheria:chronal_bloom | Design only |
| 175 | Chronomancer | Hour Guardian | Awakening | Combo | 55 | 300 | articulated floating timekeeper | guardian delays approaching attacks | spell_hour_guardian | aetheria:hour_guardian | Design only |
| 176 | Chronomancer | Stasis Lens | Awakening | Channeled | 55 | 300 | layered stopped-time lens | local sphere suspends missiles | spell_stasis_lens | aetheria:stasis_lens | Design only |
| 177 | Chronomancer | Entropy Ribbon | Awakening | Charged | 55 | 300 | unraveling chronology thread | decay hostile enchantment durations | spell_entropy_ribbon | aetheria:entropy_ribbon | Design only |
| 178 | Chronomancer | Reprise Chorus | Awakening | Instant | 55 | 300 | rotating constellation dials | replay prior stored strikes | spell_reprise_chorus | aetheria:reprise_chorus | Design only |
| 179 | Chronomancer | Thirteenth Hour | Ultimate | Instant | 95 | 1200 | colossal cathedral clock | stop nearby enemy movement while casting | spell_thirteenth_hour | aetheria:thirteenth_hour | Design only |
| 180 | Chronomancer | Epoch Reversal | Ultimate | Instant | 95 | 1200 | inverted timeglass waterfall | rewind tracked temporary world changes | spell_epoch_reversal | aetheria:epoch_reversal | Design only |
| 181 | Necromancer | Bone Thorn | Standard | Instant | 10 | 15 | twisted vertebral spear | bone spear erupts below target | spell_bone_thorn | aetheria:bone_thorn | Core cast intent + 3D client VFX/SFX; no world effect |
| 182 | Necromancer | Soul Lantern | Standard | Instant | 14 | 23 | floating skull lantern | reveal hidden hostile life signatures | spell_soul_lantern | aetheria:soul_lantern | Core cast intent + 3D client VFX/SFX; no world effect |
| 183 | Necromancer | Grave Whisper | Standard | Instant | 18 | 31 | hollow skull wind glyph | lure undead to grave mark | spell_grave_whisper | aetheria:grave_whisper | Core cast intent + 3D client VFX/SFX; no world effect |
| 184 | Necromancer | Blood Thread | Standard | Instant | 22 | 39 | braided crimson cord | siphon limited health from hostile | spell_blood_thread | aetheria:blood_thread | Design only |
| 185 | Necromancer | Rib Cage | Standard | Charged | 26 | 47 | interlocking giant ribs | raise temporary skeleton cover | spell_rib_cage | aetheria:rib_cage | Design only |
| 186 | Necromancer | Wraith Step | Standard | Channeled | 30 | 55 | ghost footstep trail | travel through a safe narrow gap | spell_wraith_step | aetheria:wraith_step | Design only |
| 187 | Necromancer | Carrion Swarm | Standard | Instant | 34 | 63 | spirit moth swarm | ghost moths distract foes | spell_carrion_swarm | aetheria:carrion_swarm | Design only |
| 188 | Necromancer | Marrow Shard | Standard | Combo | 10 | 71 | splintering bone dagger | bone fragments scatter on hit | spell_marrow_shard | aetheria:marrow_shard | Design only |
| 189 | Necromancer | Epitaph Mark | Standard | Charged | 14 | 79 | carved gravestone glyph | echo a weakened target's hurt | spell_epitaph_mark | aetheria:epitaph_mark | Design only |
| 190 | Necromancer | Crimson Pool | Standard | Instant | 18 | 15 | red mirrored ritual pool | blood ritual slows entrants | spell_crimson_pool | aetheria:crimson_pool | Design only |
| 191 | Necromancer | Spirit Bell | Standard | Channeled | 22 | 23 | vibrating spectral bell | repel hostile summoned creatures | spell_spirit_bell | aetheria:spirit_bell | Design only |
| 192 | Necromancer | Spectral Scythe | Standard | Instant | 26 | 31 | transparent crescent scythe | curving soul blade travels over ground | spell_spectral_scythe | aetheria:spectral_scythe | Design only |
| 193 | Necromancer | Gravetide | Standard | Charged | 30 | 39 | creeping spirit tide | rolling mist ghosts push foes | spell_gravetide | aetheria:gravetide | Design only |
| 194 | Necromancer | Bone Palisade | Standard | Instant | 34 | 47 | asymmetric vertebral wall | staggered temporary bone barricades | spell_bone_palisade | aetheria:bone_palisade | Design only |
| 195 | Necromancer | Grim Shepherd | Awakening | Combo | 55 | 300 | armored skeleton guardian | summon loyal skeletal defender | spell_grim_shepherd | aetheria:grim_shepherd | Design only |
| 196 | Necromancer | Soul Exchange | Awakening | Channeled | 55 | 300 | connected soul vessels | trade own health for ally restoration with limits | spell_soul_exchange | aetheria:soul_exchange | Design only |
| 197 | Necromancer | Crimson Aegis | Awakening | Charged | 55 | 300 | scarlet overlapping rune shields | convert collected marks into defense | spell_crimson_aegis | aetheria:crimson_aegis | Design only |
| 198 | Necromancer | Eidolon Procession | Awakening | Instant | 55 | 300 | marching ghost knights | call spectral champions in a line | spell_eidolon_procession | aetheria:eidolon_procession | Design only |
| 199 | Necromancer | Kingdom of Ash | Ultimate | Instant | 95 | 1200 | colossal shadow cemetery arches | temporary mausoleum reaches with hands | spell_kingdom_of_ash | aetheria:kingdom_of_ash | Design only |
| 200 | Necromancer | Last Requiem | Ultimate | Instant | 95 | 1200 | towering ceremonial ghost giant | giant spirit shields allies and strikes enemies | spell_last_requiem | aetheria:last_requiem | Design only |
