# Thalvoryn RC41 — závěrečná kontrola celého datapacku (FINAL CANDIDATE)

**Datum:** 2026-11-08. **Cílové:** Minecraft Java/Horizon Rust 26.3; uživatelem označená Nightly 2026-11-07 (`2ebf00a`), **na které však RC41 nebyla z tohoto prostředí spuštěná**.

**VERDIKT: všechny provedené statické kontroly PASS. RC41 je RELEASE CANDIDATE ke společnému hernímu testu, nikoliv dosud stabilní nebo v Horizonu runtime otestovaný release.**

## Hotový soukromý ZIP

`Thalvoryn_Horizon_Rust_RC41_FINAL_CANDIDATE_NETHER1024_MC26_3.zip`  
**8 821 048 B; SHA256 `80257f01e11bd14620ca05324f61afc0e53f3fdf40230467f0b18451482dfeb8`**.

Soukromý QA balík v konverzaci: `Thalvoryn_RC41_FULL_QA_SOURCE_AND_HISTORY.zip` (**70 996 B**, SHA256 `5e16d217dab1560318d3a3cad353456e0c5c42d420adb27d3cdda3c80570ecd7`). Obsahuje všechny reporty, nezávislé parsery, kontrolní skripty, portal-reference testy a **46 historických README** vyřazených z kořene čistého instalačního ZIPu. Tyto staré dokumenty nezmizely.

## Regresní kontrola před RC40 → po RC41

| Kontrola | Výsledek |
|---|---|
| Celkem ZIP členů | 2961 → **2918** (46 zastaralých textů odsunuto do QA, přidány 3 nové návody) |
| ZIP CRC + unikátnost cest | **PASS** |
| Všechna JSON metadata | **1681/1681 PASS** |
| NBT kompletní geometrie/palety/blokové souřadnice | **1177/1177 šablon, 4 592 879 blokových záznamů PASS** |
| Paletové indexy, hranice šablon, duplicitní bloky | **0 problémů** |
| Blokové namespace ID NBT palet | **všechny `minecraft:`** |
| Custom own refs structure set → structure → template pool → NBT | **38 setů, 311 definic, 572 poolů, 0 chybějících vlastních referencí** |
| Biome → placed feature → configured feature | **42 override biome, 162 placed features, 0 chybějících vlastních referencí** |
| Pořadí biome worldgen features | **227 DAG uzlů, 0 cyklů** |
| Loot data | **194 JSON tabulek, 2847 kontejnerů a 1751 LootTable referencí, 0 chybějících vlastních odkazů** |
| Barely | **1568 barelů, 1068 prázdných a bez LootTable** |
| Ikona `pack.png` | 128×128 PNG, **byte-identická** |
| Výšky dimenzí | OW **−64..2031** (131 sekcí); Nether **0..1023, bez stropu** (64); End **0..511** (32), noise codec OW+Nether přesně sladěný |
| Grafika/height preflight | **6/6 testů PASS** |
| Referenční portal checker (NENÍ v Horizonu) | **9/9 syntetických testů PASS** |

**Byte-diff:** Každý původní `data/*` soubor z RC40 je bajtově stejný v RC41, **kromě textu ručně spustitelné diagnostiky** `data/thalvoryn/function/horizon/status.mcfunction` (`RC40` → `RC41`). Původní struktury, všech 1177 NBT, loot a všechny generátory **nebyly** pro RC41 měněny. Změněno jen `pack.mcmeta` a historické návody, které jsou archivované v QA. Protože při celkovém auditu nebyla nalezena strukturální regrese, nepřepisovali jsme zbytečně funkční worldgen.

## Stav uživatelských 10 požadavků

1. **Overworld:** vysoké hory, terén, jeskynní vstupy, 5 tvarových Sky Islands → data staticky validní. Nové registrované biomy/F3 ID **nejsou** vyřešené.
2. **Stromy:** 48 původních NBT stromů + 60 biomových materiálových variant, vegetace → data validní; celková kolizní ochrana vyžaduje engine.
3. **Jeskyně:** Lush/Dripstone/Deep Dark habitaty, menší ruiny a přírodní prvky → pouze existující vanilla biome IDs.
4. **Nether:** 1024 bloků bez stropu, vegetace a rudy, ruiny a nehořlavé lodní NBT → **lávové lodě jsou manual-only, bez automatického umístění na zaručené lávě**.
5. **End:** origin pobřežní výběžky, malé ostrovy, vegetace a raritnější struktury → vanilla natural noise Endu je stále jen 128, i když stavební rozsah má 512.
6. **Struktury:** anti-spam, vybrané importované mapy a kontrola biome → jen **19 z 103** importovaných NBT v natural spawn; chybí full-footprint slope/liquid/trees validace.
7. **Loot:** 66 nových random custom tabulek a dekorativní empty barrels → vlastní tabulky nemusí Horizon World datapack loader umět, takže **risiko prázdných truhel**.
8. **Mobové:** 35 struktur s biome/role spawn overrides + kočka/golem podzemní osady → neověřený skutečný runtime spawn.
9. **Vanilla výškové struktury:** 4 vzácné mountain varianty plus vyšší bastion range → nelze bez full-ground check garantovat správný podklad.
10. **Horizon:** výškové noise/dimension registry PASS; ale **F3 custom biome registry, LOD renderer, portály nad lávou, runtime FPS/TPS a loot loader stále vyžadují opravy v enginu**.

## Test a omezení

**Jednorázový závěrečný test uživatele nyní dává smysl**: v novém světě jen s RC41, ne se starým RC40 nebo paralelním JJThunder/TREK. Instalační ZIP obsahuje `RC41_TEST_CHECKLIST_CZ.txt` s přesným pořadím 13 ověření a manuální `/function thalvoryn:horizon/status` diagnostikou.

- **Horizon datapack loader:** [world-generation.md](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md) dokumentuje worldgen overrides a vynechané nové biome IDs. [mod-browser.md](https://github.com/horizon-rust/horizon/blob/main/docs/mod-browser.md) uvádí, že správce world datapacků nepodporuje všechny registry, včetně loot tabulek; instalace přes browser a manuální world/datapacks mohou být odlišné.
- **MC26.3**: `pack_format=121`; oficiální [26.3 technické změny](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3).
- **Neprovedeno v běžícím enginu:** skutečný load/reload, chunk worldgen, Nether portals, FPS/TPS, far LOD, mob AI/loot chests a End dragon fight.
- **Neověřená distribuce map:** nedělat veřejný GitHub ZIP release, dokud nejsou doložena práva k některým dodaným externím stavbám.

**Issue [#2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2) zůstává otevřené**, aby se nemaskovaly engine blockery. 
