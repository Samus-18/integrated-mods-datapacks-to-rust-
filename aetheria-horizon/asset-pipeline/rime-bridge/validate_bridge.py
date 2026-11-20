"""M34 independent 3D quality/packaging gate.
*Must* be correct across Windows CRLF and Linux LF checkouts.
Checks the real source model to the exact active Horizon Rust render path.
Does not pretend this proves gameplay world-block collision or cinematic art.
"""
from pathlib import Path
import importlib.util, hashlib, json, re, struct, tempfile
from collections import Counter
ROOT=Path(__file__).resolve().parents[2]
PACK=Path(__file__).resolve().parent
OBJ=PACK/"export/rime_bridge.obj"
GLB=PACK/"export/rime_bridge.glb"
BLEND=PACK/"export/rime_bridge.blend"
RUST=ROOT/"preview-5spells-wasm/src/rime_bridge_mesh_generated.rs"
HERO=ROOT/"preview-5spells-wasm/src/rime_bridge.rs"
LIB=ROOT/"preview-5spells-wasm/src/lib.rs"
CORE=ROOT/"asset-pipeline/mesh_to_rust.py"
assert OBJ.is_file() and GLB.stat().st_size>1000 and BLEND.stat().st_size>10000
raw=OBJ.read_bytes().replace(b"\r\n",b"\n")
source_hash=hashlib.sha256(raw).hexdigest()
assert source_hash in RUST.read_text(encoding="utf8")
rawg=GLB.read_bytes()
assert rawg[:4]==b"glTF" and struct.unpack_from("<I",rawg,8)[0]==len(rawg)
chunk=struct.unpack_from("<I",rawg,12)[0]
gltf=json.loads(rawg[20:20+chunk])
assert len(gltf["meshes"])==9
assert len(gltf["materials"])==3
assert all("POSITION" in p["attributes"] for m in gltf["meshes"] for p in m["primitives"])
report=json.loads((PACK/"export/rime_bridge_QA.json").read_text(encoding="utf8"))
assert report["vertices"]==380
assert report["polygons"]==330
assert report["independent_closed_parts"]==9
assert report["floor_plates"]==7 and report["glacial_buttresses"]==2
assert report["in_game_tested"] is False
for file in ("rime_bridge_threequarter.png","rime_bridge_side.png"):
    assert (PACK/"export"/file).stat().st_size>40_000
src=RUST.read_text(encoding="utf8")
assert "pub(super) const VERTICES: [[f64;3];398]" in src
assert "pub(super) const QUADS: [([usize;4],u8);380]" in src
assert "pub(super) const PARTS: [(usize,usize);9]" in src
parts=re.findall(r"^\s*\((\d+),\s*(\d+)\),\s*//\s*(.+)$",src,re.M)
assert len(parts)==9
for i,(start,n,name) in enumerate(parts):
    assert int(start)==(i*32 if i<7 else 224+(i-7)*78)
    assert int(n)==(32 if i<7 else 78)
    assert name.startswith("Rime_Bridge_")
render=HERO.read_text(encoding="utf8")
lib=LIB.read_text(encoding="utf8")
assert "const SPELL_SLOT:usize=21;" in render
assert "begin_component(count)" in render
assert "original_mesh::QUADS[first..first+count]" in render
assert "if rime_bridge::render(cast,age,scene) { return; }" in lib
assert lib.index("rime_bridge::render(cast,age,scene)")<lib.index("volumetric_forms::render(cast,age,scene)")
# Independently rerun original mesh source generation to a temporary output.
# It checks two-faces-per-edge AND POSITIVE per-body signed volume.
spec=importlib.util.spec_from_file_location("aetheria_mesh_to_rust",CORE)
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
with tempfile.TemporaryDirectory() as tmp:
    output=Path(tmp)/"replica.rs"
    module.convert(OBJ,output)
    assert output.read_bytes().replace(b"\r\n",b"\n")==RUST.read_bytes().replace(b"\r\n",b"\n")
print("M34 PASS: real Blender 9 meshes, 3 materials, 2 source renders, watertight and all outward")
print("M34 PASS: 398 Rust vertices, 380 complete quads, 9 atomic component ranges")
print("M34 PASS: source digest, reproducibility, live Horizon spell #022 dispatch")
print("M34 NOTE: no GPU FPS, world collision, server spell effect or in-game visual acceptance")
