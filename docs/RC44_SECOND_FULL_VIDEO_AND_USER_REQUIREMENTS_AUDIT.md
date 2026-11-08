# Kompletní druhý audit tří videí a všech uživatelských požadavků

**2026-11-08 | Horizon Rust + Thalvoryn/TREK (dříve JJThunder) | stav po RC44.**  
**Tento dokument je kontrolní seznam, NE potvrzení opravených chyb ani nový testovací ZIP.** Všechny tři uživatelem poskytnuté MP4 byly znovu prohlédnuty po celém trvání, v časové mřížce po 2 sekundách a kritická místa ve vyšším rozlišení. Původní MP4 ani snímky nejsou bez výslovného souhlasu veřejně distribuovány.

Reference: [historický seznam částí 1–10](USER_REQUIREMENTS_BY_PART.md), [první audit RC43](RC43_VIDEO_TEST_THREE_CLIPS_ISSUES.md), [změny RC44](RC44_P0_CAVES_END_SHIPS_PHYSICAL_LOOT_STATIC_QA.md), [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

## A. Co skutečně ukazují videa — časová evidence

**Overworld — `horizon-client_SBbUyuE8LT.mp4` (92,4 s):**
| Čas | Pozorování (video) | Co s tím / hranice důkazu |
| --- | --- | --- |
| 0–2 s | Truhla v domu: jediná mrkev a dva malé stohy pochodní; neodpovídá dojmu hodnotné odměny | Měřit loot podle role stavby, nikoli jen „není prázdno“; nepopisovat jako zcela prázdnou |
| 6–16 s | Dům/vesnice mezi hustou džunglovou vegetací, konstrukce u vody a pod blízkými listy | Výšková/povrchová kontrola přístupu, bezpečné cesty a vegetační odstupy |
| 18–21 s | V jednom domě je vidět úzký otvor směrem dolů | Prověřit jeho průchodnost/účel; z videa samotného není jisté, zda jde o chybu |
| 22–28 s | Stromy a bambus téměř v dotyku budov; kolem 24 s **hoří velký strom** a v těsné blízkosti domů je výrazná láva | Nové důležité P0/P1: **riziko fire/lava v sousedství dřevěných staveb**, otestovat příčinu (přírodní lava spring vs mapa), ochranné zóny |
| 28–40 s | Listy, bambus/tráva těsně u verand, plotů, oken a v chodbách | Reálný clearance test kolem DOORS/PATHS a výšky 2–3 bloků, nejen exclusion jigsaw mřížky |
| 46–50 s | Velmi ostré bílé pruhy přes terén; viditelná hranice v dálce | Srovnat stejný seed s vypnutým LOD a render/simulation vzdáleností; neprohlašovat bez důkazu za „cave density“ |
| 56–64 s | Opevněná ruina s illagerem; hlídaný barel má **1 iron ingot** | Nepřiměřený loot k nebezpečí a velikosti, souvisí s fyzickým loot bake RC42; RC44 má jen statickou opravu |
| 66–71 s | Velké úseky otevřenějšího, relativně repetitivního povrchu bez snadno rozeznatelných bodů zájmu | Počet struktur ověřovat podle plochy a kategorii biotopu, nikoli spolehnout se na `spacing` |
| 72–92 s | **Obří dutiny**, téměř rovné několikadílné vysoké stěny, levitující úlomky/ostrůvky, daleko dole světle prázdná oblast, tenké struktury u stěn, výrazné rozdíly načítání | Kritická geometrie v oblasti přibližně Y110–180; rozlišit noise/carver problém od spectator cull / distant mesh. RC43 začínala cave taper až Y320; RC44 upravila matematickou váhu od Y92, **herní výsledek neznámý** |

**End — `horizon-client_NzmvzLFg7d.mp4` (35,03 s):**
| Čas | Pozorování | Co s tím |
| --- | --- | --- |
| 0–14 s | Velké ostrovní oblasti skoro výhradně **End Stone**; nahoře velmi husté malé skoro stejně tvarované fragmenty | Není to nový živý/diferencovaný End; potřebné rozdílné ostrovní tvary, profily a přírodní akcenty, nikoliv pouze nižší spawn chance |
| 14–23 s | Velká importovaná dřevěná loď je podstatnou částí trupu nad void/úzkým okrajem ostrova | RC44 **odstranila jen náhodný structure set 8 lodí**. Skutečný whole-footprint support **stále není**. Zachovat šablony, nevydávat za opravený placement |
| 24–35 s | Další velmi podobné oválné ostrovy a téměř stejné malé úlomky, místy velká prázdnota mezi zajímavými stavbami | Měřit varietu tvarů a reálnou hustotu landmarků; samotné zředění malých `minecraft:end_island` v RC44 problém diverzity neřeší |

**Nether — `horizon-client_vTVgXbtTcR.mp4` (72,7 s):**
| Čas | Pozorování | Co s tím |
| --- | --- | --- |
| 2–12 s | Velká hranatá, pevnosti podobná konstrukce / dlouhé hranaté plochy nad lávou; několik viditelných struktur existuje | Netvrdit „Nether nemá žádné struktury“. Prověřit full-support / air gap a design po jednotlivých typech |
| **13–21 s** | **Více (viditelně několik) velmi podobných purpurových/crimson lodí či plošin v jednom lávovém jezeře**, na jedné výrazný velký glowstone kříž; opakování těsně vedle sebe | **NOVÁ KRITICKÁ EVIDENCE proti dosavadnímu auditu:** problém také *lokální spam a slabá variabilita*, nejen absence. Není bezpečně určeno konkrétní NBT šablonou; `lava_skiff_*.nbt` nemají glowstone, a tedy kříž nedokazuje právě tento typ |
| 22–40 s | Rozlehlé jednotné crimson lesy a velmi dlouhé plynulé kopce/stěny; jen vzácně odlišné místo | Podmínky rozmístění landmarků v celých výškových pásech, nejen v 2D rastru; rozmanitější biomové přírodní druhy |
| 42–56 s | Prudké hranice crimson–warped, rozsáhlé jednobarevné warped porosty; velké ploché horizonty | Zlepšit míchání palet, mikrobiotopy, nepravidelnou zástavbu / reliéf, a přitom zachovat styl dimenze |
| 58–66 s | Rozsáhlá skoro holá černo-šedá plocha u lávy a bez charakteristických orientačních bodů | Změnit material/vegetation/landmark rozmanitost; ne uniformní náhodný spam |
| 68–72 s | Jediná izolovaná věž v jinak prázdnější oblasti | Přidat logické malé/mid-site typy na správnou podložku a Y |

### Zdrojové souvislosti ověřené v původním RC43 ZIPu

- `thalvoryn:nether/lava_skiff_crimson` / `warped`, **každý má zvlášť nízkou a vysokou placed_feature** (`..._high`): každá obsahuje `count: 3` a `rarity_filter chance: 10, 12 nebo 19`. To je **potenciální** zdroj lokálního shlukování, ale z videa **nelze jednoznačně přiřadit NBT k jednotlivé lodi**; glowstone kříž v záběru není důkaz RC43 malého skiff modelu. Při úpravách snižovat lokální duplicity pro všechny ship varianty / přírodní features i současné 4 kanály.
- `airships_rc23` používal absolutní Y145–190 bez podpory pod trupem; RC44 jej vypnula, nicméně nezavedla bezpečné nové umisťování.
- `minecraft:end_island` stejná šablona pro mnoho drobných vysokých End úlomků; RC44 změnila pouze rarity 22→36.
- Distant bílé pruhy a světlo pod jeskyněmi: rozdíl render/spectator/LOD nelze jen z videa uzavřít; nutný opakovatelný in-game test s přesným seed a nastavením.

## B. ÚPLNÉ zadání uživatele — nejen poslední tři opravy

Historická zadání **částí 1–10 i doplnění z uživatelových testů** zůstávají závazná. „Staticky přidáno“ není synonymum „funkční/ve hře ověřeno“.

| Oblast | Povinné požadavky | Stále otevřeno po RC44 |
| --- | --- | --- |
| **1 Overworld, highlandy a Sky Islands** | JJThunder/MAX styl s vysokými horami, výrazně různorodé biomy a jeskynní vstupy, sky islands cca Y110–180 podle POVRCHU (bez levitace nesmyslně vysoko), tvary i barevné/vegetační varianty, funkční terénní blending | Geologické dutiny/stěny, vizuálně funkční cave entrances, F3 vlastní biome ID, prověřená mapa malých ostrovů a vysokých hor |
| **2 Stromy, vegetace** | Použít všechna vhodná custom schemata/stromy, nové lesní typy, přirozená vegetace a distribuce, dveře/cesty/okna neblokovat, stromy nesmí růst skrz budovy ani loď, neopakovat jediné typy | Full footprint clearance, fire/lava safety, posoudit stromovou hustotu u stavby z videa |
| **3 Jeskyně** | Kromě zelených Lush minimálně výrazně odlišné podzemní krajiny (Dripstone/Deep Dark + vlastní styl), vlastní vegetace, minerály, přirození mobové, unikátní ruiny, podzemní TREK dungeon častěji; **otvory na povrchu se skutečnou cestou dolů** | Nejen obří dutiny; přístupnost, biomová diverzita skutečně pozorovaná, skutečné vygenerované vstupy do cave a lepší ověřené podzemní site umístění |
| **4 Nether 1024** | Bez bedrock stropu jako zvolená varianta, bloky/nativní biomy i vysoko (~Y700), živé rozdílné blackstone/basalt/crimson/warped/soul palety s plynulým blendingem, méně uniformního basalt/soul sand, více vhodných vegetací, 1k výšková varianta; rozumné rozmístění quartz/glowstone/gold/debris do všech výšek s nižší šancí nahoře; mnoho staveb i vysoko, bezpečné lávové lodě | Širší funkční 3D průzkum, různorodost, safe portal, nové skutečné biome IDs, lodě jen s plnou lavovou podporou, **bez 5 téměř stejných lodí v jednom jezeře**, bezpečné materiály |
| **5 End** | Předělat hlavní ostrov, ale zachovat dragon fight/spawn, více opravdu různých oblastí/povrchů a hornin, malé i velké nepravidelné ostrůvky, přirozená End vegetace a mobové, rare stavby s lootem, žádné „copy/paste“ ostrovy, nadměrné chorus a náhodné pruty, poctivá 512-world-height logika | End noise stále tradiční přibližně 128; vlastní F3 biomes nepřidány, drobné ostrovy stále geometricky stejné; safe ship/structure support |
| **6 TREK + originální/importované struktury** | Zahrnout širokou knihovnu dodaných map do vhodných biomů, podle dimenze a Y, různé typy, variace; povrch i podzemí častěji **cca 2–3× proti řídkému testu**, bez cluster spamu či hromady identických typů; lodě do vody/lávy nikoliv les/pole/void, velké stavby celé podepřené nebo záměrně aerial, neprolínat blízké stavby, terén, stromy, bez useknutých budov | 84/103 imported map NBT zůstalo manual-only; skutečná celá kolizní obálka, biome/height, voda/láva, svahy a lokální cluster diversity otevřené |
| **7 Loot** | Opravdové předměty v truhlách a funkčních skladových barelech; čisto dekorativní barely prázdné; **odpovídající typ stavby/biom/dimenze/vzácnost/nebezpečí**; ne 1 ingot, ne 3x mrkev, ne nesmyslně OP; různé kopie staveb různý loot, dvoutruhla hodnotit společně | RC44 jen vylepšila fyzické Items: dynamický in-game RNG není prokázán, existence custom LootTable v ZIPu sama nestačí, označení decorative vs storage nebylo vizuálně validováno pro všechny mapy |
| **8 Moby** | Pasivní a nepřátelští podle prostředí a obtížnosti, wildlife v přírodě i obsazení staveb, podzemní vesnice se skutečnými villagery, tam kde to dává smysl; Deep Dark nepřeplnit | Native spawn overrides nejsou in-game potvrzené; žádné skutečně nové mob IDs ve vanilla datapack systému |
| **9 Vanilla struktury a výška** | Vanilla vesnice/outposty/chambers/bastiony podle správného surface/height přes rozšířený overworld/nether, ne pouze Y~100/256; lodě ne na souši; zachovat původní logiku a samostatné rarity | Většina stále potřebuje seed-local in-game validaci a často runtime umístění |
| **10 Horizon engine + Photon** | Vysoký svět v reálné hře (Overworld −64..2031, Nether 0..1023, End 0..511) a přenesený dimension type; kompatibilní svět, LOD bez černé obrazovky při 16k, žádné bílé/černé distant chybné plochy, bezpečný Nether portál; vlastní biome ID ve F3, vysoké FPS/TPS, kompatibilita s Horizon Nightly. **Photon pouze Rust, vlastní Horizon shader systém, žádný Java/Iris/OptiFine backport**, správné světlo, End sky, stromy, LOD | Runtime renderer / portal code nebyl opraven buildem RC44, stejně tak WGSL Photon ani nový biome registrar. Rozlišení static pass vs actual tested-nightly povinné |

## C. Dřívější pravidla pro předání a souběžné nástroje

- Dodávat **Java i Horizon Rust** bundly (Rust build se suffix `-RUST`), a dvě dohodnuté výškové varianty (Nether 1k a alternativní vyšší), stejnou vlastní pixel-art ikonu a metadata, README, instalační a testovací příkazy / locate/spawn. **Neoznačovat RC44 za dokončenou Java ani 2k variantu.**
- Ověřenou verzi uvádět přesně **„targeted/tested on Horizon Nightly 2026-11-07, 2ebf00a / MC26.3“ pouze pokud proběhl skutečný test**. Na RC44 nebyl, proto se smí říkat jen *targeted, not yet runtime tested*. Ostatní nightly verze neověřené.
- Github dokumentaci / pracovní postup uchovávat; nereleasovat cizí mapové NBT, dokud nejsou vyjasněné licence a atribuce. Uživatelův projekt pro přátele. Žádné veřejné video uploady bez souhlasu.
- Další požadovaný produkt **Windows EXE converter/manager** (ne web): vlastní Horizon OLED UI, 4 sekce Shader Converter, Datapack Converter, Shader Manager, Datapack Manager, volitelná Horizon složka, výstupy `-RUST`, 2 input slots, Library manager/aplikace na všechny světy, přepnutí shaderů a test kompatibility. Není součástí worldgen RC44 a dosud nebyl potvrzen hotový.
- **Nedávat nehotové ZIPy**, neměnit uživateli starý save bez aktuální zálohy; na jednom seedu ověřovat in-game až po celé sadě. V každém RC před/po CRC, JSON, NBT, entity/loot, strukturové odkazy, feature DAG, výšky, ikona a pokud možno skutečné generování.

## D. Co RC44 skutečně řešila a co NE

**RC44 staticky:** matematicky tlumila cave influence od Y92; vypnula jedinou nebezpečnou End airship mřížku a zředila malé End úlomky; fyzicky zlepšila slabý loot (2 033 inventářů v 471 NBT, 901 ostatních NBT nezměněných). **Nevyřešila** opakování a správnou hustotu Nether lodí, oheň/lávu u osady, vegetační kolize, End biome diverzitu, podzemní přístup/variety, společnou hustotu struktur, plnou validaci umístění, runtime náhodný loot, LOD, portály, Photon či nový Rust engine.

### Další technické kroky (bez nového neověřeného „final“)
1. **P0:** Jedna konzistentní **site/clearance politika** pro všechny struktury: category GROUND / WATER / LAVA / AERIAL / ISLAND, celý půdorys + výška, sklon, nosná vrstva, výška tekutiny, vstup a buffer vůči stromům/lávě/kolidujícím stavbám. Nestandardní velké typy dočasně manual-only, ne falešné automatické umístění.
2. **P0:** Identifikovat v source konkrétní opakované Nether boat templates z videa na základě world save/NBT pokud dostupné, odděleně prověřit čtyři nové skiff placed features (`count:3` + rarity 10/12/19) a pravidlo *min same-template separation* přes celé lávové jezero. **Celkovou kvótu struktur nelze jen snížit**: zřídit mix malých bezpečných míst a 3D rozmístění, které navýší smysluplný průzkum bez místních klastrů.
3. **P0:** Cave density porovnat v běžném/spectator režimu na stejném seedu a **s/bez LOD** kolem Y110–180, kromě vysokého terénu 320–760; měřit stěny/caves, nepoužívat jen analytické gradienty.
4. **P1:** Zvýšit **skutečnou tvarovou** rozmanitost End a Nether, palety/floating-island noise, ne pouze rarity. Zmapovat podzemní vs povrchové starts a feature body samostatně a rozdělit podle biomů/vertikálního pásma.
5. **P1:** Průchody budov (doors/paths/staircases/bridges) a fire/lava buffer, skutečné podzemní cesty od povrchu, lootové kontrolní případy (hlídaný barel, obyčejný dům, double chest, dekorace, rare dungeon), případně realistický RNG způsob podporovaný Rust engine.
6. **Engine queue:** Rust LOD black/white, full-footprint site validator a portal forcer, custom biome registrar/F3, high End terrain, Photon Rust, runtime TPS/FPS. Testovat proti aktuálnímu enginu, ne předstírat podporu pouze datapackem.

**Akceptace:** porovnání v čerstvě generovaném světě a ověření uživatelem. Až potom případné Stable. Tento audit novou opravenou verzi NENAHRAZUJE.
