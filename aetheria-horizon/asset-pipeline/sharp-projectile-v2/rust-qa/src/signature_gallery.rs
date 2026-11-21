//! Internal QA ONLY: draw EXACT signature_forms::render Scene facets from
//! two real source animation ages. This is a perspective projection of the
//! Rust geometry with source ARGB, not a Horizon GPU/WGSL/game screenshot.
//! DO NOT use these images to claim God Wand graphical parity or client FPS.
#![cfg(test)]
use super::{Cast,Scene,add,sub,scale,cross,norm,length};
use std::{fmt::Write as FmtWrite,fs,path::Path};

type V=[f64;3];
fn dot(a:V,b:V)->f64{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn camera_view(p:V,origin_x:f64)->(f64,f64,f64){
    let camera=[13.0,73.0,19.0];
    let target=[0.0,67.3,0.0];
    let forward=norm(sub(target,camera));
    let right=norm(cross(forward,[0.0,1.0,0.0]));
    let up=norm(cross(right,forward));
    let rel=sub(p,camera);
    let distance=dot(rel,forward).max(3.0);
    let perspective=700.0/distance;
    let x=origin_x+360.0+dot(rel,right)*perspective;
    let y=410.0-dot(rel,up)*perspective;
    (x,y,distance)
}
fn show(signature:usize,name:&str)->String{
    let mut out=String::new();
    write!(&mut out,r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 820" width="1440" height="820">
    <rect width="1440" height="820" fill="#080e1c"/>
    <text x="45" y="43" font-size="32" fill="#f5f0e2"
    font-family="Segoe UI,Arial" font-weight="bold">{}</text>
    <text x="45" y="77" font-size="17" fill="#a0b5d6"
    font-family="Segoe UI,Arial">EXACT M37 RUST SOURCE WORLD QUADS. Flat colour only. NOT GPU or in-game.</text>"##,name).unwrap();
    for (panel,age) in [31.0,64.0].into_iter().enumerate(){
        let x0=panel as f64*720.0;
        write!(&mut out,r##"<rect x="{}" y="102" width="705" height="678"
            fill="#0d192c" stroke="#344c70" rx="9"/>
            <text x="{}" y="137" font-size="19" fill="#a2bddb"
            font-family="Segoe UI,Arial">SPELL {} - T+{} source ticks</text>"##,
            x0+7.0,x0+28.0,signature+1,age as usize).unwrap();
        let cast=Cast{
            kind:signature,variant:0,start:0,
            center:[0.0,65.0,0.0],direction:[0.20,-0.15,0.96]
        };
        let mut scene=Scene::with_lod(900,false);
        assert!(super::signature_forms::render(cast,age,&mut scene));
        let mut faces:Vec<_>=scene.facets.into_iter().filter_map(|(q,color)|{
            let pts=q.map(|p|camera_view(p,x0));
            if !pts.iter().all(|p|p.0.is_finite()&&p.1.is_finite()&&p.2.is_finite()){
                return None
            }
            let depth=pts.iter().map(|p|p.2).sum::<f64>()/4.0;
            Some((depth,pts,color))
        }).collect();
        // Painter's algorithm for 3D depth instead of arbitrary 2D icons.
        faces.sort_by(|a,b|b.0.total_cmp(&a.0));
        for (_,quad,c) in faces {
            let opacity=(((c>>24)&255) as f64/255.0).min(0.97).max(0.12);
            let r=(c>>16)&255;let g=(c>>8)&255;let b=c&255;
            write!(&mut out,r##"<polygon points="{:.2},{:.2} {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}"
                fill="rgb({},{},{})" fill-opacity="{:.3}"
                stroke="#9caccb" stroke-opacity=".08"
                stroke-width=".35"/>"##,
                quad[0].0,quad[0].1,quad[1].0,quad[1].1,
                quad[2].0,quad[2].1,quad[3].0,quad[3].1,r,g,b,opacity).unwrap();
        }
    }
    out.push_str("</svg>");
    out
}
#[test]
fn render_all_eleven_true_hero_source_silhouettes_for_visual_rejection(){
    const ITEMS:[(usize,&str);11]=[
        (0,"Ember Needle"),(6,"Pyre Serpent"),
        (29,"Snowblind"),(47,"Twister Seed"),
        (63,"Faultline"),(86,"Rainweaver"),
        (101,"Root Snare"),(123,"Arc Net"),
        (144,"Rift Slice"),(161,"Moment Slip"),
        (186,"Carrion Swarm"),
    ];
    let out=Path::new(env!("CARGO_MANIFEST_DIR")).join("target/visual-m37");
    fs::create_dir_all(&out).unwrap();
    for (id,name) in ITEMS{
        let svg=show(id,name);
        assert!(svg.len()>10000,"#{id} visual evidence too sparse");
        fs::write(out.join(format!("{:03}_{}.svg",id+1,
            name.to_lowercase().replace(' ','_'))),svg).unwrap();
    }
    assert_eq!(fs::read_dir(&out).unwrap().count(),11);
}
