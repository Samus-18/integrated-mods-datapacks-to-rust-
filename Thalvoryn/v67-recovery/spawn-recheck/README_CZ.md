# Thalvoryn V67 – důkladnější kontrola generování (12. 11. 2026)

**Stav: TEST BUILD, pouze statická kontrola, nikoli otestování v běžícím Minecraftu 26.3 / Horizonu.** Navazuje na V67 LOOT-RECHECK; žádná NBT šablona, samotný loot ani terénový noise se při této úpravě neměnil.

## Potvrzené příčiny a opravy

1. **Dvě staré malé 13×39×13 vstupní šablony** (`thalvoryn:entrances/highland_stone_descent`, `.../mossy_forest_descent`) se spouštěly samostatnými biome features s pravděpodobností 1/20 a 1/15 na vhodný chunk. Odstraněno jejich aktivní zařazení ve všech biomech; soubory jsou ponechané pouze jako archivní data.
2. **Čtyři další nezávislé placed features TREK monster room** v custom cave biomech se generovaly současně s řízeným `structure_set`. Jejich odkazy odstraněny z biome feature listů. Celkem s dvěma vstupy odstraněno **22 výskytů šesti feature ID** napříč definicemi biomů. Vlastní `ow_spawners` zůstává.
3. **Pět malých dekorativních fossil jeskynních místností** mělo pokus už každých 6–16 chunků, a proto byly nadměrně opakované. Pravděpodobnosti jsou nyní 1/90, 1/130, 1/110, 1/150 a 1/180. Neobsahují truhly.
4. **Šest velkých tunelů** má společný výběr `spacing=32`, `separation=25`, `frequency=0.70`. Sousední kandidátní starty ve stejné skupině tak nejsou těsně vedle sebe (typicky min. cca 416 bloků mezi starty v sousedních buňkách). Ostatní strukturální skupiny mají samostatná pravidla.
5. **Osm spawnerových struktur** ponecháno pouze ve výběru `ow_spawners`; `separation` zvýšeno z 17 na 25 chunků. **Nedotýká se** vanilla `monster_room` ani spawnerů uvnitř velkých dungeonů.
6. **Tunely v lesích:** všech šest přepnuto z `WORLD_SURFACE_WG` (zohledňuje listí) na `OCEAN_FLOOR_WG`, aby se vstup neodvozoval od vršku stromů. Biome omezení na neoceánské biomy zůstalo.
7. **32 chybných jigsaw rozsahů v Minecraftu 26.3:** při `beard_thin/beard_box` musí být horizontální max reach nejvýše 116, jinak codec odmítá definici. Staré hodnoty 127/128 opraveny na `horizontal:116`, původní vertikální dosah 127/128 zachován. Zasaženo všech šest tunelů, End Castle a některé velké importy. Toto je kritická oprava, kterou dřívější QA přehlédlo.

## Nezávislá kontrola

- **4 ze 4 ZIP PASS**, každá varianta 11 323 položek; přesně 58 upravených definic/metadata, **všech 6 835 NBT bitově beze změn**.
- Správná výška Nether 1024/2032 a `pack_format=121`, bez přibaleného Stellarity.
- Kontrola všech **533 jigsaw definic**: 0 překročení horiz./vert. limitu i limitu terénní adaptace; 0 nepovolených velikostí `size`.
- 0 aktivních odkazů na 6 starých placed feature ID v biomech, 6 aktivních velkých tunelů.
- Vanilla loď End City je zachována; nevanilla custom ships se nevrátily do aktivních End skupin.
- Loot a jeho předchozí randomizace, NBT geometrie a density funkce zůstávají přesně jako v LOOT-RECHECK.

## Co poctivě nelze prohlásit za hotové

- Žádná z variant **nebyla spuštěná přímo ve hře**. Zde je Java 21, není Java 25/Minecraft server 26.3 ani Horizon runtime.
- Propojení velkých tunelů s **náhodnými přirozenými** velkými cave systémy není garantováno; zatím končí ve vlastních komorách v NBT. Pro skutečnou záruku je potřeba specializovaná kontrola terénu/nativní generátor.
- Blending velmi velkých staveb se svahem, překrytí mezi různými `structure_set`, skutečné FPS/TPS, portály, výškové Nether rozložení a herní loot potřebují runtime test.
- Snížení horizontálního reach z 127/128 na 116 kvůli MC 26.3 znamená, že u některých **velkých jigsaw komplexů** může být menší dovolený boční rozsah; kvalitu kompletních staveb je nutné ověřit ve hře.
- Staré chunky se po změně datapacku nepřegenerují. Pro srovnání použít **zcela nový svět**, jeden ZIP, ideálně stejný seed.

## K rychlému testu

```mcfunction
/datapack list enabled
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:overworld run locate structure thalvoryn:overworld/handmade/air_vessel
/execute in minecraft:the_end run locate structure minecraft:end_city
/execute in minecraft:the_end run locate structure nova_structures:end_castle
```

Rozšířená QA je v `QA_REPORT.json`; sestavení lze opakovat pomocí `build_spawn_recheck.py` a `verify_spawn_recheck.py` (vyžadují předchozí čtyři soukromé ZIPy LOOT-RECHECK).

**Licenční omezení:** Integrované binární ZIPy obsahují uživatelem dodaná cizí data. Pouze pro soukromý test, nikoli veřejné nahrání bez prověření licencí.