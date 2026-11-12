# Thalvoryn V67 — důkladnější kontrola a oprava pevného lootu

**Datum kontroly:** 12. listopadu 2026. **Stav:** staticky ověřený TEST BUILD, ne herně otestovaná finální verze.

## Co nová nezávislá kontrola skutečně našla

- Předchozí `WORLD-ROOTFIX` pořád obsahoval **3 045** neprázdných pevně uložených inventářů v NBT truhlách/barelech (1 267 truhel, 1 778 barelů), takže dřívější tvrzení o úplném vyřešení opakujícího se lootu nebylo správné.
- Z toho **2 668** inventářů bylo v nevanilla šablonách. Zbytek **377** je v šablonách namespace `minecraft` a byl záměrně zachován (některé mohou být upravené jinými původními datapacky).
- Kromě toho existuje **254** neprázdných inventářů výdejníků, dropperů, pecí, hopperů apod. — nejsou zaměňovány za loot truhly a zůstávají nezměněné.

## Co bylo opraveno

- **502 NBT šablon**, celkem **2 668** dříve pevně naplněných kontejnerů: 1 178 normálních truhel, 1 484 barelů a 6 trapped chest.
- Pevné `Items` se změnilo na odkaz na existující vhodnou `LootTable` podle dimenze a přibližného typu stavby (jeskyně / osada / hrad / oceán / Nether / End). Není zapsaný žádný pevný loot seed.
- Původně prázdné truhly/barely, pasti, mechanismy a celá složka `data/minecraft/structure/` zůstaly nedotčené.
- **Ve výsledném ZIPu už kontrola nenašla žádné neprázdné pevné inventáře truhly/barelu mimo namespace `minecraft`**. Zbývající `minecraft` záznamy se musí zkontrolovat podle účelu; nebyly bezhlavě přepsány.

## Nezávislé kontroly

- **4/4 ZIP PASS** (CRC, soubory beze změny počtu: 11 323 na variantu, pack format 121, výšky Netheru 1024/2032).
- Ve všech čtyřech variantách se změnilo přesně **502 NBT souborů**; **žádný JSON, světová generace, biome, noise, structure set, jigsaw pool, ostatní struktury, ani metadata nebyly změněny** oproti WORLD-ROOTFIX.
- Nezávislé porovnání všech 502 NBT potvrdilo **100% shodu velikosti, palet, pozic a typů bloků i ostatních entit**. Jediné rozdíly jsou v příslušných block-entity inventářích.
- Každý nově vložený `LootTable` odkaz má v datapacku skutečný soubor.
- Upravených 502 NBT souborů je bitově stejných mezi RUST/JAVA a 1024/2032 variantami.
- Data Stellarity nejsou přibalena; žádná nová Stellarity integrace neproběhla.

## Co stále NENÍ otestováno / opraveno

- **Herní spuštění** Minecraft 26.3/Horizon Rust: funkce lootu při otevření, výkon FPS/TPS, portály, render a kolize struktur nebyly testovány.
- Napojení tunelů na přirozené jeskyně, správná poloha některých lodí a bastionů, výškové rozvrstvení dalších Nether struktur a správnost všech End struktur nezískávají žádnou záruku touto úpravou (jejich worldgen zůstal beze změny).
- 377 pevně naplněných truhel/barelů v namespace `minecraft` zůstalo záměrně beze změny, stejně jako uložené chunky starého světa.
- Veřejné zveřejnění převzatých zdrojových assetů vyžaduje kontrolu licencí.

## Doporučení pro test

Použij pouze `Thalvoryn-RUST-Nether1024-V67-LOOT-RECHECK-TEST.zip` v **novém světě**. Ve dvou stejných běžně generovaných stavbách otevři několik truhel a barelů, zvlášť v TREK hradech a domech. Zkontroluj, že obsah není identický. Pak zkontroluj jiný seed/nové chunky a výkon. Starý svět už obsahuje původně vygenerované inventáře; nový ZIP je nezmění.