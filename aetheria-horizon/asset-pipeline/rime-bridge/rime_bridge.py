"""Aetheria M34 -- Rime Bridge original physical 3D sculpture.
Seven separated but tightly interlocking ice flagstones with faceted crowns,
not rods, rings, crossed sprites, or vanilla blocks. Model coordinates:
local Blender Y = bridge direction, Blender Z = up.
"""
import bpy, math, json
from pathlib import Path
from math import sin,cos,pi,tau
from mathutils import Vector
ROOT=Path(__file__).resolve().parent;OUT=ROOT/"export";OUT.mkdir(exist_ok=True)
bpy.ops.object.select_all(action="SELECT");bpy.ops.object.delete(use_global=False)
def material(name,c,glow=0.):
    m=bpy.data.materials.new(name);m.use_nodes=True
    m.diffuse_color=(*c,1.)
    bs=m.node_tree.nodes.get("Principled BSDF")
    bs.inputs["Base Color"].default_value=(*c,1.)
    bs.inputs["Metallic"].default_value=.08
    bs.inputs["Roughness"].default_value=.26
    bs.inputs["Emission Color"].default_value=(*c,1.)
    bs.inputs["Emission Strength"].default_value=glow
    return m
mats=[
    material("01_Glacier_Deep",(.017,.17,.33),.06),
    material("02_Rime_Skyblue",(.14,.53,.82),.12),
    material("03_Opal_Frost_Shard",(.37,.74,.83),.21),
]
def signed_volume(v,facets):
    volume=0.0
    for face in facets:
        a=v[face[0]]
        for j in range(1,len(face)-1):
            b=v[face[j]]; c=v[face[j+1]]
            volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])
                +a[1]*(b[2]*c[0]-b[0]*c[2])
                +a[2]*(b[0]*c[1]-b[1]*c[0]))/6
    return volume
def build(name,vertices,faces,ids):
    assert len(faces)==len(ids)
    pre=signed_volume(vertices,faces)
    assert abs(pre)>0.01,(name,pre)
    # Autocorrect WINDING of whole watertight body for Horizon front-face
    # culling, unlike old art source which had all faces reversed.
    if pre<0:faces=[tuple(reversed(x)) for x in faces]
    assert signed_volume(vertices,faces)>0
    edges={}
    for face in faces:
        for a,b in zip(face,face[1:]+face[:1]):
            edge=tuple(sorted((a,b)));edges[edge]=edges.get(edge,0)+1
    assert set(edges.values())=={2},(name,set(edges.values()))
    me=bpy.data.meshes.new(name+"_mesh")
    me.from_pydata(vertices,[],faces);me.update()
    for m in mats:me.materials.append(m)
    obj=bpy.data.objects.new(name,me);bpy.context.collection.objects.link(obj)
    uv=me.uv_layers.new(name="UV0")
    for i,p in enumerate(me.polygons):
        p.material_index=ids[i]
        p.use_smooth=False
        for li in p.loop_indices:
            v=me.vertices[me.loops[li].vertex_index].co
            uv.data[li].uv=(v.x*.3 +.5,v.y*.16 +.5)
    return obj
def octagon(cx,cy,w,l,corner):
    x=w/2.;y=l/2.;c=corner
    return [(cx-x+c,cy-y),(cx+x-c,cy-y),
            (cx+x,cy-y+c),(cx+x,cy+y-c),
            (cx+x-c,cy+y),(cx-x+c,cy+y),
            (cx-x,cy+y-c),(cx-x,cy-y+c)]
def ice_plate(index):
    # Seven *distinct* geometry meshes: all subtly cambered and asymmetric.
    # Spaces between plates can be read as crystal seams, not accidental holes.
    center_y=(index-3)*1.39
    center_x=0.11*sin(index*1.87)
    width=2.38+0.19*cos(index*1.31)
    length=1.34 # 0.05m visible seam, no randomly fused blocks
    crest=.13+.12*sin(index*1.35+0.5)
    rings=[
        (.72,.81,-.45),
        (.93,.94,-.39),
        (1.00,1.00,-.06),
        (.75,.78,crest)
    ]
    v=[]
    outline=octagon(center_x,center_y,width,length,.24)
    for layer,(sx,sy,z) in enumerate(rings):
        for i,(x,y) in enumerate(outline):
            height=z
            # Facet arcs that read in side profile, not a flat rectangular roof
            if layer in (2,3):
                height += .055*cos(i*2.3+index*.6)
            v.append((center_x+(x-center_x)*sx,
                      center_y+(y-center_y)*sy,
                      height+.035*sin(index*1.23)))
    f=[];ids=[]
    for layer in range(3):
        for i in range(8):
            k=(i+1)%8
            f.append((layer*8+i,layer*8+k,(layer+1)*8+k,(layer+1)*8+i))
            ids.append((0,1,2)[layer] if i%4!=0 else (1,2,1)[layer])
    f.append(tuple(reversed(range(8))));ids.append(0)
    f.append(tuple(24+i for i in range(8)));ids.append(2)
    return build("Rime_Bridge_Ice_Flagstone_%02d"%(index+1),v,f,ids)
def long_frost_wing(sign):
    """A true thick, closed, continuous glacial side buttress.
    Sweeps below the flagstones with changing height and curvature;
    neither a thin rod nor an empty frame."""
    total=12
    verts=[];faces=[];ids=[]
    for i in range(total+1):
        y=-4.68+9.36*i/total
        arc=math.sin(math.pi*i/total)
        x=sign*(1.19+.07*math.sin(i*.9+sign))
        drop=.41+.80*arc+.09*math.sin(4.1*i/total*math.pi)
        # A 6-sided cross section with broad filled bevels and underside.
        section=[
            (x-sign*.14,y,.14),
            (x+sign*.27,y,.08),
            (x+sign*.43,y,-.20-.12*arc),
            (x+sign*.22,y,-drop),
            (x-sign*.10,y,-drop*.82),
            (x-sign*.28,y,-.22)
        ]
        verts.extend(section)
    n=6
    for i in range(total):
        for j in range(n):
            k=(j+1)%n
            faces.append((i*n+j,i*n+k,(i+1)*n+k,(i+1)*n+j))
            ids.append(2 if j in (0,3) and i%4==0 else 1 if j in (1,4) else 0)
    faces.append(tuple(reversed(range(n))));ids.append(0)
    faces.append(tuple(total*n+j for j in range(n)));ids.append(0)
    return build("Rime_Bridge_Glacier_Buttress_"+("L" if sign<0 else "R"),
        verts,faces,ids)
objects=[ice_plate(i) for i in range(7)]
objects.extend([long_frost_wing(-1),long_frost_wing(1)])
# Studio ground-like lighting, only source geometry (no generated mockup).
bpy.ops.object.select_all(action="DESELECT")
for ob in objects:ob.select_set(True)
bpy.context.view_layer.objects.active=objects[0]
bpy.ops.wm.obj_export(filepath=str(OUT/"rime_bridge.obj"),export_selected_objects=True,export_materials=True)
bpy.ops.export_scene.gltf(filepath=str(OUT/"rime_bridge.glb"),export_format="GLB",use_selection=True,export_apply=False)
bpy.ops.wm.save_as_mainfile(filepath=str(OUT/"rime_bridge.blend"))
scene=bpy.context.scene;scene.render.engine="BLENDER_EEVEE"
scene.render.resolution_x=1400;scene.render.resolution_y=900
scene.render.resolution_percentage=100
scene.render.image_settings.file_format="PNG"
scene.world.color=(.04,.06,.095)
def lamp(name,location,power,color,size):
    bpy.ops.object.light_add(type="AREA",location=location)
    ob=bpy.context.object;ob.name=name;ob.data.energy=power
    ob.data.color=color;ob.data.shape="DISK";ob.data.size=size
lamp("Frost key",(-3,-5,9),1050,(.69,.86,1.0),8)
lamp("Opal bounce",(2,3,6),750,(.47,.92,.98),9)
lamp("Cold underside",(-1,1,-4),350,(.20,.56,1.0),7)
bpy.ops.object.camera_add()
cam=bpy.context.object;scene.camera=cam
cam.data.type="ORTHO"
for name,pos,size in [
    ("rime_bridge_threequarter.png",(8,-12,9),13.5),
    ("rime_bridge_side.png",(13,1,5),13.0)
]:
    cam.data.ortho_scale=size;cam.location=pos
    cam.rotation_euler=(Vector((0,0,-.05))-cam.location).to_track_quat("-Z","Y").to_euler()
    scene.render.filepath=str(OUT/name);bpy.ops.render.render(write_still=True)
    assert Path(scene.render.filepath).stat().st_size>40_000
report={"vertices":sum(len(o.data.vertices) for o in objects),
"polygons":sum(len(o.data.polygons) for o in objects),
"independent_closed_parts":len(objects),"floor_plates":7,"glacial_buttresses":2,
"two_actual_blender_renders":True,
"in_game_tested":False}
(OUT/"rime_bridge_QA.json").write_text(json.dumps(report,indent=2),encoding="utf8")
print("RIME_BRIDGE_AUTHOR_PASS",report)
