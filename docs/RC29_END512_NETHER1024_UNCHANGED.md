# RC29 – Horizon Rust 26.3: Nether zůstává 1024, End stavební výška 512

**2026-11-08 — pouze privátní experiment, staticky ověřený. NENÍ herně otestováno.**

Uživatel upřesnil, že vysoký Nether z původní verze byl nežádoucí. **Nether se NESMÍ zvyšovat zpět na 2032**. Vysoký Overworld od JJThunderu a změny Endu se mají vyvíjet samostatně.

## Stav rozsahů

| Dimenze | min_y | height (počet bloků) | platný rozsah bloků |
|---|---:|---:|---|
| Overworld | −64 | 2096 | −64 až 2031 |
| Nether | 0 | 1024 | 0 až 1023 |
| End | 0 | **512** (RC28: 256) | 0 až 511 |

Horní hranice je **exclusive**: min_y + height. Takže hranice 2032 v JJThunder znamená nejvyšší umístitelný blok na Y2031.

## Změny RC28 → RC29

- **Nether:** NIC. Všech **183 souborů**, které obsahují v cestě `nether` / `the_nether`, zůstalo byte-identických. `dimension_type/the_nether.json` zůstává `min_y=0,height=1024,has_ceiling=false`, beze změny terrain/density/biomes/structure sets.
- **Overworld:** NIC ve světovém generování. `dimension_type/overworld.json` `-64,2096`, veškeré jeho noise a structure JSON i NBT zachované byte-identicky.
- **End:** `data/minecraft/dimension_type/the_end.json` výška / logická výška změněna z 256 na 512.
- **End vlastní ostrovy:** pouze pět JSON definic `thalvoryn:end/archipelago/asymmetric_1..5` dostalo různorodá startovní výšková pásma: **72–105**, **118–151**, **90–128**, **184–230** a **142–185**. Jejich všech pět 42-bloků vysokých NBT geometrií zachováno; struktury ve společném setu mají **stejnou** četnost, rozestupy a soli jako RC28. Nejvyšší varianty nyní mohou umisťovat části do výšek nad původním build limitem 256.
- `pack.mcmeta` popis RC29; přidána CZ README poznámka.

### Důležité: co End 512 NEznamená

**Rozšířil se jen build range Endu**, nikoliv jeho přirozený noise terén. `data/minecraft/dimension/the_end.json` stále odkazuje na `minecraft:end` noise settings a ty jsme bezpečně nepřepsali, protože není ověřené kompletní 26.3 vlastní End noise schéma. Takže **neslibujeme kontinenty nebo nové biomy v pásmu Y256–511**; výškové ostrovy mohou pocházet z našich jigsaw šablon. Biome ID ve F3 se také nemění.

## Statické ověření

- Vstup: `Thalvoryn_Horizon_Rust_RC28_Nether1024_26_3_HEIGHT_CODEC_FIX_TEST.zip`.
- Výstup: `Thalvoryn_Horizon_Rust_RC29_Nether1024_END512_TEST.zip`, **8 911 059 B**.
- SHA256: `491ed10b2bac73068eb6fa9e8ddd6a74cf2dcaef1b3acdf73008c2dd4c9718f0`.
- ZIP CRC PASS; **1523** JSON/metadatových souborů parsováno, **49** structure setů, **298** struktur, **563** template poolů, **0** chybějících vlastních odkazů.
- Všech **1102 NBT byte-identických** s RC28, včetně všech loot inventářů.
- **183 netherových souborů** a všechny Overworld worldgen soubory byte-identické.
- Pět výškových pásem ověřeno proti 42-blokové výšce jejich skutečných NBT šablon, všechny zůstávají uvnitř End build range 512.
- **V Horizon runtime ani Java 26.3 herně neotestováno.** Nelze garantovat, že End 512 build-height a jigsaw start výšky se v konkrétní Nightly načtou.

## Stále nevyřešeno

Plný End noise / rozdílné biomy a vegetace, správné umisťování lodí podle skutečné vody/lávy, ochrana velkých struktur před kolizí s terénem, bezpečné Nether portály, nesoulad vzdáleného LOD, vlastní biome ID ve F3, nativní Photon a konečné testování. Podklady viz [issue #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

**Distribuce:** archiv obsahuje neověřeně licencované cizí stavby a není vystaven jako veřejný binární GitHub Release. Použít v novém testovacím světě samostatně místo RC28, ne zároveň s předchozími verzemi.
