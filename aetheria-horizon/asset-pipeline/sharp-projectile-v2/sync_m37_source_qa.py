#!/usr/bin/env python3
"""Mirror the EXACT Horizon Scene helper methods into standalone Rust QA.
Never maintain a fake manually copied geometric implementation.
"""
from pathlib import Path
import re
root=Path(__file__).resolve().parents[2]
original=(root/"preview-5spells-wasm/src/lib.rs").read_text(encoding="utf8")
qa_path=root/"asset-pipeline/sharp-projectile-v2/rust-qa/src/lib.rs"
qa=qa_path.read_text(encoding="utf8")
def source_method(name):
    m=re.search(r"(?m)^    fn "+re.escape(name)+r"\(",original)
    assert m,f"Scene::{name} missing"
    opening=original.find("{",m.start())
    depth=0
    for i in range(opening,len(original)):
        depth+=1 if original[i]=="{" else (-1 if original[i]=="}" else 0)
        if depth==0:
            return original[m.start():i+1]
    raise ValueError(f"Unclosed {name}")
names=("membrane","tube","solid_crystal")
methods="\n".join(source_method(name) for name in names)
begin="// BEGIN M37 EXACT HORIZON SCENE HELPERS"
end="// END M37 EXACT HORIZON SCENE HELPERS"
insertion=(f"{begin}\nimpl Scene {{\n{methods}\n}}\n"
    '#[path="../../../../preview-5spells-wasm/src/signature_forms.rs"]\n'
    'mod signature_forms;\n'
    '#[path="../../../../preview-5spells-wasm/src/signature_shader.rs"]\n'
    f"mod signature_shader;\n#[cfg(test)]\nmod signature_gallery;\n{end}")
if begin in qa:
    before,after=qa.split(begin,1)
    _,after=after.split(end,1)
    qa=before+insertion+after
else:
    marker="#[cfg(test)]\nmod scene_tests {"
    assert qa.count(marker)==1
    qa=qa.replace(marker,insertion+"\n"+marker)
qa_path.write_text(qa,encoding="utf8",newline="\n")
for name in names:
    assert source_method(name) in qa
print("PASS M37 exact Scene helpers + signature_forms source included in Rust QA")
