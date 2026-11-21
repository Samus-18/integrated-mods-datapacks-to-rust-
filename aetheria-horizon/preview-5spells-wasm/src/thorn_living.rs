//! M40: Thorn Dart is a world-space LIVING botanical projectile.
//! Actual six-sided/closed 3D bark, asymmetrical leaf blades and barbs,
//! a directional flight and an impact root-burst; no hero billboard.
//! Shader sprites are *optional small spores*, never the main body.
//! Only a CLIENT visual: collision/damage/root growth in the world is NOT wired.
use super::{Cast,Scene,add,sub,scale,norm,cross,length};
use super::procedural_fx::sprite;
use std::f64::consts::{PI,TAU};
type V=[f64;3];
const BARK:u32=0xF05C4328;
const DARK_BARK:u32=0xE7362B20;
const SAP:u32=0xE68CBD42;
const BLADE:u32=0xEC4A8F3C;
const EDGE:u32=0xE8C9EF7A;

fn ease(x:f64)->f64{let x=x.clamp(0.0,1.0);x*x*(3.0-2.0*x)}
fn frame(c:Cast)->(V,V,V){
    let aim=if length(c.direction)>0.01{norm(c.direction)}
        else{[0.0,0.0,1.0]};
    let up=if aim[1].abs()<0.94{[0.0,1.0,0.0]}
        else{[1.0,0.0,0.0]};
    let right=norm(cross(up,aim));
    let rise=norm(cross(aim,right));
    (aim,right,rise)
}
fn launch(c:Cast)->V{add(c.center,[0.0,1.45,0.0])}
fn flight_head(c:Cast,age:f64)->V{
    let (aim,_,_)=frame(c);
    add(launch(c),scale(aim,0.9+7.0*ease((age-8.0)/23.0)))
}
fn radial(right:V,up:V,a:f64,r:f64)->V{
    add(scale(right,r*a.cos()),scale(up,r*a.sin()))
}
// Closed faceted leaf/lance made from actual 3D quads. From six sides,
// every camera direction sees material thickness instead of flat crossed cards.
fn living_blade(scene:&mut Scene,from:V,to:V,width:f64,
    right:V,up:V,twist:f64,shade:u32){
    if width<0.02{return;}
    let axis=sub(to,from);
    let middle=add(from,scale(axis,0.48));
    // M41 fix from actual RX580 screenshot: the old leaf width vector
    // ran PARALLEL to the radial growth axis, collapsing leaves into needles.
    // Width is now geometrically perpendicular to each blade's own axis.
    let axis_unit=norm(axis);
    let dot_r=axis_unit[0]*right[0]+axis_unit[1]*right[1]+axis_unit[2]*right[2];
    let dot_u=axis_unit[0]*up[0]+axis_unit[1]*up[1]+axis_unit[2]*up[2];
    let tangent=if dot_r*dot_r+dot_u*dot_u>0.0001{
        norm(add(scale(right,-dot_u),scale(up,dot_r)))
    } else {radial(right,up,twist,1.0)};
    let thickness=norm(cross(axis_unit,tangent));
    let r=scale(tangent,width);
    let n=scale(thickness,width*0.36);
    // Faceted upper/lower faces meet at a continuous sharp tip.
    let l=sub(middle,r); let rr=add(middle,r);
    let raised=add(middle,n); let lowered=sub(middle,n);
    let Some(prev)=scene.begin_component(6)else{return;};
    scene.quad([from,l,raised,to],shade);
    scene.quad([from,to,raised,rr],super::shade_argb(shade,0.90));
    scene.quad([from,to,lowered,l],super::shade_argb(shade,0.70));
    scene.quad([from,rr,lowered,to],super::shade_argb(shade,0.62));
    scene.quad([l,lowered,to,raised],super::shade_argb(shade,0.79));
    scene.quad([rr,raised,to,lowered],super::shade_argb(shade,0.95));
    scene.end_component(prev);
}
pub(super) fn render_body(c:Cast,age:f64,scene:&mut Scene){
    if !age.is_finite()||!(0.0..85.0).contains(&age)
       ||!c.center.iter().chain(c.direction.iter()).all(|v|v.is_finite())
       {return;}
    let (aim,right,up)=frame(c);
    let grow=ease((age-1.0)/7.0);
    let fade=1.0-ease((age-61.0)/21.0);
    if grow<=0.01||fade<=0.01{return;}
    let head=flight_head(c,age);
    if age<8.0{
        // A closed seed sheath unfurls into the projectile; not an empty ring.
        let base=add(launch(c),scale(aim,0.12));
        scene.solid_crystal(base,add(base,scale(aim,0.75*grow)),
            0.30*grow,0.0,DARK_BARK,6);
        for j in 0..5{
            let a=TAU*j as f64/5.0+0.31*age;
            let dir=radial(right,up,a,0.52*grow);
            living_blade(scene,add(base,scale(aim,0.2)),
                add(add(base,scale(aim,0.55*grow)),dir),
                0.17*grow,right,up,a,BLADE);
        }
        return;
    }
    let pulse=(0.82+0.18*(age*0.43).sin())*fade;
    if age<36.0 {
        // Launch/flight: short, thick ridged bark shaft, distinct clustered
        // asymmetrical living thorns and a double-layer spearhead.
        let tail=add(head,scale(aim,-2.35));
        let joint=add(tail,scale(aim,1.15));
        scene.tube(tail,joint,0.18*grow*pulse,
            0.145*grow*pulse,0.13,BARK,7);
        scene.tube(joint,add(head,scale(aim,-0.55)),
            0.145*grow*pulse,0.12*grow*pulse,0.41,DARK_BARK,7);
        let blade_base=add(head,scale(aim,-0.72));
        scene.solid_crystal(blade_base,add(head,scale(aim,0.56)),
            0.38*pulse,0.16,BLADE,7);
        for j in 0..5{
            let a=TAU*(j as f64*0.618034).fract();
            let where_at=add(tail,scale(aim,0.33+j as f64*0.32));
            let outward=radial(right,up,a,0.31+0.09*(j%2) as f64);
            let backwards=scale(aim,-0.33-0.07*j as f64);
            living_blade(scene,where_at,
                add(add(where_at,outward),backwards),
                0.17*pulse,right,up,a,
                if j%2==0{BLADE}else{BARK});
            let glint=add(where_at,scale(outward,0.58));
            scene.tube(where_at,glint,
                0.030*pulse,0.014*pulse,a,SAP,5);
        }
        // Split botanical blade shoulder, two asymmetric outer fins.
        for side in [-1.0,1.0] {
            let from=add(blade_base,scale(right,side*0.07));
            let to=add(add(head,scale(aim,-0.02)),
                add(scale(right,side*0.42),scale(up,0.07*side)));
            living_blade(scene,from,to,0.23*pulse,right,up,
                0.41+side*0.32,EDGE);
        }
        // Trailing leaflets move with the projectile, never a vertical fog.
        for j in 0..3 {
            let t=j as f64;
            let origin=add(tail,scale(aim,-0.25-0.26*t));
            let sway=radial(right,up,age*0.25+t*2.1,0.24+0.09*t);
            living_blade(scene,origin,
                add(add(origin,scale(aim,-0.55-0.10*t)),sway),
                0.11*pulse,right,up,0.25+t,BLADE);
        }
    } else {
        // M41 screenshot correction: replace the rejected sparse radial
        // toothpick/star with a CONNECTED, filled, faceted leaf-bud mass.
        // Broad tangential petals overlap its central green seed, and each
        // has genuine 3D thickness. No large empty center or same 9 spokes.
        let t=ease((age-35.0)/16.0);
        let burst=(0.58+0.42*t)*fade;
        // Irregular dense double core creates a contiguous silhouette even
        // between petals, rather than a black pinhole and star spokes.
        scene.solid_crystal(
            add(head,scale(aim,-0.52)),
            add(head,scale(aim,0.55)),
            0.68*burst,0.12,BLADE,7);
        scene.solid_crystal(
            add(head,scale(up,-0.46)),
            add(head,scale(up,0.48)),
            0.48*burst,0.38,SAP,6);
        // Seven wide overlapping living leaves, rotated in depth as a
        // flower instead of nine equally-sized needles.
        for j in 0..7{
            let jf=j as f64;
            let a=TAU*jf/7.0+0.11*(age-36.0)/30.0
                +0.07*(jf*2.1).sin();
            let forward=if j%2==0{0.19}else{-0.13};
            let start=add(head,scale(aim,-0.12));
            let tip=add(add(head,radial(right,up,a,
                (1.20+0.22*(jf*1.9).sin())*burst)),
                scale(aim,forward+0.21*t));
            living_blade(scene,start,tip,
                (0.52+0.13*(jf*1.3).sin().abs())*burst,
                right,up,a,if j%3==0{EDGE}else{BLADE});
        }
        // Inner leaves close the gaps in the body and fold forward in 3D.
        for j in 0..7 {
            let jf=j as f64;
            let a=TAU*(jf+0.43)/7.0+0.04*age;
            let start=add(head,scale(aim,0.10));
            let tip=add(add(head,
                radial(right,up,a,0.80*burst)),
                scale(aim,0.39*(jf*1.6).sin()));
            living_blade(scene,start,tip,
                0.44*burst,right,up,a,SAP);
        }
    }
}
// Only fine independent spores; dedicated optional soft-particle pipeline.
// It cannot cover the 3D model with an opaque fake halo.
pub(super) fn render_spores(c:Cast,age:f64,camera:V,scene:&mut Scene){
    if !(8.0..80.0).contains(&age)
        ||!camera.iter().chain(c.center.iter()).all(|v|v.is_finite())
        {return;}
    let (_,right,up)=frame(c);
    let hit=flight_head(c,age);
    let impact=ease((age-35.0)/10.0);
    let fade=1.0-ease((age-65.0)/15.0);
    for j in 0..20 {
        let f=j as f64;
        let theta=f*2.3999632+age*0.045;
        let spread=(0.23+0.62*impact)*(0.35+(j%5) as f64/5.0);
        let drift=radial(right,up,theta,spread);
        let world=add(add(hit,drift),[0.0,
            0.13*((age*0.12+f*0.9).sin()),0.0]);
        let sz=(0.12+0.023*(j%4) as f64)*fade.max(0.015);
        let _=sprite(scene,camera,world,sz,sz*1.45,
            f*0.13+age*0.014,0x008ABD5A,110);
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    fn cast()->Cast {Cast{kind:100,variant:0,start:0,
        center:[0.0,65.0,0.0],direction:[0.24,-0.1,0.97]}}
    #[test]
    fn distinct_charge_flight_impact_afterglow_world_volumes(){
        let mut captures=Vec::new();
        for age in [5.0,18.0,30.0,48.0,69.0]{
            let mut scene=Scene::new(500);
            render_body(cast(),age,&mut scene);
            assert!(scene.facets.len()>28,"age {age}: missing sculpted 3D body");
            assert!(scene.facets.len()<500);
            assert_eq!(scene.facets.len(),scene.group_ids.len());
            let mut lo=[f64::INFINITY;3];let mut hi=[f64::NEG_INFINITY;3];
            for (q,_) in &scene.facets {
                for p in q {for i in 0..3{
                    lo[i]=lo[i].min(p[i]);hi[i]=hi[i].max(p[i]);
                }}
            }
            assert!((hi[0]-lo[0])*(hi[1]-lo[1])*(hi[2]-lo[2])>0.01);
            captures.push(scene.facets);
        }
        for pair in captures.windows(2){assert_ne!(pair[0],pair[1]);}
    }
    #[test]
    fn m41_leaf_width_must_cross_growth_axis_not_collapse_into_a_spoke(){
        let mut scene=Scene::new(50);
        living_blade(&mut scene,[0.,0.,0.],[1.,0.,0.],
            0.55,[1.,0.,0.],[0.,1.,0.],0.0,BLADE);
        assert_eq!(scene.facets.len(),6);
        let mut low=f64::INFINITY;let mut high=f64::NEG_INFINITY;
        for (q,_) in scene.facets {for point in q{
            low=low.min(point[1]);high=high.max(point[1]);
        }}
        assert!(high-low>1.0,
           "Visual QA M40 regression: leaf was a thin radial spoke");
    }
    #[test]
    fn forward_motion_and_budget_never_fall_back_to_flat_hero_card(){
        let c=cast();
        let (dir,_,_)=frame(c);
        let a=flight_head(c,8.0);let b=flight_head(c,31.0);
        assert!(sub(b,a).iter().zip(dir).map(|(x,y)|x*y).sum::<f64>()>6.8);
        for budget in [0,12,50,100,200]{
            let mut s=Scene::new(budget);
            render_body(c,22.0,&mut s);
            assert!(s.facets.len()<=budget);
            assert!(s.facets.iter().all(|(q,_)|
                q.iter().flatten().all(|v|v.is_finite())));
        }
        let mut p=Scene::new(30);
        render_spores(c,24.0,[6.0,70.0,9.0],&mut p);
        assert!(p.facets.len()<=20);
    }
}
