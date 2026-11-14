# Automatický test Thalvorynu – bez Codexu

**Nainstalováno přímo na GitHubu.** Nemusíš instalovat HeadlessMC ani Codex.

- **GitHub Actions:** https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/workflows/thalvoryn-worldgen.yml
- **Zdroj testu:** https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/blob/main/tools/thalvoryn/ci/minecraft_smoke.py
- **Co automaticky dělá:** stáhne oficiální server Minecraft **Java 26.3**, nastaví Javu 25, vezme veřejný release balíček, vytvoří nový svět, vyvolá generování Overworld/Nether/End a uloží serverový log a JSON report.
- **Spouštění:** po změně testovacího skriptu/workflow, vydání nové GitHub Release nebo ručně v kartě Actions.
- **Současný veřejný testovaný balíček:** **RC13**, release tag `datapack`. Soukromé V76 ZIPy nejsou v GitHub Release a **nejsou tímto testem otestované**.
- **Omezení:** neprovádí vizuální kontrolu, renderer Horizonu ani LOD; krátký serverový smoke test také neprokáže, že je každá struktura ve světě správně umístěná.

## Výsledky
V GitHub Actions klikni na běh **Thalvoryn Minecraft 26.3 worldgen smoke**. Po dokončení obsahuje soubory `test-report.json`, `server.log` a `level.dat` v sekci Artifacts. Červený stav = server nebo některá kontrola selhala; nepovažovat za vydání.

Neumísťuj soukromé světy ani cizí strukturové ZIPy automaticky do veřejného repozitáře, dokud nejsou ověřené licence a případná citlivá data.

Všechny skripty používají standardní Python a oficiální Minecraft server, není třeba lokální aplikace.
