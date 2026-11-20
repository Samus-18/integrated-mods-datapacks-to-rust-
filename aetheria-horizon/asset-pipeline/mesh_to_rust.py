#!/usr/bin/env python3
"""Shared, deterministic, ORIGINAL Blender OBJ -> Horizon worldquad source.
Preserves every physical 3D face and every mesh part, no glTF placeholder.
Each source object MUST be watertight with outward-facing winding.
Usage: python asset-pipeline/mesh_to_rust.py input.obj output.rs
"""
from pathlib import Path
from collections import Counter
import argparse, hashlib, math, re

MATERIALS={
    "01_Glacier_Deep":0,
    "02_Rime_Skyblue":1,
    "03_Opal_Frost_Shard":2,
}
def sub(a,b):return tuple(x-y for x,y in zip(a,b))
def cross(a,b):return (a[1]*b[2]-a[2]*b[1],
    a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def volume(verts,faces):
    v=0.0
    for poly,_ in faces:
        for k in range(1,len(poly)-1):
            a,b,c=verts[poly[0]],verts[poly[k]],verts[poly[k+1]]
            v+=dot(a,cross(b,c))/6.0
    return v
def convert(source,destination):
    content=source.read_bytes().replace(b"\r\n",b"\n")
    digest=hashlib.sha256(content).hexdigest()
    vertices=[];parts=[];part_name=None;face_group=[]
    material=None
    def flush():
        nonlocal face_group,part_name
        if part_name is not None and face_group:
            parts.append((part_name,face_group))
        face_group=[]
    for line in content.decode("utf-8").splitlines():
        if line.startswith("o "):
            flush()
            part_name=line[2:].strip()
            assert part_name
        elif line.startswith("v "):
            pos=tuple(map(float,line.split()[1:4]))
            assert len(pos)==3 and all(math.isfinite(x) for x in pos)
            vertices.append(pos)
        elif line.startswith("usemtl "):
            material=MATERIALS[line.split(maxsplit=1)[1]]
        elif line.startswith("f "):
            assert part_name is not None and material is not None
            ids=[int(token.split("/")[0])-1 for token in line.split()[1:]]
            assert len(ids)>=3 and len(ids)%2==0,(part_name,len(ids))
            assert all(0<=i<len(vertices) for i in ids),(part_name,ids)
            assert len(set(ids))==len(ids)
            face_group.append((ids,material))
    flush()
    assert 2<=len(parts)<=64 and vertices
    converted=[]
    part_ranges=[]
    existing=len(vertices)
    for name,faces in parts:
        edges=Counter()
        for ids,_ in faces:
            for a,b in zip(ids,ids[1:]+ids[:1]):
                edges[tuple(sorted((a,b)))]+=1
        assert edges and set(edges.values())=={2},f"{name}: broken face-edge incidence {Counter(edges.values())}"
        vol=volume(vertices,faces)
        assert vol>0.0001,f"{name}: INSIDE-OUT or degenerate solid (signed volume {vol:.6f})"
        start=len(converted)
        for poly,mat in faces:
            n=len(poly)
            if n==4:
                converted.append((poly,mat))
            else:
                assert n in (6,8,10,12),f"Unexpected {n}-gon cap in {name}"
                middle=tuple(sum(vertices[ix][k] for ix in poly)/n for k in range(3))
                idx=len(vertices)
                vertices.append(middle)
                for k in range(0,n,2):
                    q=[idx,poly[k],poly[(k+1)%n],poly[(k+2)%n]]
                    converted.append((q,mat))
        length=len(converted)-start
        assert length>=6
        part_ranges.append((start,length,name,vol))
    assert len(vertices)>existing
    lines=[
        "// GENERATED from original Blender .obj by asset-pipeline/mesh_to_rust.py.",
        "// All parts outward-facing, watertight; no partial worldquad meshes.",
        f"// Canonical source OBJ SHA256: {digest}",
        f"pub(super) const VERTICES: [[f64;3];{len(vertices)}] = [",
    ]
    lines += ["    [{:.8}, {:.8}, {:.8}],".format(*x) for x in vertices]
    lines+=["];",f"pub(super) const QUADS: [([usize;4],u8);{len(converted)}] = ["]
    lines += [f"    ([{','.join(map(str,ids))}], {mat})," for ids,mat in converted]
    lines+=["];",f"pub(super) const PARTS: [(usize,usize);{len(part_ranges)}] = ["]
    for first,length,name,_ in part_ranges:
        assert "\n" not in name and "*" not in name
        lines.append(f"    ({first}, {length}), // {name}")
    lines+=["];",""]
    destination.parent.mkdir(parents=True,exist_ok=True)
    destination.write_text("\n".join(lines),encoding="utf8",newline="\n")
    print(f"PASS OBJ {source.name}: parts={len(parts)} vertices={len(vertices)} "
        f"quads={len(converted)} (base {existing} original vertices)")
    for _,length,name,vol in part_ranges:
        print(f"  {name}: {length} quads, volume={vol:.4f} outward")
    print("SHA256",digest)
if __name__=="__main__":
    a=argparse.ArgumentParser()
    a.add_argument("source",type=Path);a.add_argument("destination",type=Path)
    args=a.parse_args()
    convert(args.source,args.destination)
