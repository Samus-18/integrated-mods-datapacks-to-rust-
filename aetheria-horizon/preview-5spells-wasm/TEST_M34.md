# Aetheria Horizon — M34 INTERNAL VFX QA (not a final release)

Target: Horizon Nightly **2026-11-20**, Minecraft **26.3**, pinned SDK
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`.

**DO NOT INSTALL AS A COMPLETED GAMEPLAY MAGIC MOD.** The 200 ID catalog is a
prototype. Only original Frost Needle (#021) and Rime Bridge (#022) are
wired to genuine Blender-authored 3D models; the other 198 still use rough
generator geometry. For both ice spells, GPU beauty and FPS are NOT verified
in the user's Horizon game. Rime Bridge DOES NOT create world blocks or
collidable ice and is not safe to walk onto.

## Preview controls

- Tap **R**: change to the next of ten schools.
- Hold **R + scroll**: browse the 20 spells of the current school.
- Press **Z**: cast current spell. **G** is deliberately NOT mapped because
  it conflicted with Horizon's own key settings.
- To locate the two original scenes: choose CRYOMANCER with R, then
  choose Frost Needle (rank 1 / #021) or Rime Bridge (rank 2 / #022).
  Aim at a ground block for the intended anchored frost preview.

## Source/quality tracking

The model pipeline is reproducible: Blender `.blend` and script,
watertight OBJ/GLB, two genuine Blender camera renders, positive signed
volume face normals, canonical OBJ checksum -> exact source quads ->
test of *real Rust renderer* across 4 phases and RX580 quad budgets.

- Frost Needle M33: 3 sealed pieces, 124 full source quads.
- Rime Bridge M34: 8 distinct sealed sculpted slabs + 2 deep
  glacial buttresses, 436 full source quads.
- Pin M34 WASM CI must finish before compiling this an installation candidate.
- Guest post material is limited to Horizon world-space geometry plus
  depth-tested optional WGSL. Full fullscreen HDR bloom requires
  independently validated native/shader-pack interface.
- No authoritative multiplayer, damage/freezing/world building, inventory,
  crafting, saves or real traveling across the bridge.

Project repository notes: `docs/M34_RIME_BRIDGE_ORIGINAL_ART_RUNTIME.md`.
Original third-party references are not copied into this mod.
