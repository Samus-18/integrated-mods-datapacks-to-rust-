"""Original M33 Blender Frost Needle: a SOLID broad crystal/ice blade,
not an empty magic hoop, line, billboard, or copied game asset.
"""
import bpy,math,json
from pathlib import Path
from mathutils import Vector
from math import sin,cos,tau
ROOT=Path(__file__).resolve().parent; OUT=ROOT/"export";OUT.mkdir(exist_ok=True)
bpy.ops.object.select_all(action="SELECT");bpy.ops.object.delete(use_global=False)
def mat(name,rgb,emission):
 m=bpy.data.materials.new(name);m.diffuse_color=(*rgb,1)
 m.use_nodes=True
 p=m.node_tree.nodes.get("Principled BSDF")
 p.inputs["Base Color"].default_value=(*rgb,1)
 p.inputs["Metallic"].default_value=.05
 p.inputs["Roughness"].default_value=.22
 p.inputs["Emission Color"].default_value=(*rgb,1)
 p.inputs["Emission Strength"].default_value=emission
 return m
mats=[mat("01_Faceted_deep_ice",(.03,.19,.36),.16),mat("02_Sharp_blue_glass",(.14,.49,.79),.32),mat("03_HDR_mint_emitter",(.19,.82,.82),.55)]
def mesh(name,verts,faces,ids):
 # Imported source must have OUTWARD winding in the engine, unlike the
 # early test sculpt which looked fine double-sided in Blender but had
 # inward normals on all three closed shells.
 m=bpy.data.meshes.new(name)
 m.from_pydata(verts,[],[tuple(reversed(face)) for face in faces])
 m.update()
 o=bpy.data.objects.new(name,m);bpy.context.collection.objects.link(o)
 for material in mats:m.materials.append(material)
 uv=m.uv_layers.new(name="UV0")
 edges={}
 for i,poly in enumerate(m.polygons):
  poly.material_index=ids[i];poly.use_smooth=False
  indices=list(poly.vertices)
  for a,b in zip(indices,indices[1:]+indices[:1]):
   k=tuple(sorted((a,b)));edges[k]=edges.get(k,0)+1
  for li in poly.loop_indices:
   v=m.vertices[m.loops[li].vertex_index].co
   uv.data[li].uv=(v.x*.23+v.y*.13+.5,v.z*.18+.5)
 assert edges and set(edges.values())=={2},(name,set(edges.values()))
 return o
# +Y = flight axis (Blender OBJ export converts this to +Z).
rings=[(-2.20,.19,.17,-.09,-.05),(-1.80,.46,.30,-.08,-.05),
 (-1.35,.76,.48,.04,.08),(-.66,1.08,.67,.12,-.02),
 (-.12,1.21,.72,.03,-.08),(.52,1.00,.63,-.07,.04),
 (1.21,.72,.49,.04,.02),(2.02,.42,.30,-.04,-.04),
 (2.91,.17,.13,.03,.01),(3.74,.032,.033,0.,0.)]
n=10;v=[]
for j,(y,rx,rz,ox,oz) in enumerate(rings):
 for i in range(n):
  a=tau*i/n+.14*(j%2)-.055*j
  wobble=1+.07*sin(i*1.89+j*.91)
  ridge=1+.22*cos(2*a)**6
  v.append((ox+rx*cos(a)*wobble,y,oz+rz*sin(a)*wobble*ridge))
f=[];ids=[]
for j in range(len(rings)-1):
 for i in range(n):
  k=(i+1)%n
  f.append((j*n+i,j*n+k,(j+1)*n+k,(j+1)*n+i))
  ids.append(2 if (i+j)%7==0 else 1 if (i*3+j)%5 in (0,1) else 0)
f.extend((tuple(reversed(range(n))),tuple((len(rings)-1)*n+i for i in range(n))))
ids.extend((0,2))
parts=[mesh("Frost_Needle_v2_Solid_Crystal_Blade",v,f,ids)]
# Two closed solid shoulder fins, no radial ring / thin wire spokes.
for sign in (-1,1):
 polygon=[(sign*.57,-1.10),(sign*1.02,-1.75),(sign*1.70,-1.24),
          (sign*2.08,-.53),(sign*1.56,.27),(sign*.72,.31)]
 verts=[(x,y,z*(.60 if abs(x)>1.55 else 1.0))
        for z in (-.19,.19) for x,y in polygon]
 count=len(polygon)
 faces=[tuple(reversed(range(count))),
        tuple(count+i for i in range(count))]
 for i in range(count):
  j=(i+1)%count
  faces.append((i,j,count+j,count+i))
 if sign==1:faces=[tuple(reversed(face)) for face in faces]
 parts.append(mesh("Frost_Needle_v2_Solid_Wing_"+("L" if sign<0 else "R"),
   verts,faces,[0,1,0,2,1,0,1,0]))
bpy.ops.object.select_all(action="DESELECT")
for o in parts:o.select_set(True)
bpy.context.view_layer.objects.active=parts[0]
bpy.ops.wm.obj_export(filepath=str(OUT/"frost_needle_v2.obj"),
  export_selected_objects=True,export_materials=True)
bpy.ops.export_scene.gltf(filepath=str(OUT/"frost_needle_v2.glb"),
  export_format="GLB",use_selection=True,export_apply=False)
bpy.ops.wm.save_as_mainfile(filepath=str(OUT/"frost_needle_v2.blend"))
scene=bpy.context.scene
scene.render.engine="BLENDER_EEVEE";scene.render.resolution_x=1400
scene.render.resolution_y=850;scene.render.resolution_percentage=100
scene.render.image_settings.file_format="PNG"
scene.world.color=(.06,.08,.12)
def lamp(pos,power,col):
 bpy.ops.object.light_add(type="AREA",location=pos)
 ob=bpy.context.object;ob.data.energy=power;ob.data.color=col
 ob.data.shape="DISK";ob.data.size=6
lamp((-4,-4,7),650,(.54,.85,1.0))
lamp((3,2,4),440,(1.0,.75,.59))
lamp((3,-2,-2),300,(.30,.83,1.0))
bpy.ops.object.camera_add()
cam=bpy.context.object;scene.camera=cam;cam.data.type="ORTHO";cam.data.ortho_scale=7.5
for name,pos in [("frost_needle_v2_threequarter.png",(4,-6,3)),
                 ("frost_needle_v2_side.png",(7,-.8,2.8))]:
 cam.location=pos
 cam.rotation_euler=(Vector((0,0,0))-cam.location).to_track_quat("-Z","Y").to_euler()
 scene.render.filepath=str(OUT/name)
 bpy.ops.render.render(write_still=True)
 assert (OUT/name).stat().st_size>50000
report={"source":"original Blender sculpt, no IP copied",
 "vertices":sum(len(x.data.vertices) for x in parts),
 "faces":sum(len(x.data.polygons) for x in parts),
 "closed_parts":3,"renders":2,"in_game_tested":False}
(OUT/"frost_needle_v2_QA.json").write_text(json.dumps(report,indent=2),encoding="utf8")
print("FROST_V2_AUTHOR_PASS",report)
