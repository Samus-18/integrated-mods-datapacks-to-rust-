# Aetheria — original spatial SFX pipeline

## Sound identity
Each first-test visual spell has an **original synthesized** three-stage sound:

| Fantasy type | Charge | Cast | Impact |
|---|---|---|---|
| Phoenix | Warm growing low harmonic | Fire-breath burst | Ash-and-bass rumble |
| Glacier | Glass harmonic tension | Icy rushing crack | Jagged brittle crystal collapse |
| Worldroot | Wooden resonant growth | Branching tonal sweep | Woody thump and leaf shimmer |
| Thunder Wyrm | Static rumbling build | Fast electric crackle | Deep thunder with rapid transients |
| Astral Sunder | Low eerie pressure | Sweeping dissonant tear | Gravity-drop resonant tail |

The generator in `preview-5spells-wasm/tools/generate_sound_assets.py` uses deterministic Python standard-library oscillator/noise synthesis, a 24 kHz mono PCM stage and FFmpeg/Vorbis. No paid fal credits and no recorded/copied audio. Generated `.ogg` files and `sounds.json` are packaged by CI into the preview `.hmod`.

Horizon API: `ClientHost::play_local_sound(&SoundPlay::at("aetheria_preview:glacier_cast", SoundSource::Blocks, origin), after_ticks)` for sequential stage events. Works on native and wasm according to official Horizon docs. Preview audio is **client-local only**; multiplayer/server audio must be replicated and validated later.

Important: synthesized stems passing CI integrity is **not** subjective sound quality or user-tested sound. Tune layering, loudness, panning, environment reverb and ducking after the M100 manual test. No vanilla sound shortcuts in the intended final mod.
