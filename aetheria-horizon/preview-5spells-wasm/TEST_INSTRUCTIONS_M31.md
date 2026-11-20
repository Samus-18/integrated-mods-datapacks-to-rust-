# Aetheria Horizon — M31 GPU test v0.7.1
20. 11. 2026 | Horizon Rust | TEST, nikoliv hotovy mod

## Instalace
1. Rozbal ZIP, uvnitr najdes aetheria_preview.hmod.
2. Zavri hru a odeber stare aetheria_preview.hmod z horizon-mods a horizon-native-mods.
3. Soubor aetheria_preview.hmod vloz do slozky horizon-mods prislusne instance Horizonu.
4. V nastaveni modu aktivuj aetheria_preview a spust kreativni testovaci svet.

Test je postaveny pro Horizon SDK 0a875da91dbb0fdcb7ba8c42c11341aa229af51a (Nightly 2026-11-20).
U jinych Nightly buildu neni kompatibilita overena. Nepouziva Iris, OptiFine ani C++.

## Ovladani
G: seslat vybrane kouzlo (vlastni lokalni VFX)
N/M: predchozi/dalsi kouzlo
J/L: prepnout skolu kouzel
R + kolecko mysi: vyber kouzla
K: otevrit knihu / prehled kouzel
Z / X / C / V / B: rychla ukazka ohnive, ledove, prirodni, bleskove a astralni magie

## Co je noveho v M31
- Ve skutecnem pouzivanem WGSL shaderu je 6–10 kroku 3D volumetricke integrace pro pruhledne kouzelne clony.
- Pohybujici se fantasy hustota, svetlo uvnitr clony, prostorove sklady/zakruty.
- LEDOVY Shard Chorus (skola Cryomancer, treti kouzlo) ma cele souvisle 3D plochy mlhy a lepsi svetelne odezvy 3D krystalu.
- Nove materialy ohnive, ledove, prirodni, bleskove a astralni magie.
- Blizke efekty maji 10 vzorku; vzdalenost nad 34 bloku snizi pocet na 6.
- Hloubku vuci svetu resi WorldDepth::Tested na skutecnych 3D plochach.

## Co to jeste NENI
Tento prvni instalovatelny GPU test NEMA plne fullscreen volumetricke postprocesovani,
castecny lom sceny, native HDR bloom ani dynamicka realna svetla ve svete.
Neni zaruceno, ze vzhled uz odpovida referencnim videim.
200 ruznych kouzel ma navrhy VFX a zobrazeni, ale 170 mechanik jsou jen DesignIntent,
bez serverovych poskozeni/lecení, network sync, perzistence postavy nebo kouzelnych predmetu.
Nevystavuj to jako hotovy RPG mod.

## Overeni
Artefakt vznika jen z uspesneho GitHub Actions testu: skutecny Horizon pinned WASM build,
Rust testy, Naga WGSL validace, integrity ZIP, SHA512 a 30 OGG.
Bez screenshotu ani GPU mereni ve hre nemuzeme potvrdit profesionalni vizualni kvalitu.
Zpetna vazba po instalaci: hlavne GPU svetlo/LEDOVA MLHA, FPS a chybove logy.

Repo: https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/tree/aetheria-30-spell-test/aetheria-horizon

## Nightly 2026-11-20 API fix
Version 0.7.0 was built for an older transport ABI; Nightly 2026-11-20 refuses it. Version 0.7.1 is rebuilt with SDK 0a875da. The launcher cannot automatically recompile an already bundled WASM executable. Remove the old .hmod and install the updated one. Confirm that the in-game status is LOADED (not only launcher READY).
