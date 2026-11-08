# Požadavky uživatele – postupně zasílané části

**Záznam přijatých částí, ne finální roadmapa.** Aktualizovat při každé další zprávě uživatele. Úpravy testovaných RC balíčků jsou odlišeny od problémů vyžadujících Horizon engine.

## Část 1 — Overworld (přijato 2026-11-08)

Původní znění uživatele:
> 🌍 Overworld
>
> Nové biomy, výraznější hory, lepší jeskynní vstupy, různorodé Sky Islands a jejich výška podle terénu

Dodatečný požadavek stejné zprávy:
> chyby v datapacku ta custom ikona zapominas na to

### Rozpracováno v RC31
- [x] 4 vlastní tvarové Sky Islands + 1 dosavadní Sky Garden
- [x] Výška startu ostrovů od `WORLD_SURFACE_WG` (offset, nikoli absolutní Y)
- [x] Jeden sdílený structure set pro pět variant kvůli omezení spamu
- [x] Zvýrazněná extrémní horská větev bez změny limitu JJThunder
- [x] Horské a lesní průchody jeskyní a nový vzácný vyšší carver
- [x] 3 stylové/vegetační *varianty existujících biomes* pro podporovaný datapack systém
- [x] Nahrazení starého TREK obrázku vlastní ikonou `pack.png`
- [ ] Herní test RC31: počet a umístění ostrovů, geometrie vysokých hor, viditelnost vstupů jeskyní, kolize, výkon
- [ ] Skutečně nová biome ID/názvy ve F3: vyžaduje registraci do Horizon runtime, ne jen datapack
- [ ] Stínování/LOD vzdáleného terénu: engine otázka, neopravené

Technická dokumentace: [RC31](RC31_OVERWORLD_SKY_MOUNTAINS_CAVES_ICON_PRIVATE_TEST.md).

## Část 2 — Stromy a příroda (přijato 2026-11-08)

Původní znění uživatele:
> 🌲 Stromy a příroda
>
> Využít všechny vhodné vlastní stromy z tvých schémat, nové druhy lesů, přirozené rozmístění vegetace

Požadavek na kontrolu:
> vždy kontrola před a po jestli se něco nerozbilo

### Rozpracováno v RC32
- [x] Zkontrolováno 48 dříve importovaných vhodných stromů z přiloženého schématu, všechny už mají NBT a svůj pool, všech 48 původních dat zachováno
- [x] Přidáno 48 materiálových variant podle biomu + 12 tmavolesních variant, bez změny původní geometrie
- [x] Pět tematických lesních variant: březiny, smrkové vysočiny, luční lesy, akáciové savany a tmavé lesy; používají existující Minecraft biome ID
- [x] Sedm typů přízemních vegetačních akcentů, se vzduchovým a půdním filtrem, a tři další vzácné native stromy
- [x] Snížení lokálního spamu pomocí jednotného poolu na každou lesní skupinu, skupiny se výběrem biomů nepřekrývají
- [x] **Kontrola před a po:** ZIP CRC, JSON parse, validace NBT včetně 60 kopií, všechny odkazy, DAG pořadí feature, stejné rozměry světů, původní NBT byte-identické
- [ ] Herní test: rozložení stromů, převisy, výkonnost, chování před/po načtení chunků, kompatibilita nočního Horizon buildu
- [ ] Úplná engine kontrola kolizí s velkými budovami. Datapack neumí zaručit nekolizní umístění podle celé šířky šablony.
- [ ] Vlastní registrované biome ID a názvy ve F3: vyžaduje engine podporu, nikoli jen nové názvy v datapacku

Technický audit: [RC32](RC32_TREES_AND_NATURE_STATIC_TEST.md).

## Část 3 — Jeskyně (přijato 2026-11-08)

Původní požadavek:
> 🕳️ Jeskyně
>
> Vlastní jeskynní biomy, unikátní rostliny, bloky, mobové a podzemní struktury

Dodatečné požadavky:
> samozrejme zase kontrolu pred a po jak vzdy a hlavne vse umisti spravne a i na dbore mista at se tam hodi priklad lode ktery se hodi do netheru a jsou z toho onivzdorneho dreva tak je dej do lavu jako ze plujou pres lavu chapes ne no proste premyslej jako developer a hrac a hlavne logiku/predstavivost

### RC33 – co je skutečně implementováno v testovacím ZIPu
- [x] **3 podzemní habitaty** v *existujících* biomes: Lush Moss Grotto, Dripstone Calcite Ossuary, Deep Dark Sculk Archive
- [x] 4 přízemní vegetační feature s ověřením vzduchu/půdy a podzemní výšky
- [x] 3 nativní dekorativní minerální akcenty s omezeným nahrazováním stone/deepslate
- [x] 3 nové malé původní podzemní NBT ruiny a 2 vzácně umisťované TREK místnosti
- [x] Vypnutí dvou přehnaně hustých TREK jigsaw monster-room setů (`spacing=1`)
- [x] Opatrně změněné **původní native mob typy**: vzácné cave spider / silverfish v Dripstone Caves, původní axolotl v Lush, Deep Dark bez přirozených mobů
- [x] **Kontrola před i po:** CRC ZIP, JSON, odkazy, 3 nové NBT, zachování 1 166 původních NBT, feature DAG bez cyklu a výšky všech dimenzí beze změny
- [ ] Herní kontrola míst: **nepotvrzené**, zda se ruiny skutečně objeví v otevřených jeskyních, místo aby byly zahrabané; doladit podle logu/snímků
- [ ] **Opravdová nová biome ID**, vlastní bloky a nové druhy mobů: potřebují nativní podporu Horizon/mod, nikoli jen JSON
- [ ] **Nether lava boats**: nehořlavé lodě mají být opravdu na povrchu *souvislé lávy*, ne na pevnině / v lese / ve vzduchu; v RC33 jsou stále vypnuté, protože zatím chybí kontrola kapaliny pod celým trupem a bezpečného terénu; žádný false-positive fix
- [ ] Kontrola celého půdorysu, sousedních struktur a vegetace pro ostatní velké stavby zůstává otevřená

Dokumentace: [RC33 podrobný audit](RC33_CAVE_HABITATS_PRE_POST_AUDIT.md).

## Část 4 — Nether a průzkum hlubin/hor (přijato 2026-11-08)

Původní požadavky:
> 🔥 Nether
>
> Více přírodních variant, vlastní vegetace, různé styly krajiny, lávové lodě, více struktur a lepší rozložení rud

Doplněno v téže zprávě:
> muzes do pod zemi pridat i treba podzemni vesnice a tak ale at cesta k tomu vede az nahoru ne primo smerem nahoru proste cesticka az dolu a tak muzes do podzemi pridat i vice veci at se vyplati jit i dolu to stejny i do hor nahoru nebo dolu ... samosrejme pocitam celkove i nether ... vzdy kontrolu pred/po

### RC34 – staticky implementováno
- [x] Nová podzemní osada v pláních se souvislou klikatou cestou shora dolů (71 úseků, 61 výškových bloků), čtyřmi domy, zdroji světla a třemi villager NBT entitami.
- [x] Vzácný důvod zkoumat nejvyšší hory (emerald traces Y230..1250) a hluboké dripstone caves (minimální diamond traces Y−52..−25).
- [x] Pět biotopově odpovídajících **override existujících** netherových biome ID (žádná nově registrovaná ID) a sedm typů netherové vegetace s omezením půdy a okolního vzduchu.
- [x] Pět jemných noise-based materiálových stylů a upravené rozložení quartz/gold/ancient debris v Netheru Y0..1023.
- [x] Dvě malé vzácné zahrabané netherové ruiny, bez dalších samostatných structure-set mřížek.
- [x] Dvě nové 100% nehořlavé šablony netherových lodí (crimson a warped) jako *manuálně umisťovatelné* NBT; žádné wool/obyčejné dřevo.
- [x] Automaticky generované rizikové netherové stavby zředěny. Nether se **NESMÍ** vrátit k 2k; RC34 zachovává 1024.
- [x] **Před/po QA:** archiv CRC, JSON dekódování, všechny původní NBT beze změny, nové NBT validní včetně cesty a entit, cycle-free feature DAG a všechny světy zachovaly výšky.
- [ ] **Herní ověření v Horizon Rust** — stavby nesmí být uříznuté či zavalené, osadníci musí existovat, vegetace i rudné vrstvy v nové výšce se musí skutečně vytvářet.
- [ ] **Netherové lodě mají plout na skutečné lávě**: jejich náhodný spawn je pořád ZÁMĚRNĚ VYPNUTÝ. Požadavek není hotový bez full-footprint liquid validatoru v Horizonu.
- [ ] Vlastní nové registrované biomy ve F3, rozšířený enginový systém struktur/kolizí, nativní Photon, LOD a safe portal arrival.

[Podrobný RC34 audit](RC34_UNDERGROUND_VILLAGE_NETHER_NATURE_26_3.md).

## Část 5 — End (přijato 2026-11-08)

Původní zpráva uživatele:
> 💜 End
>
> Předělat hlavní spawn ostrov, přidat výrazně odlišné biomy a ostrovy, vlastní vegetaci a další struktury

Dodatečně potvrdil: „okej do toho“ a požadoval **kontrolu před/po, opravu chyb a kontrolu celého datapacku**.

### RC35 – implementováno staticky
- [x] Hlavní ostrov: vlastní 26.3 End `minecraft:end/islands` výšková funkce se **7 asymetrickými pobřežními výběžky**, zachované jádro dračí arény s nezměněnou density do radius 72.
- [x] Obnovena originální vanilla `minecraft:end_island_decorated` u `small_end_islands`, kterou starší override nechtěně vypustil.
- [x] Pět vzácných přírodních dekorací odlišujících existující Highlands, Midlands, Barrens a Small End Islands, + kalcitové ložisko a zachované starší purpur/amethyst akcenty.
- [x] 3 nové malé stavby (krystalový úkryt, purpurová zastávka, temná hlídka) s nativními materiály, jednou skromnou truhlou na stavbu, podkladovými pilíři a jednou sdílenou vzácnou strukturální mřížkou.
- [x] Druhý nezávislý TREK End City set vypnut, 3 vlastní přehnané variantní sety sjednocené/pozastavené, 5 dalších End skupin zředěno.
- [x] **Před/po:** ZIP CRC, parse 1619 JSON/meta, žádná chybějící aktivní vlastní structure ID, 0 duplicit salts, 0 feature-order DAG cyklů, všech 3 nových NBT kompletně nezávisle dekódováno a **všech 1174 původních NBT byte-identických**.
- [x] Zachována ikona `pack.png`, výška Overworld −64..2031, Nether 0..1023 a End 0..511.
- [ ] Herní test RC35: dragon fight, spawn platforma, výběžky, přírodní malé ostrůvky, vizuální rozdíl regionů, kolize a hustota ruin.
- [ ] Plnohodnotné nové biome ID s názvy ve F3 a kompletní nová End noise výška 512. RC35 používá stávající čtyři vanilla end biome ID a přirozený noise Endu 128.
- [ ] Kontrola podkladu celého půdorysu velkých End struktur a bezpečnost všech sky structures zůstávají nedokončené.

Podrobný [RC35 End audit](RC35_END_OVERHAUL_PRE_POST_AUDIT.md).

## Část 6 — Struktury (přijato 2026-11-08)

Původní uživatelovo zadání:
> 🏰 Struktury
>
> Dokončit využití dodaných map, správné umístění, větší rozmanitost, zabránit překrývání a prorůstání stromů.

Upřesnění: **Uživatel bude testovat až po dokončení celého seznamu**, takže u každé části provádět automatickou kontrolu před/po, především biome, Y, rarity/spacing, collision risk a loot.

### RC36 – provedeno (staticky)
- [x] Sloučeny TREK overworld common/medium/very_common do jedné weighted strukturové mřížky; méně nadměrného překryvu, stále varianty různých kategorií.
- [x] Deset tematických hlídek a outpostů sloučeno do jednoho společného výběru namísto deseti nezávislých mřížek.
- [x] Z původně vypnutých importovaných map znovu povoleny **3 menší overworldové budovy** s biome filter, `WORLD_SURFACE_WG`, `beard_thin` terénní adaptací a rare setem.
- [x] Oceánské lodě zúženy z 16 na 8 typů a mají velmi vzácný společný spawn pouze v nezamrzlých hlubokých oceánech, keel Y68 vs sea level 70.
- [x] Rizikové `sunken_city`, `coves`, 2 ocean-trade a 2 random-height Nether overworld-leak struktury vyřazeny z natural spawn; všechny template NBT zůstávají.
- [x] Pět skupin stromů má vyloučení pokusů o umístění v okolí hlavní TREK mřížky; nejde o plnou ochranu před všemi vanilla stromy.
- [x] Zkontrolované tematické inventáře v 3 znovu aktivovaných budovách a 8 lodích: NBT i platné item sloty; žádná z 11 šablon nemá všechny truhly bez lootu.
- [x] **Před/po statická kontrola:** ZIP CRC, všechny JSON, vlastní structure-set/structure/pool/NBT reference, rozestupy, exclusion DAG, 1177 nezměněných původních NBT a custom ikona, všechny rozměry světa zachované.
- [x] **Katalog původně importovaných map:** 103 NBT; 19 aktivních v natural worldgen, 84 zachováno pro pozdější bezpečné umístění.
- [ ] **Kompletní využití všech map NENÍ HOTOVO:** velké stavby nemají nativní kontrolu celého půdorysu; zde ponechány manual-only.
- [ ] **Lodě jen když kapalina pod CELOU lodí:** stále chybí skutečný validátor voda/láva, 8 zbývajících lodí nejsou garantovaně na volné hladině. Nether lávové lodě zůstávají vypnuté.
- [ ] **Úplná ochrana proti prorůstání** všech stromů a kolizím všech typů staveb vyžaduje engine-level full-footprint checks.
- [ ] **Kompletní revize starého lootu** nebyla provedena; statický full NBT loot scan narazil na časový limit. Tato část stále čeká.
- [ ] Herní test až po dokončení celého seznamu, jak si uživatel přeje.

Podrobný audit: [RC36](RC36_STRUCTURE_PLACEMENT_SAFETY_PRE_POST.md).

## Část 7 — Dynamický loot (přijato 2026-11-08)

Původní zpráva uživatele:
> 💎 Loot
>
> Dynamický loot podle stavby, biomu, dimenze a vzácnosti; žádný loot v čistě dekorativních barelech

Požadováno povinné **ověření před/po, zda se nic nerozbilo**. Testovat v samotné hře až po dokončení celého seznamu částí.

### RC37 – staticky implementováno
- [x] Kompletní sken **1 177 NBT**, **2 847 kontejnerů** (včetně 1 568 barelů), všech původních `LootTable` referencí.
- [x] **1 175 starých pevných loot truhel** převedeno na nové 26.3 `LootTable` ID; **66 nových tematických tabulek** pro vlastní mapy a TREK v Overworldu, Netheru a Endu.
- [x] Náhodná odměna dle typu stavby, dimenze a relativní rarity; **přímo z biome pozice** doplňková vzácná odměna přes `minecraft:location_check` s `predicate.biomes`. Pozor: rarity zatím typ/role odvozená z názvu, není přesně měřenou pravděpodobností skutečného spawnu.
- [x] **530 opakovaných dekorativních barelů** bez předmětů, **205** uvěřitelných skladových barelů se střídmými náhodnými zásobami. Celkem **1 068 prázdných barelů bez LootTable**.
- [x] Zachováno všech **371 původních LootTable odkazů**, všech 128 původních loot JSON, vanilla NBT, autorsky speciální předměty.
- [x] **Před/po QA + nezávislá kontrola:** CRC ZIP, dekódování všech NBT a všech 1673 JSON/meta, 431 NBT změněných pouze v `Items`/`LootTable` tagech, **bajtově identické všechny ostatní NBT části všech 1177 šablon**, stejná worldgen data, dimenze a `pack.png`, všechny nové table refs platné. Vyvážení přibližně simulováno na 2 500 loot otevřeních pro čtyři úrovně.
- [ ] **Horizon Nightly runtime podpora loot tabulek v world datapacku zůstává neověřená**; mod-browser docs dokonce uvádí částečnou podporu, zatímco portované modpacks mají vlastní loader. Bez runtime testu nelze označit nové truhly za funkční, proto RC37 není stabilní.
- [ ] Heuristické rozdělení dekorativních barelů podle četnosti může chybovat; vizuální klasifikaci všech 1 568 kusů v konkrétním světě je třeba potvrdit po dokončení seznamu.
- [ ] Plná map-to-structure efektivní četnost rarity a rozlišení každé speciální struktury může ještě chtít ruční korekce.

[Technický dokument RC37](RC37_LOOT_DYNAMIC_26_3_PRE_POST.md).

## Část 8 — Mobové + náhodný loot mezi kopiemi struktur (přijato 2026-11-08)

Původní požadavek:
> a jo at se ten loot samo i meni at toneni stejny v ty strktury kdz junajdu znovu ... 🧟 Mobové
>
> Vlastní osazení struktur, přiměřená obtížnost, správní nepřátelé a pasivní mobové

Testování až **po dokončení celého seznamu**, povinná statická kontrola **před i po**.

### RC38 – provedeno
- [x] U **35 aktivních structure JSONů** přidána nativní `spawn_overrides` kategorie podle typu/prostředí; **26 hostile** + **9 creature**. Pouze `bounding_box=piece`, `minecraft:constant=1` na pokus, ne plošný spawn přes okolí.
- [x] Příklady: huskové v pouštních ruinách; drowned v bažinách; pavouci v džungli; pillageři na hlídkách; zombíci v opuštěných domech; endermani v End ruinách; magma cubes u Nether pilíře. Farmy s ovcemi/slepicemi, tajgové domky s liškami, oáza s králíky, beehive house se včelami.
- [x] V NBT podzemní osady fyzicky připojena **1 kočka + 1 železný golem** ke třem původním villagerům. Původní entity a všechny bloky/loot zachovány.
- [x] Původní struktury vanilla, TREK villages, Deep Dark, End origin/archipelago a další existující odpovídající spawn override nezměněny.
- [x] RC37: **všech 66 nových loot tabulek** má více vážených výsledků a nemá pevný `LootTableSeed` ve vlastních NBT truhlách. Simulace 16 různých RNG seedů pro každou tabulku ověřila rozdílné možné výsledky. **Každá nová truhla se generuje jednou** při prvním použití; stará otevřená se sama znovu nenaplní a dvě různé kopie se náhodou mohou shodovat.
- [x] **Kontrola před/po + nezávislý NBT validátor**: 2947 členů ZIP, všech 1673 JSON, 1177 NBT, pouze 35 structure JSON doplněno o `spawn_overrides` a **1 NBT změněná jen o dvě nové entity**; všechny jiné bajty, worldgen, strukturální sady, ikona, loot, dimenze Overworld −64..2031 / Nether 0..1023 / End 0..511 zachovány. ZIP CRC PASS.
- [ ] **Herní test/obtížnost:** `spawn_overrides` neurčuje skutečný počet mobů, jen budoucí spawn pokusy v boxu. Pasivní `creature` override se nepoužije na první zvířata při generování chunku. Proto nové umístění ještě není potvrzené v Horizon Nightly.
- [ ] **Runtime lookup loot tabulek** běžných world datapacků v Horizonu nezaručený (stejná otevřená výhrada RC37), skutečná variabilita zatím staticky prokázaná, ne in-game.
- [ ] Nová entitní ID nejsou implementována, všechny moby jsou původní Minecraft.

Dokumentace: [RC38 audit](RC38_MOB_ECOLOGY_LOOT_VARIATION_PRE_POST.md).

## Část 9 — Vanilla struktury ve vyšších výškách (přijato 2026-11-08)

Původní zadání:
> ⚙️ Vanilla struktury
>
> Generování i ve větších výškách rozšířeného světa, ne pouze v původních vrstvách; zase ověření.

Uživatel **provede herní test až po dokončení celého seznamu**; zde povinně jen statická kontrola před/po.

### RC39 – implementováno staticky
- [x] Před úpravou potvrzeno, že **vanilla villages a pillager outposts už přirozeně používají povrchovou výšku**, nepotřebují změnu globálního limitu Y.
- [x] **4 vzácné nové varianty vanilla 26.3 strukturálních poolů** v názvosloví `thalvoryn:vanilla_highlands/...`:
  - `meadow_hamlet` — plains village na `meadow`, surface-relative start, spacing92/freq0.23;
  - `snowy_hamlet` — snowy village v `grove/snowy_slopes`, surface-relative, spacing100/freq0.22;
  - `ridge_outpost` — vanilla pillager outpost na `stony_peaks/windswept_gravelly_hills`, surface-relative, spacing116/freq0.18;
  - `mountain_trial_chambers` — vanilla trial-chamber pool v `jagged_peaks/frozen_peaks` **76 až 112 bloků pod aktuálním horským povrchem**, spacing128/freq0.15.
- [x] Méně rozlehlé jigsaw sady a výjimečné zóny vůči vanilla villages/outposts/strongholds, aby se omezilo hromadění staveb.
- [x] Existující Nether bastion range **Y90–510 → Y90–704**, beze změny původní setové četnosti, Nether stále **0–1023 bez stropu**.
- [x] **Před/po audit + nezávislý validátor:** 2947→2956 členů, 1673→1681 JSON/meta, **všech 1177 NBT** a **všech 194 loot tabulek byte-identických**, změněné pouze existující `minecraft:bastion_remnant` a chybně zastaralé `pack.mcmeta`, jiná původní data/biomes/features/height/loot/mob rules stejné. ZIP CRC PASS.
- [ ] Skutečný herní test generování v Horizon Nightly a kontrola svahů **až po dokončení seznamu**, bez ohledu na to, že statický test prošel.
- [ ] **Nether bastion Y704 může být bez podkladu**: absolutní výška bez full-footprint terénní validace. Pokud se ve hře ukáže závada, vrátit rozsah do bezpečnějšího pásma nebo doplnit engine validátor.
- [ ] **Biome není číselný výškový filtr**: nová horská osada nemá absolutní min Y zaručené, pokud existuje biome mimo typicky vysoké terény.
- [ ] Nativní vanilla mineshafts/fortresses/strongholds používají zvláštní interní generátory, nelze je jen přepsat obecným `start_height`; původní funkční pravidla zachovaná.
- [ ] Ostatní otevřené úkoly: jigsaw full footprint, RC37 runtime loot, Nether lava ships, portal safety, Horizon LOD, Photon.

**[RC39 podrobný audit](RC39_VANILLA_STRUCTURES_HEIGHT_PRE_POST.md)**.

## Část 10 — Horizon Rust (přijato 2026-11-08)

Původní uživatelské zadání:
> 🎮 Horizon
>
> LOD, bezpečný Nether portál, vlastní biome/cave jeskyně ID ve F3, kompatibilita vysokých světů a výkon. Zase ověření a kontrola.

**Uživatel bude herně testovat až po dokončení celého seznamu.** Všechny nehotové engine opravy zůstávají otevřené; nesmí být falešně označovány za hotové.

### RC40 — bezpečná diagnostika + statická validace (NEZAMĚŇOVAT za opravený engine)
- [x] Kompletní srovnání RC39/RC40: z původních 2956 ZIP souborů změněno pouze `pack.mcmeta` (RC40), přidáno pět nových diagnostických/README souborů, **všech 2955 ostatních starých souborů byte-identických**.
- [x] Zachováno všech **1177 NBT**, **194 loot tabulek**, ikona, původní terén a biome, mobové i struktury.
- [x] Výšky a noise codec: **Overworld −64..2031 / 131 sekcí**, **Nether 0..1023 bez stropu / 64 sekcí**, **End 0..511 / 32 sekcí**. `min_y`, `height`, `logical_height`, noise (OW/Nether) správně odpovídají MC26.3; `min_y+height<=2032`.
- [x] Čtyři **ručně spustitelné diagnostické funkce** `/function thalvoryn:horizon/status`, `cave_biome`, `portal_hazards`, `lod_notes`. Bez `tick/load`, teleportů a změn bloků. Vrací skutečná vanilla biome ID, nikoli nové zaregistrované ID.
- [x] Samostatná **read-only preflight diagnostika** ZIP/JSON/height/refs/LOD settings. Její **6 syntetických/regresních testů PASS** včetně zamítnutí špatného vysokého Netheru a varování u 16k LOD.
- [x] Oddělený **referenční** portal full-footprint validátor s **9 umělými unit testy PASS**, **NENÍ ZAPOJEN** do Rust enginu ani samotného ZIP datapacku.
- [ ] **Vlastní biome a cave ID zobrazená ve F3**: Horizon worldgen ignoruje nové biome ID z běžných datapacků, potřebuje registraci do Rust client/server a F3 rendereru.
- [ ] **Bezpečné skutečné průchody portálem do Netheru Y0–1023**: host portal-forcer musí hledat validní pevný podklad, volný prostor a vyloučit kapalinu pod celou plochou; zatím neupraven.
- [ ] **LOD white/black terrain**: zobrazení neobjevených struktur v synthetic far tiles, mismatch po přiblížení a černá obrazovka při extrémní vzdálenosti jsou engine/renderer problémy. Doporučený dočasný profil pro RX580: render 8–12, sim 5–6, LOD 128–256, Far Detail Reduced.
- [ ] **Výkon/TPS/FPS/VRAM** v přímém Horizon Nightly pro vysoké svety: runtime neověřen, statické checks neprokazují FPS.
- [ ] **End natural noise** má stále vanilla 128, i když se v Endu může stavět do 511; plné vyšší přirozené End ostrovy nejsou implementované.
- [ ] **Custom loot tables z RC37** stále potřebují potvrdit Horizon runtime loader (194 JSON existuje, ale to neprokazuje naplněné truhly).
- [ ] Konečný in-game test až po dokončení celé sady požadavků.

[RC40 kompatibilita a technické hranice](RC40_HORIZON_ENGINE_COMPATIBILITY_AUDIT.md).

## Další části
Čekají na postupné zaslání uživatelem. Neoznačovat dosud nezaslané části jako dokončené.
