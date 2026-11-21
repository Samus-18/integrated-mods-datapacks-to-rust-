# Aetheria optional glow — Horizon shader API 0.2

**EXPERIMENTAL, user-controlled visual add-on.** This is a separate
`.hshader` pack, **not part of** the Aetheria guest WASM `.hmod`, and
it is not auto-installed or auto-enabled. Horizon runs one own-format shader
pack at a time; using this may replace an existing selected Horizon look.
Do not install without the user's permission.

It uses **only the official documented API 0.2** post stage with
`input.color`, `previous`, `linear_sampler`, `frame.resolution`
and `frame.luminance`. Original shader code, no external assets or
borrowed Java/Fabric APIs. Eight small radius taps per pixel, tunable
luminance gate, mild chroma-aware glow, no hardware ray tracing, no motion blur,
no unsupported framebuffer bindings or claimed per-spell masking.

Source validation is performed by the official
`horizon-shader check` tool pinned to
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`
and `horizon-shader pack` builds a valid `.hshader` **only if CI passes**.

## Actual limitations

- Bright blocks and sun/lava can glow too; this is a separate global
  shader poststage, not an accurate mask of only Aetheria spell instances.
- Frame time and power consumption on the user's RX580 **NOT TESTED**.
  The `[budget]` declaration is a target, not a performance benchmark.
- It cannot make 100 individually generic body meshes cinematic.
  The 100 procedural VFX remain art QA, no full in-game beauty pass.
- It does not change game world, health, mana, damage, collisions or saves.
- The current M36 mod's 12 soft particle materials are different and run
  on its own optional guest WGSL material pipeline; this post pack is extra.

See `docs/M36_100_AUTHORED_PROCEDURAL_SCENES.md` for quality gates.
