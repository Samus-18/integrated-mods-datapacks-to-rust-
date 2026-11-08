# RC38 — Mobové ve strukturách, vyvážená obtížnost a variabilita lootu

**Soukromé experimentální sestavení Thalvoryn pro Horizon Rust / Minecraft Java 26.3, 8. listopadu 2026.** Vychází **přímo z RC37**, nevydávat za potvrzený in-game build.

## Uživatelův požadavek — část 8

> A jo ať se ten loot sám i mění, ať to není stejné v té struktuře, když ji najdu znovu.
>
> 🧟 Mobové — Vlastní osazení struktur, přiměřená obtížnost, správní nepřátelé a pasivní mobové.

Uživatel bude testovat až po dokončení celého seznamu. V každé RC požaduje kontrolu **před i po**, aby se nerozbil svět, loot, stavby, dimenze a ikona.

## Nejdůležitější rozlišení

**Loot se generuje jednorázově na nové truhle při prvním použití**, nikoli po každém návratu do stejné truhly. Nové umístění stejného typu struktury s novou truhlou může vrátit jiné zásoby díky náhodným váženým poolům; konkrétní dvě kopie se *mohou náhodou shodovat*, rozdíl nelze garantovat. Všechny uživatelské NBT truhly z RC37 nemají pevný `LootTableSeed`. Osm starších vanilla NBT šablon má uložených celkem 71 výskytů tohoto tagu; **nepřepisovali jsme je**. U všech **66 RC37 tabulek** byla provedena omezená nezávislá simulace 16 různých RNG stavů; každá měla více rozdílných možných výstupů. **Simulace není Minecraft nebo Horizon runtime!** Loot-table lookup v běžném world datapacku Horizon zatím není prokázán.

## RC38 osazení struktur

**35 aktivních definic jigsaw staveb** má nový `spawn_overrides` na `bounding_box: piece`, bez vynuceného spawnování mimo konkrétní části staveb. Nový typ mobů/nové ID entity nebylo vytvořeno, pouze vanilla entity.

- **26 nových hostile pravidel**: zombie a skeletoni v ruinách, huskové v pouštních troskách, drowned a velmi vzácná čarodějnice u bažinných ruin, spider/skeleton v džungli, pillagers u hlídek, endermani/endermiti u některých End ruin, magma cubes a zombified piglins v malém netherovém pilíři.
- **9 nových pasivních pravidel**: ovce/slepice u klidných farem a domků, králíci u pouštního ranče a oázy a zasněženého domu, lišky u tajgových domků a včely v beehive house.
- **Podzemní osada** `thalvoryn:underground/spiral_hamlet`: do původního NBT s **3 vesničany** přidána **1 kočka** a **1 železný golem**, s `PersistenceRequired=1`; všechny původní bloky, loot, entity i schodiště zachovány. Umístění vybráno do volného prostoru na existujícím nádvoří (lokální Y5). Zda Horizon NBT entity skutečně přidá do světa je třeba ověřit.
- **Nezměněno**: vanilla struktury a jejich spawn, TREK village struktury, End archipelago, Deep Dark, dračí aréna, netherové lávové lodě (stále manual-only), stromové šablony a ostatní existující kategorie spawn override.
- **33 relevantních strukturových pravidel**, která již obsahovala vlastní spawn override pro danou kategorii, jsem zachoval místo přepsání.
- Na jeden spawnovací pokus je u všech nově definovaných mobů `count: {type: minecraft:constant, value:1}`; **nejde o maximální počet mobů v oblasti**. Ongoing spawn attempts se mohou opakovat; světové mob caps, light/block spawn checks a Peaceful mode jsou stále rozhodující. Pasivní `creature` strukturová override se **neaplikuje na počáteční worldgen zvířata při vytváření chunku**; ovlivňuje příští spawn attempts. To je omezení vanilla mechaniky, nikoli dokončené fyzické osazení všech staveb.

## Před / po ověření

| Kontrola | RC37 před | RC38 po |
|---|---:|---:|
| Počet členů ZIP | 2 947 | **2 947** |
| JSON / metadata | 1 673 | **1 673** |
| NBT templates | 1 177 | **1 177** |
| Nově upravené `spawn_overrides` | — | **35** JSONů (26 monsters + 9 creatures) |
| Změněné NBT | — | **1** (pouze `entities` list, 3→5) |
| Změny ostatních NBT | — | **0** (všechny byte-identické) |
| Změny RC37 loot tabulek | — | **0** (všechny byte-identické) |
| Změny worldgen biome/noise/features/structure sets/pools | — | **0** |
| Výška Overworld | −64..2031 | **−64..2031** |
| Nether | 0..1023 | **0..1023**, bez návratu na 2k |
| End | 0..511 | **0..511** |
| Ikona `pack.png` | pixart RC31 | **byte-identická** |

**Statická kontrola: PASS** — ZIP CRC před/po, parsování všech JSON, ověření 35 diffů, žádné změny celého strukturálního JSON mimo přidanou kategorii `spawn_overrides`, platný 26.3 int-provider `minecraft:constant`, žádná stará kategorie smazaná, konzistentní biome/typový výběr, úplné zachování všech původních 200 aktivních structure ID a všech NBT mimo jednu osadu.

**Druhá nezávislá kontrola: PASS** — nezávislý Big-Endian NBT reader porovnal všechny top-level tagy osady: každý tag jiný než `entities` má identické raw bajty, **původní tři entity byte-identicky zachované**, pouze dvě nové entity připojeny. Zkontroloval veškeré reference ZIPu, vlastní JSON, šablony, data terénu, loot a ikonu. Soubor `Thalvoryn_RC38_INDEPENDENT_VALIDATION.json` je přiložen v konverzaci.

**Soukromý RC38 ZIP**: `Thalvoryn_Horizon_Rust_RC38_MOBS_BALANCED_NETHER1024_PRIVATE_TEST.zip` (**8 848 165 B**)  
**SHA-256**: `868e9df7343aeca410892453d7a4f135cbaae1dc79e6373f80b779e3ab10a972`

## NEDOKONČENO

1. **Žádný skutečný Horizon Nightly runtime test**; moby ve stavebních boxech se mohou chovat jinak nebo vůbec nespawnovat. Přiměřenost počtu a obtížnost není in-game ověřena.
2. `spawn_overrides` představuje **povolené druhy při spawnování**, nezaručený počet mobů nebo okamžité osazení struktury. Pokud se bude přidávat více fyzických obyvatel, vyžaduje to bezpečné lokální entity placement a testování.
3. Nové druhy mobů vyžadující Rust engine registry nebyly vytvořeny. Použity **jen existující vanilla entity ID**.
4. Variabilita lootu funguje podle vanilla principu jen při fungujícím lookup vlastních loot tabulek; **podpora RC37 vlastních tabulek v obyčejných Horizon world datapackách zůstává nedoložená**, jak je zaznamenáno v RC37.
5. Netherové lávové lodě, kompletní plošný collision validator, F3 custom biome IDs, safe portal arrival, LOD, Photon a distribuční práva k předloženým mapám jsou stále otevřené.
6. Nebyl vydán veřejný GitHub binární release, pouze dokumentace a private chat ZIP.

Odkazy k mechanice: [Minecraft Java 26.3 changelog](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3), [structure spawn overrides 26.3](https://minecraftdocs.dev/reference/structure-spawn-overrides), [loot seed / při prvním otevření](https://minecraftdocs.dev/systems/items/loot-tables). Předchozí [RC37 loot audit](RC37_LOOT_DYNAMIC_26_3_PRE_POST.md).

[Uživatelské požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Otevřené chyby #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).
