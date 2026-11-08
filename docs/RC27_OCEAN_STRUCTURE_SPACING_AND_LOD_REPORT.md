# Thalvoryn RC27 — Horizon Rust 26.3 / Nether 1024

**Soukromý experimentální test; ZIP prošel statickými testy, nikoliv testem přímo v běžícím Horizonu.** Návaznost: RC26 → RC27, [uživatelská hlášení #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2). Nejde o stabilní release.

## Uživatelský test RC26: 8. 11. 2026

Uživatel **otestoval Overworld** v RC26 (Nether ani End v tomto testu nezkoušel) a poslal pět screenshotů s těmito příznaky:

| Soubor v konverzaci | XYZ | Pozorování |
|---|---|---|
| `image(20261108-141434).png` | 775, 68, 678 | Velká pobřežní/oceánská stavba a loď přímo vedle sebe, podklad a ponoření vypadají nevhodně |
| `image(20261108-141447).png` | 942, 146, 707 | Mnoho lodí na poměrně krátké vzdálenosti |
| `image(20261108-141630).png` | 534, 138, 4694 | Velký farmářský ostrov v oceánu se špatným ukotvením vůči hladině |
| `image(20261108-141726).png` | 1458, 98, 6164 | Velká houbová ostrovní stavba na pobřeží, velmi nepřirozené umístění |
| `image(20261108-141819).png` | 2057, 633, 15086 | Z dálky jasně bílá oblast / nesprávně vypadající vzdálený biome, mění se při přiblížení |

**Upozornění:** Přiložené PNG nejsou kvůli omezení GitHub konektoru nahrané jako binární soubory do tohoto repozitáře. Zůstávají ve stejné ChatGPT konverzaci. Souřadnice jsou opsané přímo z uživatelských screenshotů. Tento záznam není důkaz, že šlo o čerstvě vygenerované chunky; předchozí chunky se samotnou změnou datapacku nepřegenerují.

## Zjištěná příčina nadměrného počtu lodí

RC26 již měl poměrně vzácnou vlastní skupinu `thalvoryn:overworld/imported/maritime_rc23` (spacing 104, separation 45, frequency 0.27), ale **navíc aktivní TREK `overworld/medium` obsahoval `ship_pillager`, `offshore_platform_pillager`, `ruine_bateaux`** s vlastním nezávislým náhodným rozmisťováním (spacing 29, frequency 1). Také další nezávislá skupina `thalvoryn:overworld/handmade/air_vessel` mohla generovat létající loď nad pobřežím. Takže oprava pouze importované skupiny nemohla odstranit všechny lodě.

Další TREK stavby `mushroom_island`, `cottage_island`, `sleepy_island`, `tower_island`, `villager_island` používají v Oceánu umístění závislé na jednom bodu a některé mají záporné startovní Y offsety (např. `mushroom_island` −13). `farm_island` a `floating_farm_large` nemají bezpečný test podkladu pod celým půdorysem.

## RC27: skutečné provedené změny

1. **TREK `overworld/medium`:** vyřazeny tři lodní/pobřežní struktury z aktivní náhodné skupiny (NBTo a jejich definice zachovány).
2. **TREK `overworld/rare`:** pozastaven přirozený spawn pěti nevhodně ukotvených oceánských ostrovních staveb.
3. **TREK `overworld/very_rare`:** pozastaveny dvě problematické farmářské ostrovní stavby.
4. **Thalvoryn `air_vessel`:** odstraněna pouze samostatná náhodná skupina; definice, pool i šablona zůstávají v ZIPu.
5. **Importované oceánské lodě:** stále 16 menších variant v **jediné společné skupině**, rozestupy nově **144 chunků**, separation **62**, frequency **0.14**. Povolené pouze `deep_ocean`, `deep_cold_ocean`, `deep_lukewarm_ocean`; Y65 z RC26 neměněno.
6. **Nepřepsaná geometrie:** všech 1 102 NBT šablon (včetně lootu) ponecháno byte-identicky. Neprovedeny žádné změny Netheru, Endu, world heights ani shaderu Photon.

### Co RC27 NESLIBUJE

- Zaručeně správnou hloubku vody po celé lodi / vhodnost ostrova pod celým půdorysem. Vanilla jigsaw biome filtr kontroluje start, ne celý půdorys; pro absolutní řešení je nutný nativní engine test (kapalina/svah/kolize). Rizikové stavby jsou proto **vypnuté**, nikoli opravené.
- Nemění již vygenerované chunky; testovat **nový svět jen s RC27**.
- Nedotýká se vizuálního problému LOD. Podle [Horizon graphics dokumentace](https://github.com/horizon-rust/horizon/blob/main/docs/graphics.md) vzdálený terén používá zjednodušený sampling, jeho svahy/barvy/dekorace se mohou lišit a dosud neprozkoumané struktury v přímém vzdáleném samplingu nejsou syntetizovány. Rozdíl, který se opraví po přiblížení, je tudíž pravděpodobně omezení LOD renderingu nebo sampling pipeline, ne chyba umisťování struktur v datapacku. **Bílá oblast z obrázku však nemá potvrzenou přesnou příčinu.** Doporučené srovnání: Full Detail Distance 12, LOD Distance 256, následně test s LOD Off, stejná souřadnice a seed.

## Statická validace

- ZIP CRC: PASS; **2 712** souborů, **49** aktivních structure setů, **298** structure definition JSONů, **563** template pool JSONů, **1 102** NBT šablon.
- Všechna JSON metadata/registry parsed bez chyby.
- Všechny aktivní sety jsou neprázdné, `0 ≤ separation < spacing`, žádná duplicita salts.
- 0 chybějících *vlastních* odkazů `set → structure → pool → NBT`.
- NBT data: **byte-identická s RC26**.
- Archiv: `Thalvoryn_Horizon_Rust_RC27_Nether1024_OCEAN_SPACING_LOD_TEST.zip`.
- Velikost: **8 908 870 B**; SHA-256: `ff8939b6b2f7792f1f86ffb92c825faf78f1491c206f8472fb778b2ab1b1e804`.

**Otevřeno nadále:** renderer LOD, plná kontrola vody/lávy a svahů ve vlastním enginu, bezpečný Nether portál, skutečná výška načteného světa, vlastní F3 biome registry, úplné sladění vegetace/staveb a Photon nativní kompatibilita. Binární ZIP není veřejně vydaný kvůli neověřeným licencím externích mapových staveb. 
