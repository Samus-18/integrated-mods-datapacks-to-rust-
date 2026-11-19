//! Deterministic off-screen projection of the EXACT geometry submitted by
//! Aetheria's client preview. For visual regression review in CI only.
//! NOT a Horizon screenshot, not a GPU shader test, not actual HDR/bloom.
//! No external libraries: self-contained orthographic/painter's-order SVG.
use super::{Cast, Scene, render_spell, add, sub, scale, norm, cross};
use crate::catalog30;
use aetheria_horizon::vfx::timeline::Stage;
use std::{fs,fmt::Write as _,path::Path};

#[derive(Clone,Copy)]
struct View{right:[f64;3],up:[f64;3],forward:[f64;3]}
impl View {
    fn new(eye:[f64;3])->Self{
        let forward=norm(eye);
        let right=norm(cross([0.,1.,0.],forward));
        let up=norm(cross(forward,right));
        Self{right,up,forward}
    }
    fn project(&self,p:[f64;3])->[f64;3]{
        let rel=sub(p,[0.,64.,0.]);
        [
            rel.iter().zip(self.right).map(|(a,b)|a*b).sum(),
            rel.iter().zip(self.up).map(|(a,b)|a*b).sum(),
            rel.iter().zip(self.forward).map(|(a,b)|a*b).sum(),
        ]
    }
}
fn times(slot:usize)->[f64;3]{
    // A real sample from the SAME 104-tick phase scheduler as the WASM mod,
    // not arbitrary absolute ages that might miss a very short Impact stage.
    let stages=[Stage::Cast,Stage::Impact,Stage::Afterglow];
    stages.map(|stage|{
        (0..1040).map(|i|i as f64/10.0)
            .find(|age|matches!(
                super::cinematic_beams::visual_phase(slot,*age),
                Some((s,p)) if s==stage && p>=0.52
            ))
            .expect("each distinct stage must have a sampled frame")
    })
}
fn xml(s:&str)->String{
    s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;")
      .replace('"',"&quot;").replace('\'',"&apos;")
}
fn shape_area(pts:&[[f64;2];4])->f64 {
    let mut double=0.0;
    for i in 0..4 {
        let j=(i+1)%4;
        double+=pts[i][0]*pts[j][1]-pts[j][0]*pts[i][1];
    }
    0.5*double.abs()
}
struct Face { screen:[[f64;2];4],color:u32,depth:f64,area:f64 }
struct Report {
    polygons:usize,span:f64,coverage:f64,degenerate:f64,range_y:f64
}
fn export_svg(scene:&Scene,view:View,slot:usize,stage:Stage,age:f64,path:&Path)->Report{
    let raw=scene.facets.iter().flat_map(|(corners,_)|
        corners.iter().map(|c|view.project(*c))
    ).collect::<Vec<_>>();
    assert!(!raw.is_empty());
    assert!(raw.iter().flatten().all(|v|v.is_finite()));
    let mut low=[f64::INFINITY;2];
    let mut high=[f64::NEG_INFINITY;2];
    let mut yrange=[f64::INFINITY,f64::NEG_INFINITY];
    for (q,_) in &scene.facets {
        for vertex in q {
            yrange[0]=yrange[0].min(vertex[1]-64.);
            yrange[1]=yrange[1].max(vertex[1]-64.);
        }
    }
    for c in &raw{
        for d in 0..2 {low[d]=low[d].min(c[d]);high[d]=high[d].max(c[d]);}
    }
    let span=(high[0]-low[0]).max(high[1]-low[1]);
    assert!(span>0.1 && span<80.,"unreadable or runaway geometry: {span}");
    // Display scale is stored explicitly: this does not hide the absolute
    // dimensions in blocks, which are also exported in audit.tsv.
    let fit=250./span.max(1.0);
    let shift=[(low[0]+high[0])*0.5,(low[1]+high[1])*0.5];
    let to_screen=|v:[f64;3]| ->[f64;2]{
        [210.+(v[0]-shift[0])*fit,187.-(v[1]-shift[1])*fit]
    };
    let mut faces=Vec::with_capacity(scene.facets.len());
    let mut area_sum=0.;
    let mut zero=0;
    for (corners,c) in &scene.facets{
        let pp=corners.map(|p|view.project(p));
        let xy=pp.map(to_screen);
        let area=shape_area(&xy);
        if area<0.6 {zero+=1;}
        area_sum+=area;
        let depth=pp.iter().map(|x|x[2]).sum::<f64>()/4.0;
        faces.push(Face{screen:xy,color:*c,depth,area});
    }
    faces.sort_by(|a,b|b.depth.total_cmp(&a.depth));
    let mut svg=String::with_capacity(scene.facets.len()*130+3000);
    let (display_id,name)=if slot<30 {
        (catalog30::id(slot),catalog30::spec(slot).name)
    }else{
        let global=slot-30;
        (crate::catalog200::id(global),crate::catalog200::spec(global).name)
    };
    write!(svg,"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420\" height=\"370\" viewBox=\"0 0 420 370\">\n").unwrap();
    svg.push_str("<rect width=\"420\" height=\"370\" fill=\"#101926\"/>\n");
    svg.push_str("<rect x=\"10\" y=\"10\" width=\"400\" height=\"350\" rx=\"11\" fill=\"#142333\" stroke=\"#304556\"/>\n");
    for f in faces {
        if f.area<0.20 {continue;}
        let r=(f.color>>16)&255;
        let g=(f.color>>8)&255;
        let b=f.color&255;
        let a=(f.color>>24)&255;
        if a==0{continue;}
        // Intentionally accurate alpha, but unlike Horizon this static SVG
        // cannot emulate GPU depth testing or the optional material shader.
        write!(svg,"<polygon points=\"").unwrap();
        for v in f.screen {
            write!(svg,"{:.2},{:.2} ",v[0],v[1]).unwrap();
        }
        write!(svg,"\" fill=\"#{r:02x}{g:02x}{b:02x}\" fill-opacity=\"{:.3}\" />\n",
               (a as f64/255.).min(0.86)).unwrap();
    }
    write!(svg,"<rect x=\"11\" y=\"11\" width=\"398\" height=\"41\" fill=\"#101926\" fill-opacity=\".90\"/>\n").unwrap();
    write!(svg,"<text x=\"22\" y=\"30\" fill=\"#e7f4f6\" font-family=\"sans-serif\" font-size=\"14\" font-weight=\"bold\">{:03}  {}</text>\n",display_id,xml(name)).unwrap();
    write!(svg,"<text x=\"22\" y=\"46\" fill=\"#94c0ce\" font-family=\"monospace\" font-size=\"10\">{:?} @ {:04.1} ticks  |  {:.1} blocks  |  {} surfaces</text>\n",
        stage,age,span,scene.facets.len()).unwrap();
    svg.push_str("<text x=\"22\" y=\"346\" fill=\"#668899\" font-family=\"sans-serif\" font-size=\"10\">OFFSCREEN CPU GEOMETRY PROJECTION, NOT IN-GAME SCREENSHOT</text>\n</svg>\n");
    fs::write(path,svg).expect("write visual SVG");
    Report{
        polygons:scene.facets.len(),span,
        coverage:area_sum/(400.*310.),
        degenerate:zero as f64/scene.facets.len() as f64,
        range_y:yrange[1]-yrange[0]
    }
}
#[test]
fn visual_audit_silhouette_bounds_and_optional_svg_contact_sheets(){
    let export=std::env::var_os("AETHERIA_VISUAL_AUDIT").is_some();
    let dir=Path::new("target/visual-audit");
    if export {fs::create_dir_all(dir).expect("make visual audit directory");}
    let views=[
        View::new([1.8,1.25,2.8]), // 3/4 front
        View::new([-2.3,1.75,1.1]), // back/top, reveals flat billboards
    ];
    let mut tab=String::from("slot\tspell\tstage\tview\tquads\tspan_blocks\tprojected_area_ratio\tdegenerate_fraction\ty_height_blocks\n");
    let mut html=String::from("<!doctype html><meta charset='utf-8'><title>Aetheria 30 spell visual audit</title><style>body{background:#0c131f;color:#d2e3ec;font:14px sans-serif}h1{margin:30px}section{display:grid;grid-template-columns:repeat(6,minmax(0,1fr));gap:8px}img{width:100%;border-radius:6px}p{margin:12px 30px}</style><h1>Aetheria — CPU 3D visual audit</h1><p>NOT screenshots from Horizon; approximated alpha, no world blocks, GPU depth or WGSL bloom. Each spell shown at Cast, Impact and Afterglow, from two diagonal camera angles. Fixed geometry, autoscaled display with absolute span reported.</p>");
    let mut total=0usize;
    for slot in 0..30 {
        let names=xml(catalog30::spec(slot).name);
        if export {write!(html,"<h2>#{:02}: {}</h2><section>",slot+1,names).unwrap();}
        for (stage_idx,age) in times(slot).into_iter().enumerate() {
            let stage=[Stage::Cast,Stage::Impact,Stage::Afterglow][stage_idx];
            let mut scene=Scene::new(900);
            super::render_spell(Cast{
                kind:(catalog30::id(slot)-1) as usize,
                variant:catalog30::rank(slot) as u8,
                start:0,center:[0.,64.,0.],direction:[0.,0.,1.]
            },age,&mut scene);
            assert!(!scene.facets.is_empty(),"empty spell id {} stage {:?}",slot,stage);
            assert!(scene.facets.len()<=900);
            for (view_idx,view) in views.iter().copied().enumerate(){
                let file=format!("spell_{:02}_{:02}_view{}.svg",slot+1,stage_idx+1,view_idx+1);
                // In non-export runs, we still calculate projection metrics,
                // using a temporary system directory to avoid dependencies.
                let dest=if export{dir.join(&file)}
                    else{std::env::temp_dir().join(format!("aetheria_test_{}",file))};
                let report=export_svg(&scene,view,slot,stage,age,&dest);
                if !export {let _=fs::remove_file(&dest);}
                assert!(report.degenerate<0.93,
                    "almost all faces edge-on in a camera view: spell {} view {}",slot,view_idx+1);
                assert!(report.coverage>0.0005,"almost invisible spell {} view {}",slot,view_idx+1);
                assert!(report.span<=80.0);
                total+=1;
                if export {
                    write!(tab,"{}\t{}\t{:?}\t{}\t{}\t{:.2}\t{:.5}\t{:.4}\t{:.2}\n",
                        slot+1,names,stage,view_idx+1,report.polygons,report.span,
                        report.coverage,report.degenerate,report.range_y).unwrap();
                    write!(html,"<img src='{}' alt='{} {:?} view {}'>",file,names,stage,view_idx+1).unwrap();
                }
            }
        }
        if export {html.push_str("</section>\n");}
    }
    assert_eq!(total,180);
    if export{
        html.push_str("<p>Mechanics, material shader, real lighting and the Horizon client must be validated separately. Generated by a pure Rust geometry test.</p>");
        fs::write(dir.join("index.html"),html).unwrap();
        fs::write(dir.join("geometry_report.tsv"),tab).unwrap();
        println!("AETHERIA_VISUAL_AUDIT: wrote 180 visual SVGs, geometry_report.tsv and index.html");
    }
}

#[test]
fn extended_170_spells_have_two_angle_cast_and_impact_visual_audit(){
    use aetheria_horizon::vfx::timeline::{for_spell,Stage};
    use crate::catalog200;
    let export=std::env::var_os("AETHERIA_VISUAL_AUDIT").is_some();
    let dir=Path::new("target/visual-audit/extended");
    if export {fs::create_dir_all(dir).expect("create extended audit");}
    let views=[View::new([1.8,1.25,2.8]),
               View::new([-2.3,1.75,1.1])];
    let mut gallery=String::from("<!doctype html><meta charset='utf-8'><title>Aetheria 170 extended-form art review</title><style>body{background:#0c1420;color:#d2e3ec;font:14px sans-serif}section{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:8px}img{width:100%}h2{margin:28px 10px 5px}</style><h1>170 first-draft geometry spell silhouettes</h1><p>NOT approved artwork, NOT a Horizon game screenshot. The goal is to identify generic or weak silhouettes for manual redesign.</p>");
    let mut tab=String::from("id\tname\tstage\tview\tquads\tspan_blocks\tprojected_coverage\tdegenerate_fraction\n");
    let mut checked=0;
    for global in 0..200{
        if catalog200::curated_slot(global).is_some(){continue;}
        let spec=catalog200::spec(global);
        let t=for_spell(spec.id).expect("200 timing contracts");
        let fullname=xml(spec.name);
        if export{write!(gallery,"<h2>{:03} {} — {:?}</h2><section>",
                         spec.id,fullname,super::mass_forms::semantic_form(spec.mesh)).unwrap();}
        for stage in [Stage::Cast,Stage::Impact]{
            let age=(0..1040).map(|x|x as f64/10.0).find(|age|{
                let current=*age/104.0*t.total_ticks() as f64;
                let mut elapsed=0.0;
                for p in t.phases {
                    let limit=elapsed+p.duration_ticks as f64;
                    if current<limit{
                        return p.kind==stage && (current-elapsed)/p.duration_ticks as f64>=0.55;
                    }
                    elapsed=limit;
                }
                false
            }).expect("expected stage midpoint");
            let mut scene=Scene::new(500);
            render_spell(Cast{kind:global,variant:catalog200::rank(global) as u8,
                start:0,center:[0.,64.,0.],direction:[0.,0.,1.]},
                age,&mut scene);
            assert!(!scene.facets.is_empty(),"no geometry for spell {}",spec.id);
            assert!(scene.facets.len()<=500);
            for (view_id,view) in views.iter().copied().enumerate(){
                let name=format!("spell_{:03}_{:?}_view{}.svg",spec.id,stage,view_id+1);
                let dest=if export{dir.join(&name)}
                    else{std::env::temp_dir().join(format!("aetheria_exp_{name}"))};
                let r=export_svg(&scene,view,30+global,stage,age,&dest);
                if !export{let _=fs::remove_file(&dest);}
                assert!(r.degenerate<0.97,
                    "spell {} still looks almost entirely like lines from angle {}",spec.id,view_id+1);
                assert!(r.coverage>0.0001,
                    "spell {} is nearly invisible from angle {}",spec.id,view_id+1);
                assert!(r.range_y<75.);
                if export{
                    write!(gallery,"<img src='{name}' alt='{fullname} {:?} view {}'>",
                          stage,view_id+1).unwrap();
                    write!(tab,"{}\t{}\t{:?}\t{}\t{}\t{:.2}\t{:.6}\t{:.4}\n",
                          spec.id,spec.name,stage,view_id+1,r.polygons,r.span,
                          r.coverage,r.degenerate).unwrap();
                }
                checked+=1;
            }
        }
        if export{gallery.push_str("</section>\n");}
    }
    assert_eq!(checked,680);
    if export{
        fs::write(dir.join("index.html"),gallery).unwrap();
        fs::write(dir.join("geometry_report.tsv"),tab).unwrap();
        println!("M20: rendered 680 additional preview views; ALL 170 are still DRAFT art");
    }
}


/// M24: protect the first 20 dedicated Standard signatures against a
/// renderer regression into invisible, flat or silently truncated geometry.
/// These numeric bounds are sanity checks, NOT visual beauty certification.
#[test]
fn m24_twenty_source_authored_spells_have_two_view_mass_and_impact(){
    use aetheria_horizon::vfx::timeline::for_spell;
    use crate::catalog200;
    let ids=[6_u16,7,26,27,46,47,66,67,86,87,
             106,107,126,127,146,147,166,167,186,187];
    let cameras=[View::new([1.8,1.25,2.8]),View::new([-2.3,1.75,1.1])];
    let mut total=0;
    for id in ids {
        let schedule=for_spell(id).expect("M24 spell timeline missing");
        let total_ticks=schedule.total_ticks() as f64;
        for target in [Stage::Cast,Stage::Impact]{
            let mut elapsed=0.;
            let mut sample=None;
            for stage in schedule.phases{
                let length=stage.duration_ticks as f64;
                if stage.kind==target{
                    sample=Some((elapsed+length*0.65)/total_ticks*104.0);
                    break;
                }
                elapsed+=length;
            }
            let age=sample.expect("M24 stage missing");
            let mut scene=Scene::new(500);
            render_spell(Cast{kind:(id-1) as usize,variant:0,start:0,
                center:[0.,64.,0.],direction:[0.,0.,1.]},age,&mut scene);
            assert!((24..500).contains(&scene.facets.len()),
                "M24 geometry clipped/empty: {id} {:?}, {} quads",
                target,scene.facets.len());
            for (idx,view) in cameras.iter().copied().enumerate() {
                let file=std::env::temp_dir().join(format!(
                    "aetheria_M24_{id}_{target:?}_{idx}.svg"));
                let result=export_svg(&scene,view,30+(id-1) as usize,target,age,&file);
                let _=fs::remove_file(file);
                assert!(result.coverage>0.045,
                    "M24 shape almost invisible: spell {id}, phase {target:?}, view {idx}");
                assert!(result.degenerate<0.55,
                    "M24 collapsed to edge-on spaghetti: spell {id}, phase {target:?}, view {idx}");
                assert!(result.span<40.);
                total+=1;
            }
        }
    }
    assert_eq!(total,80);
}
