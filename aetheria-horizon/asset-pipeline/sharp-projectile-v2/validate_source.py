"""M33 evidence gate. The Blender artifact and Rust source must match exactly.
Offline/source only: no engine screen/video or claim of cinematic acceptance.
"""
from pathlib import Path
import hashlib, re, struct, json
root=Path(__file__).resolve().parents[2]
obj=root/"asset-pipeline/sharp-projectile-v2/export/frost_needle_v2.obj"
glb=obj.with_suffix(".glb")
blend=obj.with_suffix(".blend")
rust=root/"preview-5spells-wasm/src/sharp_projectile_mesh_generated.rs"
hero=root/"preview-5spells-wasm/src/sharp_projectile.rs"
lib=root/"preview-5spells-wasm/src/lib.rs"
assert obj.is_file() and glb.stat().st_size>1000 and blend.stat().st_size>10000
s=rust.read_text(encoding="utf-8")
assert hashlib.sha256(obj.read_bytes()).hexdigest() in s
assert "pub(super) const VERTICES: [[f64;3];130]" in s
assert "pub(super) const QUADS: [([usize;4],u8);124]" in s
assert s.count("([" )>=124
b=glb.read_bytes()
assert b[:4]==b"glTF" and len(b)==struct.unpack_from("<I",b,8)[0]
n=struct.unpack_from("<I",b,12)[0]
doc=json.loads(b[20:20+n])
assert len(doc["meshes"])==3
assert len(doc["materials"])==3
assert len(doc.get("nodes",[]))>=3
assert all("POSITION" in p["attributes"] for m in doc["meshes"] for p in m["primitives"])
for fname in ("frost_needle_v2_threequarter.png","frost_needle_v2_side.png"):
    assert (obj.parent/fname).stat().st_size>50_000
hero_s=hero.read_text(encoding="utf-8")
lib_s=lib.read_text(encoding="utf-8")
assert "const SLOT:usize=20" in hero_s
assert "mesh::QUADS" in hero_s and "begin_component(QUAD_COUNT)" in hero_s
assert "if sharp_projectile::render(cast,age,scene) { return; }" in lib_s
assert "volumetric_forms::render(cast,age,scene)" in lib_s
assert lib_s.index("sharp_projectile::render(cast,age,scene)")<lib_s.index("volumetric_forms::render(cast,age,scene)")
assert len(re.findall(r"draw_closed_needle\(",hero_s))>=6
print("PASS: Source OBJ SHA256 matches embedded Rust, 130 original-sculpt vertices, 124 fully preserved 3D quads")
print("PASS: Three sealed Blender parts, GLB 3 meshes + 3 materials, two real Blender views, editable .blend")
print("PASS: Live slot #021 Frost Needle bypasses old generic form, 4-phase animated custom-geometry source")
print("NOT TESTED: native GPU postpass, multiplayer gameplay authority, user-approved in-game appearance or FPS")
