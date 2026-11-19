# M17 — ORIGINAL video rewatch & recent Minecraft magic research (2026-11-19)

## User's acceptance test
No giant empty outline rings, sparse rods, generic look-alikes, or geometry-count claims substituted for artistic quality. Build cohesive, individually authored magical forms with kinetic material, atmospheric layering and legible charge–release–impact–afterglow.

## Four original videos DIRECTLY reopened (not just an old summary)
The exact four original MP4 attachments were reopened, independently decoded and sampled at 12 positions each. Their original verified byte archive is in the user's ChatGPT Library. GitHub currently has the video catalog and SHA-256 references only under references/original-videos/README.md (no fake claim that the full binaries are git blobs).

- Download (3).mp4, 27.37s — at 5s a concentrated emerald/gold pillar spanning dusk sky; at 21s a bright grounded violet/golden ritual and atmospheric burst. Art lesson: strong luminous CORE + broad soft atmosphere + timing, not wire rings.
- Download (4).mp4, 58s — at 11s large black aperture with radiating sky fractures; around 30s descending mass and extensive terrain dust. Art lesson: recognizable NEGATIVE-space body, debris/dust, a scene-scale consequence, not single-line cracks.
- Download (5).mp4, 150.72s — at 93s intense pink/violet ritual across terrain; at 131s tall blue lightning/sky-beam with broad atmospheric effects, plus molten aftermath at later frames. Art lesson: staging, material glow, framing and space/light interaction are essential. Original master prompt prohibits copying sci-fi orbital weapons: use their spatial beam TECHNIQUE only.
- Download (6).mp4, 40.1s — around 9–16s enormous blue/purple crystalline eruptions, foglike luminous colored air and substantial crystals growing in phases. Art lesson: actual contiguous ice mass and layered iridescence, not handfuls of narrow crystal rods.

These are DIRECT frame observations, not a claim that the video reveals original source code or a runnable Horizon build.

## Additional CURRENT external references (not importable Horizon mods)

1. God Wand — https://www.curseforge.com/minecraft/mc-mods/god-wand — charged casting, distinct silhouettes and cinematic progression.
2. Calamity Spells — https://www.curseforge.com/minecraft/mc-mods/calamity-spells — updated Nov 2026, Forge 1.20.1; giant animated 3D visual manifestations, pillars, 26 sounds.
3. Void Manifestations — https://www.curseforge.com/minecraft/mc-mods/void-manifestations-spells-addon — persistent gravity magic, terrain reaction and sustained VFX.
4. Arcane Zenith — https://www.curseforge.com/minecraft/mc-mods/arcane-zenith — cinematic spell identity, elemental effects and combos (author claims, not audited source).
5. AdventurePack Magic — https://www.curseforge.com/minecraft/mc-mods/adventurepack-magic-spells-runes — ribbon trails and volumetric-looking waves (author claims).
6. Official Horizon API — https://playhorizon.dev/docs/apis — shader pack API 0.2 today; current project is pinned to an earlier SDK commit. Full HDR bloom/refractive post effects through a shader pack or native integration need separately proven API support. The existing WASM guest PipelineInput is only UV/color + frame.time.

The user mentioned beautiful mods made with Claude/AI; **none of the source pages verified which model produced them**. Do not attribute development to specific AI models without evidence. Do not copy protected third-party art/music.

## M17 implemented source changes
- preview-5spells-wasm/src/hero_vfx.rs — bespoke Shard Chorus (cryomancy ID 23): staggered large closed 3D glacial spires, colored prismatic veins, wide crystalline flower wings, solid terrain-rooted ice cleavage, drifting translucent colored spatial haze. Custom phases from shared Rust timeline; not rings/rod templates.
- preview-5spells-wasm/src/lib.rs — route the actual playable demo slot 5 directly to that new hero renderer; bypass legacy sculpt overlay.
- assets/aetheria_preview/shaders/pipeline/ethereal.wgsl — richer two-scale flowing material noise, glacial veins, warm vs cold palette-dependent chromatic cues and controlled alpha without overstating HDR output.
- Automated test for finite/grounded physical crystal geometry and visible broad quad faces under the shared 900-quad capacity; existing 180-view snapshot CI remains the appearance debugging baseline.

## NOT accomplished by this pass
Real 3D world lighting, physically correct HDR/bloom, GPU volumetric fog, world-space depth refraction, real block reaction, 30 completed professional-grade distinct effects, verified user-Nightly screenshots or production multiplayer gameplay. The pinned guest renderer alone cannot supply these; study and implement a compatible native/Horizon shader-pack bridge rather than faking their existence. On RX 580, performance still requires real GPU measurements.
