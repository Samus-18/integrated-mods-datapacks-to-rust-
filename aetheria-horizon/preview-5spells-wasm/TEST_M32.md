# Aetheria Horizon — M32 ART REBUILD v0.8.0
Testovni mod pro Horizon Nightly 2026-11-20 (MC 26.3)
SDK: 0a875da91dbb0fdcb7ba8c42c11341aa229af51a

## Instalace
- Uzavri Horizon.
- Smaz stare Aetheria aetheria_preview.hmod z instance slozky horizon-mods, abys neinstaloval dva moduly najednou.
- Z tohoto balicku rozbal aetheria_preview.hmod do horizon-mods instance.
- V launcheru mod aktivuj a spust hru. V menu modu ve hre over, ze je LOADED.
- Vytvor testovaci kreativni svet, abys pri kontrolach neprisel o postup.

## NOVÉ JEDNODUCHÉ OVLADANI (jen dve klavesy)
**R (kratce stisknout)**: dalsi z 10 magickych skol.
**R (drzet) + kolecko mysi**: vybirat z 20 kouzel dane skoly.
**G**: seslat vybrane kouzlo na misto pred hracem.

Zadne K/N/M/J/L/Z/X/C/V/B bindingy. Obri 200-kartove menu K uz neni soucasti aktivniho UI.
Mala trojice v nabidce nad hotbarem se zobrazi jen pri vyberu.

## M32 ZMENY VFX
- Stary zivy renderer plny tyci, kruhu, rutinnich 'spikes' a kostlivych pruhu je preskocen.
- Aktivni renderer: vyplnene 3D povrchy, pohyblive siroke objemove zavese a objemove obaly, velke nejehlove utrzene monolity.
- Rozlisuje vsech 10 skol a take puvodni vyznam kazdeho katalogoveho popisu 'mesh' (vlna / nebe / portaly / masiv / organismy apod.).
- Translucentni atmosferu realne fragmentove shaduje WGSL integrovana hustejsi/prostorova materialova funkce; blizky LOD 10 kroku / vzdaleny 6.
- Krystaly, hmoty a masivni povrchy maji normalove odlesky a hloubkovy test.
- Rozpocet zustava pod 900 world-quads na snimek a fair schedulovani netrha uzavrene komponenty.

## CO JE STALE VYVOJOVA VERZE
- Neni to jeste profesionalni cinematic HDR render jako referencni videa. Horizon guest world-pipeline nema scene-color/depth buffer pro plny fullscreen post/bloom.
- 200 jmen a casovani kouzel: existuji. 200 individualne dokoncenych a vizualne schvalenych kouzel: NE.
- 170 DesignIntent nema serverovou herni mechaniku ani multiplayer synchronizaci.
- Vizualni kontrola v Rust offscreen QA neni gameplay FPS/HD screenshot; kvalitni in-game zaznam je nutny.
- Pokud neco bude spatne, posli kratke video 10-20s s konkretni skolou, kouzlem a FPS, ne cely 2minutovy klip.

ZDROJE (vyzkum; nezahrnuji kopirovany zdrojovy kod):
Horizon official shaders API 0.2: https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md
Orbital Laser technical explanation: https://github.com/Stink-O/orbital-laser/blob/main/docs/HOW-IT-WORKS.md
Spell Engine (Java mod, UX/particles study): https://github.com/ZsoltMolnarrr/SpellEngine
Magic Particles Lib (Java mod, 3D effect extrusion): https://www.curseforge.com/minecraft/mc-mods/magic-particles-lib

Mod v Rust + WGSL, zadny port na Java, zadny C++/Iris/OptiFine.
