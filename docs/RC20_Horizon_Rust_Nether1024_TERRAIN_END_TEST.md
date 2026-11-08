# Thalvoryn RC20 — Horizon Rust 26.3 (Nether 1024)

**Experimental private test build, static checks only.** Built directly from RC19, with its original NBT building templates retained. **No in-game Horizon runtime test or renderer validation was performed.**

## RC19 → RC20

1. Reordered the Nether 26.3 material rule so the bedrock-floor gradient precedes optional biome-related surface colours. The floor definition remains unchanged.
2. Tuned the existing Nether `nether_surface` density function's noise spatial scales: broad gravel noise from xz_scale 0.8 / weight 0.10 to 0.018 / 0.42; jagged noise from 1000 / 0.02 to 0.16 / 0.11. Added one mild upper-density variation term to `nether_final_density`, with an altitude-dependent amplitude gradient Y300–790. These are **parameter changes**, not a demonstrated solution to terrain cutoffs in the running engine.
3. Increased the weighted 14-variant End island/ruin set frequency via spacing 17→14, separation 6→5.
4. Increased vanilla End-city placement: spacing 40→32; separation 16→11; frequency 0.59→0.72.
5. End small-island placed feature rarity 45→24 (placement attempts rather than guaranteed islands).
6. Inspected manageable existing NBT inventories for missing / weak loot. **No qualifying underfilled custom TREK / Thalvoryn inventories were found or rewritten in this pass.** RC18 prefilled inventory logic remains unchanged; no new claim of biome-sensitive live loot.

## Static validation

- File `Thalvoryn_Horizon_Rust_RC20_Nether1024_BIOME_STRUCTURE_LOOT_TEST.zip`
- Size: **8,889,022 bytes**
- SHA-256: `dcf19cc3c81a9c964e2c89cd021f6b104aaf84928dde0bb0e48213bdbde21a5b`
- ZIP CRC: passed
- JSON and pack metadata: **1,574** valid
- Structure, pool, NBT custom-resource links: **0** missing targets in static reference check
- **1,102 NBT templates retained byte-identically** from RC19; only seven existing JSON/metadata files modified and one text note added.
- Nether noise-settings and dimension-type heights both remain **1024**.

## Unresolved

- **No in-game world-generation test.** Changes to terrain silhouettes, biome visibility and structure placement are hypotheses until tested in Horizon with a **new world** and only this pack.
- No custom biome IDs/F3 biome names, no guaranteed surface portal arrival (requires supported Horizon engine behavior).
- Tree collision, pre-existing structures, old map block entities and mobs are not all validated.
- Photon shader was **not changed** in RC20; not a finished native Horizon shader port.
- Redistribution rights for user-supplied third-party map buildings have **not been verified**. This experimental binary has **not been posted as a public GitHub release**. Get author permission first.

For new tests disable RC19 and earlier Thalvoryn variants, and start a clean world; existing chunks are not regenerated.
