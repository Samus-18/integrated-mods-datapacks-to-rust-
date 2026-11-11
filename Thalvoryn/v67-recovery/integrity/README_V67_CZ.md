# Thalvoryn V67 – izolovaná opravná testovací sada

**Verze:** V67, Minecraft 26.3 / cílový Horizon Nightly `2ebf00a` (2026-11-07).  **Stav:** nezávisle staticky ověřeno, **nikoli herně otestováno**.

## Opraveno oproti V67 ISOLATED

- Stellarity není součástí balíčku. End používá originální vanilla `minecraft:the_end` biome source; End City a jejich vanilla loď zůstávají.
- Systém jigsaw: **167 cyklů záložních template poolů** přerušeno; přidáno **10 chybějících poolů**. Nether skeleton camp má připojitelné pevnostní věže/obranu, piglin camp ore a jockey mají existující vlastní šablony. Tři chybějící *volitelné dekorativní* pooly mají bezpečnou prázdnou variantu (nepřidávají nesouvisející stavby).
- End City: změna na spacing 60 chunků, separation 25, frequency 0,52; velké End stavby jsou vzácnější. End Castle má ground-relative umístění na `end_highlands` s částečnou terénní adaptací; Rust má dodatečnou kontrolu sklonu a výšky.
- Spawnerové struktury v Overworldu: spacing 30 chunků, separation 12, frequency 0,27.
- **Původní Overworld density funkce nebyly měněny**, žádné NBT šablony nebyly přepsány, výšky Netheru 1024/2032 jsou oddělené.
- Custom pirátské lodě v Netheru a Endu nejsou v aktivních strukturálních skupinách; Overworld obsahuje mořské lodě a vzducholoď.

## Nezávislé automatické kontroly

- **4/4 ZIP PASS:** CRC, JSON syntax, 121 pack format a správné výšky Netheru.
- 1 393 template poolů, 24 structure setů, 479 unikátních aktivních strukturálních ID, 6 835 NBT šablon na variantu.
- Ze 5 659 dosažitelných NBT a ~10 378 odkazů na jigsaw pooly: **0 chybějících vlastních poolů, 0 chybějících šablon**. **0 chybějících vlastních loot ID** v prohledaných NBT.
- **0 cyklů záložních poolů**, struktury v aktivních skupinách nejsou zdvojené.
- Test pořadí features nenašel cyklus v přibalených aktivních definicích biomů. Není to důkaz pro nepřibalené vanilla registry.
- Všechny NBT bloky a Overworld density soubory jsou oproti izolovanému základu bitově nezměněné.

## Omezení / co zůstává nutné ověřit

- **Hra a rendering nebyly spuštěny**. Automatizované statické testy nemohou garantovat bezkolizní generování staveb v konkrétním seedu, napojení tunelů na procedurální jeskyně, End draka/portály ani výkon TPS/LOD.
- Tři dekorativní jigsaw výplně jsou úmyslně volitelné, protože původní zdroje neobsahují vhodné díly. Případné prázdné výklenky tedy nejsou vyřešeny novými vlastními modely.
- Pozor na nové chunky: změna datapacku **nepřegeneruje již vytvořené chunky**. Pro korektní srovnání vytvořte **úplně nový svět pro každou výškovou variantu**.
- Obsahuje materiály ze zdrojů nahraných uživatelem. Pouze soukromé testování; veřejné vydání vyžaduje kontrolu licencí zdrojů.

## Rychlý test v Horizonu – Nether1024

```mcfunction
/datapack list enabled
/gamemode spectator
/execute in minecraft:overworld run locate structure thalvoryn:overworld/handmade/air_vessel
/execute in minecraft:overworld run locate structure thalvoryn:underground/v67_cave_tunnel_01
/execute in minecraft:the_nether run locate structure nova_structures:nether_skeleton_camp_fort
/execute in minecraft:the_end run locate structure nova_structures:end_castle
/execute in minecraft:the_end run locate structure minecraft:end_city
```

Nezapomeňte prohlédnout celé budovy, otevřít skutečné truhly, projít portály a zkusit uložit/načíst nový svět. Pokud se něco usekne, potřebujeme **seed, dimenzi, XYZ a video**.

**Nepředstíráme 100% herní kompatibilitu bez skutečného spuštění hry.**