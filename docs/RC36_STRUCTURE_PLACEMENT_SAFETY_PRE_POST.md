# RC36 — Struktury: rozestupy, biome vhodnost, mapy a loot

**Horizon Rust / Minecraft Java 26.3 · 8. 11. 2026 · soukromý testovací ZIP, pouze statické ověření; nikoliv stabilní verze.**

## Uživatelské zadání – část 6

> 🏰 Struktury
>
> Dokončit využití dodaných map, správné umístění, větší rozmanitost, zabránit překrývání a prorůstání stromů.

Uživatel bude vše testovat **až po dokončení celého seznamu**. V rámci tohoto kroku požaduje kontrolu **před/po**, výskytu/sancí/vhodnosti a lootu.

## Příčina chyb v RC35

- TREK Overworld **very_common (spacing16/freq1), common (24/freq1), medium (29/freq1)** tvořily **tři nezávislé mřížky**. I správně omezená náhodná hustota v jednotlivých mřížkách se mohla lokálně překrýt. Řada dalších jednotlivých hlídek měla samostatnou mřížku.
- Původně zaslané mapy obsahují také obrovské budovy, dlouhé lodě a pobřežní stavby. **Biome tag a jeden WORLD_SURFACE_WG start nezaručují rovný pevný podklad nebo vodu pod celým půdorysem.**
- TREK `cold_red_trade`, `small_red_trade`, `sunken_city` a `coves` používaly nezabezpečené oceánské/pobřežní umístění. Některé jiné netherové ruiny používaly pevnou náhodnou Y pozici bez testu podkladu.
- RC35 mělo **46 aktivních structure-setů**, **211 různých aktivních structure IDs** a **1177 NBT**.

### RC36: konkrétně změněno

1. **TREK Overworld common+medium+very_common** nyní používají **jednu sdílenou weighted skupinu** `trek:overworld/ecology_mix_rc36`, spacing **30**, separation **13**, frequency **0.80**, vyloučení 8 chunků kolem `minecraft:villages`. Váhy: původní common 3, medium 1, very_common 1. Ukládané definice ani NBT neodstraněny, odstraněny pouze staré nezávislé structure-set JSONy.
2. **Sedm tematických hlídek + tři outposty** z Thalvorynu nyní sdílejí jedinou skupinu `thalvoryn:overworld/frontier_mix_rc36` (spacing 74 / sep 29 / frequency 0.43), která vylučuje blízkost vesnic. Biome podmínky jednotlivých definic zachované.
3. Z dodaných importovaných map byly pro opatrný natural spawn znovu vybrány **3 menší budovy**:
   - `thalvoryn:overworld/imported/buildings_20` (17×22×30), plains/meadow/savanna
   - `.../buildings_26` (17×33×19), forest/taiga/old-growth pine
   - `.../buildings_29` (13×32×11), plains/meadow
   Všechny mají start `WORLD_SURFACE_WG` + Y0, `terrain_adaptation=beard_thin`; jediný vzácný structure set spacing **115**, sep **48**, freq **0.26**, exclude villages 10 chunků. **Beard thin není úplná kontrola svahu**; nevhodně rozlehlé importované budovy dál zůstávají manual-only.
4. Z 16 dosavadních hlubokooceánských lodí zůstává ve výběru **8 užších šablon**, spacing **192**, sep **80**, freq **0.085**, jen `deep_ocean/deep_cold_ocean/deep_lukewarm_ocean`, start base **Y68** vzhledem k sea level **70**. Původní lodě měří cca **46–64 bloků délky**; kontrola pouze startovního biomu **nezaručuje**, že trup nikde nenarazí na pobřeží. Tato skutečnost zůstává otevřená.
5. **Pět skupin stromových NBT grove** dostalo exclusion zone 5 chunků vůči hlavnímu TREK setu. To může omezit kolize *těchto skupin*, **nikoliv všech vanilla stromů**, jiných structures, feature vegetace nebo jigsaw světových úprav.
6. **Nether:** dvě staré generické `trek:nether/medium/overworld_leak_*` struktury s pevnou Y64–340 už se náhodně nespouštějí; nether pillar je vzácnější (spacing77 / sep33 / freq.22). Nether height **1024 zůstala nezměněná**.
7. **End:** imported airship set vzácnější (spacing140 / sep57 / freq.13); žádná editace jeho NBT, nového terénu Endu ani dalších biome features.
8. **Loot:** všech **1177 NBT byte-identických s RC35**. Nezávisle ověřeny NBT pozice a 11 dostupných importovaných map (3 nové budovy + 8 aktivních lodí), zkontrolované chest item sloty. Žádná z těchto 11 šablon nemá veškerý loot prázdný. **Nebyly však kompletně přehodnoceny všechny staré truhly v ostatních NBT**; plné inv-scan překročilo časový limit a není prohlašováno za dokončené.

### Audit před/po

| Kontrola | RC35 | RC36 |
|---|---:|---:|
| Členů ZIP | 2891 | **2880** |
| JSON + pack metadata | 1619 | **1607** |
| NBT struktury | 1177 | **1177**, všechny bitově stejné |
| Aktivní structure sety | 46 | **34** |
| Aktivní unikátní ID struktur | 211 | **200** |
| Změněných existujících souborů | – | **21** |
| Přidané soubory | – | **4** |
| Odstraněné staré structure-set JSONy | – | **15** |

**PASS (statické):** před/po ZIP CRC, 1607 JSON a pack metadata dekódovatelných, vlastní set→structure→pool→NBT odkazy, unikátní soli dle typu placementu, exclusion-zone odkazy existují a nezacyklí se, platné spacing/separation/frequency, platné sloty pro 11 přímo zkoumaných aktivních NBT, 1177 původních NBT zachováno, pixel-art `pack.png` beze změny; přesně zachována data terénu/biomů/features i build height pro **Overworld −64…2031, Nether 0…1023, End 0…511**.

**Soukromý ZIP:** `Thalvoryn_Horizon_Rust_RC36_STRUCTURE_PLACEMENT_PREPOST_PRIVATE_TEST.zip`  
**Velikost:** 9 057 776 B  
**SHA-256:** `324245726a5a03529750df544f7a42a3c7c870d8a9df48c625331a45ed2ecefb`.

**Manifest importovaných šablon** `Thalvoryn_RC36_STRUCTURES_CATALOG_CZ.md` v chatu: celkem **103 NBT** importovaných map, z toho **19** ve výběru pro přirozené generování a **84** uchováno pro ruční použití nebo bezpečné pozdější zprovoznění. **Zachování NBT neznamená dokončené využití všech dodaných map.**

## Příští práce, kterou nelze vydávat za hotovou

- **Full-footprint validation**: lodě potřebují kontrolu přítomnosti a hloubky vody/lávy pod celým trupem a bezprostředního terénu, budovy slope/ground/cliff checks a kolizní zóny. Exclusion-zone na úrovni mřížek neumí vyřešit každý překryv.
- **Native Horizon support / runtime checks**: v této konverzaci nebyl spuštěn konkrétní Horizon Nightly, takže žádné in-game umístění, výkonnost, LOD nebo spawn mobů nejsou potvrzené.
- **Všechny staré kontejnery / tematický loot / dekorativní barely** zatím nejsou plně přehodnocené; v RC36 pouze nedošlo ke změnám. Obsah z předchozích oprav může vyžadovat samostatnou revizi.
- **Nether lávové lodě** zůstávají vypnuté, dokud je nelze bezpečně generovat na skutečné lávě.
- Staré chunky z RC35 se změnou datapacku nepřegenerují, ale uživatel bude testovat až dokončený celý seznam.
- Autorská práva k dodaným mapovým assetům nejsou potvrzená; **nejde o veřejný GitHub Release**.

Příslušné metody: [Minecraft 26.3 structure placement](https://minecraftdocs.dev/systems/worldgen/structure-placement) a [Horizon worldgen](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md).

[Požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Seznam závad #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
