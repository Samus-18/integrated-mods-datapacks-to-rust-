# RC30 — Horizon Rust 26.3 / Nether 1024 / End 512

**Soukromé testovací sestavení, nikoli stabilní release.** Navazuje přímo na RC29.

## Jediná změna worldgen

`data/thalvoryn/worldgen/placed_feature/end/high_sky_islands.json`:
- `height_range` od **Y 68–132** na **Y 104–276** (využívá část rozsahu rozšířeného Endu Y 0–511)
- `rarity_filter.chance` **24 → 36** (nižší četnost pokusů, nikoli přesný počet ostrovů)

Žádné další biome/terrain/noise, structure-set ani NBT změny se neprovedly. Nejedná se o nový nativní End noise generátor, přírodní terén nad Y255 stále nemusí existovat mimo samostatně generované ostrovy.

## Výškové nastavení zachováno

| Dimenze | min_y | height | Y bloky |
|---|---:|---:|---|
| Overworld | -64 | 2096 | -64 až 2031 |
| Nether | 0 | 1024 | 0 až 1023 |
| End | 0 | 512 | 0 až 511 |

**Nether se nesmí zvětšit zpět na 2k.** Všechny jeho datové soubory jsou byte-identické s RC29.

## Ověření
- ZIP CRC **PASS**; **2715** souborů
- Parsování **1523** JSON/metadata PASS
- **1102** NBT šablon zachováno byte-identicky
- **169** Nether-souvisejících a **849** Overworld-souvisejících souborů byte-identických dle auditní klasifikace cest
- Změněny pouze dva původní soubory: příslušný End placed_feature a `pack.mcmeta`; nový `RC30_CHANGELOG_CZ.txt`
- Soubor: `Thalvoryn_Horizon_Rust_RC30_Nether1024_END512_SKY_ISLANDS_TEST.zip`
- Velikost **8 911 584 B**, SHA256 `b48f69937165edbc0ef2c679c6365f6b098c740af7b5db2bcdf3af57e627b5d3`
- **Spuštění Horizon runtime: NEPROVEDENO.** Jde o kontrolu struktury dat, ne potvrzení hry.

## Otevřené chyby

Dokončení zůstává v [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2): omezení strukturových kolizí podle celé plochy, porovnání distant LOD versus plné chunky, portal safe arrival, nové F3 biome ID, nativní Photon shader, plná End density/noise terénní integrace.

Při testu použít **nový svět s jedinou RC30**. Staré chunky se nepřegenerují. Dříve dodané mapové stavby mají neověřené licence pro veřejnou redistribuci, proto ZIP není veřejný GitHub Release.
