# RC31 – Overworld: nové Sky Islands, horské průchody, vegetace a ikona

**Horizon Rust 26.3 · 8. 11. 2026 · soukromý experimentální testovací balíček, ne stabilní release.** Vychází z RC30. Navazuje na první část uživatelova seznamu: „🌍 Overworld – Nové biomy, výraznější hory, lepší jeskynní vstupy, různorodé Sky Islands a jejich výška podle terénu“ a upozornění, že custom ikona byla zapomenuta.

## Hotové v zipu (staticky)

- **4 nové Sky Islands**: `verdant_terrace`, `crescent_crag`, `twin_isles`, `amber_ridge`. Nové autorské NBT šablony s různými tvary a vegetací: **10 997, 8 619, 13 726, 11 328 blokových pozic**. Palety `id` dle 26.3; všechny blokové pozice bez duplicit a uvnitř deklarovaných rozměrů.
- Sky Garden z RC30 zachována; nově společná pětivariantová `thalvoryn:sky/sky_garden` structure set mřížka: **spacing 51, separation 21, frequency 0.66** (oproti 64/22/0.48). Jde přibližně o **2,2× více pokusů na jednotku plochy**, nikoliv o potvrzené 2,2× více skutečně vygenerovaných struktur. Žádné čtyři nezávislé nové spawn-mřížky.
- Nové jigsaw startovní výšky používají `WORLD_SURFACE_WG` a *offset* od aktuálního terénu **54–102 bloků** dle varianty; původní Sky Garden upravena z 92–110 na 75–96. Celý půdorys se **neověřuje**, takže kolize, vysoké hory a návaznost na skutečný terén je nutné otestovat přímo v Horizonu. Nepřidány na oceány.
- **JJThunder extrémní hory**: v aktivní extrémní větvi jsem upravil amplitudu mountain factor `0.25→0.31`, jagged `0.055→0.067` a statický offset `0.60→0.64`. Velká výšková definice JJThunder `min_y=-64, height=2096` i baseline `-64..2032` zůstala zachována. Výsledná výška hor a vzhled krajiny **ještě neověřeny herním testem**.
- **Jeskynní průchody**: `highland_cave_passage` max Y `170→360`, probability `0.014→0.020`, mírně větší horizontální poloměr; `woodland_cave_passage` max Y `116→158`, probability `0.018→0.024`. Nová vzácná `alpine_cliff_passage` s rozsahem **Y180–550**, pravděpodobnost 0.006, přidaná do relevantních horských existujících biomů. **Tím nelze garantovat viditelný vstup v každém pohoří** – jen více příležitostí protínat povrch.
- **Biome-like varianty** uvnitř **existujících vanilla ID** (nevznikla nová ID): speciální drobné vegetační akcenty `birch_hollow` (old_growth_birch_forest), `spruce_ridges` (windswept_forest) a `oak_flower_valley` (flower_forest) plus omezené odlišné barvy vegetace. **Vlastní jména biomů ve F3 nejsou hotová**: současný Horizon dle [worldgen docs](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md) ignoruje nepodporované nově přidané biome ID v datapacku. Není legitimní tvrdit, že RC31 přidává tři nové registrované biomy.
- **Vlastní ikona**: kořenový `pack.png` nahrazuje starý obrázek TREK. Je to vlastní pixel-art noční krajina s horami, jeskyní a létajícím ostrovem (PNG 128×128). Poznámka: [Horizon README](https://github.com/horizon-rust/horizon) uvádí chybu zobrazování ikon některých packů, takže i správný `pack.png` nemusí být v konkrétní Nightly vidět.

## Ověřeno programově

- Předloha: RC30 `Thalvoryn_Horizon_Rust_RC30_Nether1024_END512_SKY_ISLANDS_TEST.zip`.
- Archiv RC31: **2 732 souborů**, **17 nových**, **14 upravených**.
- Všechny JSON soubory dekódované, ZIP CRC/testzip PASS.
- Nové čtyři NBT prošly **nezávislým big-endian NBT parserem**, s legalitou rozměrů, pozic, paletových indexů a nulou duplicitních souřadnic.
- Struktura odkazů set → structure → pool → NBT pro všech pět sky variant existuje.
- Graf pořadí worldgen feature podle biome kroků nemá nalezené cykly.
- Nether, End a všech **1 102 předchozích NBT** zůstaly *byte-identické*.
- **Overworld −64…2031**, **Nether 0…1023**, **End 0…511** beze změny.
- ZIP soubor `Thalvoryn_Horizon_Rust_RC31_OVERWORLD_SKY_CAVES_ICON_PRIVATE_TEST.zip` (**8 885 374 B**), SHA-256 `b0fc4f873bdf556159478878881269ef54c6bade96cf3a85f2ce4e8ccca59daf`.

## Nevydávat za dokončené

**Přímý test v běžící hře Horizon Rust nebyl proveden**. Nelze zaručit vizuální kvalitu nových ostrovů, četnost, podporu všech modifikovaných carverů, rychlost generování, absenci kolizí nebo vzhled hor. Nové F3 biome ID a názvy zůstávají závislé na engine. Není dořešené vzdálené LOD ani Photon. Další části uživatova seznamu přijdou postupně.

**Testovat:** nový svět s *pouze RC31*, neprolínat RC30 ani starší JJThunder. Zkontrolovat běžný les/květnatou louku, vysokohorské vstupy, Sky Islands a runtime log. Vygenerované staré chunky se nemění.

**Licencování:** starší dodané cizí stavby v úplném ZIPu nemají potvrzená práva pro veřejnou redistribuci. Binární ZIP nepublikovat veřejně na GitHub Release bez povolení autorů. Zápis tohoto textu práva nevytváří.
