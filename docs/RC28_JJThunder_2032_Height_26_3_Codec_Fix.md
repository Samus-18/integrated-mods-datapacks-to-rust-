# RC28 — ověření JJThunder limitu 2032 a oprava 26.3 dimension codec

**2026-11-08 · Horizon Rust 26.3 · soukromý testovací ZIP · staticky ověřeno, NE testováno ve hře.**

## Ověřeno: původní JJThunder
Originální **JJThunder To The Max** uvádí rozmezí Overworldu **Y −64 až Y 2032** a hory vysoké až přibližně 1 900 bloků. Zdroj: [autorův Modrinth](https://modrinth.com/datapack/jjthunder-to-the-max), který uvádí licenci **All Rights Reserved**. Původní ZIP není v tomto commitu přikládán.

## Konkrétní chyba RC27
RC27 měla numericky správně `dimension_type/overworld.json` height **2096**, `noise_settings/overworld.json` výšku **2096** a gradient `thunderfusion:overworld_overworld_baseline` od −64 do 2032. **Ale:** aktivní soubory `data/minecraft/dimension_type/overworld.json` a `the_nether.json` stále používaly stará pole `bed_works`, `natural`, `piglin_safe`, `respawn_anchor_works`, `ultrawarm`, `effects`, `has_raids` a další. Pro Minecraft 26.3 patří jejich chování do nového systému `attributes`. Horizon při odmítnutí dimension-type codecu může přejít na výchozí vanilla výšku: [Horizon world-height docs](https://github.com/horizon-rust/horizon/blob/main/docs/world-height.md). **Je to pravděpodobná příčina, ne výsledek přímého testu aktuálního světa.**

## RC28 – opravy
1. `data/minecraft/dimension_type/overworld.json`: 26.3 formát `attributes` převzatý ze starší funkční Java varianty projektu, zachováno `min_y=-64`, `height=2096`, `logical_height=2096` → bloky Y −64 až 2031.
2. `data/minecraft/dimension_type/the_nether.json`: 26.3 formát `attributes`, `min_y=0`, `height=1024`, `logical_height=1024`, `has_ceiling=false` → Y 0 až 1023, bez logického stropu.
3. `pack.mcmeta`: nápis RC28. Přidaná krátká CZ poznámka.

End měl správný 26.3 formát a nebyl změněn. Stejně tak **celý zbytek RC27 zůstal beze změny**, včetně hustot JJThunder, biome/climate map, 1 102 NBT struktur, lootu a pravidel pro lodě.

Kontrolovaný referenční řetězec: `minecraft:overworld` → `thunderfusion:overworld` noise settings (−64 + 2096) → `thunderfusion:overworld_overworld_combined_fix` → `thunderfusion:overworld_overworld_baseline` s koncovým Y 2032.

## Statické testy
- ZIP CRC a integrita PASS
- **1 523** JSON/metadatových souborů úspěšně parsováno
- **1 102 NBT šablon** byte-identických s RC27
- Pouze tři původní změněné soubory + 1 nová README poznámka
- Obsolete field keys v opravených dimension types: **0**
- Odkazy na vlastní noise settings pro Overworld a Nether jsou platné
- **Horizon runtime test: NOT RUN**, přesná příčina původního world-height symptomu bez aktuálního `latest.log` a `level.dat` stále není absolutně potvrzená.

**Soukromý archiv:** `Thalvoryn_Horizon_Rust_RC28_Nether1024_26_3_HEIGHT_CODEC_FIX_TEST.zip`, 8 910 247 B  
**SHA-256:** `adc37db7b1af1d85477df4b80842adf3c475bb66c1a1e5a3594e7282b53a274c`

## Jak testovat
Nový svět, pouze RC28 a žádný dřívější JJThunder/Thalvoryn/ThunderFusion datapack. Po načtení ověřit aktivní balíček a `latest.log`. Staré chunky se automaticky nepřepočítají. Připravený čtecí skript `Horizon_World_Height_Diagnostic_RC28.py` analyzuje pack, svět `level.dat` a log a ověřuje nový formát dimension types.

## Stále otevřeno
LOD vzdáleného terénu a struktur; bezpečný Nether portál a test vody/lávy pod loděmi; úplné F3 biome ID, strukturální kolize a Photon shader. Sledovat [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

**Binární RC28 ZIP nenahrávat veřejně bez ověření práv na cizí mapové stavby v balíčku.**
