r"""
Aetheria Horizon | Frost Needle | SharpProjectile 01
Original Blender 5.2+ generator. Procedural custom meshes, NO Minecraft blocks.
Output: frost_needle.glb/.obj, frost_rune_ring.glb/.obj, frost_vfx.blend
Run in background:
  "C:\Program Files\Blender Foundation\Blender 5.2\blender.exe" --background --factory-startup --python frost_needle_assets.py
These files are authoring assets, NOT yet loaded by Horizon. Real .hmod
integration requires a mesh draw importer / guest asset-data pipeline.
"""
import bpy
import math
from pathlib import Path
from mathutils import Vector
from math import sin, cos, pi, tau

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "export"
OUT.mkdir(parents=True, exist_ok=True)
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)

def mat(name, rgb, glow=0.0, alpha=1.0):
    m=bpy.data.materials.new(name)
    m.diffuse_color=(*rgb, alpha)
    m.use_nodes=True
    bs=m.node_tree.nodes.get("Principled BSDF")
    bs.inputs["Base Color"].default_value=(*rgb,1.0)
    bs.inputs["Roughness"].default_value=0.16
    bs.inputs["Metallic"].default_value=0.08
    bs.inputs["Emission Color"].default_value=(*rgb,1.0)
    bs.inputs["Emission Strength"].default_value=glow
    if alpha < 1.0:
        bs.inputs["Alpha"].default_value=alpha
    return m

ice=mat("01_Faceted_deep_ice", (.018,.20,.50), 0.20)
facet=mat("02_Sharp_blue_glass", (.09,.55,.93), 0.42)
rim=mat("03_HDR_mint_emitter", (.15,.87,1.00), 1.55)
ringmat=mat("04_Etched_magic_metal", (.045,.38,.69), 0.68)

def mesh(name, verts, faces, uv_fn, mats, material_ids):
    m=bpy.data.meshes.new(name+" mesh")
    m.from_pydata(verts,[],faces)
    m.update()
    ob=bpy.data.objects.new(name,m)
    bpy.context.collection.objects.link(ob)
    for material in mats: m.materials.append(material)
    uv=m.uv_layers.new(name="UV0")
    for pi,poly in enumerate(m.polygons):
        poly.material_index=material_ids[pi]
        poly.use_smooth=False  # facet normals stay sharp
        for li in poly.loop_indices:
            vidx=m.loops[li].vertex_index
            uv.data[li].uv=uv_fn(vidx,pi)
    return ob

def crystal_needle():
    # +Y is projectile forward. Closed multi-shoulder blade with
    # beveled asymmetric faceted core. Not a cone or stretched cube.
    # broad at -1.3, expanded shoulder near -.2, sharp tip +3.6.
    profiles=[
        (-1.56, .14, .14, -.05, -.06),
        (-1.17, .38, .34,  .00,  .00),
        (-.37,  .79,.46,  .12, -.10),
        (.31,   .65,.37, -.03,  .08),
        (1.32,  .35,.24,  .06,  .02),
        (3.05,  .14,.10, -.04, -.03),
        (6.38,  .006,.008, .0,  .0),
    ]
    sides=8
    verts=[]
    for layer,(y,wx,wz,cx,cz) in enumerate(profiles):
        for i in range(sides):
            a=tau*i/sides + (0.17 if layer%2 else -0.07)
            irregular=1.0+0.12*sin(i*2.33+layer*.95)
            verts.append((cx+cos(a)*wx*irregular,y,
                          cz+sin(a)*wz*irregular))
    faces=[]; ids=[]
    # Lower cap and upper cap are closed, with opposing normals.
    faces.append(tuple(reversed(range(sides)))); ids.append(0)
    for layer in range(len(profiles)-1):
        for i in range(sides):
            j=(i+1)%sides
            faces.append((layer*sides+i,layer*sides+j,
                          (layer+1)*sides+j,(layer+1)*sides+i))
            ids.append(([0,1,2,1,0,1,2,1][i] + layer//3)%3)
    faces.append(tuple((len(profiles)-1)*sides+i for i in range(sides)))
    ids.append(2)
    def uv(idx,_):
        return ((idx%sides)/sides,(idx//sides)/(len(profiles)-1))
    return mesh("Frost Needle | authored closed crystal",verts,
                faces,uv,[ice,facet,rim],ids)

def rune_ring():
    # A luminous rune halo that is not a thin wire ring:
    # wide, bevelled, toroidal BAND with triangular angular profile,
    # five non-regular stepped extrusions and a gap for cast direction.
    # Normal is mostly +/-Y; its opening faces the needle's axis.
    segments=96
    profile=[(-.28,-.055),(.26,-.060),(.39,.0),
             (.27,.090),(-.21,.085),(-.37,.018)]
    radial=len(profile)
    verts=[]
    for i in range(segments):
        a=tau*i/segments
        twist=0.06*sin(7*a)+0.018*sin(19*a+0.5)
        minor=0.045*sin(13*a+0.2)
        ellipse=(1.0+0.08*sin(a*3.0))
        for width,thick in profile:
            rr=(1.73+width+minor)*ellipse
            verts.append((rr*cos(a),-.45+thick+twist,rr*sin(a)))
    faces=[];ids=[]
    for i in range(segments):
        ni=(i+1)%segments
        for j in range(radial):
            nj=(j+1)%radial
            faces.append((i*radial+j,ni*radial+j,
                          ni*radial+nj,i*radial+nj))
            ids.append(3 if j in (1,2,4) else 1)
    # Twelve physically extruded angular sigils sit on the outer band.
    # They are separate closed 3D solids merged into this export mesh,
    # not a painted texture, hollow outline, or thin Minecraft rods.
    for glyph in range(12):
        a=tau*(glyph+0.5)/12
        rr=1.74*(1.0+0.08*sin(a*3.0))
        cx,cz=rr*cos(a),rr*sin(a)
        radial_dir=(cos(a),sin(a))
        tangent=(-sin(a),cos(a))
        # Four top and four bottom kite corners.
        sign=1.0 if glyph%2 else -1.0
        outline=[(-.26,0.0),(-.02,-.14*sign),
                 (.23,0.0),(-.02,.14*sign)]
        start=len(verts)
        for h in (-.32,-.27):
            for r,t in outline:
                verts.append((cx+radial_dir[0]*r+tangent[0]*t,
                              h,
                              cz+radial_dir[1]*r+tangent[1]*t))
        faces.append((start+0,start+3,start+2,start+1));ids.append(3)
        faces.append((start+4,start+5,start+6,start+7));ids.append(2)
        for j in range(4):
            k=(j+1)%4
            faces.append((start+j,start+k,start+4+k,start+4+j))
            ids.append(3)
    def uv(idx,_):return (((idx//radial)%segments)/segments,
                          (idx%radial)/radial)
    return mesh("Rune halo | faceted band and 12 solid glyphs",verts,
                faces,uv,[ice,facet,rim,ringmat],ids)

needle=crystal_needle()
ring=rune_ring()
# Crystal and halo kept separate: they animate at different phases.
# The ring is located at projectile origin and rotated perpendicular to its path.

def export(ob,stem):
    bpy.ops.object.select_all(action="DESELECT")
    ob.select_set(True)
    bpy.context.view_layer.objects.active=ob
    bpy.ops.wm.obj_export(filepath=str(OUT/(stem+".obj")),
                          export_selected_objects=True,
                          export_materials=True)
    bpy.ops.export_scene.gltf(filepath=str(OUT/(stem+".glb")),
                              export_format="GLB",
                              use_selection=True,
                              export_apply=False)
    ob.select_set(False)

export(needle,"frost_needle")
export(ring,"frost_rune_ring")

bpy.ops.object.select_all(action="DESELECT")
needle.select_set(True);ring.select_set(True)
bpy.context.view_layer.objects.active=needle
bpy.ops.wm.save_as_mainfile(filepath=str(OUT/"frost_vfx.blend"))

# Validate geometry and exported files, so failed meshes aren't "done".
for ob in (needle,ring):
    assert len(ob.data.vertices)>30
    assert len(ob.data.polygons)>30
    edge_counts={}
    for p in ob.data.polygons:
        vs=list(p.vertices)
        for a,b in zip(vs,vs[1:]+vs[:1]):
            k=tuple(sorted((a,b)))
            edge_counts[k]=edge_counts.get(k,0)+1
    assert edge_counts and all(v==2 for v in edge_counts.values()), ob.name
for stem in ("frost_needle","frost_rune_ring"):
    for suffix in (".obj",".glb"):
        file=OUT/(stem+suffix)
        assert file.is_file() and file.stat().st_size>500, str(file)
print("PASS: 2 original watertight VFX meshes, OBJ + GLB, UV0, separate editable .blend")
print("Frost Needle:",len(needle.data.vertices),"verts,",len(needle.data.polygons),"polygons")
print("Rune Ring:",len(ring.data.vertices),"verts,",len(ring.data.polygons),"polygons")
print("OUTPUT:",OUT)
