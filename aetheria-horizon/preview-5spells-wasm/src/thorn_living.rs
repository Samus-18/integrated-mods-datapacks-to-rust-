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
    let r=add(scale(right,width*twist.cos()),
        scale(up,width*twist.sin()));
    let n=add(scale(right,-width*0.38*twist.sin()),
        scale(up,width*0.38*twist.cos()));
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
    let fade=1.0-ease((age-67.0)/17.0);
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
        // Impact and aftermath: the dart BREAKS into a spreading thorn
        // flower/root-star around the hit location, no unchanged arrow held
        // in the air for five seconds.
        let t=ease((age-35.0)/18.0);
        let burst=(0.52+0.48*t)*fade;
        for j in 0..9 {
            let a=TAU*j as f64/9.0+0.19*(age-36.0)/30.0;
            let r=radial(right,up,a,1.15+1.20*t);
            let start=add(head,scale(aim,-0.10));
            let bend=add(add(start,scale(r,0.40*burst)),
                scale(aim,0.12+0.14*(j%3) as f64));
            let tip=add(add(start,scale(r,(0.80+0.34*(j%3) as f64)*burst)),
                scale(aim,-0.32-0.35*t));
            scene.tube(start,bend,0.115*burst,0.067*burst,
                a,DARK_BARK,5);
            living_blade(scene,bend,tip,
                0.25*burst,right,up,a,
                if j%3==0{EDGE}else if j%3==1{BLADE}else{BARK});
        }
        // Bright inner connected leaf petals after penetration, not a halo.
        for j in 0..5{
            let a=TAU*j as f64/5.0+PI*0.11;
            let origin=add(head,scale(aim,0.02));
            living_blade(scene,origin,
                add(add(origin,scale(aim,0.22*t)),
                    radial(right,up,a,0.75*burst)),
                0.18*burst,right,up,a,SAP);
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
    for j in 0..12 {
        let f=j as f64;
        let theta=f*2.3999632+age*0.045;
        let spread=(0.21+0.48*impact)*(0.35+(j%4) as f64/4.0);
        let drift=radial(right,up,theta,spread);
        let world=add(add(hit,drift),[0.0,
            0.08*((age*0.12+f*0.9).sin()),0.0]);
        let sz=(0.08+0.025*(j%3) as f64)*fade.max(0.015);
        let _=sprite(scene,camera,world,sz,sz*1.7,
            f*0.13+age*0.014,0x007BBB52,110);
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
        assert!(p.facets.len()<=12);
    }
}
