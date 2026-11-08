# RC39 — Vanilla stavební pooly ve vyšších výškách, regrese před/po

**8. 11. 2026 · Horizon Rust / Minecraft Java 26.3 · soukromý TEST, jen staticky ověřeno.** Základ RC38: `Thalvoryn_Horizon_Rust_RC38_MOBS_BALANCED_NETHER1024_PRIVATE_TEST.zip`.

## Požadavek uživatele — část 9

> ⚙️ Vanilla struktury — Generování i ve větších výškách rozšířeného světa, ne pouze v původních vrstvách. Zase ověření.

**Herní test provede uživatel až po dokončení celého seznamu.** Požaduje zachování všeho z částí 1–8, opravu chyb a kontrolu před/po.

## Původní stav — co má již přirozeně povrchovou výšku

V Minecraft Java 26.3 začínají běžné vanilla vesnice a pillager outposty na `WORLD_SURFACE_WG` s offsetem 0: nejsou natvrdo omezeny na starou výšku Y320. [Oficiální 26.3 vanilla village_snowy.json](https://github.com/InventivetalentDev/minecraft-assets/blob/26.3/data/minecraft/worldgen/structure/village_snowy.json). Naopak mineshafty, kanonické strongholdy a Nether fortress používají nativní code-generated typy; přidat jim samotný `start_height` JSON by **nebyla oprava**, proto nejsou násilně přepsané.

Původní RC38 `minecraft:stronghold_big` už startuje **50–110 bloků pod lokálním povrchem** jako součást vanilla concentric stronghold setu a zůstává nezměněný. Původní Nether bastion override měl **Y90–510** při vysokém Netheru Y0–1023.

## Nové čtyři vzácné varianty z původních vanilla 26.3 poolů

| Vlastní strukturové ID v RC39 | Původní vanilla pool 26.3 | Biomy | Výškové pravidlo | Grid spacing, freq |
|---|---|---|---|---|
| `thalvoryn:vanilla_highlands/meadow_hamlet` | `minecraft:village/plains/town_centers` | Meadow | přesně `WORLD_SURFACE_WG`, offset 0 | 92 chunků; 0.23 |
| `thalvoryn:vanilla_highlands/snowy_hamlet` | `minecraft:village/snowy/town_centers` | Grove, Snowy Slopes | povrch +0 | 100 chunků; 0.22 |
| `thalvoryn:vanilla_highlands/ridge_outpost` | `minecraft:pillager_outpost/base_plates` | Stony Peaks, Windswept Gravelly Hills | povrch +0 | 116 chunků; 0.18 |
| `thalvoryn:vanilla_highlands/mountain_trial_chambers` | `minecraft:trial_chambers/chamber/end` | Jagged Peaks, Frozen Peaks | `WORLD_SURFACE_WG` −112 až −76 | 128 chunků; 0.15 |

- Všechny čtyři mají **vlastní jigsaw structure definici a jednu vyhrazenou structure-set skupinu**. Používají originální vanilla 26.3 pooly, **nejsou to nové NBT**.
- Vesnicím jsem zmenšil `size` na 3 a 2, u hlídky na 2 a u Trial Chamber na 8; cílem jsou menší stavby a méně výčnělků na svazích.
- Každá skupina má **exclusion_zone** proti vanilla `minecraft:villages`, `minecraft:pillager_outposts` nebo `minecraft:strongholds`, podle typu. Skupiny jsou vzácné a mají biomy vzájemně oddělené, ale nejsou filtrovány skutečným číselným Y.
- Trail Chambers nově používají **nativní 26.3 Pool Aliases** a `terrain_adaptation=encapsulate` z vanilla struktury. Neznamená to záruku dokončené podpory v konkrétním Horizon buildu ani jistý přístup z jeskyně.
- Původní vanilla village, stronghold, ancient city, Trial Chamber, mineshaft, End City, Nether Fortress definice ani jejich existující structure sets se **nemění**.

### Netherové Bastiony v 1k Netheru

Původní `data/minecraft/worldgen/structure/bastion_remnant.json` změněný v dřívějších RC měl rozmezí startu **Y90–510**, RC39 **Y90–704**. Jediná existující mřížka bastionů, žádná nová nezávislá spawnová mřížka, původní jigsaw pool a ostatní definice beze změny. **Startovní Y je absolutní, není automaticky podle povrchu** – ve vysoké řídké horní části Netheru může být stavba i nevhodně umístěná. Celoplošný validátor podkladu dosud není implementovaný.

## Regresní test RC38 PŘED → RC39 PO

| Metrika | RC38 před | RC39 po |
|---|---:|---:|
| Členů ZIPu | 2 947 | **2 956** |
| JSON + pack metadata | 1 673 | **1 681** |
| NBT šablon | 1 177 | **1 177**, všechna byte-identická |
| Loot tabulek | 194 | **194**, všech 194 byte-identických |
| Nové struktury / nové sety | 0 | **4 + 4**, 1 changelog |
| Změněné staré soubory | — | **2** (`bastion_remnant.json`, `pack.mcmeta`) |
| Odstraněné staré soubory | — | **0** |

**PASS, build a samostatný nezávislý kontrolní skript:**
- ZIP CRC před i po; všech **1 681** JSON/metadat validně parsováno.
- Všech **1 177** původních NBT byte-identických; žádné změny jejich geometrie, inventářů, entit, palet ani spawnů.
- Všech **194** loot tabulek byte-identických (včetně RC37 náhodných, bez pevného seedu); mob rules RC38 byte-identické.
- Všechny staré `worldgen/biome`, `worldgen/noise`, `worldgen/density_function`, `worldgen/feature`, `worldgen/placed_feature` a `worldgen/structure_set` soubory byte-identické.
- Všechna nová biome ID existují v aktuálním custom Overworld climate source (53 biome ID), pool ID přesně odpovídají vanilla 26.3 datům, nové sety ukazují na existující nové struktury, spacing/separation/frequency a exclusion sety platné, žádný nový duplicitní salt.
- Original pixel-art `pack.png` beze změny.
- Overworld **min_y −64, height 2096** (Y−64 až 2031), Nether **0,1024** (Y0 až 1023) beze stropu, End **0,512** (Y0 až 511) beze změny.
- `pack.mcmeta` nyní správně uvádí RC39; předchozí RC38 metadata omylem stále uváděla RC37.

**Soukromý ZIP:** `Thalvoryn_Horizon_Rust_RC39_VANILLA_HIGH_ALTITUDE_NETHER1024_PRIVATE_TEST.zip`  
Velikost **8 860 129 B**; SHA256 `bdf452b26cc9da000f7137c76af8c32cddde0373e19dc85a0035462b71e9779f`.

## Co NENÍ hotové

1. **Nikdo nespustil aktuální Horizon Nightly s RC39**. Statický test ≠ herní kontrola mapového generování, interiérů Trial Chamber, raidů, villager AI, lootu ani povrchových svahů.
2. Highland biomy nejsou numerickým výškovým filtrem. Stavba bude v konkrétním horním biomu, ale **není garantována určitá minimální Y** ani kompletní rovný podklad/ochrana před překryvem.
3. Žádná engine-validace terénu celé plochy: obzvlášť rozšířené Bastiony Y704 mohou být ve vzduchu. Při závěrečném testu zkontrolovat a podle výsledku zúžit výšky nebo přidat nový placement validator.
4. Vanilla native special generators (mineshaft, fortress, stronghold) nemají obecný jigsaw start-height hack a zůstávají na původních funkčních pravidlech.
5. Podpora vlastních RC37 loot tabulek v aktuálním Horizon runtime je stále neověřená. Nether lava boats, safe portal arrival, vzdálené LOD a Photon zůstávají ve [známých problémech #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
6. Nezveřejňovat celý testovací ZIP na veřejném GitHub release, protože staré dodané mapové assety mají neověřená práva pro redistribuci.

**Požadavek:** Uživatel bude herně testovat až po dokončení všech částí [seznamu požadavků](USER_REQUIREMENTS_BY_PART.md).

Technická reference: [Oficiální Minecraft Java Edition 26.3](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3), [Horizon world-generation](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md), [vanilla 26.3 structures and pools](https://github.com/InventivetalentDev/minecraft-assets/tree/26.3/data/minecraft/worldgen).
