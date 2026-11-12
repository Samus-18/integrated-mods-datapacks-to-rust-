# Thalvoryn V69 — cílené opravy rozmístění a mobů

**Stav:** soukromá testovací verze pro Minecraft 26.3 / Horizon Rust Nightly `2ebf00a`. **4/4 statické kontroly prošly, herní test stále NEPROBĚHL.**

## Co je skutečně změněno oproti V68

- TREK Nether: `nether_pillar`, `overworld_leak_1`, `overworld_leak_2` nově začínají relativně vůči výšce reálného povrchu (`WORLD_SURFACE_WG`, −6 až −2), místo náhodného absolutního Y64–340. Mají šanci generovat v různých výškách Netheru podle terénu.
- Nether `cinder_viaduct` a `skybridge_ruin`: přidáno `beard_thin` přizpůsobení k terénu (vodorovný rozsah je pod 116).
- Pouze Horizon Rust: 15 netherových a 14 endových definic má doplněnou kontrolu nevhodného okolního terénu. U Netheru také začátek v lávě a výškové meze podle varianty (Nether 1024/2032). **Java tyto speciální kontroly neumí**, používá jen standardní worldgen.
- Omezeny přemrštěné skupiny mobů v `stray_fort`, `toxic_lair`, `sealing_halls` a `end_castle`. Například stray 32→4, cave spider 16→3 a snižují se extrémní spawn váhy.
- Aktualizované označení **V69** a `/function thalvoryn:version`.

## Co bylo nezávisle staticky ověřeno

- 4/4 ZIP CRC, JSON/MCMeta parse, `pack_format` 121, bez duplicit.
- **6 835 NBT šablon beze změn**, žádné úpravy jejich bloků ani uložených inventářů.
- Beze změn loot tabulky, biome definice, dimension/noise/density, `structure_set` a placed features oproti V68.
- 478 aktivních unikátních struktur, žádné nově aktivované pirátské lodě v Netheru/Endu. Vnitřní End Castle pooly `boat/docks` jsou prázdné; vanilla End City loď zůstává.
- Kontroly nehlásí neplatný horizontální dosah u nových adaptovaných jigsaw struktur. Rust 2032 používá vlastní horní výškové limity.

## Co tato verze NESLIBUJE

- Žádné spuštění Horizon 26.3 ani Minecraft Java 26.3 neproběhlo; není zde Java 25/Horizon klient.
- Není potvrzena plynulost TPS/FPS, správné dosednutí všech gigantických struktur nebo jejich úplné dokončení v každém seedu.
- Jde o ochranu *startovních pozic*, ne zázračné terraformování celé vesnice. Java nedostává Rust-only kontroly.
- Tunely mají samostatné jeskynní komory; napojení na procedurálně generované přírodní caves zatím není zaručeno.
- V balíčku zůstávají některé velké importované jigsaw stavby s horizontálním rozsahem 128 a bez terrain adaptation; nelze je bezpečně převést na `beard_thin` bez změny dosahu, která by mohla useknout další části. Neoznačovat je za opravené.

## Test

V novém světě, pouze s jedním ZIPem své platformy a výšky Netheru:

```mcfunction
/function thalvoryn:version
/gamemode spectator
/execute in minecraft:the_nether run locate structure trek:nether/common/nether_pillar
/execute in minecraft:the_nether run locate structure trek:nether/medium/overworld_leak_1
/execute in minecraft:the_nether run locate structure nova_structures:sealing_halls
/execute in minecraft:the_end run locate structure nova_structures:end_castle
```

Kontrolovat zejména polohu staveb, množství mobů, zda se svět generuje rychleji, a znovunačtení uloženého světa. Nová verze nikdy neopraví už uložené chunky.

**Licenční omezení:** integrované soubory jsou soukromé testovací kopie z uživatelem dodaných balíčků. Nahrát na veřejný GitHub pouze vlastní skripty, nikoliv binární balíčky cizích assetů bez prověření licencí.