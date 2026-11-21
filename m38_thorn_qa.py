from pathlib import Path
root=Path(__file__).resolve().parent
base=root/"aetheria-horizon"
guest=base/"preview-5spells-wasm"
def patch(path,old,new,all_matches=False):
    data=path.read_text(encoding="utf-8")
    n=data.count(old)
    assert n>0 if all_matches else n==1,(path,n,old[:70])
    path.write_text(data.replace(old,new),encoding="utf-8",newline="\n")
p=guest/"src/signature_shader.rs"
patch(p,"//! M37.2 true procedural PIXEL SHAPES for eleven specifically named spells.",
        "//! M38 true procedural PIXEL SHAPES for twelve specifically named spells.")
patch(p,"const SIGNATURES:[usize;11]=[0,6,29,47,63,86,101,123,144,161,186];",
        "const SIGNATURES:[usize;12]=[0,6,29,47,63,86,100,101,123,144,161,186];")
# Preserve existing index mapping by appending 100 at end: only one correct list.
p.write_text(p.read_text(encoding="utf-8").replace(
 "const SIGNATURES:[usize;12]=[0,6,29,47,63,86,100,101,123,144,161,186];",
 "const SIGNATURES:[usize;12]=[0,6,29,47,63,86,101,123,144,161,186,100];"
 ),encoding="utf-8",newline="\n")
patch(p,"const BASE_RGB:[u32;11]=[","const BASE_RGB:[u32;12]=[")
patch(p,"    0x00B588EB, // Carrion Swarm\n];",
        "    0x00B588EB, // Carrion Swarm\n    0x008EE86B, // Thorn Dart #101\n];")
patch(p,"    let shapes:[(f64,f64,f64,f64);11]=[",
        "    let shapes:[(f64,f64,f64,f64);12]=[")
patch(p,"        (6.4,5.8,2.0,0.08),  // ghost skull-bird swarm\n    ];",
        "        (6.4,5.8,2.0,0.08),  // ghost skull-bird swarm\n"
        "        (2.65,5.40,1.45,0.74), // Thorn Dart: barbed wooden projectile\n    ];")
patch(p,"fn exactly_eleven_name_bound_shader_material_ids_no_generic_colour_clones()",
        "fn exactly_twelve_name_bound_shader_material_ids_no_generic_colour_clones()")
patch(p,"assert_eq!(SIGNATURES.len(),11);","assert_eq!(SIGNATURES.len(),12);")
patch(p,"assert_eq!(tags.len(),11);","assert_eq!(tags.len(),12);")
patch(p,".len(),11);",".len(),12);")
patch(p,"*m<=233","*m<=254")
patch(p,"fn all_eleven_actual_proxies_camera_facing_bounded_and_animated()",
        "fn all_twelve_actual_proxies_camera_facing_bounded_and_animated()")
p=guest/"assets/aetheria_preview/shaders/pipeline/signature_sdf.wgsl"
patch(p,"// Aetheria M37.2","// Aetheria M38")
patch(p,"// input.color.a encodes one of 11 named materials",
        "// input.color.a encodes one of 12 named materials")
patch(p,"    }else{\n        // CARRION SWARM","    }else if(which==10){\n        // CARRION SWARM")
patch(p,"        aura=0.33*smoke*(0.5+0.5*n);\n    }\n    return vec3<f32>",
"""        aura=0.33*smoke*(0.5+0.5*n);

    }else{
        // THORN DART (#101): sharp asymmetric pointed leaf-wood projectile.
        // Seven hard barbs, living veins, leading tip; never green smoke.
        let q=vec2<f32>(p.x-0.12*sin((p.y+0.5)*3.9-t*0.22),p.y);
        let range=smoothstep(-0.97,-0.79,q.y)*
            (1.0-smoothstep(0.87,1.01,q.y));
        let taper=0.19-0.12*smoothstep(-0.61,0.88,q.y);
        let shaft=(1.0-smoothstep(taper*0.60,taper+0.026,abs(q.x)))*range;
        let head=(1.0-smoothstep(0.28,0.37,
            abs(q.x)+max(0.0,0.49-q.y)*0.72))*
            smoothstep(0.38,0.56,q.y)*
            (1.0-smoothstep(0.86,1.0,q.y));
        var thorns=0.0;
        var veins=0.0;
        for(var k=0;k<7;k=k+1){
            let f=f32(k);
            let y=-0.74+f*0.208;
            let side=select(-1.0,1.0,(k%2)==0);
            let origin=vec2<f32>(0.035*side+0.06*sin(y*3.7),y);
            let tip=vec2<f32>(side*(0.35+0.08*sin(f*2.1)),
                y-0.26-0.05*sin(t*1.1+f));
            let d=a37_seg(q,origin,tip);
            thorns=max(thorns,a37_line(d,0.068-0.004*f));
            veins=max(veins,a37_line(d,0.028)*0.78);
        }
        let x=q.x-0.11*sin(q.y*7.2-t*1.4);
        let sap=a37_line(abs(x),0.060+0.024*n)*range;
        body=clamp(max(shaft,max(head,thorns*0.83)),0.0,1.0);
        rim=clamp(max(head*0.94,max(veins,sap*0.85)),0.0,1.0);
        aura=0.21*body*n+
            0.33*sap*(1.0-smoothstep(0.32,0.95,abs(q.y)));
    }
    return vec3<f32>""")
patch(p,")),0,10);",")),0,11);")
p=guest/"src/shader_validation.rs"
patch(p,"for id in 0..=10 {","for id in 0..=11 {")
patch(p,'|| id==10 && actual.contains("CARRION SWARM"));',
        '|| id==10 && actual.contains("CARRION SWARM")\n'
        '                || id==11 && actual.contains("THORN DART"));')
patch(p,'assert!(actual.contains("a37_edge(p)"));',
        'assert!(actual.contains("a37_edge(p)"));\n'
        '        assert!(actual.contains("clamp(i32(round((input.color.a*255.0-23.0)/21.0)),0,11)"));')
p=guest/"src/signature_forms.rs"
needle="/// The true source geometry replaces (does not merely recolor) generic\n"
fallback="""// Name-specific contiguous 3D fallback (only if GPU SDF is absent).
fn thorn_dart(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);
    if grow<0.03||fade<0.03{return;}
    let mut body=Vec::new();
    for i in 0..18{
        let t=i as f64/17.0;
        let bend=0.09*(t*TAU*1.5-age*0.026).sin();
        body.push(local(c,bend,1.0+0.28*t,0.6+5.2*t+age*0.024));
    }
    connected_body(scene,&body,0.28*grow*fade,WOOD,8);
    shard(scene,body[16],add(body[17],[0.0,0.12,0.70]),
        0.32*fade,SAP);
    for j in 0..7{
        let k=2+j*2;
        let sign=if j%2==0 {1.0}else{-1.0};
        let base=body[k];
        let tip=add(base,[sign*(0.74+0.10*(j as f64*0.8).sin())*grow,
            0.19,-0.28]);
        shard(scene,base,tip,0.16*fade,mutate(SAP,j));
        scene.membrane(base,tip,
            add(tip,[sign*0.13,0.13,0.13]),
            add(base,[sign*0.06,0.21,-0.14]),mutate(LEAF,j));
    }
}
"""
patch(p,needle,fallback+needle)
patch(p,"id,0|6|29|47|63|86|101|123|144|161|186",
        "id,0|6|29|47|63|86|100|101|123|144|161|186")
patch(p,"        101=>roots(c,age,scene),",
        "        100=>thorn_dart(c,age,scene),\n        101=>roots(c,age,scene),")
patch(p,"[0,6,29,47,63,86,101,123,144,161,186]",
        "[0,6,29,47,63,86,100,101,123,144,161,186]",True)
for p in (guest/"Cargo.toml",guest/"horizon-mod.toml"):
    patch(p,'version = "0.8.5"','version = "0.8.6"')
p=guest/"horizon-mod.toml"
patch(p,"M37 Name-True GPU Signature QA","M38 Name-True Real GPU Thorn Dart QA")
patch(p,"11 named signed-distance GPU signature silhouettes",
        "12 named signed-distance GPU signature silhouettes")
p=guest/"src/lib.rs"
patch(p,"generic solid forms for eleven high-priority signature spells.",
        "generic solid forms for twelve high-priority signature spells.")
p=root/".github/workflows/aetheria-preview-wasm.yml"
patch(p,"assert m['version']=='0.8.5'","assert m['version']=='0.8.6'")
# Preserve the original M37 CI/source tests; the new branch inherits them.
p=base/"docs/M38_ACTUAL_GAME_SCREENSHOT_QA.md"
p.write_text("""# M38 real-game visual QA
On 2026-11-21 the user's PC was connected with explicit permission.
Installed official Horizon Nightly 2026-11-20; existing user mod was M32
v0.8.0, not M37. M37 .hmod 0.8.5 was downloaded from a SUCCESS GitHub
Actions artifact, SHA256:
5f364d39a3c270f8ffc88418899bb989437b3d1171b4c8d884c18fdf7d3026cc
It was tested via horizon-ctl --dev-mod in a fully separate QA
game-dir and QA world, without overwriting the user's real M32 install.
Real Vulkan Radeon RX580 GPU game screenshots were collected:
preview-5spells-wasm/target/m37-runtime-qa/baseline-m37.png
and M37-Ember-t0200/t0700/t1200/t2300.png. Filenames incorrectly say
Ember: user's active selected spell was WILD Thorn Dart (index 100).
The real images showed a pale green diffuse smoke pillar, vague flat
transparent surfaces. ART FAIL. User was actively giving game inputs,
so avoid competing with them over the camera/keyboard. User later exited.
The 75fps figure was observed while loaded in forest and is NOT a
controlled under-spell benchmark.

M38 response: dedicated sharp barbed projectile SDF material for
Thorn Dart, now 12th hero signature, plus whole original 3D
tapered stem with alternating side spikes for safe shader failure.
No unmasked proxy quads allowed. Fresh pinned Naga/WASM CI required,
followed by ACTUAL new in-game screenshot before any quality approval.
The remaining 88 of the first 100 do not yet have unique GPU hero SDFs.
Not 100 production-ready spells, no server-authoritative damage/physics.
Never substitute AI-generated concept art for gameplay screenshots.
""",encoding="utf-8",newline="\n")
print("M38 PATCH OK: 12 GPU SDF signatures, Thorn Dart shader+3D fallback")
