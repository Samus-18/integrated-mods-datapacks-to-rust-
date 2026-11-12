# Thalvoryn V68 – soukromá testovací verze

**Verze:** Minecraft 26.3 / cíl Horizon Rust nightly 2026-11-07 (`2ebf00a`).

**Stav:** 4/4 archivy prošly statickou kontrolou; generování ve skutečné hře ani TPS/FPS nebyly testovány.

## Co se změnilo vůči V67 CAVE-GEOMETRY

1. Odstraněno **47 neaktivních definic** custom lodí pro End (22) a Nether (25), včetně 47 kořenových jigsaw poolů. Uložené NBT šablony lodí zůstávají jako neaktivní zdrojová data, protože zatím nejsou všechny jejich případné návaznosti ve starých importech plně prozkoumané. **Vanilla End City a její loď zůstávají**; overworldové lodě a vzducholoď zůstávají.
2. **12 malých povrchových staveb** (7 Overworld, 4 Nether, 1 End) přepnuto na `beard_thin` adaptaci pro lepší propojení se zemí. Toto může zmírnit ostré hrany, **nezaručuje** opravu každého levitujícího domu nebo celé velké vesnice.
3. End Castle: Endermite spawn weight **400 → 35**, maximální velikost skupiny **16 → 2**. Změna omezuje potenciální zahlcení moby; skutečný dopad na TPS dosud nezměřen.
4. Aktualizovány `pack.mcmeta` a funkce `/function thalvoryn:check`, `/function thalvoryn:help`, `/function thalvoryn:horizon/status`. Nový příkaz `/function thalvoryn:version` hlásí **V68**, platformu a výšku Netheru.
5. **Záměrně beze změny:** všechny NBT šablony (6 835), loot tabulky, strukturové skupiny a rozmístění, výšková nastavení Netheru, Overworld/End worldgen a šest variant tunelů. Tím je zachována posloupnost oprav V67.

## Nezávislá kontrola

- 4/4 ZIP CRC a JSON validní (11 231 položek v každém archivu).
- 94 odstraněných kořenových lodních definic/poolů, 17 změněných souborů, 2 nové soubory.
- 478 aktivních ID struktur v registrech přibalených `structure_set` a žádné dvojí aktivní ID; netherové/endové importované lodě mezi nimi nejsou.
- Přesné porovnání NBT, lootů, density functions, dimenzí a `structure_set` proti předchozí V67: bez rozdílů.
- `data/stellarity/` v archivu není.
- Platný `pack_format: 121` a označení V68.

## Jak testovat

1. Zazálohuj původní svět. **Vytvoř nový svět** pro nový datapack, ideálně se stejným seedem pro porovnání. Staré chunky se nepřegenerují.
2. Vlož jediný ZIP pro svou platformu do datapacků nového světa; **nekombinuj Nether 1024 a 2032** v jednom světě.
3. `/function thalvoryn:version` a `/datapack list enabled`.
4. Ověř End Castle (mobové), menší pozemní stavby na svazích a Nether ruiny; při problému ulož dimenzi, seed, XYZ a video.

## Co stále není vyřešeno

- Garantované propojení každého tunelu s přirozenou velkou jeskyní.
- Bezchybné dosednutí všech **velkých** jigsaw komplexů a bezkolizní chunková geometrie v různých seedech.
- Měření TPS/FPS, portálů, Netheru 2032 a End dračího ostrova v běžícím Horizonu.
- Případné zbývající výkonové problémy pocházející z rozsáhlého worldgenu.

**Licenční poznámka:** Archivy obsahují importy z uživatelem dodaných cizích projektů. Určeno pro soukromé testování; ZIPy nezveřejňovat na veřejném GitHubu bez vyřešení licencí. Na GitHub lze dát jen build skripty a reporty.