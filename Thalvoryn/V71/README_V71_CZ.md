# Thalvoryn V71 — technický hotfix (pouze kódové testy)

**Navazuje na V70.** Build pro Java 26.3 a Horizon Rust, každá platforma Nether 1024 / 2032. Nejedná se o herně potvrzený finální release.

## Konkrétní opravy

- **6 podzemních tunelů:** každému přidány dva zakřivené boční průchody z již spojité vlastní velké jeskyně až na hranici NBT šablony. Celkem přidáno 3,672 bloků vzduchu napříč šesti šablonami. NBT velikost, palety, původní entity a původní bloky mimo vyřezané chodby nebyly změněny. Geometrická průchodnost mezi vstupem, komorou a oběma bočními vývody ověřena flood-fillem.
- **4 podzemní dungeony Netheru** (`black_spiral`, `nether_trial`, `piglin_donjon`, `sealing_halls`) místo pevné výšky Y10–34 dostaly náhodné výšky Y28 až Y864 (Nether 1024) / Y1872 (Nether 2032). Toto je *potenciální* výška startu; ne každá pozice bude mít dutinu nebo otevřený přístup. Při výběru vysokých pozic mohou být komplexy uvnitř terénu.
- **Bažinná Witch Villa** má umístění podle `WORLD_SURFACE_WG` s offsetem -2 místo pevného Y63.
- **Rust-only:** dalších 30 povrchových větších staveb dostalo kontrolu sklonu a kapaliny; 6 netherových velkých staveb má rozšířenou kontrolu sklonu na poloměr dvou chunků; 2 velké End stavby mají omezený povolený rozdíl výšek na povrchu.
- **Výkon:** nepřidávaly se nové structure sety ani nové spawnerové placed features. Po prvním příliš širokém filtru byly kontroly zúženy jen na 30 pozemních struktur. To je opatření proti zbytečné zátěži, **nikoli naměřený nárůst FPS/TPS**.

## Ověření

- 4/4 ZIP CRC testy; JSON syntaktické ověření a kontrola rozsahů jigsaw struktur.
- 6/6 tunelů fyzicky průchozích uvnitř NBT až k oběma vývodům (BFS nad explicitními vzdušnými bloky).
- V Java ZIPu se kromě šesti NBT tunelů, čtyř netherových výškových definic, bažinné vily, verze a changelogu neměnily další soubory.
- V Rust ZIPu navíc 30 terénních kontrol, 6 netherových a 2 endové. Žádný soubor namespace Stellarity.
- Aktivní root jigsaw pooly vlastní struktury existují; není zdvojené aktivní structure ID.

## Co **NELZE** pokládat za opravené

- **Spojení s procedurální (náhodnou) jeskyní nelze zaručit pomocí statické NBT šablony**. Obě chodby vedou na okraj šablony, kde je průnik možný, ne jistý. Vždy jsou ale spojeny s velkou jeskynní komorou vytvořenou přímo strukturou.
- Celý terén pod velkými stavbami není automaticky modelovaný na každém bloku. Horizon kontroluje vybrané startovní pozice, Java má méně možností; velké hrady a jigsaw vesnice na složitém terénu mohou mít chyby.
- Nebyl spuštěn Horizon, Minecraft Java, ani měření FPS/TPS, ani měření generování chunků na několika seedech.
- Úpravy struktury se projeví jen v dosud negenerovaných chunkech. Pro objektivní test použít nový svět, **jen jeden ze čtyř ZIPů současně**.

## Testovací příkazy

```mcfunction
/datapack list enabled
/function thalvoryn:version
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:the_nether run locate structure explorify:black_spiral
/execute in minecraft:the_end run locate structure nova_structures:end_castle
```

Vnitřní ID tunelů stále obsahuje `v67`, protože jejich přejmenování by rozbilo `/locate` příkazy a návaznosti. **Verze celého balíčku je V71**.

Obsahuje převzaté materiály z uživatelem poskytnutých balíčků; veřejná redistribuce vyžaduje kontrolu licencí.