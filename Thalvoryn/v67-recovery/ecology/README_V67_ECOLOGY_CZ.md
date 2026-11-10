# Thalvoryn V67 – Ecology & Structure Rarity / soukromý herní test

**Základ:** čtyři `V67-INTEGRATION-TEST-UNRELEASED` balíčky. **Nejde o finální herně otestovanou verzi.** Všechny čtyři výstupy jsou kandidáti pro **nový** svět v Minecraft Java 26.3 / Horizon Rust, nikoliv aktualizace existujícího světa.

## Zásadní opravy podle videí

- Žádné pirátské / normální / custom lodě se **nevyskytují v Netheru ani Endu**. Vyřazeno 25 netherových a 29 End lodních či námořních definic. Zachované jsou jako neaktivní data, aby se nic neztratilo. Vanilla End City a její originální loď **zůstávají**.
- Odstraněny zdvojené aktivace stejného custom ID z více nezávislých structure setů. 18 sjednocených kategorií, všechny nevanilla aktivní definice mají přesně jednu vlastnickou skupinu. Vanilla sety i speciální rozložení strongholdu zůstávají zvlášť.
- Spawnerové místnosti z TREK se již nepokoušejí o generování v každém chunku (`spacing: 1, frequency: 1`). Nyní společná skupina: `spacing: 21, separation: 9, frequency: 0.34`, tedy značně řidší.
- End City: `spacing: 77`, `separation: 31`, `frequency: 0.42`. Velké custom End stavby mají `spacing: 105`, `frequency: 0.37`, maximální přípustný `exclusion_zone.chunk_count: 16` vůči vanilla End City. **Nejde o matematickou záruku vzdálenosti všech skutečných staveb**; jde o omezení startů s kontrolou sousedních skupin.
- Velká overworldová vzducholoď `thalvoryn:overworld/handmade/air_vessel` se objeví jen v overworldové sky/airship skupině a výše nad generovaným povrchem. Šest nových plovoucích ostrovů také přemístěno výše; NBT tvary z předchozí V67 zachovány.
- Šest strmě klesajících jeskynních tunelů má `beard_box` pro silnější zakomponování do okolního terénu. Jejich vlastní stavěná dutina na konci **existuje**, ale **nelze zaručit spojení s nezávisle náhodně vytvořenou přírodní jeskyní** bez doplňkové procedurální logiky v enginu.
- Netherová jigsaw místa `nova_structures:nether_*` původně používala fixní výšky Y31–60; nově používají `WORLD_SURFACE_WG` s výškovým offsetem pro povrch vysokých hor. Horizon Rust má navíc ověřené `horizon:start_checks` pro lávu/sklon (Java bez tohoto rozšíření).
- Overworldová density function, noise settings i biome JSON byly v této iteraci **zachovány bitově**. Nehrozí další náhodná změna JJThunder v tomto konkrétním buildu, ale ani to ještě **neřeší** požadavek na větší nížiny, který bude nutné ladit podle původních zdrojů a hry.

## Rozložení aktivních struktur

| Skupina | Počet definic | Spacing (chunků) | Frequency pokusu |
| --- | ---: | ---: | ---: |
| Overworld malé | 116 | 26 | 0.77 |
| Overworld osady / domy | 109 | 39 | 0.62 |
| Overworld věže / pevnosti | 58 | 66 | 0.48 |
| Overworld epické hrady | 13 | 112 | 0.34 |
| Overworld oceánské lodě | 36 | 54 | 0.57 |
| Sky islands | 11 | 44 | 0.64 |
| Velká vzducholoď | 1 | 119 | 0.32 |
| Spawnerové místnosti | 8 | 21 | 0.34 |
| Podzemní dungeony | 26 | 48 | 0.52 |
| Velké podzemní oblasti | 4 | 99 | 0.36 |
| Jeskynní tunely / vstupy | 6 | 35 | 0.60 |
| Nether drobné | 17 | 30 | 0.70 |
| Nether ruiny | 13 | 44 | 0.63 |
| Nether velké | 5 | 86 | 0.39 |
| Nether podzemní | 1 | 60 | 0.45 |
| End ostrovy a skály | 19 | 33 | 0.65 |
| End malé památky | 24 | 48 | 0.58 |
| End velké památky | 7 | 105 | 0.37 |

**474 strukturálních definic** je v těchto 18 skupinách. Dalších **10 nevanilla definic** zůstává v původních vanilla/special rozmístěních (celkem **484** aktivních nevanilla definic). **54** nepatřičných námořních/custom lodí je neaktivních. Datapack stále obsahuje všechny definice a NBT, tedy nic nebylo smazáno z knihovny zdrojů.

Spacing a frequency ovlivňují pokusy o generování. Biomy, nerovný terén a výškové podmínky mohou generování dále zředit; **nejde o zaručenou vzdálenost hotových staveb**.

## Testovací příkazy – přímo do hry

```mcfunction
/datapack list enabled
/function thalvoryn:check
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:overworld/handmade/air_vessel
/execute in minecraft:overworld run locate structure thalvoryn:sky/v67_sky_isle_01
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:overworld run locate structure trek:monster_room/lower/default
/execute in minecraft:the_nether run locate structure nova_structures:nether_keep
/execute in minecraft:the_nether run locate structure thalvoryn:nether/highland_ruins
/execute in minecraft:the_end run locate structure nova_structures:end_castle
/execute in minecraft:the_end run locate structure minecraft:end_city
```

Ověřit také, že `thalvoryn:nether/handmade/blackstone_frigate` a `thalvoryn:end/handmade/sky_caravan` **nemají přirozené spawny**. Jejich definice zůstávají v souborech jako neaktivní, takže ruční `/place structure` může být stále možné.

## Omezení

1. **NEPROBĚHL runtime test Java 26.3 ani Horizon Rust.** Prvotní zkouška pouze v novém světě.
2. Jediný vanilla `exclusion_zone` může odkazovat právě na jeden jiný structure set; nelze zaručit 100% zákaz všech kolizí napříč 18 kategoriemi. Hra sama dále provádí biome/terrain filtr.
3. Připojení tunelů k přirozeným caves, umístění velkých staveb, animace/Lights/LOD, loot při otevření a portály čekají na herní test.
4. Všechny binární výsledky obsahují díla třetích stran dodaná uživatelem. **Neveřejná testovací verze; nepublikovat veřejný ZIP na GitHub bez vypořádání licencí.** Zdrojový patch, manifest a QA bez těchto assetů lze uložit do repozitáře.
5. `end-castle`/`end-island-only-50-done` jsou uložené světy, `abridged` vyžaduje vlastní modový typ. Bez konverze/worldgen portu nebyly aktivovány jako nezávislé struktury.

## Souborová kontrola

`V67_ECOLOGY_QA.json`: 4/4 CRC, 15 391 položek/ZIP, 7 300 identických NBT šablon, 549 JSON definic struktur, 25 structure setů celkem (18 nových kategorií), nulové duplicitní aktivace nevanilla definic a 0 chybějících vlastních šablon a start poolů. Statický test srovnává Overworld noise a biomy s předchozí V67, a potvrzuje, že jsou nezměněné.