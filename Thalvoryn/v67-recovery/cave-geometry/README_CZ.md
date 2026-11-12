# Thalvoryn V67 – CAVE GEOMETRY TEST (12. 11. 2026)

**Stav:** 4/4 statický PASS; nebylo spuštěno v Minecraft 26.3 ani v Horizon Rust. Toto je **nová testovací větev, nikoli finální vydání**. Všechny změny navazují na `V67-SPAWN-RECHECK-TEST` (a tedy zahrnují předchozí opravy lootu a spamu struktur).

## Co se opravdu změnilo

- Šest šablon `thalvoryn:underground/v67_cave_tunnel_01` až `_06` není už prakticky stejná geometrie. Mají šest různě vinutých strmých tras, odlišné boční komory, širší koncové dutiny, různé kamenné styly, podpěry a schody.
- Dutiny jsou vlastní **strukturní umělé jeskyně**, ne automatické napojení na přirozeně generovanou jeskyni; to nelze garantovat pouhou definicí datapacku.
- Každá šablona má rozsah 79×93×114 bloků (původně 35×73×82), přibližně 36–49 tisíc bloků vzduchu a jeden průchod od vstupu do velké komory. Původní šablony měly přibližně 9,6 tisíce bloků vzduchu.
- Šest startovních definic používá výšku `OCEAN_FLOOR_WG` + `start_height: -85`, aby nové vyšší vstupy nevznikaly nad povrchem. To je správně podle mechaniky výškových offsetů jigsaw struktur, ale **skutečný kontakt se svahy není ve hře otestovaný**.
- Výskyt šestice tunelů **nebyl navýšen**: společný set `spacing=32`, `separation=25`, `frequency=0.70`. Žádné nové nezávislé spawnerové sety.
- Loot, ostatní NBT, End, Nether, Overworld biomy i terénové density funkce nebyly změněny. Stellarity nebylo přidáno.

## Nezávislý test kódem

- 4/4 archivy bez chyby ZIP CRC, 11 323 položek v každém.
- Přesně 13 změn na variantu: **6 NBT**, **6 definic jigsaw** a `pack.mcmeta`.
- U každého tunelu jsou všechny souřadnice v mezích šablony, bez duplicit. Flood-fill prokázal spojení vstupního vzdušného prostoru se spodní dutinou (obvykle přes 27–33 tisíc dosažitelných vzduchových bloků v cílové části).
- Všechny nové NBT jsou shodné mezi Java a Rust a mezi oběma výškami Netheru.
- 1 212 souborů density/biome/loot zůstalo bitově shodných s předchozí V67.

## Jak testovat

Ve zcela novém světě testuj nejdřív RUST Nether1024, pouze jeden ZIP:

```mcfunction
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_04
```

Najdi skutečný otvor do země, proleť dolů do komory, zkontroluj sklon schodů, výšku vstupu, případné useknutí koncové jeskyně a záseky při generování chunků. Poznač seed, dimenzi a XYZ.

**Omezení:** Bez běžící hry není možné prokázat korektní dosednutí na konkrétní terén, bezkolizní umístění, přirozené navázání na worldgen caves ani FPS/TPS. Nová geometrie má více blokových operací, a proto může generování tunelů dočasně zpomalit. Na starém světě se existující chunky nepřegenerují. Veřejné vydání integrovaných cizích assetů vyžaduje kontrolu licencí.