# Aetheria · Archetype 01 / SharpProjectile

**First non-gameplay artist asset experiment.** This is intentionally **one spell only**:
Frost Needle. The old 200 prototype renderers must NOT be presented as complete.

## Generate in Blender 5.2+
```powershell
& "C:\Program Files\Blender Foundation\Blender 5.2\blender.exe" --background --factory-startup --python frost_needle_assets.py
```
Run this from the folder containing frost_needle_assets.py. Script outputs
`export/frost_needle.glb`, `export/frost_needle.obj`, `export/frost_rune_ring.glb`,
`export/frost_rune_ring.obj`, and `export/frost_vfx.blend`.

Optional: run `render_frost_preview.py` using Blender's --python switch.
It renders two **actual Blender Workbench** previews (not AI pictures, not gameplay).

### Verified on user's Windows PC
- Blender 5.2.1 LTS and Python 3.11.
- Frost Needle: 56 vertices, 50 faces; closed, sharp, asymmetric loft, UV0.
- Rune ring: 672 vertices, 648 faces; closed heavy geometry plus 12 solid extruded sigils, UV0.
- Both exported to OBJ/GLB and passed edge-incidence checks (two polygons/edge).

### Next: real Rust + WGSL first and only archetype
- **Mesh import:** GLB assets do **not** automatically become Horizon world quads. Need
  custom guest asset loader/triangle batches or a verified engine-native mesh route.
- **Material:** noise/FBM/Voronoi UV distortion, density dissolve, computed face normals
  and Frenel edge highlights, correct HDR pipeline and translucent depth treatment.
- **4 phases:** 0.2s charge rune -> fast spatial projectile with curved trail ->
  impact volume/shards -> short restrained impact screen pass only if native HDR
  path supports it.
- **Performance:** PC target Radeon RX 580. Use a frame-time and face-budget test.
- **Acceptance:** real in-game GPU video and FPS, not just success of Blender export,
  Rust tests, Naga parser, or synthetic screenshots.

**Important:** the previous live mod did not actually place Minecraft ice blocks to
create its VFX; its old procedural quads merely LOOKED like crude blocky scaffolds.
The active selector had already changed to volumetric_forms.rs. We will retire that
prototype from the **live** render path only after SharpProjectile genuinely renders
and passes an in-game comparison. Deleting dependent source blindly before that
would break the build without improving the graphics.
