# RC40 – Horizon Rust compatibility: LOD / portály / F3 / vysoké světy / výkon

**8. listopadu 2026 · Horizon Rust & Minecraft Java 26.3 · soukromý testovací datapack.**

Uživatel zaslal část 10:

> 🎮 Horizon — LOD, bezpečný Nether portál, vlastní biome/cave jeskyně ID ve F3, kompatibilita vysokých světů a výkon; zase kontrola a ověření.

Uživatel bude v Horizonu testovat až po dokončení celého seznamu. **RC40 NENÍ dokončená implementace engine oprav.** Je to bezpečné před/po statické ověření rozměrů a neinvazivní sada diagnostik; nebezpečné workaroundy se vůbec neaplikují.

## Před/po RC39 → RC40

- Archiv **2956 → 2961** členů.
- Jediný změněný stávající soubor: `pack.mcmeta` (verze RC39 → RC40).
- Nových souborů právě **5**: 4 explicitně spouštěné `data/thalvoryn/function/horizon/{status,cave_biome,portal_hazards,lod_notes}.mcfunction` a `RC40_HORIZON_COMPAT_CZ.txt`.
- Všech **2955 ostatních původních členů** zcela byte-identických – tj. **1177 NBT** (včetně RC38 dvou osadních mobů), **194 loot tabulek**, struktury, všechny biome/noise/density/feature definice, stromy, Nether a End JSON, custom `pack.png`.
- ZIP CRC PASS, **1681 JSON a metadata** validní, 38 structure setů, 204 aktivních unique structure refs, vlastní structure-set→structure/start_pool odkazy prozkoumané.
- Výšky: **Overworld −64..2031** = 131 sekcí; **Nether 0..1023** = 64 sekcí, bez stropu; **End 0..511** = 32 sekcí. Obě vlastní noise settings (OW/Nether) se přesně shodují s `dimension_type` výškou.
- `pack_format` 121 pro MC 26.3, vlastní PNG ikona zachována.
- Přidané příkazy neobsahují `fill`, `setblock`, `tp`, `teleport`, `summon` nebo jiný zásah do světa; **žádné funkce nejsou zapojené na tick/load**.
- Nový **read-only** Python preflight má **6 úspěšných testů**, včetně odhalení chybně změněného Nether height a upozornění na extrémní LOD16k. **9 syntetických testů referenčního safe portal kandidáta PASS**, *není součástí hry*.
- Výsledek nezávislého preflightu: **PASS_WITH_ENGINE_LIMITS (0 errors, 3 warnings)**.
- Archiv `Thalvoryn_Horizon_Rust_RC40_HORIZON_COMPAT_NETHER1024_PRIVATE_TEST.zip`, **8 863 656 B**, SHA256 `2fa687e5acc2a945911a26ddb36ec8d733ff77220e3a4faa58b2f24994070989`.

## Ruční in-game diagnostiky (NE automatická oprava)

```
/function thalvoryn:horizon/status
/function thalvoryn:horizon/cave_biome
/function thalvoryn:horizon/portal_hazards
/function thalvoryn:horizon/lod_notes
```

První funkce informuje o dimenzích; druhá používá `execute if biome ~ ~ ~` a *hlásí právě existující vanilla ID* pro Lush, Dripstone, Deep Dark, Nether a End; třetí upozorní, když **bezprostředně v okolí hráče** je láva (nemá úplný validator a neřeší přechod); čtvrtá ukáže grafická doporučení. Fungování v aktuálním Nightly **NEOVĚŘENO**.

## Proč požadavky ještě nejsou hotové

1. **F3 custom biome/cave ID** – Horizon [worldgen](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md) nyní nové biome ID v běžném datapacku ignoruje, včetně climate points. Nutné zaregistrovat biomy v Rust enginu nebo podporovaném native/content mod API, napojit F3, klientské registry, barvy/zvuky a sampling. Funkce `cave_biome` je jen provizorní diagnostika vanilla ID, neslibovat custom F3.
2. **LOD vizuální shoda** – Horizon [graphics](https://github.com/horizon-rust/horizon/blob/main/docs/graphics.md) vysvětluje, že daleké LOD zjednodušuje terén; neprozkoumané struktury se nemusejí synteticky generovat a zobrazení se může po přiblížení změnit. **Datapack to neumí opravit.** Doporučení pro RX 580 / 16 GB: render 8–12, simulation 5–6, LOD 128–256, Far Detail Reduced, ne 16k. Žádný runtime GPU test.
3. **Bezpečný Nether portál** – oprava patří do Rust portal placement: horizontální coordinate scale 8:1, **výškové rozpětí jen 0..1023**, validace pevného podloží, vzduchu a kapalin pod *celým* půdorysem, rozumná blízká alternativní pozice / postavení plošiny pokud nutné, plus automatizované integrační testy. Doložený reference checker 9 synthetic tests **není zapojený do Horizonu**.
4. **Vysoké světy a výkon** – `dimension_type` a noise rozměry splňují MC26.3 codec ([world-height.md](https://github.com/horizon-rust/horizon/blob/main/docs/world-height.md)), ale bez skutečného spuštění nelze prohlásit renderer/meshing/save/thread/TPS/LOD za otestované pro tyto konkrétní světové funkce. V Endu zůstává přirozený `minecraft:end` noise vysoký 128 i když je stavěcí výška 512.
5. **Loot tables RC37** v obyčejném world datapacku mohou stále narážet na neúplnou podporu instalační cesty [Horizon mod browser](https://github.com/horizon-rust/horizon/blob/main/docs/mod-browser.md). RC40 nemění 194 loot JSONů; fungování v reálném Nightly neověřeno.

## Co přesně otestovat ve finále

- První spuštění zcela nového světa: kontrolovat Horizon log pro `dimension_type` odmítnuté definice / fallback; porovnat F3 hranice a `/locate structure`.
- Přelet do vzdálenosti více než 12 chunků: stejné oblasti při LOD 0/128/256, porovnat bílý vzdálený terén, struktury a přechod po přiblížení.
- Přechod Nether portálem z Y≈70, Y≈500 a Y≈900, nad lávou, na okraji jeskyně i u velkého masivu; při návratu ověřit 8:1 XZ, bezpečný blok a žádný freeze.
- Biome report v hlubokých jeskyních a F3: má přiznaně vanilla ID, nové názvy zbývá doplnit engine patchem.
- Pozorovat FPS / TPS a použité VRAM/RAM při Overworld131/Nether64/End32 sekcích; nedělat závěry pouze z formátových kontrol.
- Loot v nově generovaných strukturách v běžícím Horizonu a NBT osadních mobů, protože předchozí RC nemají runtime parity.

**Specifikace a spustitelná pomocná testovací sada**: v chatu `Thalvoryn_RC40_HORIZON_AUDIT_AND_ENGINE_REFERENCES.zip` včetně `thalvoryn_rc40_preflight.py` (read-only), jeho negativních testů, referenčního portal kandidáta a jeho unit testů. Technický detail je v [otevřeném issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

## Otevřený status

**NEDOKONČENO**: vlastní F3 biomy, kompatibilita F3 pro custom caves, bezpečný skutečný portál v enginu, odstranění rozdílů LOD, herně změřený výkon, kompletní podpora lootu. **RC40 pouze statická kontrola + diagnostika**. Nepublikovat binární NBT pack veřejně bez práv na původní cizí mapy.
