# M41 — strip the single giant billboard from the 100 authored VFX presets
2026-11-21, SOURCE-ONLY, QA / art verdict PENDING.

## Why the source was visually wrong
`authored_100_fx::render_hero()` drew **one giant camera-facing soft quad**
for most of the hundred names (three for a handful of structural effects).
An alpha mask is not the same as a spatial body, and a single mask cannot
express the 48 different motion patterns already authored. It tended to
obscure the actual separate 3D form.

## What changed
- Use **8–12 bounded small shader material fragments** distributed along
  each named spell's unique pattern (4–6 at reduced detail).
- Distinct per-spell pose, path, 3D offset, age, phase, twist, motion and
  material tag. Real world-space source form remains separate; no giant
  2D slab is accepted as a *hero*.
- Preserved all 100 named authored presets, 48 pattern families, ten schools,
  and bounded GPU fragments without unmasked rectangle fallback.
- Cap this light layer plus secondary motes at 80 quads per effect.
- Physical authoring: pattern pose is the anchor, not generic circle/rod.
- 32/32 local **exact-source Rust harness** tests pass including a new
  all-100 multi-piece non-giant-card regression guard (RX580-friendly cap).

## Not claimed
No 100 finished visual effects, no HDR/light buffers, no proof of pretty
actual gameplay, no server-authoritative combat or unlock mechanisms.
This is a repair to systemic overlay quality, to be followed by real
Horizon Vulkan screenshots and continued per-spell art direction. M40
ID-100 Thorn is excluded from this general soft overlay path and instead
uses its own distinct 3D body with dedicated spores.

The user's 100-of-200 first-playtest gate is **NOT MET**.

## M40 visual failure evidence motivating M41
M40 actual Radeon RX580 Vulkan screenshots archived under docs/qa/actual-horizon
showed a sparse star made of sharp needle spokes, not a solid leaf growth.
Source cause was the leaf width vector parallel to outward radial growth.
M41 corrects leaf tangent/perpendicular thickness, overlapping foliage
and a large connected seed core, and adds a regression test for this error.
33/33 local Rust QA tests pass, real M41 game QA still pending.
