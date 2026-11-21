from pathlib import Path
root=Path(__file__).resolve().parent
guest=root/"aetheria-horizon"/"preview-5spells-wasm"
def change(file,old,new):
 p=guest/file
 s=p.read_text(encoding="utf8")
 n=s.count(old)
 assert n==1,(file,n,old[:85])
 p.write_text(s.replace(old,new),encoding="utf8",newline="\n")
wg="assets/aetheria_preview/shaders/pipeline/signature_sdf.wgsl"
start="""    }else{
        // THORN DART (#101): sharp asymmetric pointed leaf-wood projectile.
"""
end="""    }
    return vec3<f32>"""
s=(guest/wg).read_text(encoding="utf8")
a=s.index(start);b=s.index(end,a)
replacement="""    }else{
        // THORN DART (#101). M39 *actual GPU-photo driven* revision:
        // precise tapered penetrating projectile with a continuous narrow
        // wood shaft, long needle point, five small reverse barbs, thin
        // sap filaments and hard edge highlights. Not an oversized leaf fan.
        // Source source-matched against real Vulkan RX580 M38 screenshots:
        // M38's wide seven-barb shape produced a huge pale green fan.
        let y=p.y;
        let q=vec2<f32>(p.x-0.025*sin(y*7.2-t*0.48),y);
        let body_clip=smoothstep(-0.97,-0.83,y)*
            (1.0-smoothstep(0.47,0.53,y));
        let stem_width=mix(0.108,0.070,
            smoothstep(-0.9,0.48,y));
        let stem=(1.0-smoothstep(stem_width-0.026,
            stem_width+0.020,abs(q.x)))*body_clip;
        // A tapered triangular spearhead, wide at the shoulder around
        // y=0.47 and literally zero width at the y=0.98 sharp tip.
        let head_y=smoothstep(0.37,0.48,y)*
            (1.0-smoothstep(0.93,0.995,y));
        let head_width=max(0.0,0.23*(0.98-y)/0.52);
        let head=(1.0-smoothstep(max(0.005,head_width-0.027),
            head_width+0.026,abs(q.x)))*head_y;
        var barb=0.0;
        var fine_barb=0.0;
        for(var k=0;k<5;k=k+1){
            let f=f32(k);
            let j=-0.72+f*0.225;
            let sign=select(-1.0,1.0,(k%2)==0);
            let root=vec2<f32>(sign*0.063,j);
            let tip=vec2<f32>(sign*(0.17+0.027*f),
                j-0.14-0.015*sin(t*0.7+f));
            let dist=a37_seg(q,root,tip);
            barb=max(barb,a37_line(dist,0.043));
            fine_barb=max(fine_barb,a37_line(dist,0.016));
        }
        // Delicate center sap and glint. Very little free-floating noise.
        let sap=a37_line(abs(q.x-0.014*sin(y*9.0-t*1.3)),
            0.022+0.008*n)*body_clip;
        let bevel=a37_line(abs(abs(q.x)-stem_width*0.88),
            0.025)*body_clip;
        body=clamp(max(stem,max(head,barb*0.78)),0.0,1.0);
        rim=clamp(max(head*0.87,max(sap*0.90,
            max(bevel*0.58,fine_barb*0.86))),0.0,1.0);
        aura=clamp(0.14*body*n+0.15*sap+0.10*barb*n,
            0.0,0.32);
"""
s=s[:a]+replacement+s[b:]
s=s.replace("// Aetheria M38","// Aetheria M39")
p=guest/wg;p.write_text(s,encoding="utf8",newline="\n")
# Further visual contrast: real M38 shader rendered near-white light green
# (washed-out due full-body emitter). Keep a dark wood core with vivid sap edge.
change(wg,
"""    let glow=tint*(0.67*body+1.15*rim+0.44*haze)+
        vec3<f32>(0.24,0.29,0.34)*(0.34+0.66*n)*inner;""",
"""    var glow=tint*(0.67*body+1.15*rim+0.44*haze)+
        vec3<f32>(0.24,0.29,0.34)*(0.34+0.66*n)*inner;
    if(id==11){
        // Warm dark wooden core, charged bright green blade and veins:
        // readable volume, no screen-wide pale pastel triangle.
        glow=vec3<f32>(0.20,0.31,0.095)*body+
             vec3<f32>(0.39,1.10,0.20)*rim+
             vec3<f32>(0.17,0.52,0.08)*haze;
    }""")
rs="src/signature_shader.rs"
change(rs,"(2.65,5.40,1.45,0.74), // Thorn Dart: barbed wooden projectile",
       "(1.28,3.00,1.25,0.50), // Thorn Dart: compact readably pointed dart")
change(rs,"    let center=add(cast.center,[0.0,lift,0.0]);",
"""    let center=if cast.kind==100 {
        // Projectiles should clearly TRAVEL along the actual aim, not form a
        // fixed, vertical gas cloud at the ground target for five seconds.
        let travel=4.2*smooth((age-10.0)/39.0);
        add(add(cast.center,[0.0,lift,0.0]),
            scale(cast.direction,travel))
    }else {add(cast.center,[0.0,lift,0.0])};""")
change(rs,"    let dissolution=1.0-smooth((age-78.0)/26.0);",
"""    let dissolution=if cast.kind==100 {
        1.0-smooth((age-57.0)/22.0)
    }else {1.0-smooth((age-78.0)/26.0)};""")
for f in ["Cargo.toml","horizon-mod.toml"]:
 change(f,'version = "0.8.6"','version = "0.8.7"')
change("horizon-mod.toml","M38 Name-True Real GPU Thorn Dart QA",
       "M39 Visual-QA Sharp Name-True Thorn Dart")
wf=root/".github/workflows/aetheria-preview-wasm.yml"
s=wf.read_text(encoding="utf8")
assert s.count("assert m['version']=='0.8.6'")==1
s=s.replace("assert m['version']=='0.8.6'","assert m['version']=='0.8.7'")
s=s.replace("M38-INTERNAL-Thorn-Dart","M39-INTERNAL-SHARP-Thorn-Dart")
s=s.replace("cp TEST_M38.md target/m31-gpu-test/README-TEST.md",
            "cp TEST_M39.md target/m31-gpu-test/README-TEST.md")
wf.write_text(s,encoding="utf8",newline="\n")
doc=guest/"TEST_M39.md"
doc.write_text("""# M39 original Horizon GPU visual QA (development only)
2026-11-21 / Horizon Nightly 2026-11-20 / Minecraft 26.3.
M38 actual client screenshot showed an ugly oversized pale-green fan instead of
Thorn Dart. This M39 tries a much narrower wooden/sap point with smaller
reverse barbs, compact world-space hero size, restrained halo and real
flight along cast direction. No AI image is evidence.
Only GitHub pinned Horizon SDK/Naga can certify the source build.
Actual RX580 Vulkan screenshots across charge/flight/impact required.
Other 88/100 signature shaders and server effects still NOT finished.
Existing M32 player install must stay untouched; use isolated --dev-mod.
""",encoding="utf8")
docs=root/"aetheria-horizon"/"docs"/"M39_REAL_VULKAN_REFINEMENT.md"
docs.write_text("""# M39 — Thorn Dart refinement from real M38 GPU photos
2026-11-21. User refuses AI concepts: use actual Horizon screenshots.
Full pinned M38 WASM QA run 44915344977 SUCCESS, 29/29 local source Rust
tests and Github core/Rapier SUCCESS. Actual M38 v0.8.6 shader on Radeon
RX580 Vulkan captured at 1280x720 in an isolated game-dir. Selection verified:
HUD says 'Thorn Dart / WILD 01/20'. Four cast phase frames show a huge
washed-out leaf-like green fan/shadow, despite eliminating old fog. ART FAIL.
QA images in target/m38-runtime-qa not committed until explicitly selected
and reviewed. FPS sampled in report, not a comprehensive benchmark.
M39 tightens taper and barbs, reduces hero world-space size, adds dark
wood core/light green highlights and forward projectile travel. Await
new pinned CI and actual GPU photos BEFORE claiming visual success.
Still not 100 completed spells. Main installed M32 untouched.
""",encoding="utf8")
print("M39 fixed narrow thorn, compact world size, travel animation, contrast and QA metadata")
