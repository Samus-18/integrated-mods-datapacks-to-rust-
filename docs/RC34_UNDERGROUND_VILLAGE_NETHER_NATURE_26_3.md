# RC34 — Podzemní vesnice, odměny za průzkum a přirozenější Nether

**Datum: 8. 11. 2026 · Horizon Rust 26.3 · testovací privátní ZIP, pouze statická kontrola.**

Základ: `Thalvoryn_Horizon_Rust_RC33_CAVES_Nether1024_PRIVATE_TEST.zip`. Uživatel doplnil podzemní vesnice s **klikatou cestou ze zemského povrchu až dolů**, odměny za průzkum do hlubin i do hor, a část 4 původních požadavků: „🔥 Nether – Více přírodních variant, vlastní vegetace, různé styly krajiny, lávové lodě, více struktur a lepší rozložení rud.“ Vždy požaduje **kontrolu před i po** a správné umisťování staveb.

## Provedené změny

### Podzemní vesnice
- Přidána **jedna originální NBT podzemní osada** `thalvoryn:underground/spiral_hamlet`, půdorys **29×29**, výška **70 bloků**.
- **Propojený 71úsekový sestup**, souřadnicově validovaný: z výšky lokální Y65 až na Y4, rozdíl 61 bloků; každé dva sousední body se liší o jeden horizontální blok a max. jeden výškový blok. U každé schodové pozice je v NBT zaručený prostor pro hlavu. Trasa **zatáčí** po stěnách, není to svislá díra.
- Ve spodní síni jsou **čtyři malé domy, veřejné nádvoří, zdroj světla a čtyři skromné rudné nálezy**, NBT zahrnuje **tři entity minecraft:villager**. Zda Horizon skutečně spawnuje entitní NBT z jigsaw šablon musí potvrdit hra.
- Umisťování: **pouze plains/sunflower_plains**, `WORLD_SURFACE_WG` + offset **−65**, vzácná sdílená mřížka **spacing 92, separation 42, frequency 0.18**. Bez kontroly celého půdorysu není jisté, že vstup nebude částečně zasypaný terénem; **toto je předmět testu**, ne vyřešený problém.
- Příkaz k hledání: `/locate structure thalvoryn:underground/spiral_hamlet`. Fungování příkazu v konkrétní Nightly nebylo ověřené.

### Prozkoumat výšiny a hloubiny
- `thalvoryn:rewards/summit_emerald` – vzácné malé rudné pokusy pouze v `stony_peaks`, Y230–1250, jen native ore feature.
- `thalvoryn:rewards/cave_diamond_trace` – velmi vzácný malý podzemní diamantový nález ve `dripstone_caves`, Y−52 až −25. Nepřidává přehnaný loot ani dekorativní barely.
- Obě funkce používají pouze původní Minecraft bloky a jsou biome-filtered.

### Nether 1 024 beze změny výšky
- Přidáno **5 kompletních biome override JSONů pro existující vanilla 26.3 ID**: nether_wastes, crimson_forest, warped_forest, basalt_deltas a soul_sand_valley. Základní seznamy vanilla features, ambient efektů, mobů, barvy mlhy a entity spawn weights jsou převzaté z [Minecraft 26.3 assets](https://github.com/InventivetalentDev/minecraft-assets/tree/26.3/data/minecraft/worldgen/biome); k nim jsou přidány nové feature. **Nová registrovaná ID biomů nevznikla.**
- **7 malých biotopově omezených přírodních porostů:** ember mushrooms (nether wastes), crimson roots a fungus, warped roots a sprouts, ash fungus (basalt deltas), soul fungi (soul sand valley). Pro všechny platí **air v cíli + správná půda pod nimi**, scan ke skutečné pevné podlaze v max. 22 blocích a Y38–890.
- **5 velmi vzácných lokálních povrchových palet** pomocí stávajících 26.3 material rules (magma ve wastes/crimson, blackstone ve warped, smooth basalt v deltě, vzácné bone blocks v soul valley). Jde o selektivní noise-floor akcenty, nikoli o přidání celých nových biomů. Bedrock pravidlo zůstává první.
- **2 nové malé podpovrchové ruiny:** cinder_reliquary a basalt_cellar, **11×8×11**; použity vzácné fossil placed features pouze v odpovídajících biomech. Mohou být částečně zasypané, ale to odpovídá jejich zamýšlenému typu *buried cache*. Několik rudných bloků uvnitř, žádné dekorativní kontejnery s chybným lootem.
- Dvě rizikové velké netherové vysuté ruiny byly výrazně zředěny, stejně jako čtyři tematické watch/camp struktury – žádné nové překrývající se nezávislé structure-set mřížky.
- **Křemen:** 32→44 pokusů, výška Y10..984. **Nether zlato:** 18→22, Y10..984. **Ancient debris:** large chance 1/20 a rozsah 300..900, small chance 1/39 a rozsah 6..430; navíc velmi vzácná malá vysoká vrstva Y760..1004 (1/85 pokusů/chunk), zapojená do všech 5 vanilla nether biomů. Pokusy ≠ garantované žíly, jsou omezené dostupným netherrackem.
- **2 malé ohnivzdorné lodní NBT šablony** `thalvoryn:nether/lava_skiff_crimson` a `lava_skiff_warped` z crimson/warped planks/fences/stairs a blackstone; žádná wool/oak ani jiné běžné hořlavé plachty. **ZÁMĚRNĚ SE NEGENERUJÍ NÁHODNĚ**, dokud Horizon nepodporuje ověření celé lodní plochy nad lávou; jinak by vznikly opět na pevnině a ve vzduchu. Jsou to šablony pro ruční/engine-placement.
- Nether height **min_y=0,height=1024** (Y0 až 1023), bez stropu, se zachovaným `dimension_type` i `noise_settings`. Overworld −64/2096, End 0/512 nezměněny.

## Povinné porovnání RC33 → RC34

| Kontrola | RC33 (před) | RC34 (po) |
|---|---:|---:|
| Počet souborů | 2 837 | **2 872** |
| JSON a pack metadata | 1 575 | **1 604** |
| NBT šablony | 1 169 | **1 174** |
| Změněné existující NBT | — | **0**, všechny původní bajtově identické |
| Nové NBT | — | **5** (vesnice, 2 lávové lodě, 2 netherové ruiny) |
| Nové native Nether biome override | 0 | **5** |
| Nové Nether vegetační feature | 0 | **7** |
| Feature-order DAG uzly | — | **221**, bez cyklu |

**Výsledek statické kontroly: PASS** – oba ZIPy projdou CRC, všechna JSON data jsou parsovatelná, všech pět nových NBT úplně dekódováno nezávislým Big-Endian NBT parserem (celé palety, entity, souřadnice, validní indexy, žádné duplicitní blokové pozice), reference našich struktur/poolů/features existují, structure-set spacing/separation/frequency validní, žádný cyklus pořadí feature, všech **1 169 původních NBT beze změny**, stejně jako `pack.png`. Výškové limity zachovány.

**Soukromý ZIP**: `Thalvoryn_Horizon_Rust_RC34_NETHER1024_UNDERGROUND_VILLAGE_NATURE_TEST.zip` – **9 051 469 B**  
**SHA-256**: `b4a7ac64b5a26c7e8ca09ef71fc920155e918c3b31e70a1170871f599e7f066d`.

## Stále nedokončeno / neslibovat

1. **Horizon runtime test nebyl proveden.** Staticky korektní NBT nebo worldgen JSON samy o sobě nezaručují in-game přístupnost vesnice, funkční entity nebo výslednou hustotu vegetace. Testovat pouze v novém světě s RC34 a skutečným logem.
2. **Nová F3 biome ID** a původní custom stromy/bloky/mobové z dalších částí stále mohou vyžadovat registraci v Rust enginu.
3. **Nether lávové lodě** dosud nemají bezpečnou automatickou umisťovací podmínku. V MC26.3 existuje `volume_match` block predicate, ale samotné jigsaw structure-set neaplikuje plnohodnotnou kapalinnou kontrolu, a vanilla `fossil` placement posouvá pozici pod povrch; proto nepředstírat, že lodě na lávě nyní plují automaticky. Potřebný je engine patch.
4. Hlavní Nether terrain noise, safe portal arrival, vzdálené LOD, collision footprint a Photon nejsou tímto patchem hotové.
5. Licence externích předložených mapových staveb neověřené; ZIP nebyl publikován veřejně na GitHub Release.

[Uživatelské požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Známé chyby #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

**Datové zdroje pro kompatibilitu:** [Oficiální Minecraft Java Edition 26.3](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3) a [Horizon worldgen](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md).
