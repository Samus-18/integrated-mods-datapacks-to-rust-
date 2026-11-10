# Thalvoryn V67 – integrovaná testovací oprava (10. 11. 2026)

**TEST BUILD – NEVYDANÉ.** Určeno pro Minecraft Java 26.3 a Horizon Rust (cílový testovací commit 2026-11-07 `2ebf00a`). Kontroly jsou statické. Tyto soubory **nebyly spuštěny ve hře**. Testovat ve zcela novém světě, nespojovat různé výškové varianty v jednom světě.

## Co bylo upraveno

- Neaktivní definice Thalvorynu: aktivovány importované velké lodě, ruiny a podstatná část dříve nedostupných struktur (6 nových kategoriálních spread setů). `air_vessel` je výslovně zaregistrovaná velká vzducholoď.
- 6 nových nepravidelných sky islands s vystupujícím, zúženým kamenitým spodkem (místo malé ploché desky); celkem 6 nových rozměrných NBT ostrovů a staré varianty ponechané v nižší váze.
- 6 různých strmých tunelů do podzemí. Každý vyústí do vlastní, explicitně vytesané podzemní dutiny. **Nejde o zaručené spojení s náhodně generovanou přirozenou jeskyní.** Vstupy mají různé materiály, lehce zakřivenou trasu, skutečný sestup a `beard_thin` adaptaci.
- Podzemní carvery častější a více soustředěné v nižších výškách; vanilla monster room šance snížena z 1/7 na 1/13, deep monster room z 1/9 na 1/17.
- Mírně snížen RC31 později přehnaný mountain factor/jaggedness na původní hodnoty .25/.055. **Nejde o plnou obnovu binárně identického původního JJThunder.** Zachována výška Overworldu 2096.
- Nether: dříve nepoužité lodě a pevnosti mají vlastní rozmístění. U povrchových ruin preferována `WORLD_SURFACE_WG`, Rust varianta používá ověřené Horizon `start_checks`. Zůstává úmyslné vysoké létající umístění pro vzdušné lodě.
- End: zapojen moderní (nikoli legacy 26.2 overlay) worldgen ze Stellarity včetně 24 biome ID, noise nastavení přizpůsobené výšce Endu 512. Velké památky, hrady, End lodě a monumenty přesunuty do jediného spread setu se vzájemným minimálním rozestupem 28 chunků (448 bloků); vyloučení v okruhu 18 chunků od End City. End City samo dostalo větší minimální spread.
- Data dalších nahraných balíčků zapojena jako nativní struktury: TREK 26.3, Dungeons and Taverns 26.3, Ancient City Overhaul, Explorify, Structures, Structures+, HellishTrials a experimentální datapacková data UndergroundVillages. Stellarity obsahuje i vlastní chrámy a lodě. Nejvíc nových struktur pochází z DnT a V67 obsahuje celkem 7 300 NBT šablon.
- Opraveno 570 obyčejných truhel bez NBT v pěti dříve neaktivních importovaných šablonách (58 + 25 + 237 + 124 + 126). Dekorativní barely lodí jsou schválně nezměněny.
- Opraveno 8 chybných odkazů na chybějící NBT v původním Dungeons & Taverns: 6 prázdných bezpečných fallbacků a 2 náhrady známou existující šablonou.

## Co není kompatibilní / nebylo kompletně portováno

- `abridged` používá `lithostitched:delegating` (vlastní modový typ struktury). Pouhé přidání JAR dat do datapacku by nebylo funkční; mod zde nebyl portován do Rust engine.
- `end-castle.zip` a `end-island-only-50-done.zip` jsou **uložené herní světy (.mca)**, ne datové balíčky. Nejsou přeměněné na samostatné jigsaw šablony. Jiné End hrady ze zdroje DnT jsou zařazené.
- Shader Photon, Windows EXE konvertor a velký samostatný mod **nejsou součástí této opravy datapacku**.
- Vizuální propojení tunelů s procedurálními caves, End dragon arena, portal travel, skutečné loot otevření, chunk LOD/TPS, a bezpečné umístění všech importovaných struktur v Rust/Java **nebylo ověřeno spuštěnou hrou**.
- Přidané zdrojové datapacky a JARy mohou obsahovat cizí autorská díla. Tyto integrované ZIPy jsou určeny pro soukromý test z uživatelem dodaných zdrojů. **Nevydávat je veřejně na GitHub bez ověření licencí všech zdrojů.**

## Doporučený rychlý test – Horizon RUST Nether1024

```mcfunction
/datapack list enabled
/function thalvoryn:check
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:overworld run locate structure thalvoryn:overworld/handmade/air_vessel
/execute in minecraft:overworld run locate structure thalvoryn:sky/v67_sky_isle_01
/execute in minecraft:the_nether run locate structure thalvoryn:nether/handmade/blackstone_frigate
/execute in minecraft:the_end run locate structure nova_structures:end_castle
/execute in minecraft:the_end run locate biome stellarity:the_hallow
/execute in minecraft:the_end run locate structure minecraft:end_city
```

Příkaz `/locate` vrací X/Z. Ve Spectator módu se na dané souřadnice teleportuj vhodnou výškou dle biomu. Zvlášť testuj End draka, End gateway, Nether portál, výšku 700 a 950 pro 1024 variantu, loot v truhlách, pevnost na kopci, FPS/TPS a ukládání světa. V každé dimenzi navštiv alespoň tři nové oblasti.

## Statické kontroly

`v67_integration_static_qa.json`: 15 506 souborů na variantu, 6 148 JSON/meta, 549 definic struktur a 540 v aktivních skupinách, 1 439 poolů, nulové vlastní chybějící odkazy. `v67_feature_order_qa.json`: 0 detekovaných cyklů pořadí features v aktivních biome ID. `v67_qa_four_variants.json`: 4/4 CRC, NBT shodné napříč variantami, End height 512, Nether 1024/2032 a native Horizon-only checks jen v Rust ZIP.

**GitHub obsahuje jen veřejně bezpečný build skript, QA skripty a metadata, nikoli převzaté třetí strany nebo ZIPy.** Ověřovací build se dá sestavit místně z uživatelem vlastněných zdrojových archivů.