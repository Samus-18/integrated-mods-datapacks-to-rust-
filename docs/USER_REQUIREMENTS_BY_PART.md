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

## Další části
Čekají na postupné zaslání uživatelem. Neoznačovat dosud nezaslané části jako dokončené.
