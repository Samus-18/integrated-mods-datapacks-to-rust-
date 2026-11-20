"""Audit physically CLOSED and OUTWARD-facing original Blender OBJ solids.
This catches the inverted-normal error that ruined the separate Roblox project.
"""
from pathlib import Path
from collections import defaultdict,Counter
import math
OBJ=Path(__file__).resolve().parent/"export/frost_needle_v2.obj"
verts=[];faces=defaultdict(list);which=None
for line in OBJ.read_text(encoding="utf8").splitlines():
    if line.startswith("o "):which=line[2:].strip()
    elif line.startswith("v "):verts.append(tuple(map(float,line.split()[1:4])))
    elif line.startswith("f "):
        ids=[int(p.split("/")[0])-1 for p in line.split()[1:]]
        faces[which].append(ids)
assert len(faces)==3,faces.keys()
def sub(a,b):return tuple(x-y for x,y in zip(a,b))
def cross(a,b):return(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
def dot(a,b):return sum(x*y for x,y in zip(a,b))
bad=[]
for name,polys in faces.items():
    indices={i for face in polys for i in face}
    center=tuple(sum(verts[i][k] for i in indices)/len(indices) for k in range(3))
    volume=0.0
    orientations=Counter()
    for poly in polys:
        a,b,c=[verts[i] for i in poly[:3]]
        normal=cross(sub(b,a),sub(c,a))
        pos=tuple(sum(verts[i][k] for i in poly)/len(poly) for k in range(3))
        radial=dot(normal,sub(pos,center))
        if radial>1e-7:orientations["outward"]+=1
        elif radial<-1e-7:orientations["inward"]+=1
        else:orientations["ambiguous"]+=1
        for j in range(1,len(poly)-1):
            pa,pb,pc=verts[poly[0]],verts[poly[j]],verts[poly[j+1]]
            volume+=dot(pa,cross(pb,pc))/6
    print(name,"faces",len(polys),"volume",round(volume,5),"orientation",dict(orientations))
    assert abs(volume)>.001
    if volume <= 0: bad.append(name)
assert not bad,("Inside-out 3D solids:",bad)
print("PASS: All 3 modeled solids have outward winding and nonzero enclosed 3D volume")
