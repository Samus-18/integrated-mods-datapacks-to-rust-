//! M32 in-game art restart: broad filled 3D surfaces, NOT thin tubes,
//! outline rings, wire scaffolds, or one more shader painted on sticks.
//! This replaces the old active 200-spell render path for this experimental
//! build. Distinct authored spell descriptions still influence silhouettes;
//! fully individually approved 200 artistic scenes remain future work.
//! The low-opacity surfaces feed the actual WGSL 3D density integration.
//! Draw-budget components are atomic, so LOD cannot tear a volume in half.
use super::{Cast,Scene,add,shade_argb};
use crate::{catalog200,mass_forms};
use mass_forms::Form;
use std::f64::consts::{PI,TAU};

type V=[f64;3];
const SOLID:[u32;10]=[
    0x00C85624,0x0064B6D2,0x007AB9BD,0x009C8471,0x004A9DC8,
    0x0064A86C,0x006A8DB3,0x006954A8,0x00B29BA1,0x008575A6
];
const GLOW:[u32;10]=[
    0x00FFA34C,0x00B5EFFF,0x00D4F1EA,0x00E3BA85,0x007EDAF3,
    0x0097ECAA,0x00D2E5FF,0x00BCA2EF,0x00E6D4FF,0x00D6BADF
];
fn col(rgb:u32,opacity:f64)->u32{
    (((opacity.clamp(0.0,1.0)*255.0).round() as u32)<<24)|(rgb&0x00ff_ffff)
}
fn p(base:V,x:f64,y:f64,z:f64)->V{
    add(base,[x,y,z])
}
fn face_tint(rgb:u32,opacity:f64,light:f64)->u32{
    shade_argb(col(rgb,opacity),light)
}
fn motion(age:f64,seed:f64)->f64{
    age*0.023+seed*0.17
}
fn curtain_point(
    origin:V,school:usize,form:Form,panel:usize,
    u:f64,v:f64,age:f64,seed:f64,
    width:f64,height:f64,depth:f64,
    back:f64
)->V {
    // The center shifts across a large continuous curved surface. The
    // surface must be at least several blocks WIDE, not a beam or outline.
    let a=TAU*panel as f64/4.0+seed*0.11;
    let local=u*width*(0.61+0.38*(PI*v*0.5).sin());
    let t=motion(age,seed);
    let r=(2.8*u+4.4*v+t).sin();
    let twist=(3.7*u-3.1*v+t*0.67).cos();
    let fold=(5.3*v+1.6*u-t*0.62).sin();
    let (side,h,forward)=match school {
        // Fire: broad, asymmetric upward billows, not needle flames.
        0=>(local*(0.85+0.3*v),
            height*(v.powf(0.76)) + 0.31*r,
            depth*(0.46*r+0.22*twist)+v*v*1.18),
        // Frost: huge fractured slab-like folds and cold drifting haze.
        1=>(local*(0.98+0.06*(u*4.0).sin()),
            height*v+0.24*(3.0*u+v).sin(),
            depth*(0.36*fold+0.22*r)),
        // Air: large billowing sails with rising windward edges.
        2=>(local*1.30,
            height*(0.36+v*0.52)+0.7*(u*2.2+t).sin(),
            depth*(0.74*fold+0.46*r)),
        // Stone: stepped broad sediment strata rather than sharp rods.
        3=>(local,
            height*v+0.17*(u*2.0).floor(),
            depth*(0.18*twist+0.16*r)),
        // Water: curling horizontal wave crests with a foamy crown.
        4=>(local*1.18,
            height*(0.17+v*0.47)+0.88*(PI*v).sin(),
            depth*(1.05*fold+0.20*r)+v*0.66),
        // Druid: leaf-shaped canopies which spread outward at the top.
        5=>(local*(0.35+0.86*(PI*v).sin().abs()),
            height*v+0.29*fold,
            depth*(0.54*twist+0.23*r)),
        // Lightning: masses of storm cloud, energy cracks live in WGSL.
        6=>(local*(0.68+0.53*(PI*v).sin()),
            height*(0.48+0.37*v)+0.39*fold,
            depth*(0.70*r+0.36*twist)),
        // Astral: folded reality with a large dark dimensional veil.
        7=>(local*(0.87+0.18*fold),
            height*v+0.24*(u*3.0+t).sin(),
            depth*(0.89*fold+0.48*twist)),
        // Time: offset thick glass faces, sweeping not spinning rings.
        8=>(local+0.45*(v*4.0+t).sin(),
            height*v+0.23*(u*3.0).sin(),
            depth*(0.43*fold+0.18*twist)),
        // Souls: multiple billowing sheets of a wide spectral cloak.
        _=>(local*(0.45+0.69*v),
            height*v+0.47*(u*2.2+v*2.1+t).sin(),
            depth*(0.82*fold+0.31*r))
    };
    let semantic=match form{
        Form::Phoenix=>0.32*(u*4.0+v*7.0+t).sin(),
        Form::SkyRift=>0.42*(u*1.7-v*4.0+t).cos(),
        Form::Wave=>0.38*(u*3.4+v*2.6+t).sin(),
        Form::Flora=>0.27*(u*5.1+v*1.7+t).cos(),
        Form::Time=>0.19*(4.0*v).floor(),
        Form::Cloud=>0.23*r,
        _=>0.09*twist
    };
    let along=forward+semantic+back*0.10;
    // Non-parallel adjacent sheets occupy different 3D world positions.
    p(origin,side*a.cos()-along*a.sin(),
        h-0.55,
        side*a.sin()+along*a.cos())
}
fn curtain(
    scene:&mut Scene,origin:V,school:usize,form:Form,index:usize,
    age:f64,seed:f64,width:f64,height:f64,fade:f64
) {
    // 7x5 cells = 35 quads per face; front + back share one atomic group.
    let (cols,rows) = if scene.reduced_detail {(5,4)}else{(7,5)};
    let cells=cols*rows;
    let Some(previous)=scene.begin_component(cells*2)else{return};
    let radius=1.2+index as f64*0.2;
    let center=p(origin,
       radius*(TAU*index as f64/4.0+0.5).sin(),
       0.0,
       radius*(TAU*index as f64/4.0+0.5).cos());
    let point=|i:usize,j:usize,back:f64|->V{
        let u=2.0*i as f64/cols as f64-1.0;
        let v=j as f64/rows as f64;
        curtain_point(center,school,form,index,u,v,age,seed,
                      width,height,0.78+index as f64*0.10,back)
    };
    for i in 0..cols {
        for j in 0..rows {
            let a=point(i,j,1.0);
            let b=point(i+1,j,1.0);
            let c=point(i+1,j+1,1.0);
            let d=point(i,j+1,1.0);
            let alpha=(0.12+0.045*(PI*(j as f64+0.5)/rows as f64).sin())*fade;
            let light=(0.76+0.18*(i as f64/cols as f64))*
                (0.91+0.07*(index%3) as f64);
            scene.quad([a,b,c,d],face_tint(GLOW[school],alpha,light));
            let back_a=point(i,j,-1.0);
            let back_b=point(i+1,j,-1.0);
            let back_c=point(i+1,j+1,-1.0);
            let back_d=point(i,j+1,-1.0);
            scene.quad([back_d,back_c,back_b,back_a],
                face_tint(SOLID[school],alpha*0.76,0.79));
        }
    }
    scene.end_component(previous);
}
fn ellipsoid(
    scene:&mut Scene,o:V,r:V,age:f64,seed:f64,rgb:u32,
    opacity:f64
){
    // A whole asymmetric organic shell, 72 large curved quadrilaterals.
    // No segment is ever a line, a tube, or a hollow circle.
    let (cols,rows) = if scene.reduced_detail {(10,5)}else{(12,6)};
    let Some(previous)=scene.begin_component(cols*rows)else{return};
    let vertex=|i:usize,j:usize|->V{
        let a=TAU*i as f64/cols as f64;
        let b=-PI*0.5+0.13+
            (PI-0.26)*j as f64/rows as f64;
        let cr=b.cos();
        let wobble=1.0+0.13*(3.0*a+seed*0.31+age*0.017).sin()*
            (2.0*b+seed*0.07).cos();
        p(o,r[0]*cr*a.cos()*wobble,
          r[1]*b.sin()*(1.0+0.07*(a*2.0+age*0.02).cos()),
          r[2]*cr*a.sin()*wobble)
    };
    for i in 0..cols{
        for j in 0..rows{
            let q=[vertex(i,j),vertex(i+1,j),
                   vertex(i+1,j+1),vertex(i,j+1)];
            let brightness=0.64+0.32*(0.5+0.5*(i as f64*2.4+seed).sin());
            scene.quad(q,face_tint(rgb,opacity,brightness));
        }
    }
    scene.end_component(previous);
}
fn monumental_plate(
    scene:&mut Scene,o:V,angle:f64,width:f64,
    height:f64,thickness:f64,opacity:f64,rgb:u32,seed:f64
){
    // A wide fractured slab with six exterior surfaces. Short broad
    // armor/ice/stone masses; no tall thin pillars.
    let Some(previous)=scene.begin_component(6) else{return};
    let lean=0.35*(seed*2.3).sin();
    let local=[
      [-width*0.54,-0.40,-thickness],
      [width*0.57,-0.40,-thickness],
      [width*0.43+lean,height,-thickness*0.61],
      [-width*0.63+lean,height*0.82,-thickness*0.55],
      [-width*0.54,-0.40,thickness],
      [width*0.57,-0.40,thickness],
      [width*0.43+lean,height,thickness*0.61],
      [-width*0.63+lean,height*0.82,thickness*0.55]
    ];
    let pt=|i:usize|->V{
        let a=local[i];
        p(o,a[0]*angle.cos()-a[2]*angle.sin(),
          a[1],
          a[0]*angle.sin()+a[2]*angle.cos())
    };
    for (side,idx) in [
        [0,1,2,3],[4,7,6,5],[0,4,5,1],
        [3,2,6,7],[1,5,6,2],[0,3,7,4]
    ].into_iter().enumerate(){
        scene.quad(idx.map(pt),face_tint(rgb,opacity,
            0.59+0.07*(side%5) as f64));
    }
    scene.end_component(previous);
}
/// M32.1: 18 genuinely different silhouettes selected from the authored
/// registry mesh meaning. A central sphere is now ALLOWED ONLY when that
/// spell actually calls for an orb/beast/cloud: no more same generic oval
/// across 10 classes. No line/ring/tube primitives run here.
pub(super) fn render(cast:Cast,age:f64,scene:&mut Scene){
    if !(0.0..104.0).contains(&age)||!age.is_finite(){return;}
    let slot=cast.kind.min(199);
    let school=catalog200::school(slot);
    let rank=catalog200::rank(slot);
    let form=mass_forms::semantic_form(catalog200::spec(slot).mesh);
    let seed=(slot+1) as f64;
    let grow=(0.49+0.51*(age/15.0).clamp(0.,1.)).min(1.0);
    let fade=((104.0-age)/19.0).clamp(0.,1.);
    let reach=(2.65+0.19*(rank%7) as f64)*grow;
    let height=(4.65+0.22*(rank%6) as f64)*
        if rank>=18 {1.37}else if rank>=14 {1.18}else{1.0}*grow;
    let center=cast.center;
    let front=SOLID[school];
    let glow=GLOW[school];
    // Volumes are deliberately used in different compositions according
    // to the spell's original mesh description, NOT school-wide recolours.
    match form {
        Form::Crystal=>{
            // Asymmetric broad glacial fault. The ice wall is the hero;
            // translucent air hangs BETWEEN whole cliffs, never around rods.
            for i in 0..7 {
                let x=(i as f64-3.0)*0.93;
                let z=(i as f64*1.73+seed).sin()*1.06;
                monumental_plate(scene,p(center,x,0.,z),
                    0.25+(i as f64*0.8+seed*0.12).sin()*0.37,
                    (1.8+0.6*(i%3) as f64)*grow,
                    height*(0.47+0.08*(i%4) as f64),
                    0.59,fade*0.91,
                    if i%4==0{glow}else{front},seed+i as f64);
            }
            curtain(scene,center,school,form,1,age,seed,
                reach*1.10,height*0.66,fade*0.72);
        }
        Form::Boulder=>{
            // Tectonic tumbling chunks; not a vertical palisade.
            for i in 0..9{
                let a=TAU*(i as f64/9.0)+seed*0.25;
                let r=0.95+0.43*(i%4) as f64;
                monumental_plate(scene,
                    p(center,a.cos()*r,(i%3) as f64*0.47,a.sin()*r),
                    a+0.56,2.4*grow,1.8*grow+height*0.16,
                    0.68,fade*0.86,if i%3==0{glow}else{front},
                    seed+i as f64*1.12);
            }
            curtain(scene,center,school,form,0,age,seed,
                reach*1.18,height*0.70,fade*0.72);
        }
        Form::Architecture=>{
            // Flying broad walls and fractured vaults, large void between.
            for i in 0..6{
                let sign=if i%2==0 {-1.0}else{1.0};
                let tier=i/2;
                monumental_plate(scene,p(center,
                    sign*(2.7-0.6*tier as f64),
                    0.50*tier as f64,0.30*(tier as f64-1.0)),
                    0.11*sign+seed*0.015,2.7*grow,
                    height*(0.48+0.07*tier as f64),0.66,
                    fade*0.91,if i==4{glow}else{front},seed+i as f64);
            }
            // One heavy stone lintel spans the tall architecture.
            monumental_plate(scene,p(center,0.,height*0.63,0.),
                PI*0.5,reach*1.8,1.24*grow,0.95,
                fade*0.89,front,seed+19.0);
            curtain(scene,center,school,form,2,age,seed,
                reach,height*0.62,fade*0.48);
        }
        Form::Blade=>{
            // Broad sweeping physical blade body, NOT a needle.
            for i in 0..4{
                let az=-0.52+i as f64*0.34;
                monumental_plate(scene,p(center,
                    (i as f64-1.5)*0.85,(i%2) as f64*0.35,
                    0.4*(i as f64).sin()),az+seed*0.01,
                    (2.8+0.3*(i%2) as f64)*grow,
                    height*(0.55+0.07*i as f64),0.54,
                    fade*0.86,if i==1{glow}else{front},
                    seed+i as f64);
            }
            curtain(scene,center,school,form,1,age,seed,
                reach*1.35,height*0.78,fade*0.65);
            curtain(scene,center,school,form,3,age,seed,
                reach*0.78,height*0.61,fade*0.52);
        }
        Form::Armor=>{
            // Four angled closed armor shields make a protective shell;
            // the middle is OPEN, no generic filled egg.
            for i in 0..6{
                let a=TAU*i as f64/6.0;
                monumental_plate(scene,p(center,a.cos()*1.63,
                    (i%3) as f64*0.43,a.sin()*1.63),
                    a+0.53,2.65*grow,height*0.52,0.66,
                    fade*0.77,if i%3==0{glow}else{front},
                    seed+i as f64);
            }
            curtain(scene,center,school,form,1,age,seed,
                reach*0.94,height*0.73,fade*0.54);
        }
        Form::Glyph=>{
            // Monumental occult TABLET with inset facets, not a rune ring.
            monumental_plate(scene,p(center,0.,height*0.10,0.),
                0.10+seed*0.01,reach*2.0,height*0.92,0.83,
                fade*0.87,front,seed);
            for i in 0..6 {
                let x=((i%3) as f64-1.0)*reach*0.48;
                let y=(i/3) as f64*height*0.41+height*0.13;
                monumental_plate(scene,p(center,x,y,-0.90),
                    0.05,reach*0.38,height*0.27,0.19,
                    fade*0.79,glow,seed+i as f64);
            }
            curtain(scene,center,school,form,1,age,seed,
                reach*1.3,height*0.95,fade*0.45);
        }
        Form::Relic=>{
            // Dense hovering relic + fractured engraving plates.
            monumental_plate(scene,p(center,0.,height*0.29,0.),
                PI*0.25,reach*1.12,height*0.48,1.02,
                fade*0.85,front,seed);
            for i in 0..5{
                let a=TAU*i as f64/5.0;
                monumental_plate(scene,p(center,2.1*a.cos(),
                    height*0.50+0.26*(i%3) as f64,2.1*a.sin()),
                    a+0.45,1.4*grow,height*0.33,0.53,
                    fade*0.80,if i%2==0{glow}else{front},
                    seed+i as f64);
            }
            curtain(scene,center,school,form,2,age,seed,
                reach,height*0.9,fade*0.46);
        }
        Form::SkyRift=>{
            // A jagged empty dark sky opening: broad offset curtains on
            // opposing planes, plus solid torn slabs at its edges. No ring.
            curtain(scene,p(center,-0.9,0.,0.),school,form,0,age,seed,
                reach*1.44,height*1.22,fade*0.94);
            curtain(scene,p(center,0.7,0.,0.8),school,form,2,age,seed,
                reach*1.15,height*1.16,fade*0.68);
            for i in 0..4{
                let sign=if i%2==0 {-1.0}else{1.0};
                monumental_plate(scene,p(center,sign*reach*0.91,
                    (i/2) as f64*height*0.44,0.33*sign),
                    -0.25*sign,1.5*grow,height*0.43,0.36,
                    fade*0.57,front,seed+i as f64);
            }
        }
        Form::Flora|Form::Canopy|Form::Phoenix=>{
            // Huge, connected translucent LEAVES/WINGS, not twigs.
            // Different canopy styles arise from registry semantic class
            // and school-specific fold dynamics in curtain_point.
            for i in 0..4{
                curtain(scene,center,school,form,i,age,seed,
                    reach*(1.12+0.13*(i%2) as f64),
                    height*(0.69+0.12*(i%3) as f64),fade*0.96);
            }
        }
        Form::Wave=>{
            // Multi-layer breaking water curtains, horizontal not oval.
            for i in 0..4{
                curtain(scene,p(center,
                    (i as f64-1.5)*0.54,0.,i as f64*0.42),
                    school,form,i,age,seed,
                    reach*(1.55-0.12*i as f64),
                    height*(0.60+0.12*(i%2) as f64),fade*0.84);
            }
        }
        Form::Ribbon=>{
            // Woven broad vortex sheets with negative space in the middle.
            for i in 0..4{
                curtain(scene,p(center,
                    0.4*(i as f64).sin(),0.,0.65*(i as f64).cos()),
                    school,form,i,age,seed,
                    reach*(0.89+0.10*i as f64),
                    height*(0.75+0.07*i as f64),fade*0.90);
            }
        }
        Form::Time=>{
            // Two offset walls of temporal glass / sandglass. NO circles.
            for i in 0..6 {
                let sign=if i%2==0{-1.0}else{1.0};
                monumental_plate(scene,p(center,sign*(1.1+0.5*(i/2) as f64),
                    height*0.21*(i/2) as f64,0.47*(i as f64).sin()),
                    sign*0.36,1.5*grow,height*0.37,0.43,
                    fade*0.58,if i%3==0{glow}else{front},seed+i as f64);
            }
            curtain(scene,center,school,form,0,age,seed,
                reach*1.02,height,fade*0.78);
            curtain(scene,center,school,form,2,age,seed,
                reach*1.22,height*0.90,fade*0.63);
        }
        Form::Avatar=>{
            // Massive shoulder/chest silhouette assembled from broad
            // sculpted armored slabs, NOT a pencil-limb skeleton.
            monumental_plate(scene,p(center,0.,height*0.12,0.),0.22,
                reach*1.32,height*0.63,0.72,fade*0.84,front,seed);
            for i in 0..4{
                let sign=if i%2==0{-1.0}else{1.0};
                monumental_plate(scene,p(center,
                    sign*(1.12+0.62*(i/2) as f64),
                    height*(0.2+0.37*(i/2) as f64),0.4),
                    sign*(0.35+0.2*(i/2) as f64),
                    1.5*grow,height*0.30,0.47,fade*0.70,
                    if i%3==0{glow}else{front},seed+i as f64);
            }
            curtain(scene,center,school,form,3,age,seed,
                reach*1.06,height*0.90,fade*0.46);
        }
        Form::Cloud=>{
            // TWO staggered billowing storm masses and a low broad shadow
            // veil. Real cores are appropriate only for cloud semantics.
            ellipsoid(scene,p(center,-0.6,height*0.53,-0.3),
                [2.80*grow,height*0.36,1.93*grow],
                age,seed,front,fade*0.52);
            ellipsoid(scene,p(center,1.1,height*0.42,0.48),
                [2.46*grow,height*0.29,2.17*grow],
                age,seed+3.0,glow,fade*0.39);
            curtain(scene,center,school,form,1,age,seed,
                reach*1.43,height*0.65,fade*0.79);
        }
        Form::Beast=>{
            // Big bodily creature mass, angular armor and broad wings;
            // NOT a coiling straw with rings.
            ellipsoid(scene,p(center,-0.78,height*0.43,0.0),
                [2.40*grow,height*0.32,1.39*grow],
                age,seed,front,fade*0.70);
            monumental_plate(scene,p(center,1.20,height*0.36,0.0),
                0.34,1.8*grow,height*0.47,0.55,
                fade*0.77,glow,seed+2.0);
            curtain(scene,center,school,form,1,age,seed,
                reach*1.43,height*0.84,fade*0.80);
            curtain(scene,center,school,form,3,age,seed,
                reach*1.46,height*0.80,fade*0.79);
        }
        Form::Artifact=>{
            // Bright 3D orb only where the source actually calls for one.
            ellipsoid(scene,p(center,0.,height*0.50,0.),
                [2.08*grow,height*0.38,1.97*grow],
                age,seed,glow,fade*0.73);
            for i in 0..4{
                let a=TAU*i as f64/4.0;
                monumental_plate(scene,
                    p(center,2.0*a.cos(),height*0.36,
                        2.0*a.sin()),a+0.57,
                    1.45*grow,height*0.34,0.43,
                    fade*0.73,front,seed+i as f64);
            }
            curtain(scene,center,school,form,1,age,seed,
                reach*0.96,height*0.77,fade*0.48);
        }
        // A few mesh names are abstract Canopy; already handled above.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_200_spell_previews_have_broad_mass_not_wire_scaffolds(){
        for slot in 0..200 {
            let mut s=Scene::new(500);
            render(Cast{kind:slot,variant:0,start:0,
                center:[0.0,64.0,0.0],direction:[0.0,0.0,1.0]},44.0,&mut s);
            assert!(s.facets.len()>=60 && s.facets.len()<=500,
                "M32 volume body is missing for spell {}",slot+1);
            assert_eq!(s.facets.len(),s.group_ids.len());
            let mut broad=0;
            for (q,_) in &s.facets {
                let u=[q[1][0]-q[0][0],q[1][1]-q[0][1],q[1][2]-q[0][2]];
                let v=[q[3][0]-q[0][0],q[3][1]-q[0][1],q[3][2]-q[0][2]];
                let len=|a:V| (a[0]*a[0]+a[1]*a[1]+a[2]*a[2]).sqrt();
                if len(u)>0.25 && len(v)>0.25 {broad+=1;}
                assert!(q.iter().flatten().all(|c|c.is_finite()));
            }
            assert!(broad>s.facets.len()*3/5,
                "M32 spell {} still projects as thin stick geometry",slot+1);
        }
    }
    #[test]
    fn render_falls_silent_after_lifetime(){
        let mut s=Scene::new(500);
        render(Cast{kind:5,variant:0,start:0,center:[0.0;3],
            direction:[0.0,0.0,1.0]},105.0,&mut s);
        assert!(s.facets.is_empty());
    }
}
