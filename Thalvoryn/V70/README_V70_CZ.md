# Thalvoryn V70 — soukromá testovací verze (Minecraft 26.3 / Horizon Rust)

**Stav:** 4/4 prošly nezávislou statickou kontrolou. **Neproběhl skutečný herní test v Java 26.3 ani Horizon Rust.** Tato verze nenahrazuje důkaz funkčnosti jednotlivých struktur.

## Nové oproti V69

1. **Podzemí vs. povrch:** jedna skupina `ow_dungeons` obsahovala 15 povrchových dungeonů a jen 11 skutečně podzemních. V70 ponechává 11 podzemních v `ow_dungeons` (spacing 48 chunků, separation 19, frequency 0,47) a 15 povrchových přesouvá do `ow_surface_dungeons` (spacing 51, separation 23, frequency 0,33). Povrchová skupina navíc odmítá blízkost `ow_settlements` do 10 chunků. Jsou to šance na pokus o generování, nikoli pevné vzdálenosti hotových staveb.
2. **Oceánské lodě — pouze Rust:** 33 lodních definic má kontrolu hlubokého oceánu rozšířenou dle délky/šířky jejich vlastních NBT šablon. Radii 3–8 chunků místo původních 2. Úmyslně je pravděpodobnější odmítnutí lodi u pobřeží. Lodě mohou být vzácnější, ne přidané do Endu ani Netheru.
3. **Šest pozemních staveb — pouze Rust:** doplněna kontrola okolního sklonu v oblasti 2 chunků (výškový rozdíl maximálně 24 bloků) a mokrého startu. Vybrané struktury: `nova_structures:illager_camp`, `nova_structures:remnant_bunny_base`, `trek:overworld/medium/claw_pillager`, `trek:overworld/medium/oasis`, `trek:overworld/medium/wild_stone`, `trek:overworld/medium/small_pyramide`.
4. Všechna čtyři balení nyní obsahují označení V70 a stejné `/function thalvoryn:version` API.

**Výslovně zachováno:** všech 6 835 NBT šablon bitově shodných s V69, všechny loot tabulky, density a noise, biomy, definice dimenzí, Nether 1024/2032, End City. Ve všech čtyřech ZIPech je 478 unikátních aktivních ID struktur, bez duplicit mezi skupinami. Stellarity v nich není.

## Co stále není potvrzené / co nedělá

- Java nemá `horizon:start_checks`, takže 33 lodních ochranných pravidel a 6 terénních kontrol je jen v Rust variantách. Ani v Rustu kontrola biomů **neprokazuje**, že všechny bloky lodi budou nad vodou.
- Velké stavby stále mohou být částečně ve vzduchu, překrývat jiný obsah nebo být useknuté. `beard_thin` není univerzální terraformování.
- Šest podzemních tunelů ústí do vlastních modelovaných komor; **spojení s náhodně generovanou přirozenou jeskyní není zaručeno**.
- Počet struktur a změna filtrování může ovlivnit TPS; bez herního runtime a měření nelze tvrdit opak.
- V již vygenerovaných regionech světa se staré struktury nepředělají. Každou variantu testovat v novém světě.

## Doporučený krátký test

```
/datapack list enabled
/function thalvoryn:version
/gamemode spectator
/execute in minecraft:overworld run locate structure nova_structures:catacomb
/execute in minecraft:overworld run locate structure nova_structures:surface_trial_dungeon_desert
/execute in minecraft:overworld run locate structure thalvoryn:overworld/imported/ships_01
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:the_nether run locate structure nova_structures:nether_port
/execute in minecraft:the_end run locate structure minecraft:end_city
```

`/locate` jen najde polohu, neprovede teleport. K testům potřebujeme seed, souřadnice a případně video. **Nenačítejte společně více variant V70 nebo samostatné zdrojové datapacky, které jsou už integrované.**

## Kontrola

- `V70_INDEPENDENT_QA.json`: nezávislý ZIP diff V69→V70, CRC/JSON, přesné strukturální změny, 25 skupin, validita random spread, zákaz duplicit, kontrola Stellarity.
- `V70_CROSS_VARIANT_AUDIT.json`: hash všech NBT v každé ze čtyř variant, konkrétní rozměry lodí a přepočet kontrolních radií.
- `V70_BUILD_MANIFEST.json`: sha256 a soupis změn.
- `build_v70.py`, `verify_v70.py`, `v70_cross_audit.py`: reprodukovatelný zdroj a QA, vyžaduje původní V69 a `nbt_codec_v67.py`.

**Licence:** ZIP obsahuje převzaté herní assety z uživatelem dodaných balíčků. Nezveřejňovat kompletní binární ZIPy na veřejném GitHubu bez prověření autorských práv a licencí všech použitých zdrojů.