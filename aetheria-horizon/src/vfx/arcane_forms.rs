//! Original fantasy-only **volume geometry** prototypes for the ten arcane schools.
//!
//! These are real, closed, indexed 3D surfaces (crystals, limbs, plumes, membranes),
//! NOT billboards or color-swapped beams. Their geometry can be uploaded by a
//! future Horizon renderer adapter, but this portable module does not issue GPU
//! calls, mutate blocks or pretend to implement server-side gameplay.
//!
//! Each school has a hand-authored spatial composition and its own time evolution.
//! This is a sculpting foundation, not ten completed spells.
use std::f32::consts::{PI, TAU};
use super::geometry::{CrystalForm, Mesh, Vertex, faceted_crystal};

const EPS: f32 = 0.0001;
type Vec3 = [f32;3];

fn add(a:Vec3,b:Vec3)->Vec3 {[a[0]+b[0],a[1]+b[1],a[2]+b[2]]}
fn sub(a:Vec3,b:Vec3)->Vec3 {[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn mul(a:Vec3,f:f32)->Vec3 {[a[0]*f,a[1]*f,a[2]*f]}
fn dot(a:Vec3,b:Vec3)->f32 {a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn cross(a:Vec3,b:Vec3)->Vec3 {
    [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
}
fn magnitude(a:Vec3)->f32 {dot(a,a).sqrt()}
fn unit(a:Vec3)->Vec3 {let d=magnitude(a); if d<EPS {[0.,1.,0.]}else{mul(a,1./d)}}
fn lerp(a:Vec3,b:Vec3,t:f32)->Vec3 {add(mul(a,1.-t),mul(b,t))}
fn clamp01(t:f32)->f32 {t.clamp(0.,1.)}
fn smooth(t:f32)->f32 {let t=clamp01(t);t*t*(3.-2.*t)}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum School {
    Pyro,Cryo,Aero,Geo,Hydro,Druid,Electro,Astral,Chrono,Necro
}
impl School {
    pub fn from_id(class_id:u8)->Option<Self> {
        Some(match class_id {
            0=>Self::Pyro, 1=>Self::Cryo,2=>Self::Aero,3=>Self::Geo,
            4=>Self::Hydro,5=>Self::Druid,6=>Self::Electro,7=>Self::Astral,
            8=>Self::Chrono,9=>Self::Necro,_=>return None
        })
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum GeometryIdentity {
    PhoenixFeathers, GlacialEruption, TempestVortex, MountainGauntlet,
    TidalLeviathan, LivingWorldroot, StormWyvern, AstralShatter,
    SandsOfTime, AncestralRibcage
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Material {
    Ember, Ice, Wind, Stone, Water, Wood, Leaf, Lightning,
    Rift, Starlight, Brass, Bone, Soul
}
#[derive(Debug)]
pub struct FormPart {
    pub mesh:Mesh,
    pub material:Material,
    pub opacity:f32,
    pub emissive:f32,
}
#[derive(Debug)]
pub struct ArcaneForm {
    pub identity:GeometryIdentity,
    pub parts:Vec<FormPart>,
    pub age_ticks:u32,
}
impl ArcaneForm {
    fn push(&mut self,mesh:Mesh,material:Material,opacity:f32,emissive:f32) {
        // Guard against corrupted geometry and accidentally unbounded effects.
        if mesh.vertices.len()<4 || mesh.indices.len()<6 || self.parts.len()>=180 {return;}
        self.parts.push(FormPart {mesh,material,
            opacity:opacity.clamp(0.,1.),emissive:emissive.clamp(0.,3.0)});
    }
    pub fn triangle_count(&self)->usize {
        self.parts.iter().map(|x|x.mesh.indices.len()/3).sum()
    }
}
fn transform(mesh:&mut Mesh,offset:Vec3) {
    for v in &mut mesh.vertices {v.position=add(v.position,offset);}
}
fn crystal(height:f32,radius:f32,offset:Vec3,lean:[f32;2],spin:f32)->Option<Mesh> {
    let mut mesh=faceted_crystal(CrystalForm {
        height,radius,sides:6,lean,twist:spin
    })?;
    transform(&mut mesh,offset);
    Some(mesh)
}
/// Closed spherical form: two poles and longitude rings, topologically a sphere.
fn globe(at:Vec3,radius:Vec3,longitude:usize,latitude:usize)->Mesh {
    let n=longitude.clamp(5,20);
    let l=latitude.clamp(2,10);
    let mut m=Mesh::default();
    m.vertices.push(Vertex{position:add(at,[0.,radius[1],0.]),uv:[0.5,0.]});
    for ring in 1..l {
        let v=ring as f32/l as f32;
        let polar=PI*v;
        for j in 0..n {
            let u=j as f32/n as f32;
            let az=u*TAU;
            let (s,c)=az.sin_cos();
            m.vertices.push(Vertex {
                position:add(at,[
                    radius[0]*polar.sin()*c,radius[1]*polar.cos(),
                    radius[2]*polar.sin()*s
                ]),uv:[u,v]
            });
        }
    }
    let south=m.vertices.len() as u32;
    m.vertices.push(Vertex{position:add(at,[0.,-radius[1],0.]),uv:[0.5,1.]});
    for j in 0..n {
        let a=1+j as u32;
        let b=1+((j+1)%n) as u32;
        m.indices.extend_from_slice(&[0,b,a]);
    }
    for ring in 0..l.saturating_sub(2) {
        let base=1+(ring*n) as u32;
        let next=base+n as u32;
        for j in 0..n {
            let a=base+j as u32;
            let b=base+((j+1)%n) as u32;
            let c=next+j as u32;
            let d=next+((j+1)%n) as u32;
            m.indices.extend_from_slice(&[a,b,c,b,d,c]);
        }
    }
    let base=1+((l-2)*n) as u32;
    for j in 0..n {
        m.indices.extend_from_slice(&[
            south, base+j as u32,base+((j+1)%n) as u32
        ]);
    }
    m
}
/// Closed, faceted, curved anatomical tube. The caps are explicit and
/// consecutive rings connect with two triangles per side.
fn limb(path:&[Vec3],widths:&[f32],sides:usize)->Option<Mesh> {
    let n=sides.clamp(5,10);
    if path.len()<2 || path.len()!=widths.len()
       || path.len()>96 || !path.iter().flatten().all(|v|v.is_finite())
       || !widths.iter().all(|&r|r.is_finite() && r>EPS) {return None;}
    if path.windows(2).any(|w|magnitude(sub(w[1],w[0]))<EPS) {return None;}
    let mut m=Mesh::default();
    m.vertices.push(Vertex{position:path[0],uv:[0.5,0.]});
    for i in 0..path.len() {
        let tangent=unit(if i==0 {sub(path[1],path[0])}
            else if i+1==path.len(){sub(path[i],path[i-1])}
            else {sub(path[i+1],path[i-1])});
        let guide=if tangent[1].abs()>0.92 {[1.,0.,0.]}else{[0.,1.,0.]};
        let sideways=unit(cross(tangent,guide));
        let up=unit(cross(sideways,tangent));
        for j in 0..n {
            let a=TAU*j as f32/n as f32;
            let p=add(path[i],add(
                mul(sideways,widths[i]*a.cos()),
                mul(up,widths[i]*a.sin())
            ));
            m.vertices.push(Vertex{position:p,uv:[j as f32/n as f32,
                i as f32/(path.len()-1) as f32]});
        }
    }
    let end=m.vertices.len() as u32;
    m.vertices.push(Vertex{position:*path.last()?,uv:[0.5,1.]});
    for j in 0..n {
        m.indices.extend_from_slice(&[
            0,1+j as u32,1+((j+1)%n) as u32
        ]);
    }
    for i in 0..path.len()-1 {
        let a=1+(i*n) as u32;
        let b=1+((i+1)*n) as u32;
        for j in 0..n {
            let a0=a+j as u32;
            let a1=a+((j+1)%n) as u32;
            let b0=b+j as u32;
            let b1=b+((j+1)%n) as u32;
            m.indices.extend_from_slice(&[a0,a1,b0,a1,b1,b0]);
        }
    }
    let last=1+(path.len()-1)*n;
    for j in 0..n {
        m.indices.extend_from_slice(&[
            end,last as u32+((j+1)%n) as u32,last as u32+j as u32
        ]);
    }
    Some(m)
}
/// A real-thickness wing/leaf: curved diamond top, bottom and side walls.
/// 10 vertices, 16 triangles, all perimeter edges closed.
fn feather(root:Vec3,tip:Vec3,width:f32,bulge:f32)->Option<Mesh> {
    if !(width>0.02 && width.is_finite() && bulge.is_finite()) {return None;}
    let axis=sub(tip,root);
    if magnitude(axis)<EPS{return None;}
    let side=unit(cross(unit(axis),if axis[1].abs()>0.9 {[0.,0.,1.]}else{[0.,1.,0.]}));
    let middle=lerp(root,tip,0.52);
    let boundary=[
        root,
        add(middle,mul(side,width)),
        tip,
        sub(middle,mul(side,width)),
    ];
    let thickness=width*0.16;
    let mut mesh=Mesh::default();
    for dir in [1.,-1.]{
        for p in boundary {
            mesh.vertices.push(Vertex {
                position:add(p,[0.,dir*thickness,bulge*0.15]),
                uv:[0.5+dir*0.5,0.5]
            });
        }
    }
    mesh.vertices.push(Vertex {
        position:add(middle,[0.,thickness*1.8,bulge]),
        uv:[0.5,0.5]
    });
    mesh.vertices.push(Vertex {
        position:add(middle,[0.,-thickness*1.8,-bulge]),
        uv:[0.5,0.5]
    });
    for i in 0..4u32 {
        let j=(i+1)%4;
        mesh.indices.extend_from_slice(&[
            8,i,j, 9,4+j,4+i, i,4+i,j, j,4+i,4+j
        ]);
    }
    Some(mesh)
}
fn add_limb(form:&mut ArcaneForm,pts:&[Vec3],r:&[f32],mat:Material,alpha:f32,glow:f32){
    if let Some(m)=limb(pts,r,6){form.push(m,mat,alpha,glow)}
}
fn add_crystal(form:&mut ArcaneForm,h:f32,r:f32,origin:Vec3,lean:[f32;2],
               phase:f32,mat:Material,alpha:f32,glow:f32){
    let f=smooth(phase);
    if f>0.01 {
        if let Some(m)=crystal(h*f,r*f.max(0.06),origin,
                              [lean[0]*f,lean[1]*f],h*0.13){
            form.push(m,mat,alpha,glow);
        }
    }
}
fn add_feather(form:&mut ArcaneForm,root:Vec3,tip:Vec3,w:f32,b:f32,
               mat:Material,opacity:f32,glow:f32){
    if let Some(mesh)=feather(root,tip,w,b){form.push(mesh,mat,opacity,glow)}
}

/// Ten hand-authored school shapes; stage and origin are determined by the
/// caller. No palette swaps or beam/quadrilateral-only output.
pub fn sculpt_school(class_id:u8,elapsed_ticks:u32,quality:u8)->Option<ArcaneForm> {
    let school=School::from_id(class_id)?;
    // 0..120 ticks; ease in, impact at ~40t, dissolve after ~80t.
    if elapsed_ticks>=120 {return None;}
    let grow=smooth(elapsed_ticks as f32/38.);
    let impact=smooth((elapsed_ticks as f32-30.)/19.);
    let fade=1.-smooth((elapsed_ticks as f32-83.)/37.);
    let flutter=(elapsed_ticks as f32*0.07).sin();
    let q=quality.clamp(1,3) as usize;
    let mut form=ArcaneForm {
        identity:match school {
            School::Pyro=>GeometryIdentity::PhoenixFeathers,
            School::Cryo=>GeometryIdentity::GlacialEruption,
            School::Aero=>GeometryIdentity::TempestVortex,
            School::Geo=>GeometryIdentity::MountainGauntlet,
            School::Hydro=>GeometryIdentity::TidalLeviathan,
            School::Druid=>GeometryIdentity::LivingWorldroot,
            School::Electro=>GeometryIdentity::StormWyvern,
            School::Astral=>GeometryIdentity::AstralShatter,
            School::Chrono=>GeometryIdentity::SandsOfTime,
            School::Necro=>GeometryIdentity::AncestralRibcage,
        },
        parts:Vec::new(),age_ticks:elapsed_ticks
    };
    let pulse=0.95+0.05*flutter;
    match school {
        School::Pyro=>{
            // Two enormous flame-feather wings, 3D body and flowing tail;
            // feather bones unfurl from a shared living shoulder joint.
            form.push(globe([0.,2.3*grow,0.],[0.47,0.75,0.45],12,6),
                      Material::Ember,fade,1.2);
            for side in [-1f32,1.] {
                for i in 0..(5*q) {
                    let u=(i as f32+1.)/(5*q) as f32;
                    let angle=PI*u*0.86;
                    let root=[side*0.38,2.5*grow,0.];
                    let wing=[side*(1.0+5.3*u)*grow,
                        (2.2+4.*angle.sin()+flutter*0.3*u)*grow,
                        -1.6*u*u];
                    add_feather(&mut form,root,wing,0.18+0.54*(1.-u),
                                0.1+0.5*u,Material::Ember,fade*0.91,1.1);
                }
            }
            for tail in 0..5 {
                let s=tail as f32-2.;
                add_feather(&mut form,[s*0.13,1.9*grow,0.],
                    [s*0.32,(-1.8-1.3*(s*1.4).cos())*grow,-0.65],
                    0.24,0.17,Material::Ember,fade,0.9);
            }
        }
        School::Cryo=>{
            // Ground-bound *solid* serrated ice storm, delayed rows.
            for row in 0..(4*q) {
                let u=row as f32/(4*q) as f32;
                for col in 0..5 {
                    let s=col as f32-2.;
                    let wave=(elapsed_ticks as f32-(row as f32*3.+col as f32*0.65))/20.;
                    let p=smooth(wave);
                    let h=2.+5.5*(1.-u)*pulse+0.6*(row as f32*1.7+s).cos().abs();
                    add_crystal(&mut form,h,0.26+0.34*(1.-u),
                                [s*(0.7+u),-0.32,row as f32*1.35-2.],
                                [0.1*s,0.5*(row as f32*2.1+s).sin()],
                                p,Material::Ice,fade,0.35);
                }
            }
        }
        School::Aero=>{
            // Twisting atmospheric pressure column: three independent
            // thick helices with divergent sails, no rings or laser beams.
            for helix in 0..3 {
                let mut path=Vec::new();
                let mut widths=Vec::new();
                let turn=helix as f32*TAU/3.;
                for k in 0..=18 {
                    let t=k as f32/18.;
                    let ang=turn+5.*PI*t+elapsed_ticks as f32*0.02;
                    let radius=(0.45+2.8*t)*grow;
                    path.push([ang.cos()*radius,0.4+6.1*t*grow,ang.sin()*radius]);
                    widths.push(0.16+0.32*t);
                }
                add_limb(&mut form,&path,&widths,Material::Wind,fade*0.74,0.3);
            }
            for i in 0..8 {
                let a=TAU*i as f32/8.+elapsed_ticks as f32*0.028;
                add_feather(&mut form,[a.cos()*0.6,2.*grow,a.sin()*0.6],
                    [a.cos()*3.3*grow,3.3*grow,a.sin()*3.3*grow],
                    0.23,0.4,Material::Wind,fade*0.58,0.28);
            }
        }
        School::Geo=>{
            // Articulated stone hand emerging from terrain, separate palm,
            // blocky knuckles/fingers rather than a stone-colored sphere.
            form.push(globe([0.,2.*grow,0.],[1.1,1.2,0.64],9,4),
                      Material::Stone,fade,0.08);
            for finger in 0..5 {
                let x=finger as f32-2.;
                let base=[x*0.4,2.55*grow,0.2];
                let middle=[x*0.48,3.55*grow,-0.3+0.3*(finger as f32).sin()];
                let end=[x*0.37,4.05*grow,-0.7];
                let widths=[0.29,0.31,0.20];
                add_limb(&mut form,&[base,middle,end],&widths,
                         Material::Stone,fade,0.07);
                form.push(globe(middle,[0.36,0.34,0.31],8,4),
                          Material::Stone,fade,0.07);
            }
            for i in 0..4{
                let a=i as f32*TAU/4.;
                add_crystal(&mut form,1.2,0.26,
                    [2.0*a.cos(),-0.35,2.0*a.sin()],[0.3*a.cos(),0.2*a.sin()],
                    grow,Material::Stone,fade,0.1);
            }
        }
        School::Hydro=>{
            // Water dragon: thick serpentine body, fish-like fin crests, jaw,
            // paired swept fins and a pair of living water horns.
            let mut path=Vec::new();
            let mut widths=Vec::new();
            for k in 0..=25 {
                let t=k as f32/25.;
                path.push([
                    (3.2*t+elapsed_ticks as f32*0.014).sin()*(1.4+0.4*t)*grow,
                    (1.4+2.5*t+0.5*(9.*t+elapsed_ticks as f32*0.02).sin())*grow,
                    (8.*t-3.)*grow
                ]);
                widths.push((0.48*(1.-t)+0.12)*grow.max(0.08));
            }
            add_limb(&mut form,&path,&widths,Material::Water,fade*0.79,0.45);
            for k in (4..25).step_by(3) {
                let t=k as f32/25.;
                let base=path[k];
                add_feather(&mut form,base,
                    add(base,[0.5*(t*12.).sin(),(0.8+0.8*t)*grow,-0.65]),
                    0.34,0.25,Material::Water,fade*0.7,0.4);
            }
            let head=*path.last().unwrap();
            form.push(globe(head,[0.76,0.58,0.92],12,6),
                      Material::Water,fade*0.75,0.45);
            for side in [-1f32,1.] {
                add_feather(&mut form,
                    add(head,[side*0.45,0.25,0.]),
                    add(head,[side*1.2,1.2,0.8]),0.22,0.12,
                    Material::Water,fade*0.8,0.5);
            }
        }
        School::Druid=>{
            // Living roots and unfolding leaves, not uniformly lit green arcs.
            let trunk=[
                [0.,-0.55,0.],[0.2,1.0*grow,0.1],
                [-0.2,2.5*grow,0.1],[0.0,4.3*grow,-0.25]
            ];
            add_limb(&mut form,&trunk,&[0.67,0.58,0.45,0.25],
                     Material::Wood,fade,0.05);
            for branch in 0..(4*q) {
                let ang=TAU*branch as f32/(4*q) as f32;
                let lift=2.6+(branch%4) as f32*0.45;
                let mid=[ang.cos()*1.6*grow,lift*grow,ang.sin()*1.6*grow];
                let end=[ang.cos()*3.0*grow,(lift+1.)*grow,
                         ang.sin()*3.0*grow];
                add_limb(&mut form,&[[0.,1.7*grow,0.],mid,end],
                         &[0.29,0.18,0.055],Material::Wood,fade,0.05);
                for leaf in 0..4 {
                    let spin=ang+leaf as f32*TAU/4.;
                    add_feather(&mut form,mid,
                        add(end,[spin.cos()*0.76,(0.1+0.4*flutter)*grow,
                                 spin.sin()*0.76]),
                        0.36,0.15,Material::Leaf,fade*0.9,0.14);
                }
            }
            for root in 0..8 {
                let a=TAU*root as f32/8.;
                add_limb(&mut form,&[
                    [0.,-0.2,0.],[a.cos()*1.,-0.55,a.sin()*1.],
                    [a.cos()*2.8,-0.4,a.sin()*2.8]
                ],&[0.27,0.2,0.07],Material::Wood,fade,0.04);
            }
        }
        School::Electro=>{
            // An actual wyvern silhouette built from coiling vertebrae,
            // two multi-segment wings and talons.
            let mut path=Vec::new();
            let mut widths=Vec::new();
            for i in 0..=18 {
                let t=i as f32/18.;
                path.push([0.55*(t*9.+elapsed_ticks as f32*0.09).sin(),
                           1.1+2.8*t*grow,4.*t-2.2]);
                widths.push(0.31*(1.-t)+0.09);
            }
            add_limb(&mut form,&path,&widths,Material::Lightning,fade,1.3);
            for side in [-1f32,1.]{
                let shoulder=[side*0.25,2.7*grow,0.3];
                for i in 0..5 {
                    let u=(i as f32+1.)/5.;
                    let tip=[side*(1.+4.9*u)*grow,
                             (3.6+1.2*(PI*u).sin())*grow,
                             -1.4*u];
                    add_feather(&mut form,shoulder,tip,0.24+0.48*(1.-u),
                                0.23,Material::Lightning,fade*0.83,0.8);
                }
            }
            for claw in [-1f32,1.]{
                let base=[claw*0.42,1.7*grow,0.4];
                add_limb(&mut form,&[
                    base,add(base,[0.3*claw,-0.55,-0.1]),
                    add(base,[0.6*claw,-0.95,-0.5])
                ],&[0.19,0.15,0.025],Material::Lightning,fade,0.85);
            }
        }
        School::Astral=>{
            // A split, staggered *physical* folded rift with asymmetrical
            // jagged polyhedra and stars, no circular flat portal disc.
            form.push(globe([0.,2.3*grow,0.],[0.67,1.5,0.38],12,7),
                      Material::Rift,fade*0.73,0.25);
            for i in 0..(5*q) {
                let u=i as f32/(5*q) as f32;
                let ang=u*TAU*1.65+elapsed_ticks as f32*0.014;
                let radius=2.0+3.7*u;
                let pos=[
                    ang.cos()*radius*grow,
                    (1.+4.0*u)*grow,
                    ang.sin()*0.9
                ];
                add_crystal(&mut form,0.95+3.2*u,0.15+0.4*(1.-u),
                            pos,[ang.cos()*1.4,ang.sin()*0.7],
                            grow,Material::Rift,fade*0.85,0.66);
                if i%3==0 {
                    add_feather(&mut form,pos,
                        add(pos,[ang.cos()*1.3,-0.8,ang.sin()*1.2]),
                        0.27,0.14,Material::Starlight,fade*0.6,0.55);
                }
            }
        }
        School::Chrono=>{
            // Hovering 3D hourglass: top/bottom bulbs, narrow flowing sand,
            // orbiting curved brass hands. Time is embodied as an object.
            form.push(globe([0.,3.45*grow,0.],[1.3,1.25,1.3],14,7),
                      Material::Brass,fade*0.88,0.15);
            form.push(globe([0.,0.75*grow,0.],[1.2,1.15,1.2],14,7),
                      Material::Brass,fade*0.85,0.16);
            let mut stream=Vec::new();
            let mut widths=Vec::new();
            for i in 0..=22 {
                let t=i as f32/22.;
                let angle=TAU*t*3.+elapsed_ticks as f32*0.024;
                stream.push([
                    angle.cos()*(0.15+0.6*t),4.0*grow-3.6*t,
                    angle.sin()*(0.15+0.6*t)
                ]);
                widths.push(0.065+0.03*t);
            }
            add_limb(&mut form,&stream,&widths,Material::Starlight,fade*0.77,0.45);
            for hand in 0..4 {
                let a=TAU*hand as f32/4.+elapsed_ticks as f32*0.02;
                let base=[a.cos()*0.7,2.1*grow,a.sin()*0.7];
                let end=[a.cos()*2.0,(1.7+0.3*(hand as f32).sin())*grow,a.sin()*2.0];
                add_limb(&mut form,&[base,end],&[0.14,0.07],
                         Material::Brass,fade,0.25);
            }
        }
        School::Necro=>{
            // Living rib cage with two curved rows of ribs and a skull, all
            // solid bone, animated to close on target rather than glowing lines.
            add_limb(&mut form,&[
                [0.,-0.3,0.],[0.,1.1*grow,0.],[0.,4.0*grow,0.]
            ],&[0.26,0.29,0.22],Material::Bone,fade,0.11);
            for side in [-1f32,1.] {
                for rib in 0..(3*q) {
                    let u=rib as f32/(3*q) as f32;
                    let y=0.8+3.1*u;
                    let curl=1.3+0.25*(rib as f32).sin().abs();
                    let points=[
                        [0.,y*grow,0.],
                        [side*curl*0.9,y*grow+0.23,-0.6],
                        [side*curl*1.2,y*grow-0.2,-1.6],
                        [side*0.4,y*grow-0.5+impact*0.2,-1.9]
                    ];
                    add_limb(&mut form,&points,&[0.18,0.16,0.12,0.05],
                             Material::Bone,fade,0.12);
                }
            }
            form.push(globe([0.,4.7*grow,0.],[0.68,0.84,0.67],12,7),
                      Material::Bone,fade,0.13);
            for i in 0..4 {
                let s=i as f32-1.5;
                add_crystal(&mut form,0.48,0.12,
                    [s*0.21,4.05*grow,-0.42],
                    [0.,-0.26],grow,Material::Bone,fade,0.08);
            }
        }
    }
    if form.parts.is_empty(){return None;}
    Some(form)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap,HashSet};
    fn unique_edges(mesh:&Mesh)->bool {
        let mut edges=BTreeMap::<(u32,u32),u8>::new();
        for tri in mesh.indices.chunks_exact(3) {
            for (a,b) in [(tri[0],tri[1]),(tri[1],tri[2]),(tri[2],tri[0])] {
                let edge=(a.min(b),a.max(b));
                *edges.entry(edge).or_insert(0)+=1;
            }
        }
        edges.values().all(|&count|count==2)
    }
    #[test]
    fn every_school_has_real_watertight_volumes_not_billboards() {
        let mut identities=HashSet::new();
        for class in 0..10u8 {
            let form=sculpt_school(class,52,2).expect("school sculpt missing");
            assert!(identities.insert(form.identity));
            assert!(form.parts.len()>=5,"school {class} too few shapes");
            assert!(form.triangle_count()>=160,"school {class} too flat");
            assert!(form.triangle_count()<=20_000,"school {class} unbounded");
            for part in &form.parts {
                assert!(unique_edges(&part.mesh),"school {class} has an open mesh");
                assert_eq!(part.mesh.indices.len()%3,0);
                assert!(part.mesh.indices.iter().all(|&i|
                    (i as usize)<part.mesh.vertices.len()));
                assert!(part.mesh.vertices.iter().all(|v|
                    v.position.iter().chain(v.uv.iter()).all(|n|n.is_finite())));
                let mut min=[f32::INFINITY;3];
                let mut max=[f32::NEG_INFINITY;3];
                for v in &part.mesh.vertices {
                    for i in 0..3 {
                        min[i]=min[i].min(v.position[i]);
                        max[i]=max[i].max(v.position[i]);
                    }
                }
                assert!((0..3).all(|i|max[i]-min[i]>EPS),
                    "school {class} part is flat");
            }
        }
        assert!(School::from_id(10).is_none());
    }
    #[test]
    fn animation_has_temporal_growth_and_bounded_lifetime() {
        for class in 0..10 {
            let early=sculpt_school(class,6,2).expect("no telegraph");
            let peak=sculpt_school(class,54,2).unwrap();
            assert!(peak.triangle_count()>0);
            assert!(early.triangle_count()>0);
            let early_min=early.parts.iter().flat_map(|p|p.mesh.vertices.iter())
               .map(|v|v.position[1]).fold(f32::INFINITY,f32::min);
            let early_max=early.parts.iter().flat_map(|p|p.mesh.vertices.iter())
               .map(|v|v.position[1]).fold(f32::NEG_INFINITY,f32::max);
            let peak_min=peak.parts.iter().flat_map(|p|p.mesh.vertices.iter())
               .map(|v|v.position[1]).fold(f32::INFINITY,f32::min);
            let peak_max=peak.parts.iter().flat_map(|p|p.mesh.vertices.iter())
               .map(|v|v.position[1]).fold(f32::NEG_INFINITY,f32::max);
            // Some classes expand their silhouette rather than increase in part count.
            assert!((peak_max-peak_min)>(early_max-early_min)*1.12,
                "school {class} is not growing");
            assert!(sculpt_school(class,120,2).is_none());
            assert!(sculpt_school(class,9999,2).is_none());
        }
    }
    #[test]
    fn quality_and_counts_are_safe_and_deterministic(){
        for class in 0..10 {
            let a=sculpt_school(class,52,1).unwrap();
            let b=sculpt_school(class,52,3).unwrap();
            let c=sculpt_school(class,52,3).unwrap();
            assert!(a.triangle_count()<=b.triangle_count());
            assert_eq!(b.triangle_count(),c.triangle_count());
            assert!(b.parts.len()<=180);
        }
        assert!(limb(&[],&[],6).is_none());
        assert!(limb(&[[0.,0.,0.],[0.,0.,0.]],&[1.,1.],6).is_none());
    }
}
