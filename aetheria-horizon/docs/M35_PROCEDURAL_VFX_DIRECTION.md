# Aetheria VFX decision M35 — procedural first, no shortcuts

Decision recorded 2026-11-21, following the user's repeated objections to low-quality rods, rings and Blender-heavy detours. This is the **binding art/engineering direction** for future spell work. Preserve the user's words, previously promised behavior, and the actual state of implementation.

## User's exact instructions (Czech, original)

> "ano ale fakt zjisti prvne protoze nevim jestli to co lidi delaji ty effekty JSOU MODELY podivej se poradne a analyzuj to do podrobna to fakt nejsou modely"

> "tak si tenhle zpusob zapis jak vzdy i zpravy co rikas a udelej to a snaz se zadne obechcavky jak doposud delas"

Earlier corrections:
> "PROSTE ZNOVU se koukni na ty videa vyhledej si i dalsi vytvori rust kouzel ... ne slop"

> "jsou v tom modelu mezery ... a jsou moc takovy blokovy ostry ne lehky smooth" (this one described a separate **Roblox** project, not the Horizon renderer; preserve the project distinction)

User has a **limited allowance of authorized remote PC actions**; they explicitly objected to exhausting it on two spell models. **Do not use Remote Desktop Commander for this milestone.** Use the GitHub connector, GitHub Actions, and available in-session tools. Never claim an actual player video or game screenshot when only offscreen geometry is available.

## What the assistant promised in the previous reply

- Effects of the intended cinematic quality are often built with **animated textured particles, procedural geometry, dynamic motion and GPU shader materials**, **not a Blender object for every spell**.
- Verified public examples use: particle emitters, animated flipbook masks, ribbon trails, nontrivial material shading, screen-space post chains and bloom. Other implementations may use some genuine meshes for solid props (crystals, blades, weapons).
- Reuse the existing Rust client and pinned Horizon interfaces; no invented Java-to-Rust API mapping, no falsely claimed full-screen HDR hook in guest WASM.
- The existing 2 model-based spells and 198 draft effects are **not** 200 approved, production-quality spells.
- The user wants action on the full 200-spell magic system, not a lengthy series of single-model milestones.

## Verified public reference techniques (study, do not copy their code/assets)

God Wand / Atomic:
- `https://github.com/HyIsNoob/god-wand/blob/main/common/src/main/resources/assets/minecraft/shaders/program/godwand_atomic.fsh` — depth-aware, noise-driven procedural heat/plasma.
- `https://github.com/HyIsNoob/god-wand/blob/main/common/src/main/resources/assets/minecraft/shaders/post/godwand_atomic.json` — offscreen pass, bloom horizontal/vertical and composite.
- `https://github.com/HyIsNoob/god-wand/blob/main/common/src/main/java/net/godwand/client/GateDraw.java` — ribbons, gradients, textured faces, soft sprites, camera-relative fading.
- `https://github.com/HyIsNoob/god-wand/blob/main/common/src/main/java/net/godwand/client/FugaRenderer.java` — custom animated flame geometry, particles and delayed/phase-synced effects.
- `https://github.com/HyIsNoob/god-wand/blob/main/common/src/main/java/net/godwand/client/AtomicPostFx.java` — post chain prewarming and explicit shaders-disabled fallback.

Photon VFX `https://low-drag-mc.github.io/LowDragMC-Doc/en/photon2/`: particle emitters, modules, animations, trails and shader materials.
SpellEngine `https://github.com/ZsoltMolnarrr/SpellEngine`: synchronized casting choreography, actions, particles and audio.
The above are research references **not copied sources**. They are primarily for Java Minecraft and are not a Horizon guest plug-in.

## Architecture and hard acceptance rules

1. **Procedural first.** Original authored shape = position/time/velocity/forces/seed/phase + a shader material; use an external 3D model only for a truly solid object. Do not mass-produce 200 Blender meshes.
2. **GPU material-aware quads**: for the *verified Horizon guest* `Hooks::submit_custom_geometry` / `PipelineQuad` / optional `RenderPipeline`, make small camera-facing geometry that the fragment shader shapes into volumetric-looking **soft-edged** motes, flames, spindrift, sparks and droplets. Never leave visibly rectangular cards. All effects remain depth-tested. The WASM guest cannot assume a full-screen depth/scene-texture post hook exists.
3. **Distinct schools and ranks**: fire buoyant turbulence, frost cold spindrift, wind sheared vortices, stone heavy debris, water ballistic spray, flora pollen, lightning electrical scatter, astral drifting motes, time discontinuous echoes, souls restless wisps. These are procedural *accent emitters*, **not 200 independently approved hero effects**. Use deterministic per-spell seeds and phase envelopes.
4. **Safety and FPS**: bounded instance count and world quads, distance/overlap LOD, phase stop, deterministic behavior, finite coordinate guards, no extra permanent allocations and no GPU depth/bloom promises. On optional shader failure, **fallback to existing solid VFX**, not obvious opaque rectangular particle cards.
5. **CI verification**: compile against pinned Horizon SDK; test 200 IDs, at least 10 divergent schools, time progression, camera-facing quads, no NaNs, bounded geometry, optional pipeline fallback contract and shader Naga validation. The offscreen SVG test is *not* an actual GPU screenshot.
6. **Real-world spell effects** require separately verified server-authoritative gameplay hooks; don't call a ghost Rime Bridge collidable or make a client-only particle damage entities.
7. **Quality gate**: evaluate actual Horizon footage and RX580 FPS before calling any effect polished. No fake beauty approval, no user-facing final ZIP at a prototype stage. Keep draft PR; preserve working M34.
8. **Efficiency**: develop in GitHub branches, batch work, concise factual updates; no remote PC actions unless the user explicitly re-enables them.

## Progress counters — always distinguish

- Catalog: 200 metadata/intent entries.
- M34: 2 original Blender-bodied previews; 198 legacy geometry drafts.
- M35 target: **shared procedural GPU material and bounded emitter across 200 selectable spells**. This does *not* upgrade the count of polished/accepted hero spells to 200.
- Game-verified fully polished/usable spell count: **0** until real runtime evidence supports changing it.

## Next milestones after M35

- Verify M35 CI and shader via official pinned SDK.
- Improve distinct spell-level choreography via authored emitter configurations; don't stop at a shared color swap.
- Establish gameplay-authoritative VFX events/timestamps and final HDR/bloom bridge only via documented Horizon shader pack/native capabilities.
- Record actual in-game performance and user visual feedback. Fix issues and test again before release.
