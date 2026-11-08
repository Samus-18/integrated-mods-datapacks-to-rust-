# RC42 — opravy podle skutečného uloženého světa

**8. 11. 2026 · Horizon Rust / Minecraft 26.3 · privátní experimentální sestavení; zatím bez testu v běžícím enginu.**

## Forenzní audit nahraného RC41 světa

Rozbalený uložený svět potvrzuje aktivní RC41. Ze skutečných MCA regionů: **Overworld 9092 chunků, 41 structure starts celkem; Nether 4095 chunků, 10 starts (jen 2 vlastní); End 6044 chunků, 1 vlastní start**. Placed-feature ruiny se v těchto počtech nemusejí objevit, jde o starts ze structures registru.

**Loot byl skutečně vadný:** v Overworldu 1001 uložených kontejnerových block entities: **551 prázdných bez LootTable**, dalších **447 pouze s LootTable bez fyzických Items**, jen 3 s naplněnými Items. Screenem viděné prázdné barely byly potvrzeny přímo z uložených chunk NBT. Biome palety v prozkoumaném Overworldu obsahují rozsáhlé lush_caves, ale žádné dripstone_caves ani deep_dark.

## Opravy RC42 datapacku

- **2818 původně prázdných/tabulkových NBT kontejnerů** dostalo fyzický Items s vanilkovými 26.3 item IDs; 9 ručně naplněných originálů zachováno. Žádná vlastní loot-tabular registrace není pro jejich obsah nutná.
- **190 nových NBT inventárních variant** + upravených **95 aktivních startovních template poolů**, takže opakované kopie téže stavby mohou vybírat jednu ze tří variant. Nejde o loot náhodně přepočítávaný při otevření.
- **Povrch:** hlavní TREK spacing 30→19, frequency 0.8→0.84; přibližně **2.6× vyšší teoretická hustota pokusů**, podobně další rozumně bezpečné povrchové sady. Oceánské lodě bez plošné kontroly vody nejsou zahuštěny.
- **Nether:** tematické kempy/hledání spacing 51→27–28, frequency .30→.55–.57, podzemní skrýše méně vzácné a ve více výškách; **Nether height 1024 beze změny**.
- **Jeskyně:** starý RC41 měl tři základní cave/canyon carvery s probability 0. RC42 je obnovuje (cave .025, extra cave .020, canyon .007), horské průchody častější; rozlišení lush/dripstone v biome-source i na ose weirdness, Deep Dark o něco dostupnější. Čtyři nové výškové podzemní placed features Y112–720, větší šance na ruiny. Podzemní osada má více povolených lesních biomů.
- **End:** mírně zvýšený výskyt různých existujících End islands/landmarks a malých sky islands. Dragon arena nedotčena. **Vlastní nová biome ID ve F3 stále nepodporuje Horizon world-datapack registry**.

## Povinný audit a artefakty

RC41→RC42: **ZIP CRC PASS, všech 1685 JSON/metadat PASS, 1367 NBT (1177 původních +190 variant)**. Nezávislý test prokázal, že **473 původních NBT se změnilo POUZE v Items/LootTable, 704 jsou byte-identické**. Všech 190 nových variant má shodné všechny non-inventory bajty jako originál, pool refs 190/190 existují, žádné neplatné item sloty nebo chybějící nové worldgen odkazy. Všechny rozměry dimenzí a vlastní pack.png zachovány.

Soukromý datapack ZIP: Thalvoryn_Horizon_Rust_RC42_WORLD_FEEDBACK_LOOT_DENSITY_CAVES_NETHER1024_TEST.zip, **10 104 421 B**, SHA256 `805e5c6c9927e38d36c232ecedb0707344bfa5204d23806062884f4a2b0c0cc9`.

Volitelná samostatná oprava UŽ ULOŽENÝCH chunků z přesného snapshotu, který uživatel nahrál: **20 MCA region files, 863 prázdných kontejnerů, 285 upravených chunků**. Nezávislá kontrola 9708 chunků v regionech: 9423 raw NBT beze změny; 285 změněných chunků se mimo Items/LootTable bajtově neliší od předlohy. Overlay ZIP **118 735 433 B**, SHA256 `541421355f144790df293b281c7ba06a80b28bb8756071d3f98036eb5970e43e`. **NENÍ DATAPACK!** Aplikovat jen po záloze a pouze pokud svět od nahrání nebyl upraven; jinak by se přepsal novější progress. Kompletní soukromá analýza a skripty byly odevzdány v chatu.

## Stále otevřené / neoznačovat STABLE

RC42 **nebyla spuštěna v aktuálním Horizon Nightly**. Nová četnost carverů/struktur, loot Items loading, cave biome variability, FPS/TPS a End/Nether rendering je nutné herně ověřit. Změny generátoru zasáhnou jen **nové chunky**. LOD, F3 vlastní biome ID, safe Nether portal a full-footprint voda/láva/svahy u lodí/staveb vyžadují Rust engine změny. Nether lava ships zůstaly bez náhodného spawnu. Některé cizí NBT mapy nemají vyřešená práva k veřejné redistribuci, proto žádný veřejný binární GitHub Release.

Evidence v otevřeném [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).