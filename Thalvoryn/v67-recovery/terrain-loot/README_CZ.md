# Thalvoryn V67 – opravy terénu, lodí a lootu (soukromý test)

**Cíl:** Minecraft Java 26.3 a Horizon Rust (testovací verze kolem `2ebf00a`). **Není herně otestováno.** Testujte jen na novém světě, protože již vytvořené chunky se po výměně datapacku nepřegenerují. Nepoužívejte vedle tohoto ZIPu samostatný TREK/Stellarity/DnT.

## Konkrétní změny oproti předchozí V67 INTEGRITY

1. **Loot:** odstraněno 415 pevně uložených `LootTableSeed` hodnot z 94 NBT šablon v každé variantě. Obsah se nyní může náhodně vyhodnocovat podle aktuálního světa, místo aby stejné šablony měly opakovaně stejné nastavení. *Není zaručeno, že dva náhodné výsledky nikdy nebudou shodné.*
2. **Různorodost:** 444 odkazů na generický loot v několika velkých importovaných Overworld šablonách bylo rozděleno mezi existující loot tabulky podle typu stavby a konkrétního kontejneru. Tři chudé/přehnané loot tabulky `Structures+` upraveny na rozumnější náhodné zásoby. Dekorativní kontejnery bez LootTable a pevné předměty, které jsou součástí mechanismů/pastí, jsou zachovány.
3. **Oceánské lodě:** 27 importovaných velkých lodí omezeno na hluboké nezamrzající oceány. Výška usazení určena z hustoty jejich spodních palub a přepočtena k hladině 63, aby trupy nebyly celé nad hladinou. Rust verze používá kontrolu širšího okolí biotopu. Toto není univerzální kontrola pod každým bodem obřího trupu.
4. **Přístavní/obchodní lodě:** 9 menších oceánských obchodních staveb omezeno na hluboké nezamrzající oceány. V jejich Rust kontrolách se nezakazuje kapalina: oceánská loď má být ve vodě.
5. **Osady a pozemní stavby:** 116 Java / 126 Rust definic s lepší projekcí výšky podle terénu **bez listí stromů**. Pro rozsáhlé stavby bylo použito `beard_box`; Horizon Rust navíc omezuje rozdíl výšek v okolí a odmítá povrchové stavby ve vodě. **V Java toto omezení sklonu není možné garantovat běžným datapackem.** Žádná z těchto změn není automatická adaptace každého NBT bloku na svah.
6. **Jeskynní vstupy:** šest již existujících strmých tunelů zůstává, každá NBT obsahuje vlastní velkou vytesanou dutinu. Jejich sdílený structure set je mírně častější (`spacing` 35 → 30 chunků, `frequency` 0.60 → 0.63). **Propojení s náhodnou přírodní jeskyní není zaručeno.**

## Co bylo kódem ověřeno

`QA_GROUND_LOOT.json` – 4/4 ZIP CRC + JSON, 11 323 položek a 6 835 NBT v každé variantě, stejné NBT napříč JAVA/RUST i 1024/2032, správné výšky dimenzí, žádný pevný `LootTableSeed`, žádné nečekané změny Minecraft/Overworld/End/worldgen souborů, platformní rozdíly pouze ve zvláštních kontrolách Horizonu.

## Co zůstává nezaručené

Skutečná přirozená generace lodí na všech pobřežích a seedech, spoje jigsaw staveb, světlo, přístup do jeskyní, načítání Endu/Netheru, FPS/TPS a funkčnost všech otevřených truhel. To vyžaduje herní test. Příčinou zobrazeného FPS nelze jen ze screenshotu označit datapack.

## Rychlý test na NOVÉM světě

```mcfunction
/gamemode spectator
/execute in minecraft:overworld run tp @s 1847 513 547
/execute in minecraft:overworld run tp @s -9268 293 -4946
/execute in minecraft:overworld run locate structure thalvoryn:overworld/imported/ships_01
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/loot give @s loot structuresplus:farmer
/execute in minecraft:the_end run locate structure minecraft:end_city
```

Pro srovnání se starými screenshoty použij *stejný seed*, ale nový svět. `/locate` vrací X/Z; ve Spectatoru se teleportuj nad lokací. Loot vyzkoušej v několika **skutečně nově vygenerovaných** stavbách, nejen příkazem. Při chybě přilož seed, dimenzi, XYZ, název struktury a video.

**Licence:** ZIP obsahuje soubory z dodaných třetích stran. Soukromé testování; nepublikovat veřejně bez kontroly licencí.