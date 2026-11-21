//! Exact M33 hero renderer compiled and unit-tested without downloading the
//! entire Horizon SDK. Mirrors only the original Scene/Cast ABI used by this
//! artist asset. NOT a substitute for the real pinned Horizon .hmod build.
#![allow(dead_code)]
use std::f64::consts::TAU;

#[derive(Clone,Copy)]
struct Cast {
    kind:usize,variant:u8,start:u64,
    center:[f64;3], direction:[f64;3],
}
struct Scene {
    facets:Vec<([[f64;3];4],u32)>,
    group_ids:Vec<usize>,
    active_group:Option<usize>,
    next_group:usize,
    budget:usize,
    reduced_detail:bool,
}
impl Scene {
    fn new(budget:usize)->Self {Self::with_lod(budget,false)}
    fn with_lod(budget:usize,reduced_detail:bool)->Self {
        Self{facets:Vec::new(),group_ids:Vec::new(),active_group:None,
            next_group:0,budget,reduced_detail}
    }
    fn begin_component(&mut self,faces:usize)->Option<Option<usize>>{
        if self.facets.len().saturating_add(faces)>self.budget{return None;}
        let prev=self.active_group;
        if prev.is_none(){self.active_group=Some(self.next_group);self.next_group+=1;}
        Some(prev)
    }
    fn end_component(&mut self,previous:Option<usize>){self.active_group=previous;}
    fn quad(&mut self,points:[[f64;3];4],color:u32){
        if self.facets.len()<self.budget && points.iter().flatten().all(|v|v.is_finite()){
            let id=match self.active_group{
                Some(id)=>id,
                None=>{let id=self.next_group;self.next_group+=1;id}
            };
            self.facets.push((points,color));self.group_ids.push(id);
        }
    }
}
fn add(a:[f64;3],b:[f64;3])->[f64;3]{[a[0]+b[0],a[1]+b[1],a[2]+b[2]]}
fn sub(a:[f64;3],b:[f64;3])->[f64;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn scale(a:[f64;3],s:f64)->[f64;3]{[a[0]*s,a[1]*s,a[2]*s]}
fn length(a:[f64;3])->f64{(a[0]*a[0]+a[1]*a[1]+a[2]*a[2]).sqrt()}
fn norm(a:[f64;3])->[f64;3]{scale(a,1.0/length(a).max(0.000001))}
fn cross(a:[f64;3],b:[f64;3])->[f64;3]{
    [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
}
fn shade_argb(argb:u32,brightness:f64)->u32 {
    let b=brightness.clamp(0.0,1.0);
    let mul=|s:u32|->u32{(((argb>>s)&255) as f64*b).round() as u32};
    (argb&0xFF00_0000)|(mul(16)<<16)|(mul(8)<<8)|mul(0)
}
#[path="../../../../preview-5spells-wasm/src/sharp_projectile.rs"]
mod sharp_projectile;
#[path="../../../../preview-5spells-wasm/src/rime_bridge.rs"]
mod rime_bridge;

// Mimic the three immutable catalog index functions ONLY. This fast,
// dependency-free harness compiles and executes the EXACT production M35
// emitter source; full Horizon SDK ABI still gets separate CI coverage.
mod catalog200 {
    pub const SPELL_COUNT:usize=200;
    pub fn school(slot:usize)->usize{slot.min(SPELL_COUNT-1)/20}
    pub fn rank(slot:usize)->usize{slot.min(SPELL_COUNT-1)%20}
}
#[path="../../../../preview-5spells-wasm/src/procedural_fx.rs"]
mod procedural_fx;
#[path="../../../../preview-5spells-wasm/src/authored_100_data.rs"]
mod authored_100_data;
#[path="../../../../preview-5spells-wasm/src/authored_100_fx.rs"]
mod authored_100_fx;
#[cfg(test)]
mod scene_tests {
    use super::*;
    #[test]
    fn m33_single_spell_global_hard_budget_900(){
        let cast=Cast{
            kind:20,variant:0,start:0,
            center:[0.0,64.0,0.0],direction:[0.0,0.0,1.0],
        };
        for reduced in [true,false]{
            for age in 0..104{
                let mut s=Scene::with_lod(900,reduced);
                assert!(sharp_projectile::render(cast,age as f64,&mut s));
                assert!(s.facets.len()<=900);
                assert_eq!(s.group_ids.len(),s.facets.len());
                assert_eq!(s.facets.len()%124,0);
                assert!(s.facets.iter().all(|(q,c)|{
                    *c!=0 && q.iter().flatten().all(|v|v.is_finite())
                }));
            }
        }
    }
    #[test]
    fn m33_no_render_other_slots_or_bad_direction(){
        let mut s=Scene::new(900);
        assert!(!sharp_projectile::render(Cast{
            kind:21,variant:0,start:0,
            center:[0.,0.,0.],direction:[0.,0.,1.]
        },20.0,&mut s));
        assert!(s.facets.is_empty());
    }
}


/// Actual M36 Rust SOURCE geometry projections for visual review.
/// These are NOT GPU/WGSL outputs, not game screenshots, and not art approval.
/// Each displayed point is derived from the exact authored_100_fx::render
/// source quads, at 2 different ages with varied geometry for 100 spells.
#[cfg(test)]
mod m36_gallery {
    use super::*;
    use std::{fs,fmt::Write as FmtWrite};
    const SCHOOLS:[&str;10]=[
        "PYROMANCY","CRYOMANCY","AEROMANCY","GEOMANCY","HYDROMANCY",
        "DRUID","ELECTROMANCY","ASTRAL","CHRONOMANCY","NECROMANCY",
    ];
    const PALETTE:[&str;10]=[
        "#ff9a54","#83eaff","#bdffe0","#e0bd8f","#62c9ff",
        "#91fbb0","#c2d1ff","#dbb2ff","#ffe5a8","#dfbee9",
    ];
    fn paint_spells(school:usize)->String{
        let width=2100;
        let height=780;
        let mut s=String::new();
        write!(&mut s,r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
        <defs><radialGradient id="glow"><stop stop-color="#ffffff" stop-opacity=".86"/>
        <stop offset=".33" stop-color="{tone}" stop-opacity=".55"/>
        <stop offset="1" stop-color="{tone}" stop-opacity="0"/></radialGradient>
        <linearGradient id="bg" x1="0" x2="1" y1="0" y2="1">
        <stop stop-color="#070e20"/><stop offset=".55" stop-color="#111b34"/>
        <stop offset="1" stop-color="#070b13"/></linearGradient></defs>
        <rect width="2100" height="780" fill="url(#bg)"/>
        <text x="42" y="55" fill="{tone}" font-family="Segoe UI,Arial" font-size="30"
         font-weight="bold">AETHERIA / {title} / 10 SOURCE-SCULPTED SPELLS</text>
        <text x="42" y="83" fill="#9baecb" font-family="Segoe UI,Arial" font-size="17">
        Real Rust source vertices at two times, with an SVG material-mask approximation.
        NOT a Horizon GPU screenshot. NOT 100 finished spells.</text>"##,
        tone=PALETTE[school],title=SCHOOLS[school]).unwrap();
        for index in 0..10 {
            let slot=school*20+index;
            let spec=authored_100_data::author_for(slot).unwrap();
            let col=index%5;let row=index/5;
            let x=42+col*406;let y=112+row*324;
            write!(&mut s,r##"<rect x="{x}" y="{y}" width="386" height="308" rx="14"
                fill="#0c1830" fill-opacity=".87" stroke="#33435c"/>
                <text x="{tx}" y="{ty}" fill="#e2ecff" font-size="19"
                font-family="Segoe UI,Arial" font-weight="bold">{:03} {}</text>
                <text x="{tx}" y="{sy}" fill="{tone}" font-size="13"
                font-family="Segoe UI,Arial">{:?}</text>"##,
                spec.id,spec.name,spec.pattern,
                tx=x+13,ty=y+29,sy=y+49,tone=PALETTE[school]).unwrap();
            for (k,age) in [31.0,62.0].into_iter().enumerate(){
                let pane_x=x as i32+14+k as i32*188;
                let pane_y=y as i32+63;
                write!(&mut s,r##"<rect x="{pane_x}" y="{pane_y}"
                     width="176" height="216" rx="7" fill="#080d19"/>
                     <text x="{label_x}" y="{label_y}" fill="#a8bfd1"
                     font-size="12" font-family="Segoe UI,Arial">T+{}</text>"##,
                     age as i32,label_x=pane_x+9,label_y=pane_y+17).unwrap();
                let cast=Cast{kind:slot,variant:index as u8,start:0,
                    center:[0.0,65.0,0.0],direction:[0.13,-0.20,0.96]};
                let mut scene=Scene::new(100);
                authored_100_fx::render(cast,age,[9.,72.,12.],&mut scene);
                let cx=(pane_x+88) as f64;
                let cy=(pane_y+125) as f64;
                for (corners,color) in scene.facets {
                    let p=corners.iter().copied().fold([0.,0.,0.],add);
                    let center=scale(p,0.25);
                    let u=cx+center[0]*23.0+center[2]*3.8;
                    let v=cy-(center[1]-65.0)*25.0+center[2]*3.6;
                    if u<pane_x as f64+4.0||u>(pane_x+172) as f64
                       ||v<(pane_y+18) as f64||v>(pane_y+210) as f64 {
                        continue;
                    }
                    let rad=(length(sub(corners[0],center))*27.0).clamp(2.0,14.0);
                    // Display only the material-silhouette category that
                    // the actual GPU shader selects. SVG styles approximate
                    // (never claim a pixel-perfect Horizon GPU capture).
                    let tag=(color>>24) as u8;
                    let tone=PALETTE[school];
                    match tag {
                        139=>{
                            write!(&mut s,r##"<polygon points="{:.2},{:.2} {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}"
                                fill="url(#glow)" opacity=".82"/>"##,
                                u,v-rad*1.2,u+rad*0.65,v,u,v+rad*1.2,u-rad*0.60,v).unwrap();
                        }
                        154=>{
                            write!(&mut s,r##"<ellipse cx="{u:.2}" cy="{v:.2}" rx="{:.2}"
                                ry="{:.2}" transform="rotate(32 {u:.2} {v:.2})"
                                fill="url(#glow)" opacity=".72"/>"##,
                                rad*0.62,rad*1.35).unwrap();
                        }
                        169=>{
                            write!(&mut s,r##"<polyline points="{:.2},{:.2} {:.2},{:.2}
                                {:.2},{:.2} {:.2},{:.2}" fill="none"
                                stroke="{tone}" stroke-width="{:.2}" stroke-linejoin="round"
                                opacity=".71"/>"##,
                                u-rad*0.45,v+rad,u+rad*0.25,v+rad*0.16,
                                u-rad*0.10,v-rad*0.12,u+rad*0.4,v-rad,rad*0.30).unwrap();
                        }
                        184=>{
                            write!(&mut s,r##"<path d="M{:.2},{:.2} Q{:.2},{:.2} {:.2},{:.2}"
                                fill="none" stroke="{tone}" stroke-width="{:.2}"
                                opacity=".64" stroke-linecap="round"/>"##,
                                u-rad,v+rad,u+rad*0.66,v,u+rad,v-rad,rad*0.52).unwrap();
                        }
                        199=>{
                            write!(&mut s,r##"<polygon points="{:.2},{:.2} {:.2},{:.2}
                                {:.2},{:.2} {:.2},{:.2}" fill="none"
                                stroke="{tone}" stroke-width="{:.2}" opacity=".67"/>"##,
                                u,v-rad,u+rad,v,u,v+rad,u-rad,v,rad*0.25).unwrap();
                        }
                        _=>{
                            write!(&mut s,r##"<circle cx="{u:.2}" cy="{v:.2}" r="{rad:.2}"
                                fill="url(#glow)" opacity=".74"/>"##).unwrap();
                        }
                    }
                }
            }
        }
        s.push_str("</svg>");
        s
    }
    #[test]
    fn generate_genuine_rust_source_geometry_gallery_for_one_hundred_spells(){
        let out=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/visual-m36");
        fs::create_dir_all(&out).unwrap();
        for school in 0..10 {
            let src=paint_spells(school);
            assert!(src.len()>12_000,"missing rendered source geometry");
            fs::write(out.join(format!("{:02}_{}_10_spells.svg",
                school,SCHOOLS[school].to_lowercase())),src).unwrap();
        }
    }
}
