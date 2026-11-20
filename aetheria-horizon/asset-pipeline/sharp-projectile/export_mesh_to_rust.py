"""Reproducible SharpProjectile V2 OBJ -> closed world-quad Rust source.
Three original watertight sculpt parts, 6/10-gon caps converted to quads.
Never substitute a procedural renderer for the source mesh.
"""
from pathlib import Path
from collections import Counter
import hashlib, math
root=Path(__file__).resolve().parents[2]
src=root/"asset-pipeline/sharp-projectile-v2/export/frost_needle_v2.obj"
dst=root/"preview-5spells-wasm/src/sharp_projectile_mesh_generated.rs"
vertices=[]
faces=[]
mat="";materials={"01_Faceted_deep_ice":0,"02_Sharp_blue_glass":1,"03_HDR_mint_emitter":2}
for raw in src.read_text(encoding="utf-8").splitlines():
    if raw.startswith("v "):
        v=tuple(map(float,raw.split()[1:4]))
        assert len(v)==3 and all(math.isfinite(x) for x in v)
        vertices.append(v)
    elif raw.startswith("usemtl "):
        mat=raw.split(maxsplit=1)[1].strip()
        assert mat in materials,mat
    elif raw.startswith("f "):
        indices=[int(s.split("/")[0])-1 for s in raw.split()[1:]]
        assert all(0<=i<len(vertices) for i in indices)
        faces.append((indices,materials[mat]))
assert len(vertices)==124 and len(faces)==108,(len(vertices),len(faces))
edge_counts=Counter()
for indices,_ in faces:
    for a,b in zip(indices,indices[1:]+indices[:1]):
        edge_counts[tuple(sorted((a,b)))]+=1
assert len(edge_counts)>30 and set(edge_counts.values())=={2},"Original Blender OBJ must be a closed manifold"
quads=[]
for indices,material in faces:
    n=len(indices)
    if n==4:quads.append((indices,material))
    elif n>=6 and n%2==0:
        center=tuple(sum(vertices[idx][k] for idx in indices)/n for k in range(3))
        mid=len(vertices);vertices.append(center)
        for j in range(0,n,2):
            quads.append(([mid,indices[j],indices[(j+1)%n],indices[(j+2)%n]],material))
    else:raise AssertionError(f"Unsupported {n}-sided face: not lossless")
assert len(vertices)==130 and len(quads)==124,(len(vertices),len(quads))
lines=[
"// GENERATED FROM OUR ORIGINAL BLENDER FROST NEEDLE. DO NOT EDIT BY HAND.",
"// Rebuild using asset-pipeline/sharp-projectile/export_mesh_to_rust.py.",
f'// SOURCE OBJ SHA256: {hashlib.sha256(src.read_bytes()).hexdigest()}',
"// Planar 6/10-gon caps are split into intact quad fans.",
f"pub(super) const VERTICES: [[f64;3];{len(vertices)}] = [",
]
lines += ["    [{:.6}, {:.6}, {:.6}],".format(*v) for v in vertices]
lines += ["];",
f"pub(super) const QUADS: [([usize;4],u8);{len(quads)}] = [",
]
lines += [f"    ([{','.join(map(str,ids))}], {mat})," for ids,mat in quads]
lines += ["];",""]
dst.write_text("\n".join(lines),encoding="utf-8",newline="\n")
print(f"FROST_OBJ_BUILT {len(vertices)} verts {len(quads)} independent 4-sided faces material counts {dict(Counter(x[1] for x in quads))}")
print(f"OBJ_SHA256 {hashlib.sha256(src.read_bytes()).hexdigest()} source {src.name}")
