# Požadavky uživatele – postupně zasílané části

**Záznam přijatých částí, ne finální roadmapa.** Aktualizovat při každé další zprávě uživatele. Úpravy testovaných RC balíčků jsou odlišeny od problémů vyžadujících Horizon engine.

## Část 1 — Overworld (přijato 2026-11-08)

Původní znění uživatele:
> 🌍 Overworld
>
> Nové biomy, výraznější hory, lepší jeskynní vstupy, různorodé Sky Islands a jejich výška podle terénu

Dodatečný požadavek stejné zprávy:
> chyby v datapacku ta custom ikona zapominas na to

### Rozpracováno v RC31
- [x] 4 vlastní tvarové Sky Islands + 1 dosavadní Sky Garden
- [x] Výška startu ostrovů od `WORLD_SURFACE_WG` (offset, nikoli absolutní Y)
- [x] Jeden sdílený structure set pro pět variant kvůli omezení spamu
- [x] Zvýrazněná extrémní horská větev bez změny limitu JJThunder
- [x] Horské a lesní průchody jeskyní a nový vzácný vyšší carver
- [x] 3 stylové/vegetační *varianty existujících biomes* pro podporovaný datapack systém
- [x] Nahrazení starého TREK obrázku vlastní ikonou `pack.png`
- [ ] Herní test RC31: počet a umístění ostrovů, geometrie vysokých hor, viditelnost vstupů jeskyní, kolize, výkon
- [ ] Skutečně nová biome ID/názvy ve F3: vyžaduje registraci do Horizon runtime, ne jen datapack
- [ ] Stínování/LOD vzdáleného terénu: engine otázka, neopravené

Technická dokumentace: [RC31](RC31_OVERWORLD_SKY_MOUNTAINS_CAVES_ICON_PRIVATE_TEST.md).

## Další části
Čekají na postupné zaslání uživatelem. Neoznačovat dosud nezaslané části jako dokončené.
