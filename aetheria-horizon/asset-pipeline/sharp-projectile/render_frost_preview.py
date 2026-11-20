import bpy, math
from mathutils import Vector
from pathlib import Path
base=Path(r"C:\Users\samue\Desktop\chatgpt propojeni\export")
bpy.ops.wm.open_mainfile(filepath=str(base/"frost_vfx.blend"))
scene=bpy.context.scene
scene.render.engine="BLENDER_WORKBENCH"
scene.display.shading.light="STUDIO"
scene.display.shading.color_type="MATERIAL"
scene.display.shading.show_shadows=True
scene.display.shading.show_cavity=True
scene.display.shading.cavity_type="BOTH"
scene.display.shading.curvature_ridge_factor=1.8
scene.display.shading.curvature_valley_factor=1.1
scene.display.shading.show_specular_highlight=True
scene.display.shading.background_type="WORLD"
scene.display.shading.show_object_outline=True
scene.render.resolution_x=1280
scene.render.resolution_y=800
scene.render.resolution_percentage=100
scene.camera=None
bpy.ops.object.camera_add(location=(7.6,-11.6,6.4))
cam=bpy.context.object
target=Vector((0,1.1,0))
cam.rotation_euler=(target-cam.location).to_track_quat("-Z","Y").to_euler()
cam.data.type="ORTHO"
cam.data.ortho_scale=8.9
scene.camera=cam
scene.render.image_settings.file_format="PNG"
scene.render.filepath=str(base/"frost_needle_blender_real_preview.png")
bpy.ops.render.render(write_still=True)
assert (base/"frost_needle_blender_real_preview.png").stat().st_size>20000
# Orthogonal second view reveals the long crystal needle, not only
# its cross-section visible from the ring-facing camera.
cam.location=(9.1,-2.0,5.6)
target=Vector((0,1.55,0))
cam.rotation_euler=(target-cam.location).to_track_quat("-Z","Y").to_euler()
cam.data.ortho_scale=11.0
scene.render.filepath=str(base/"frost_needle_side_real_preview.png")
bpy.ops.render.render(write_still=True)
assert (base/"frost_needle_side_real_preview.png").stat().st_size>20000
print("RENDER PASS: 2 real Blender previews, no AI, no Minecraft screenshot")
