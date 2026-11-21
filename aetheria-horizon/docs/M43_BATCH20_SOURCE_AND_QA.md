# M43 — First twenty cross-school post-FX spell profiles
2026-11-21 / **source milestone, internal art QA only**

The user requires **work in batches of twenty**, not weeks of repainting
one Thorn Dart, and **real in-game images only**. This changes project
workflow: 20 per implementation/validation pass, no fake AI screenshots
and no numbered "done" without gameplay and art acceptance.

## 20 exact canonical 200-spell registry IDs
Two named spells from each school, so the first set explores genuinely
different artistic materials, not 20 orange recolours:

| School | Registry ID => post profile |
|---|---|
| Pyromancer | 1 Ember Needle => 1; 2 Cinder Fan => 2 |
| Frost | 21 Frost Needle => 3; 22 Rime Bridge => 4 |
| Wind | 41 Gale Palm => 5; 42 Razor Crescent => 6 |
| Earth | 61 Stone Knuckle => 7; 62 Crystal Traverse => 8 |
| Water | 81 Tide Needle => 9; 82 Flowstep => 10 |
| Nature | 101 Thorn Dart => 11; 102 Root Snare => 12 |
| Storm | 121 Spark Fork => 13; 122 Static Halo => 14 |
| Astral | 141 Star Shard => 15; 142 Gravity Thread => 16 |
| Time | 161 Second Hand => 17; 162 Moment Slip => 18 |
| Soul | 181 Bone Thorn => 19; 182 Soul Lantern => 20 |

## What now really exists in source
- Original, not extracted from God Wand: `shaderpacks/aetheria_spell20`.
- 20 individually implemented WGSL 3D spatial field branches (not generic
  geometry fallback), unique color/material identities, different motions.
- 4 fullscreen Horizon API 0.2 post passes: half-res emission ray-integral,
  horizontal+vertical downsample bloom, final linear-light scene composite.
- Read `PostInput.depth`, `view_position` to clip effect behind world;
  place local 3D form relative to actual camera and aim. Local screen lensing,
  no globally bleaching framebuffer. Quality option defaults conservative.
- Demo preview is DISABLED by default; optional single profile or cycle
  1–20 for visual acceptance, without installing God Wand or touching Photon.
- Pure Rust `post_profiles20.rs` maps exactly those 20 canonical registry IDs
  to phased VFX metadata and safe finite camera-relative `frame.mods` rows,
  with strict unit/zero-input tests.
- Local exact-source Rust QA: 35/35 passing, includes 2 new adapter tests.
- Official Horizon `horizon-shader check` and deterministic pack action:
  `.github/workflows/aetheria-m43-postfx20.yml`. Must pass before GPU QA.

## The critical remaining actual-integration steps
1. .hmod WASM custom-quad pipeline cannot access scene color/depth. These
   first 20 are a stand-alone Horizon .hshader, **not yet connected to
   gameplay casts**. Need native `Hooks::shader_params` bridge to pack's
   8 vec4 slot uniforms (if permitted by the user's Horizon installation).
2. Only one shaderpack selected in Horizon. No disabling Photon without
   explicit user action or safe compatible composite. Test isolated pack,
   preserve all original saves, options and mods.
3. Fullscreen raymarch may be expensive on RX580. Shader validation alone
   does not establish quality or performance. Measure actual GPU cost,
   quality low/high and occlusion in real Vulkan game.
4. Make audio, hit/damage, cooldown, mana, server sync, world effect,
   each skill animation and multi-player authority functional. Then
   capture real screenshot / video at 3 angles and accept/reject visuals.
5. Some skill semantics (Rime Bridge, Crystal Traverse, Stone Knuckle)
   demand true solid 3D objects and collision, not only volume/light.
6. The **user requested 100 fully complete skills** before playtesting
   and no incomplete user ZIP. Current done count: **NOT 20**. This is a
   20-profile graphics foundation pending real validation.

## Why this is a different route from M39–M41
The real God Wand 1.6.1 JAR uses multiple tailored GPU passes, 3D
projection/depth reconstruction, generated art atlases, authored world
geometry, game sound and phase-specific server logic. Our earlier giant
alpha card/needle-star was not equivalent. Read
`docs/research/GOD_WAND_161_RENDER_AUDIT.md` for precise class inventory,
technical inspiration and original-all-rights-reserved asset policy.
