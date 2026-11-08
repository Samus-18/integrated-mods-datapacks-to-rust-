# Thalvoryn RC33 — Jeskynní příroda a podzemní ruiny (test)

**Horizon Rust / Minecraft Java 26.3 · 2026-11-08 · testovací archiv, ne stabilní sestavení.**

## Zadaná část 3

> 🕳️ Jeskyně — Vlastní jeskynní biomy, unikátní rostliny, bloky, mobové a podzemní struktury.
>
> Vždy kontrola před a po. Struktury umisťovat logicky podle prostředí. Nehořlavé netherové lodě mají plout po lávě, nikoli viset nad lesem.

### Důležitá technická hranice

**Nová vlastní biome ID/F3 názvy, nové blokové ID a skutečně nový druh moba nebyly přidány.** Horizon zatím ignoruje nová biome ID z datapacku a nový blok nebo entity typ vyžaduje engine/mod registry. RC33 proto používá **3 výrazně odlišné podzemní habitaty v existujících vanilla cave biomech** a přiřazuje jim pouze nativní bloky/bytosti. To není plná implementace požadovaných vlastních biomů či mobů.

## Přidané habitaty

| Prostředí | Materiály / vegetace | Entity | Podzemní stavby |
|---|---|---|---|
| **Lush Caves – mechové síně** | vzácné `moss_carpet` a `azalea` pouze nad povolenou půdou, původní cave vines a glow lichen zachovány | původní axolotl, glow squid a netopýři zachováni; méně příliš častých slimes | nový `moss_library` a vhodný TREK lush fossil monster room |
| **Dripstone Caves – kalcitové katakomby** | vzácný kalcit, drobné ametystové žíly, hnědé a červené houby na vhodných podkladech | malá šance na cave spider a silverfish; nadměrná váha drowned snížena | nový `calcite_ossuary` a vzácný TREK dripstone fossil monster room |
| **Deep Dark – tuff/sculk archiv** | vzácná hlubinná tuffová mineralizace; vanilla sculk zachován | **žádní přirozeně spawnovaní mobové**, v souladu s původním Deep Dark; Warden mechanika nezměněna | nový `sculk_archive` jako vzácná zahrabaná zřícenina |

### Princip umístění

- Čtyři podzemní květinové/dekorační varianty využívají **omezení absolutní Y**, zároveň musí být alespoň **12 bloků pod skutečným povrchem**, hledají pevnou podlahu a vyžadují **vzduch nad odpovídajícím nativním blokem**.
- Tři minerální feature nahrazují **jen stone/deepslate ore-replaceable** bloky v daném jeskynním biomu. Žádné volně visící plošné přebarvování bloků.
- Nové původní mini-ruiny mají rozměry **13×8×13, 13×9×13 a 11×8×11**, s různými materiály a vnitřním prostorem. Jejich underground `minecraft:fossil` placement má navíc Y limit, biome filtr, >18 bloků pod povrchem, nízkou četnost a `max_empty_corners_allowed=4`. **Skutečný přístup z otevřené jeskyně nelze z JSON zaručit**; mohou být zčásti zahrabané, takže až hra rozhodne o kvalitě jejich umístění. Žádné dekorativní barely s lootem se do nových ruin nepřidaly.
- Dvě dříve velmi agresivní TREK `monster_room/upper` a `monster_room/lower` structure-set mřížky (**spacing 1 chunk, frequency 1**) byly **odstraněny z náhodného jigsaw výběru**. Původní šablony/feature procesory zůstaly v ZIPu. Místo toho jdou dvě malé tematické TREK místnosti přes vzácné biome-specific fossil placed-features (cca 1/72 pokusů na chunk + původní pravidla úspěšného umístění).
- **Netherové nehořlavé lodě nejsou aktivovány.** Umístění celé lodi do lávového jezera/oceánu vyžaduje kontrolu lávové hladiny a hloubky v celé její ploše; samotný biome tag není bezpečné řešení. Nether zůstává přesně jak v RC32; nejde o dokončenou opravu lávových lodí.

## Povinná kontrola PŘED / PO

| Kontrola | RC32 před | RC33 po |
|---|---:|---:|
| Souborů ZIP | 2 813 | **2 837** |
| JSON a `pack.mcmeta` | 1 555 | **1 575** |
| NBT šablon | 1 166 | **1 169** |
| Nových podzemních NBT | 0 | **3** |
| Aktivních TREK monster-room structure-setů s `spacing=1` | 2 | **0** |
| Biome feature DAG uzlů | 163 | **175**, bez cyklů |
| Původní NBT | 1 166 | **všech 1 166 byte-identicky** |

**Static QA: PASS**
- Vstupní i výstupní ZIP integrity/CRC; všechny JSON parsovatelné.
- Nezávislý Big-Endian NBT parser úspěšně přečetl všechny **3 nové šablony**, bez duplicitních blokových souřadnic a chybných indexů.
- Všechny nové cave feature mají platné vlastní odkazy na feature nebo NBT; každá je přiřazena výhradně do `lush_caves`, `dripstone_caves` nebo `deep_dark`.
- `minecraft:deep_dark` nemá nové přirozené spawny mobů.
- Žádný cyklus mezi pořadím worldgen feature ve všech přepsaných biomech.
- **183 netherových souborů a 232 endových souborů byte-identických s RC32**.
- `pack.png` z RC31/32 zachován byte-identicky.
- **Overworld −64..2031 / Nether 0..1023 / End 0..511** zachovány.

Výsledek: **`Thalvoryn_Horizon_Rust_RC33_CAVES_Nether1024_PRIVATE_TEST.zip`**  
Velikost: **9 018 316 B**  
SHA-256: `77e845c2ecf9d28d84cb67fe3692c3db6974698cd699b1677d63e6a831d9dda1`

## NEDOKONČENO — neoznačovat jako hotové

- Přímo ve hře nebylo možno otestovat skutečnou čitelnost/platnost těchto feature/structure registrací, výskyt, výkon ani vstupy do podzemních ruin. Statické testy ≠ runtime test.
- Vlastní nově registrovaná jeskynní biome ID a jejich názvy ve F3, vlastní nové bloky a nové druhy mobů vyžadují engine/mod kód.
- Přesný full-footprint terrain / liquid / cave accessibility validator pro všechny velké stavby, včetně Nether lava ships, není implementován.
- Nether portal safe arrival, distant LOD mismatch, další uživatelovy částí a Photon stále otevřené.
- Stejně jako dříve se ZIP s obsahem cizích map nesmí veřejně publikovat bez ověření přerozdělovacích práv.

**Test:** nový svět jen s RC33, ne současně s RC32. Zaměřit se na lush/dripstone/deep dark v Overworldu; po kontrole nejbližších dungeonů a podzemních ruin porovnat reálný spawn s touto dokumentací.

[Požadavky po částech](USER_REQUIREMENTS_BY_PART.md) · [Evidence hlášených chyb #2](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/issues/2).

Zdroje pro změny worldgenu: [Minecraft Java 26.3 changelog](https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3), [Horizon world-generation documentation](https://github.com/horizon-rust/horizon/blob/main/docs/world-generation.md).
