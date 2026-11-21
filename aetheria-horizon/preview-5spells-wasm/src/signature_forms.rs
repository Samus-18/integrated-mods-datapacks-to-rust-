//! M37: NAME-TRUE dimensional spell bodies. Not sprites masquerading as a
//! serpent, bridge, rift, roots or lightning net. Procedural 3D surfaces
//! feed the actual Horizon world-quad renderer. NO AI concepts or Blender
//! exports. The ten signatures below deliberately OVERRIDE generic M32
//! solid bodies; the M35/M36 shader particle envelope still layers on top.
//! Client visual ONLY: cracks do not edit terrain; roots do not restrain;
//! Arc Net cannot damage; the ice bridge elsewhere is still not collidable.
use super::{Cast,Scene,add,sub,scale,norm,cross,length,shade_argb};
use std::f64::consts::{PI,TAU};
type V=[f64;3];
const FIRE:u32=0xE9FF6D24; const EMBER:u32=0xA8F63814;
const ICE:u32=0xB8A9E8FD; const SNOW:u32=0x9FE5F9FF;
const WIND:u32=0x86C2FFE2;const LEAF:u32=0xB497EC7C;
const STONE:u32=0xF36F7482;const LAVA:u32=0xDFFF9C28;
const WATER:u32=0xB34CACF4;const FOAM:u32=0xA5C8FAFF;
const WOOD:u32=0xF28A6A40;const SAP:u32=0xB3B9EB68;
const PLASMA:u32=0xB7A0CBFF;const ARC:u32=0xED72ADFF;
const RIFT:u32=0xE2B256FB;const VOID:u32=0xCE170A35;
const TIME:u32=0xE8FFD783;const GHOST:u32=0xB0D064C6;
fn ease(t:f64)->f64{let t=t.clamp(0.0,1.0);t*t*(3.0-2.0*t)}
fn phase(age:f64)->(f64,f64){
    (ease((age-3.0)/27.0),
     1.0-ease((age-76.0)/27.0))
}
fn v3(x:f64,y:f64,z:f64)->V{[x,y,z]}
fn local(c:Cast,x:f64,y:f64,z:f64)->V{
    let horizontal=[c.direction[0],0.0,c.direction[2]];
    let d=if length(horizontal)>0.001{norm(horizontal)}else{[0.0,0.0,1.0]};
    let r=[d[2],0.0,-d[0]];
    add(c.center,add(add(scale(r,x),v3(0.0,y,0.0)),scale(d,z)))
}
fn vlerp(a:V,b:V,t:f64)->V{add(a,scale(sub(b,a),t))}
fn mutate(color:u32,i:usize)->u32{
    shade_argb(color,match i%5 {0=>0.95,1=>0.73,2=>0.86,3=>0.66,_=>1.0})
}
fn ribbon(scene:&mut Scene,curve:&[V],width:f64,color:u32,vertical:bool){
    if curve.len()<2||!width.is_finite()||width<=0.01{return;}
    let faces=(curve.len()-1)*6;
    let Some(prev)=scene.begin_component(faces)else{return;};
    for i in 0..curve.len()-1{
        let dir=norm(sub(curve[i+1],curve[i]));
        let perpendicular=if vertical{
            let a=cross(dir,[1.0,0.3,0.0]);
            if length(a)>0.001{norm(a)}else{[0.0,0.0,1.0]}
        }else{
            let a=cross(dir,[0.0,1.0,0.0]);
            if length(a)>0.001{norm(a)}else{[1.0,0.0,0.0]}
        };
        let w=width*(0.35+0.65*((i as f64+0.5)/curve.len() as f64*PI).sin());
        let b=scale(perpendicular,w);
        scene.membrane(sub(curve[i],b),sub(curve[i+1],b),
            add(curve[i+1],b),add(curve[i],b),mutate(color,i));
    }
    scene.end_component(prev);
}
fn connected_body(scene:&mut Scene,pts:&[V],radius:f64,color:u32,sides:usize){
    if pts.len()<2{return;}
    let Some(prev)=scene.begin_component((pts.len()-1)*sides)else{return;};
    for i in 0..pts.len()-1 {
        let t=i as f64/(pts.len()-1) as f64;
        let a=radius*(0.89-0.69*ease(t));
        let b=radius*(0.89-0.69*ease((i+1) as f64/(pts.len()-1) as f64));
        scene.tube(pts[i],pts[i+1],a.max(0.02),b.max(0.02),
            t*PI*0.19,mutate(color,i),sides);
    }
    scene.end_component(prev);
}
fn shard(scene:&mut Scene,from:V,to:V,width:f64,color:u32){
    scene.solid_crystal(from,to,width,0.21,color,6);
}
fn serpent(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);
    if grow<0.03||fade<0.03{return;}
    let spin=age*0.032;
    let count=19usize;
    let mut spine=Vec::with_capacity(count);
    for i in 0..count{
        let t=i as f64/(count-1) as f64;
        // Thick two-turn BODY with a rising neck and upward-facing head.
        let theta=TAU*(1.60*t+0.13)+spin;
        let rad=(2.70-1.40*t)*grow;
        let y=0.50+3.80*t+0.35*(theta*0.65).sin();
        spine.push(local(c,rad*theta.cos(),y*grow,1.7+rad*theta.sin()));
    }
    connected_body(scene,&spine,0.68*fade,FIRE,6);
    let h=spine[count-1];
    let previous=spine[count-3];
    let forward=norm(sub(h,previous));
    // A distinct muzzle/head, lower jaw and paired antler horns.
    let muzzle=add(h,scale(forward,0.82*fade));
    scene.tube(h,muzzle,0.51*fade,0.34*fade,0.2,0xFFFFBA60,6);
    let jaw=add(h,[0.0,-0.29*fade,0.0]);
    shard(scene,jaw,add(muzzle,[0.0,-0.22*fade,0.0]),0.19*fade,EMBER);
    for side in [-1.0,1.0]{
        let horn=add(h,[side*0.31*fade,0.30*fade,-0.10]);
        shard(scene,horn,add(horn,[side*0.24*fade,0.78*fade,0.19]),0.17*fade,LAVA);
        let eye=add(h,[side*0.34*fade,0.12*fade,0.26]);
        shard(scene,eye,add(eye,[side*0.07,0.20,0.18]),0.08*fade,0xFFFFFFA0);
    }
    // Broad flame fins along the articulated spine, not disjoint sparks.
    for j in (2..count-2).step_by(3){
        let center=spine[j];
        let wing=add(center,[0.0,(0.62+0.20*((j as f64)*2.0+spin).sin())*fade,0.0]);
        let tail=spine[(j+2).min(count-1)];
        scene.membrane(center,tail,add(tail,[0.0,0.18,0.0]),wing,
            mutate(0xAFFF6A1B,j));
    }
}
fn ember(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);
    let step=ease((age-8.0)/32.0);
    if grow<0.03||fade<0.05{return;}
    let base=local(c,0.0,1.10,0.3+3.3*step);
    let tip=local(c,0.0,1.10,3.7+5.7*step);
    // A heavy pointed core (not a hair-thin pole) and forged blade fins.
    shard(scene,base,tip,0.70*grow*fade,0xFFFFD36C);
    for j in 0..4{
        let spin=TAU*j as f64/4.0+age*0.014;
        let wing=v3(spin.cos()*0.68*fade,spin.sin()*0.68*fade,0.0);
        let a=local(c,wing[0],1.12+wing[1],0.9+3.3*step);
        let b=local(c,wing[0]*1.75,1.12+wing[1]*1.75,2.5+4.1*step);
        scene.membrane(a,b,tip,base,mutate(0xBFFF8530,j));
    }
    for j in 0..3{
        let back=local(c,(j as f64-1.0)*0.27,1.07,0.25+2.0*step);
        let trail=local(c,(j as f64-1.0)*0.72,1.2+0.35*j as f64,
            -2.3+1.6*step);
        scene.membrane(back,base,tip,trail,0x77FFB24F);
    }
}
fn snowblind(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.05||fade<0.05{return;}
    let t=age*0.063;
    // Three broad 3D twisting blizzard drapes with a volumetric snow core.
    for layer in 0..3{
        let mut curve=Vec::new();
        for i in 0..11{
            let u=i as f64/10.0;
            let a=TAU*(u*1.05+layer as f64/3.0)+t;
            let r=(0.8+2.5*u)*grow;
            curve.push(local(c,r*a.cos(),0.6+3.0*u,r*a.sin()));
        }
        ribbon(scene,&curve,0.52*fade,SNOW,true);
    }
    for j in 0..9{
        let a=TAU*j as f64/9.0+t;
        let radius=(0.6+(j%4) as f64*0.55)*grow;
        let center=local(c,radius*a.cos(),0.9+(j%3) as f64*0.47,radius*a.sin());
        shard(scene,center,
            add(center,[0.6*a.cos()*fade,0.7+(j%3) as f64*0.28,
                0.5*a.sin()*fade]),0.23*fade,mutate(ICE,j));
    }
    let core=local(c,0.0,1.7,0.0);
    for j in 0..4 {
        let a=TAU*j as f64/4.0;
        shard(scene,core,add(core,[1.5*a.cos()*grow,0.65,1.5*a.sin()*grow]),
            0.37*fade,ICE);
    }
}
fn twister(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.03||fade<0.03{return;}
    let spin=age*0.074;
    for arm in 0..4{
        let mut path=Vec::new();
        for i in 0..15{
            let t=i as f64/14.0;
            let a=TAU*(arm as f64/4.0+t*2.15)+spin;
            let radius=(0.21+2.5*t.powf(0.74))*grow;
            path.push(local(c,radius*a.cos(),0.2+4.75*t,radius*a.sin()));
        }
        ribbon(scene,&path,0.36*fade,mutate(WIND,arm),true);
    }
    let bottom=local(c,0.0,0.2,0.0);
    shard(scene,bottom,local(c,0.0,1.15,0.0),0.27*fade,0xD8E2FFC9);
    for j in 0..8{
        let a=TAU*j as f64/8.0+spin;
        let r=(0.7+(j%3) as f64*0.63)*grow;
        let p=local(c,r*a.cos(),0.6+(j%4) as f64*0.5,r*a.sin());
        scene.membrane(p,add(p,[0.35*a.cos(),0.30,0.4*a.sin()]),
            add(p,[0.4*a.cos(),0.8,0.40*a.sin()]),
            add(p,[0.0,0.30,0.0]),LEAF);
    }
}
fn faultline(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.05||fade<0.03{return;}
    // Actual broken earth slab silhouette around a jagged luminous chasm;
    // purely visual. Does NOT deform Minecraft terrain.
    let mut crack=Vec::new();
    for i in 0..15{
        let z=(i as f64-7.0)*0.83*grow;
        let x=0.65*(i as f64*1.65).sin()*grow;
        crack.push(local(c,x,0.14+0.09*((i*7)%3) as f64,z));
    }
    ribbon(scene,&crack,0.27*fade,LAVA,false);
    for j in 0..10{
        let z=(j as f64-4.5)*1.20*grow;
        let x=0.60*(j as f64*1.65).sin()*grow;
        for side in [-1.0,1.0]{
            let edge=local(c,x+side*0.32,0.13,z);
            let outer=local(c,x+side*(1.7+0.4*((j%3) as f64))*grow,
                0.16+(j%4) as f64*0.13*grow,z+0.5*((j%2) as f64));
            let back=local(c,x+side*(1.7+0.4*((j%3) as f64))*grow,
                0.09,z-0.52);
            let corner=local(c,x+side*0.36,0.13,z-0.58);
            scene.membrane(edge,outer,back,corner,
                mutate(STONE,j+(side>0.0) as usize));
        }
        if j%3==0{
            let from=local(c,x+0.4,0.05,z);
            shard(scene,from,local(c,x+0.9,0.85+0.2*grow,z+0.33),0.35*fade,STONE);
        }
    }
}
fn rainweaver(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.04||fade<0.04{return;}
    for strand in 0..4{
        let mut path=Vec::new();
        for i in 0..15{
            let u=i as f64/14.0;
            let a=u*TAU*2.1+TAU*strand as f64/4.0+age*0.033;
            let rad=(1.5+0.45*a.sin())*grow;
            let center=local(c,rad*a.cos(),
                0.6+3.1*(u*PI).sin()+0.22*a.cos(),rad*a.sin()+1.1);
            path.push(center);
        }
        ribbon(scene,&path,0.26*fade,mutate(WATER,strand),true);
    }
    for i in 0..12{
        let a=i as f64*TAU/12.0+age*0.019;
        let from=local(c,1.7*a.cos()*grow,2.1+0.6*a.sin(),1.1+1.7*a.sin()*grow);
        let to=sub(from,[0.0,0.6+0.16*(i%3) as f64,0.0]);
        scene.membrane(from,add(from,[0.10,0.0,0.0]),
            add(to,[0.13,0.0,0.0]),to,FOAM);
    }
}
fn roots(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.06||fade<0.03{return;}
    for arm in 0..6 {
        let a=TAU*arm as f64/6.0;
        let mut path=Vec::new();
        for i in 0..10{
            let u=i as f64/9.0;
            let spiral=a+1.50*u+0.15*(age*0.044+arm as f64).sin();
            let r=(2.65-1.65*u)*grow;
            path.push(local(c,r*spiral.cos(),0.07+4.1*u*grow,r*spiral.sin()));
        }
        connected_body(scene,&path,0.48*fade,mutate(WOOD,arm),6);
        let point=path[9];
        let tip=add(point,[-0.35*a.cos(),0.2,-0.35*a.sin()]);
        shard(scene,point,tip,0.22*fade,SAP);
        let branch=path[6];
        let fork=add(branch,[0.65*a.cos(),0.72+0.1*arm as f64,0.65*a.sin()]);
        scene.tube(branch,fork,0.20*fade,0.07*fade,0.1,WOOD,6);
        scene.membrane(branch,fork,add(fork,[0.26*a.cos(),0.31,0.25*a.sin()]),
            add(branch,[0.15,0.2,0.0]),LEAF);
    }
}
fn arc_net(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.05||fade<0.04{return;}
    let mut nodes=Vec::new();
    for i in 0..6 {
        let a=TAU*i as f64/6.0;
        nodes.push(local(c,2.5*a.cos()*grow,0.7+0.5*(i%2) as f64,
            2.5*a.sin()*grow+0.6));
    }
    let hub=local(c,0.0,2.7*grow,0.6);
    // Actual linked network (spokes + ring + spokes subdivided), unlike
    // unrelated blue ticks. Electricity is intentionally branching.
    for i in 0..6{
        let a=nodes[i];let b=nodes[(i+1)%6];
        for (start,end,seed) in [(a,b,i),(a,hub,i+6)]{
            let mut curve=Vec::new();
            for j in 0..5{
                let t=j as f64/4.0;
                let v=vlerp(start,end,t);
                let k=(i as f64*17.3+j as f64*3.8+age*0.18).sin();
                curve.push(add(v,[0.17*k*grow,0.21*k.sin()*grow,
                    0.12*k.cos()*grow]));
            }
            ribbon(scene,&curve,0.11*fade,mutate(ARC,seed),true);
        }
        shard(scene,a,add(a,[0.0,0.32*fade,0.0]),0.19*fade,PLASMA);
    }
    shard(scene,hub,add(hub,[0.0,0.74,0.0]),0.4*fade,0xFFD1EDFF);
    for i in 0..3 {
        let a=nodes[i*2];let b=nodes[(i*2+1)%6];
        scene.membrane(a,b,hub,add(hub,[0.0,-0.25,0.0]),0x4899BAFF);
    }
}
fn rift_slice(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.05||fade<0.03{return;}
    let mut upper=Vec::new();let mut lower=Vec::new();
    for i in 0..13{
        let t=i as f64/12.0;
        let x=(t-0.5)*7.2*grow;
        let serration=0.25*(t*PI*8.0+age*0.025).sin();
        let arc=1.65*(1.0-(2.0*t-1.0).powi(2));
        upper.push(local(c,x,1.7+arc+serration,0.82+0.32*t));
        lower.push(local(c,x,1.2-arc*0.8-serration*0.5,0.28-0.20*t));
    }
    // TWO physical irregular rims surrounding a folded deep black-purple
    // membrane. A thin 2D screen slash cannot substitute for a void tear.
    let Some(prev)=scene.begin_component(12*6) else{return;};
    for i in 0..12{
        scene.membrane(upper[i],upper[i+1],lower[i+1],lower[i],
            mutate(VOID,i));
    }
    scene.end_component(prev);
    for (edge,col) in [(&upper,RIFT),(&lower,0xCD723CF2)] {
        ribbon(scene,edge,0.27*fade,col,true);
    }
    for i in 0..7{
        let a=(i as f64*1.7+age*0.035)*TAU/7.0;
        let base=local(c,2.3*a.cos()*grow,1.0+(i%3) as f64*0.8,
            0.2+1.1*a.sin()*grow);
        shard(scene,base,add(base,[0.54*a.cos(),0.57,0.6*a.sin()]),0.20*fade,RIFT);
    }
}
fn moment_slip(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.04||fade<0.03{return;}
    let g=(age*0.029).sin();
    // Three time-offset ECHO silhouettes and splintered clock fragments.
    // Each angular hourglass half is its own broad sheet, not a generic ring.
    for j in 0..3 {
        let x=(j as f64-1.0)*1.45+0.20*g;
        let z=1.7+(j as f64)*1.1;
        let y=1.5+0.20*j as f64;
        let top=local(c,x,y+1.2*grow,z);
        let bottom=local(c,x,y-1.1*grow,z);
        let left=local(c,x-0.70*grow,y,z);
        let right=local(c,x+0.70*grow,y,z);
        scene.membrane(top,left,bottom,right,mutate(0x729EACFF,j));
        shard(scene,left,right,0.18*fade,TIME);
    }
    // Twelve time shards have displaced positions rather than a smooth
    // perfect clock ring. Some are frozen, others skip as age advances.
    for i in 0..12 {
        let jump=((age*0.21+i as f64*0.8).floor()*0.7).sin();
        let a=TAU*i as f64/12.0+0.11*jump;
        let r=2.2+0.17*(i%3) as f64;
        let pos=local(c,r*a.cos()*grow,1.8+r*a.sin()*grow,1.4);
        let tip=add(pos,[0.25*a.cos(),0.29+0.17*jump.abs(),0.13]);
        shard(scene,pos,tip,0.15*fade,mutate(TIME,i));
    }
}
fn carrion(c:Cast,age:f64,scene:&mut Scene){
    let (grow,fade)=phase(age);if grow<0.04||fade<0.03{return;}
    let spin=age*0.046;
    // A flock of ACTUAL bat/raven-like silhouettes with body+two swept
    // membranes per creature, not a cloud of magenta pixel dots.
    for i in 0..7{
        let u=i as f64/7.0;
        let a=TAU*(u+0.33*u*u)+spin;
        let r=(1.0+3.0*u)*grow;
        let o=local(c,r*a.cos(),1.1+2.3*u,r*a.sin()+1.0);
        let flap=(spin*3.2+i as f64*1.3).sin();
        let body=add(o,[0.0,0.15,0.0]);
        let beak=add(body,[0.0,0.03,0.66]);
        shard(scene,body,beak,0.21*fade,mutate(GHOST,i));
        for side in [-1.0,1.0]{
            let shoulder=add(body,[side*0.24,0.06,0.0]);
            let wing=add(body,[side*(1.14+0.40*u)*fade,
                0.36+0.35*flap,0.07]);
            let tip=add(body,[side*0.61,0.12-0.24*flap,-0.58]);
            scene.membrane(shoulder,wing,tip,body,mutate(0xA48F4EA7,i));
        }
    }
    let center=local(c,0.0,1.9,1.0);
    for i in 0..4{
        let a=TAU*i as f64/4.0+spin;
        let base=add(center,[1.1*a.cos(),0.15*i as f64,1.1*a.sin()]);
        shard(scene,base,add(base,[0.2*a.cos(),0.68,0.2*a.sin()]),
            0.22*fade,mutate(GHOST,i));
    }
}
/// The true source geometry replaces (does not merely recolor) generic
/// visual bodies for these ten reference spells. ID is zero-based.
pub(super) fn render(c:Cast,age:f64,scene:&mut Scene)->bool{
    let id=c.kind;
    if !matches!(id,0|6|29|47|63|86|101|123|144|161|186){return false;}
    if !age.is_finite()||!(0.0..104.0).contains(&age) ||
        !c.center.iter().chain(c.direction.iter()).all(|x|x.is_finite()){
        return true;
    }
    match id{
        0=>ember(c,age,scene),
        6=>serpent(c,age,scene),
        29=>snowblind(c,age,scene),
        47=>twister(c,age,scene),
        63=>faultline(c,age,scene),
        86=>rainweaver(c,age,scene),
        101=>roots(c,age,scene),
        123=>arc_net(c,age,scene),
        144=>rift_slice(c,age,scene),
        161=>moment_slip(c,age,scene),
        186=>carrion(c,age,scene),
        _=>{}
    }
    true
}
#[cfg(test)]
mod tests{
    use super::*;
    fn cast(id:usize)->Cast{Cast{
        kind:id,variant:0,start:0,
        center:[0.0,65.0,0.0],direction:[0.21,-0.15,0.97]
    }}
    #[test]
    fn source_hero_spells_are_actual_contiguous_3d_volumes_not_only_sprites(){
        let ids=[0,6,29,47,63,86,101,123,144,161,186];
        for id in ids{
            let mut scene=Scene::with_lod(900,false);
            assert!(render(cast(id),48.0,&mut scene));
            assert!(scene.facets.len()>35,"#{id} cannot form a main shape");
            assert_eq!(scene.facets.len(),scene.group_ids.len());
            assert!(scene.facets.len()<=900);
            assert!(scene.facets.iter().all(|(q,_)|
                q.iter().flatten().all(|x|x.is_finite())));
            let mut pts=scene.facets.iter().flat_map(|(q,_)|q.iter());
            let first=*pts.next().unwrap();let mut low=first;let mut high=first;
            for p in pts{
                for k in 0..3{
                    low[k]=low[k].min(p[k]);high[k]=high[k].max(p[k]);
                }
            }
            assert!((high[0]-low[0]).max(high[2]-low[2])>1.4,
                "#{id} too narrow to recognize");
            assert!(high[1]-low[1]>0.65,"#{id} flat 2D squiggle");
            assert!((high[0]-low[0])*(high[1]-low[1])*
                (high[2]-low[2])>1.2,"#{id} no 3D depth");
        }
    }
    #[test]
    fn source_hero_movement_changes_geometry_over_time(){
        for id in [0,6,29,47,63,86,101,123,144,161,186]{
            let mut a=Scene::new(900);let mut b=Scene::new(900);
            assert!(render(cast(id),25.0,&mut a));
            assert!(render(cast(id),63.0,&mut b));
            assert_ne!(a.facets,b.facets,"#{id} has frozen visual body");
        }
    }
    #[test]
    fn unrelated_189_spells_keep_existing_m36_path(){
        for id in 0..200{
            if [0,6,29,47,63,86,101,123,144,161,186].contains(&id){
                continue;
            }
            let mut scene=Scene::new(120);
            assert!(!render(cast(id),48.0,&mut scene));
            assert!(scene.facets.is_empty());
        }
    }
    #[test]
    fn overload_never_punches_random_holes_in_source_geometry(){
        for id in [0,6,29,47,63,86,101,123,144,161,186]{
            for cap in [0,30,80,150,280,500,900]{
                let mut scene=Scene::new(cap);
                assert!(render(cast(id),46.0,&mut scene));
                assert!(scene.facets.len()<=cap);
                assert_eq!(scene.facets.len(),scene.group_ids.len());
                let mut seen=std::collections::BTreeSet::new();
                let mut prev=None;
                for group in scene.group_ids{
                    if prev!=Some(group) {
                        assert!(seen.insert(group),"geometry component split");
                    }
                    prev=Some(group);
                }
            }
        }
    }
}
