# RC37 – dynamický loot TREK/Thalvoryn, vyčištění dekorativních barelů

**8. 11. 2026 · Horizon Rust / Java 26.3 · pouze soukromý statický test, nikoli stabilní verze.**
Předloha: `Thalvoryn_Horizon_Rust_RC36_STRUCTURE_PLACEMENT_PREPOST_PRIVATE_TEST.zip`.

## Požadavek uživatele – část 7

> 💎 Loot
>
> Dynamický loot podle stavby, biomu, dimenze a vzácnosti; žádný loot v čistě dekorativních barelech.
>
> Zase ověřování a kontrola před/po, jestli není něco rozbité.

**Uživatel chce herní testovat až po dokončení celého seznamu požadavků. RC37 proto zatím nevyžaduje test.**

## Audit RC36 PŘED změnou

Úplně dekódováno **všech 1 177 NBT** a inventáře v nich:
- **2 847 kontejnerů** (včetně 1 568 barelů, 1 251 běžných truhel, trapped/ender chests a dispenserů).
- **371** kontejnerů již mělo existující `LootTable` referenci – jejich tagy jsou v RC37 zachované.
- **1 175 běžných/trapped odměnových truhel** v mapových a TREK/Thalvoryn šablonách bylo vhodných k převedení na dynamický loot. Několik autorsky speciálních předmětů bylo zachováno, vanilla Minecraft šablony se neupravovaly.
- Významná část TREK zámků a sídel měla dlouhé řady skoro stejných předem naplněných barelů. Rozpoznání jejich dekorativní role vyžaduje úsudek; není součástí NBT anotací.

## RC36 → RC37 – skutečně provedené změny

1. **1 175 truhel převedených** z pevného seznamu `Items` na `LootTable` s náhodným obsahem při otevření (když to loader podporuje).
2. **205 zásobních barelů** dostalo střídmý dynamický skladový loot. **530 opakovaných naplněných barelů** bylo vyprázdněno a nemá žádný `LootTable`. Po úpravě je **1 068 barelů prázdných bez loot tabulky** (včetně těch prázdných už v RC36). Jeden zvláštní autorský barel byl zachován.
3. **66 nových vlastních loot tabulek** pro 26.3, pod `data/thalvoryn/loot_table/chests/rc37/`. Tabulky kombinují:
   - **Druh stavby** (vesnice/sklady, pevnosti, lodě, jeskyně, ruiny, svatyně), odvozený z názvu původní šablony.
   - **Dimenzi** (Overworld, Nether, End) s tematicky odpovídajícími přírodními materiály a zásobami.
   - **Vzácnost** (common/uncommon/rare/legendary) odvozenou zejména ze strukturální kategorie/názvu; **není to přesný přepočet efektivní pravděpodobnosti spawnu všech jigsaw struktur**.
   - **Skutečný biom** jako **volitelný bonus při otevření** pomocí `condition.type=minecraft:location_check` a `predicate.biomes`. Základní loot funguje i mimo tento biom, pokud loader umí loot tabulky.
   - Skladové barely neobsahují vzácné poklady; dražší odměny jsou samostatný nízkopravděpodobnostní pool.
4. Zachováno **všech 128 původních loot tabulek** beze změny; spolu s novými je jich 194. **Všech 371 původních LootTable odkazů zůstalo**.
5. Zcela zachovány Overworld JJThunder, Nether vysoký **1 024 bloků**, End stavební výšky **512**, původní všechna worldgen JSON a pixelart `pack.png`.

## Kontrola PŘED/PO (nezávislý validátor)

| Kontrola | RC36 před | RC37 po |
|---|---:|---:|
| Celkem souborů | 2 880 | **2 947** |
| JSON + pack metadata | 1 607 | **1 673** |
| Loot tabulek | 128 | **194** |
| NBT šablon | 1 177 | **1 177** |
| Byte-identických NBT | 1 177 vůči sobě | **746 nezměněných**, 431 změněných **jen v inventářích** |
| Nové `LootTable` reference | 0 | **1 380** |
| Všechny původní `LootTable` reference | 371 | **371 zachovaných** |
| Prázdné barely bez lootu | — | **1 068** |

**VÝSLEDEK: PASS** – CRC obou ZIP, rozbalení a úplné parsování každé NBT, JSON validní, vlastní loot-table odkazy existují, žádná kombinace nových `Items` a `LootTable`, 26.3 struktura funkcí `modifier` / `type`, kontrolní loot simulace **2 500 otevření pro každou úroveň vzácnosti**, žádná změna existujících worldgen/biome/structure-set/feature JSON.

**Nezávislé byte-by-byte porovnání:** z předchozího a nového NBT se odstraní jen inventární `Items` a `LootTable` tagy. U **všech 1 177 šablon** jsou poté celé dekomprimované soubory **bajtově identické**. Tím je prokázáno, že se při převodu nezměnily blokové pozice, stavy, geometrie, palety, všechny ostatní blokové entity ani uložené entity.

**ZIP**: `Thalvoryn_Horizon_Rust_RC37_DYNAMIC_LOOT_NETHER1024_PRIVATE_TEST.zip`  
**SHA-256**: `15e3f11009dd6a59a3a3493925a9571595b340cbc17ed3dcc13d5edeb383d01c`  
**Stav**: pouze **staticky ověřený**, ve hře Horizon Nightly **neotestovaný**.

## Otevřená omezení – NEDOKONČENO

- Horizon Rust dokumentuje **neúplnou podporu loot tabulek u běžných datapacků** (v portovaných modpackových datech je podporuje jinak). Je tedy možné, že část chests odkazujících na vlastní `thalvoryn:...` tabulky zůstane v určité Nightly prázdná. RC37 **nesmí být vydávána za prokazatelně funkční loot runtime** bez engine testu. Zdroj: [Horizon mod-browser](https://github.com/horizon-rust/horizon/blob/main/docs/mod-browser.md) a [modpacks](https://github.com/horizon-rust/horizon/blob/main/docs/modpacks.md).
- Pojmenování struktur určuje kategorii a vzácnost heuristicky. Není to ruční prověrka každého chest/merchant role a **není prokázána správnost každé jednotlivé odměny**.
- Barely se na sklady/dekoraci rozlišují deterministicky podle **hustoty v jedné šabloně**. Skutečnou vizuální úlohu všech 1 568 barelů není možné potvrdit bez herní kontroly.
- Pokud loader neumí nový vlastní `LootTable` nebo `location_check`, **nedělat z RC37 finální ZIP**. Bezpečný zpětný základ je RC36.
- Netherové lávové lodě, plná kontrola kolizí, safe portal arrival, LOD, nové biome ID ve F3 a Photon jsou nezávislé otevřené úkoly.
- Cizí mapové šablony mají neověřená distribuční práva; binární ZIP není veřejný GitHub release.

## Artefakty (v chatu, ne na veřejném GitHub Release)
- `Thalvoryn_RC37_LOOT_PRE_POST_AUDIT.json` – úplný soupis změn a příkladů
- `Thalvoryn_RC37_INDEPENDENT_VALIDATION.json` – nezávislé ověření geometrie a odkazů
- `Thalvoryn_RC37_LOOT_CATALOG_CZ.md` – kombinace typ/dimenze/vzácnost/biom
- `build_thalvoryn_rc37_loot.py` + `rc37_nbt_survey.py` + `validate_thalvoryn_rc37_independent.py` – reprodukovatelný build, parsování a kontrola

[Požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Otevřené závady #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
