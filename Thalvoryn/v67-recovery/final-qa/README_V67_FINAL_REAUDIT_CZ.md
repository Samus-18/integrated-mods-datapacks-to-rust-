# Thalvoryn V67 — audit lootu, kategorií, rozmístění a návaznosti na terén

**11. 11. 2026 — TESTOVACÍ SOUKROMÁ VARIANTA, NE FINÁLNÍ GAMEPLAY RELEASE.**

Čtyři archivy: Minecraft Java / Horizon Rust, Nether 1024 / 2032. Oprava vychází z existujícího `V67_ECOLOGY_PRIVATE_TEST`, nikoli z jiného buildu. Původní čtyři ZIPy zůstávají bez změny.

## Co se reálně změnilo

1. **Loot 26.3:** 17 původních loot tabulek bylo převedeno na novou syntaxi `modifier` / `condition` / `type` v odpovídajících kontextech. Zároveň byly založeny **14 existujícím lootem podložené aliasy** pro 18 chybějících LootTable referencí v NBT. Nevymýšlí se nový náhodný loot. Dekorativní sudy se neplní.
2. **Biome/umístění:** 32 souborů definic struktur mělo biomy, které v aktuálním zdroji dimenze nebyly; byly přeřazeny do odpovídajících aktivních biotopů. Celkem **11 nesprávných kategoriálních přiřazení** bylo opraveno (Nether/Overworld; hloubkové vs povrchové stavby; mořský monument přesunut k oceánským strukturám). Zastaralé odkazy na vanilla biome tagy zůstaly zachovány tam, kde existují v 26.3.
3. **4 nefunkční importy deaktivovány:** `nova_structures:illager_manor`, `nova_structures:nether_keep`, `nova_structures:village_jungle`, `nova_structures:village_swamp`. Jejich root jigsaw pool ID nejsou ani v těchto ZIPech, ani ve vanilla 26.3. Soubory struktur v ZIPech zůstávají pro pozdější opravu, ale *netvrdíme*, že se nyní generují.
4. **Zapuštění:** šest strmých podzemních tunelů používá nový výškový offset relativní k povrchu a `beard_thin`, aby byl vstup blíže stěně a zemní adaptaci. Čtyři povrchové netherové stavby používají `WORLD_SURFACE_WG` plus Rust `horizon:start_checks`. Šestnáct importovaných mořských lodí má pozici u hladiny nastavenou konzistentně. **Toto není záruka bezkolizního dosednutí na každém seedu.**
5. **End:** třináct vnitřních End jigsaw poolů bylo upraveno, odstraněno 75 námořních/dokových kandidátů v End Castle a příbuzných zdrojích. Vanilla End City loď zůstává. Custom pirátské lodě v Netheru a Endu nejsou aktivní.
6. **Ochrana:** žádná z **7 300 NBT šablon** nebyla změněna. Nejsou přepsány Overworld density funkce, tedy neřeší se touto opravou návrat k binárně identickému originálnímu JJThunderu. 69 truhel se záměrně prázdným seznamem `Items: []` je zachováno — jejich automatické naplnění by bylo neodůvodněné.

## Co nová nezávislá kontrola potvrzuje

- **4/4 ZIPy:** CRC bez chyby, bez duplicit, stejných **15 405 položek**, z toho **7 300 NBT**, rozdíly Java/Rust respektují native Horizon start checks.
- **480 aktivních nevanilla definic** (549 souborů definic celkem) a každá aktivní právě v jedné skupině. Čtyři neúplné importy jsou neaktivní.
- **1 234 loot tabulek**, **820 vnitřních loot závislostí**, 0 chybějících interních cílů, 0 cyklů odkazů. Vlastní chybějící LootTable ID z auditu NBT jsou pokryta aliasy.
- **0 chybějících aktivních jigsaw start poolů** po ověření proti obsahu ZIPu a známým vanilla 26.3 poolům.
- Podle statické kontroly jsou platné cílové aktivní přímé biome ID, délky výšky Nether/End i základní syntaxe vlastní loot struktury.

## Co stále nelze označit za ověřené

- Neproběhl běžící Minecraft Java 26.3 ani Horizon klient. Není ověřené vytvoření světa, End Dragon/gateway, portály, FPS/TPS/LOD, fyzické otevření truhel či přirozený spawn.
- **64 jigsaw definic nemá ani projekci na výškovou mapu, ani terrain adaptation**; řada z nich jsou záměrně ostrovy v Endu, mořské stavby na definované hladině a hluboké dungeony. Bez hry nelze potvrdit, zda všechny velké stavby dosedají čistě na zem.
- Šest tunelů vede do vlastních vytvořených komor, nikoli zaručeně do libovolné přirozené procedurální jeskyně.
- Mnoho importovaných struktur pochází z cizích datapacků/JARů; tento soukromý test není oprávnění k veřejnému redistribuování jejich dat. Na veřejném GitHubu jsou jen skripty a zprávy, ne 35 MB ZIPy.

## Doporučená herní zkouška (nejprve RUST 1024, v novém světě)

```mcfunction
/datapack list enabled
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:overworld run locate structure nova_structures:trident_trial_monument
/execute in minecraft:overworld run locate structure thalvoryn:overworld/handmade/air_vessel
/execute in minecraft:the_nether run locate structure nova_structures:nether_port
/execute in minecraft:the_end run locate structure nova_structures:end_castle
/execute in minecraft:the_end run locate structure minecraft:end_city
```

V každé stavbě otevřít minimálně 3 různé truhly a barely; pozorovat dosednutí na terén/ostrov a porovnat 3 různé seedy. U End Castle zkontrolovat, že se v jednotlivých jigsaw částech negenerují pirátské doky. Pokud se hra zasekne nebo struktura visí, doložit XYZ, seed a video.

**Závěr:** opravena řada prokazatelných datových chyb; statické kontroly jsou čisté. Prohlašovat kompletní herní kompatibilitu či 100% bezproblémové blending by však bylo nepravdivé.