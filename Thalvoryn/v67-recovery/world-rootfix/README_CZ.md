# Thalvoryn V67 — oprava podle skutečného uloženého světa

**Datum:** 12. 11. 2026. **Stav:** OPRAVENÝ TESTOVACÍ DATAPACK, ne plně herně ověřená finální verze.
Cílová verze dat: Minecraft 26.3 (pack format 121) / Horizon Rust (původně testovaný commit `2ebf00a`).

## Co přesně ukázal tvůj svět `New World (7).rar`

Z archivu byl skutečně rozbalen a přečten svět včetně regionových souborů:

- **27 643 chunků** Overworldu, **2 917** Netheru a **2 664** Endu.
- Overworld měl **9 493 truhel** s pouhými dvěma stále stejnými čtyřpoložkovými inventáři (4 753 + 4 740 kusů).
- Tyto inventáře odpovídají dvěma truhlám v `data/trek/structure/monster_room/dripstone.nbt`.
- Původní `thalvoryn:caves/trek_dripstone_room` měl `rarity_filter chance: 5`, takže se mohl opakovat zhruba v každém pátém vhodném chunku; horní varianta měla šanci 1/7.
- Bastion používal **náhodnou absolutní výšku Y90–704**, bez `project_start_to_heightmap`; to umožňovalo létající pevnosti.
- Zakázaná custom End City ze zdroje TREK skutečně obsahuje **276 bloků bílé vlny jako plachty a dřevěné prvky** v NBT šabloně `end_ship.nbt`.
- Přímo v uloženém světě je jako datapack aktivní jen V67 a `vanilla`. **Stellarity se v aktivním ZIPu neinstaluje**; přítomen je jen text "no Stellarity" v popisu původního souboru.

## Co se změnilo ve čtyřech ZIPech

1. Čtyři extrémně časté *placed features* pro TREK místnosti v dripstone/lush jeskyních mají nyní rarity **1/650, 1/900, 1/700, 1/950** místo 1/5, 1/7, 1/6 a 1/9. Tím se odstraňuje hlavní zdroj tisíců stejných místností v nově generovaných chunkech.
2. **10 pevných inventářů** ve zdrojových šablonách TREK monster room bylo převedeno na skutečné `LootTable` bez pevného seedu. Původní dva často se opakující inventáře se nyní generují z různých loot tabulek; ostatní kontejnery se záměrně neměnily plošně.
3. Samostatná skupina spawnerů: `spacing 42`, `separation 17`, `frequency .31`, místo `30 / 12 / .27`.
4. **6 původních tunelových variant zůstává**; společná skupina má `spacing 29`, `separation 12`, `frequency .68`. Není zaručeno spojení s náhodnou procedurální jeskyní — v šabloně je vlastní koncová dutina.
5. **Bastion:** z absoluní Y90–704 na povrchové umístění s offsetem -9 a `beard_thin`; Horizon Rust má navíc kontrolu lávy a nerovnosti terénu. Zachována původní vanilla bastion jigsaw sestava a loot.
6. **End:** zakázána custom `trek:end/medium/end_city` s vlněnými plachtami, deaktivováno 6 rizikových End boat/dock poolů. **Vanilla `minecraft:end_city` (včetně její původní lodi) je beze změny.**
7. Všechny Overworld density/noise/biome soubory jsou **bitově nezměněné**. Stellarity data nebyla znovu přidána.

## Výsledky kódových testů

- **4/4 ZIP PASS, 11 323 položek každý, bez duplicit, CRC OK**, JSON a metadata 26.3 OK.
- **Přesně 17 upravených souborů na variantu**, bez přepsání terénu, jeskynních carverů nebo jiných NBT šablon mimo cílové místnosti.
- **10/10 původně pevných inventářů** v postižených TREK šablonách převedeno na loot table.
- NBT obsah bitově shodný ve všech čtyřech variantách.
- Vanilla End City zachována; poznatky z uloženého světa zdokumentované samostatně.

## Co stále NENÍ ověřené

- Skutečné **FPS/TPS po změně**, průchod portály, chunkgen a jigsaw kolize v Horizon/Java.
- Jestli všechny další větší pevnosti, vesnice a lodě bezpečně dosedají na svahy.
- Napojení všech šesti tunelů přímo na přirozené jeskyně. Datapack nedokáže automaticky garantovat napojení do náhodného velkého cavernu jen pomocí statické NBT šablony.
- Vanilla Nether Fortress používá zvláštní generátor a její výšku tento hotfix nepřepisuje.
- **Dříve vytvořené chunky ve světě obsahují staré stavby a truhly dál.** Změna datapacku je nikdy neodstraní; proto je pro ověření nutný **nový svět**.

## Test: úplně nový svět

Pro srovnání použij **stejný seed uloženého světa: `[seed uloženého světa – v soukromé kopii README]`**, pouze jeden ZIP a žádný samostatný Stellarity/TREK:

```mcfunction
/datapack list enabled
/gamemode spectator
/execute in minecraft:the_nether run locate structure minecraft:bastion_remnant
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:the_end run locate structure minecraft:end_city
```

Zkoumej hlavně dripstone caves v okolí XYZ `-459 121 731`, první přirozeně nalezený bastion a loot v truhlách dvou různých podzemních místností. Ve starém světě se problém objeví dál, protože má již uložené chyby.

**Součástí archivu není upravený uložený svět.** Originální `New World (7).rar` nebyl změněn.

**Licence:** Smíšené assety dodaných balíčků nejsou schválené k veřejnému redistribuování. ZIPy uchovávej pro soukromé testování; GitHub může obsahovat samostatný zdrojový patch a QA reporty, ne převzatý obsah modů.