# Thalvoryn RC35 – End, hlavní ostrov, biotopy a méně spamujících struktur

**2026-11-08 · Horizon Rust / Minecraft Java 26.3 · pouze PRIVÁTNÍ TEST.** Základ: RC34. Herní test Horizonu neproveden.

## Zadání uživatele – část 5

> 💜 End
>
> Předělat hlavní spawn ostrov, přidat výrazně odlišné biomy a ostrovy, vlastní vegetaci a další struktury.
>
> Ověření před/po, kontrola celého balíčku. Nejen ctrl+C/ctrl+V ostrovy, žádná nevhodně umístěná nebo příliš blízká struktura.

## Zjištěný problém před opravou RC34

1. **Zdvojená mřížka End Cities:** vanilla `minecraft:end_cities` (spacing 32, frequency .72) a navíc `trek:end/medium` (spacing 30, frequency .6375) vybíraly End City **nezávisle**. To výrazně zvyšovalo hustotu opakujících se staveb.
2. End má **více samostatných velmi aktivních strukturálních skupin** s četností 0.42–0.58 a rozestupy 58–68; mohou být blízko sebe.
3. Override biomu `minecraft:small_end_islands` v RC34 neobsahoval originální **`minecraft:end_island_decorated`**, pouze `thalvoryn:end/high_sky_islands`. RC35 původní feature obnovuje.
4. Hlavní End používá **oficiální Minecraft 26.3** `minecraft:end/islands` density, nikoli NBT strukturu. Pro bezpečnost dračí arény není vhodné ji překrýt velkou importovanou stavbou.

## Implementace RC35

### Hlavní spawn ostrov
- Nové override `data/minecraft/worldgen/density_function/end/islands.json` vychází ze **skutečného 26.3** `minecraft:end/islands` (source [vanilla assets](https://github.com/InventivetalentDev/minecraft-assets/blob/26.3/data/minecraft/worldgen/density_function/end/islands.json)).
- Základní centrální radius a původní `minecraft:end_outer_islands` zůstávají, k pobřeží se přidává **sedm různých výběžků** s nepravidelnými směry, poloměry a lokální výškovou hustotou.
- **Dračí aréna je chráněná:** v celém diskovém okruhu **radius ≤72 bloků** zůstala původní hlavní ostrovní density **matematicky totožná**; analytický sampling ověřil **1009 bodů** v tomto okruhu. Sloped cheese, vanilla End noise settings, dragon spikes, origin platform a dimenze **nepřepsány**.
- Výsledný tvar ostrova a draka **nebyl herně vykreslen/testován**, není proto prokázán ani úplný dynamický přístup z příletové plošiny.

### Pět malých přírodních akcentů v EXISTUJÍCÍCH biomových ID
- Highlands: end-rod glimmer reeds a vzácná kalcitová ložiska.
- Midlands: malé fialové koberce/nízké purpur-slab stylizované porosty.
- Barrens: šedé přízemní porosty, **velmi vzácné obsidiánové stébly** z původní feature.
- Small End Islands: řídké fialové porosty a vzácné purpur žíly.
- Všechny nové `minecraft:simple_block` feature vyžadují **air v cíli a end_stone přímo pod blokem**; výběr biomu filtruje `minecraft:biome`.
- **Nová vlastní biome ID / nové bloky / nové entity ID ve F3 zde nejsou.** V Horizon Rust je aktuální omezení registrace nových biomes přes datapack.

### Další skutečně odlišné malé stavby
- `thalvoryn:end/rc35/crystal_shelter` – 9×10×9 end-stone / amethyst úkryt, **92** bloků.
- `thalvoryn:end/rc35/midland_waystation` – 11×10×11 purpurová zastávka, **156** bloků.
- `thalvoryn:end/rc35/barren_watch` – 9×12×9 temná hlídka, **107** bloků.
- Každá obsahuje **1 truhlu se třemi pevnými skromnými End zásobami** (ender pearl / chorus fruit / amethyst shard nebo obsidian), žádné loot dekorativních barelů.
- Umístění je biome-specific a projikované na `WORLD_SURFACE_WG`, s krátkými podporami pod podlahou a **jedinou vzácnou sdílenou mřížkou** (spacing 102, separation 41, frequency 0.18), nikoli třemi mřížkami.
- **Pozor:** biom a výška pod bodem startu nedokážou garantovat rovnost celého podkladu; strmé svahy vyžadují celopůdorysový engine validátor.

### Anti-spam
- Odstraněna druhá nezávislá TREK End City skupina (šablony zůstaly).
- Tři samostatné variantní End spawn skupiny (barren_obelisks, midland_grove, small_island_cache) vypnuty; první dvě varianty mohou být vybrány společnou vzácnou skupinou, původní NBT/definice zachovány.
- Pět dalších nezávislých skupin Endu zředěno (sky_caravan, sky_sanctuary, landmarks, diverse_islands, highland_ruins). Vanilkové End Cities zůstaly **beze změny**.

## Povinná statická kontrola před/po

| Kontrola | RC34 před | RC35 po |
|---|---:|---:|
| ZIP soubory | 2 872 | **2 891** |
| JSON + pack metadata | 1 604 | **1 619** |
| NBT šablony | 1 174 | **1 177** |
| Staré NBT zachované beze změny | 1 174 | **všech 1 174** |
| Aktivní structure sety | 49 | **46** |
| Nové vlastní End landmarks | 0 | **3** |
| Dvojitá End City mřížka | 2 | **1** |
| End origin/radius ≤72 originální density | výchozí | **identická při 1009 analytických vzorcích** |

**PASS:** oba ZIP CRC; všechny JSON parsovatelné; žádné duplicitní structure-set salts; všechna aktivní vlastní structure ID v registrech; žádný cyklus pořadí worldgen features v celé sadě přepsaných biomů (**173 uzlů**). Všech 3 nových NBT bylo nezávisle úplně dekódováno (rozměry, bloky, palety, jedinečné souřadnice a správné 3 inventáře), **1 174 původních NBT byte-identických**. Výška: **Overworld min_y −64, height 2096**; **Nether 0, height 1024**; **End 0, height 512**. `pack.png` zůstal byte-identický.

ZIP: `Thalvoryn_Horizon_Rust_RC35_END_ORIGIN_BIOMES_PRIVATE_TEST.zip` (**9 061 429 B**)  
**SHA-256:** `1bd57efa5bdc775459387e62a105941290cb7700a47aa8f264724b078321be35`

## NEDOKONČENO – nepřehánět

- **Žádný Horizon runtime test.** Nevíme, jak vypadají tvary pobřeží, zda všechny feature umí konkrétní Nightly a zda se některé objekty na nerovném povrchu neodříznou.
- End build height 512 **neznamená** native terrain noise 512: `minecraft:end` noise settings stále používají **height 128**, vysoké odtržené ostrovy jsou samostatné placed features / NBT.
- Nově registrované biome ID se jménem ve F3, nové bloky a mobové vyžadují úpravy registrů/nativní API Horizonu.
- Není vyřešená obecná kontrola kapaliny/svahu/kolizí všech velkých struktur, safe Nether portal spawn, distant LOD ani Photon.
- Original user-provided map assets have unverified redistribution rights; **nezveřejňovat binární testovací archiv na GitHub jako veřejný release**.
- Stávající již generované chunky se **nemění**: pro test založit nový svět, připojit jen RC35, zkontrolovat hlavní End ostrov, End Cities, malé ostrůvky, LOD a nejbližší trojici ruin.

[Požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Hlášené chyby #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
