//! CPU procedural meshes; upload through a verified Horizon render API later.
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vertex{pub position:[f32;3],pub uv:[f32;2]}
#[derive(Debug,Default)]
pub struct Mesh{pub vertices:Vec<Vertex>,pub indices:Vec<u32>}
pub fn rune_ring(inner:f32,outer:f32,segments:u32)->Mesh {
 assert!(inner>0.0&&outer>inner&&segments>=3&&segments<=4096);
 let mut mesh=Mesh::default();
 for i in 0..=segments{
    let angle=i as f32*std::f32::consts::TAU/segments as f32;
    let (s,c)=angle.sin_cos();
    for (radius,v) in [(inner,0.0),(outer,1.0)]{
        mesh.vertices.push(Vertex{position:[radius*c,0.0,radius*s],
             uv:[i as f32/segments as f32,v]});
    }
    if i<segments {let k=i*2;mesh.indices.extend_from_slice(&[k,k+1,k+2,k+1,k+3,k+2]);}
 }
 mesh
}
pub fn crystal_spike(height:f32,radius:f32,sides:u32)->Mesh{
 assert!(height>0.0&&radius>0.0&&sides>=3&&sides<=256);
 let mut m=Mesh::default();
 m.vertices.push(Vertex{position:[0.0,height,0.0],uv:[0.5,0.0]});
 for i in 0..sides{
    let angle=i as f32*std::f32::consts::TAU/sides as f32;
    let (s,c)=angle.sin_cos();
    m.vertices.push(Vertex{position:[radius*c,0.0,radius*s],uv:[i as f32/sides as f32,1.0]});
 }
 for i in 0..sides {m.indices.extend_from_slice(&[0,i+1,(i+1)%sides+1]);}
 m
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn ring_is_closed(){let m=rune_ring(1.0,2.0,32);assert_eq!(m.vertices.len(),66);assert_eq!(m.indices.len(),192);}
 #[test]fn spike_is_3d(){let m=crystal_spike(4.0,1.0,6);assert_eq!(m.indices.len(),18);assert_eq!(m.vertices[0].position[1],4.0);}
}


/// A **closed** faceted ice volume: base cap + six-sided body + tapered tip.
/// Unlike paired bright billboards, it is a true manifold triangle mesh;
/// an engine adapter can upload the vertex/index buffers to a 3D pipeline.
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CrystalForm {
    pub height:f32,
    pub radius:f32,
    pub sides:u8,
    pub lean:[f32;2],
    pub twist:f32,
}
pub fn faceted_crystal(form:CrystalForm)->Option<Mesh> {
    let CrystalForm {height,radius,sides,lean,twist}=form;
    let n=usize::from(sides);
    if !height.is_finite()||!radius.is_finite()||!twist.is_finite()
       || !lean.iter().all(|v|v.is_finite())
       || height<=0.01||radius<=0.01||!(5..=8).contains(&n) {return None;}
    let mut mesh=Mesh::default();
    let shoulder=[lean[0]*0.58,height*0.58,lean[1]*0.58];
    let tip=[lean[0],height,lean[1]];
    mesh.vertices.push(Vertex{position:[0.0,0.0,0.0],uv:[0.5,0.5]}); // 0 cap centre
    for ring in 0..2 {
        let factor=if ring==0 {1.0}else{0.71};
        let (cx,cy,cz)=if ring==0 {(0.0,0.0,0.0)}
                       else {(shoulder[0],shoulder[1],shoulder[2])};
        for i in 0..n {
            let a=(i as f32)*(std::f32::consts::TAU/n as f32)+twist;
            let (s,c)=a.sin_cos();
            mesh.vertices.push(Vertex{
                position:[cx+radius*factor*c,cy,cz+radius*factor*s],
                uv:[i as f32/n as f32, ring as f32*0.58],
            });
        }
    }
    let apex=mesh.vertices.len() as u32;
    mesh.vertices.push(Vertex{position:tip,uv:[0.5,1.0]});
    for i in 0..n {
        let a=1+i as u32;
        let b=1+((i+1)%n) as u32;
        let c=1+n as u32+i as u32;
        let d=1+n as u32+((i+1)%n) as u32;
        // Horizontal cap normal faces down, sides face out, top points to tip.
        mesh.indices.extend_from_slice(&[0,a,b, a,c,b, b,c,d, c,apex,d]);
    }
    Some(mesh)
}
/// Emits nothing before a delayed wave reaches the crystal; grows as a solid,
/// with its side radius expanding along with the height.
pub fn crystal_rising(form:CrystalForm, age_ticks:f32, delay_ticks:f32,
                      grow_ticks:f32)->Option<Mesh> {
    if !age_ticks.is_finite()||!delay_ticks.is_finite()
       ||!grow_ticks.is_finite()||grow_ticks<=0.0{return None;}
    let t=((age_ticks-delay_ticks)/grow_ticks).clamp(0.0,1.0);
    if t<=0.0 {return None;}
    let ease=t*t*(3.0-2.0*t);
    faceted_crystal(CrystalForm{
        height:form.height*ease,
        radius:form.radius*ease.max(0.06),
        lean:[form.lean[0]*ease,form.lean[1]*ease],
        ..form
    })
}
#[cfg(test)]
mod manifold_tests {
    use super::*;
    use std::collections::BTreeMap;
    fn form(sides:u8)->CrystalForm {
        CrystalForm {height:7.2,radius:1.5,sides,
                     lean:[0.8,-0.35],twist:0.2}
    }
    #[test] fn every_facet_is_a_non_degenerate_closed_triangle_mesh() {
        for sides in 5..=8 {
            let mesh=faceted_crystal(form(sides)).unwrap();
            assert_eq!(mesh.vertices.len(),2*sides as usize+2);
            assert_eq!(mesh.indices.len(),12*sides as usize);
            assert!(mesh.indices.iter().all(|&i|(i as usize)<mesh.vertices.len()));
            let mut edges=BTreeMap::<(u32,u32),u8>::new();
            for tri in mesh.indices.chunks_exact(3){
                for (a,b) in [(tri[0],tri[1]),(tri[1],tri[2]),(tri[2],tri[0])] {
                    let pair=if a<b {(a,b)}else{(b,a)};
                    *edges.entry(pair).or_default()+=1;
                }
            }
            assert!(edges.values().all(|&n|n==2),"manifold side count");
            // 3D geometry occupies x/y/z dimensions, not a 2D sheet.
            for axis in 0..3 {
                let a=mesh.vertices.iter().map(|v|v.position[axis])
                     .fold(f32::INFINITY,f32::min);
                let b=mesh.vertices.iter().map(|v|v.position[axis])
                     .fold(f32::NEG_INFINITY,f32::max);
                assert!(b-a>0.25);
            }
        }
    }
    #[test] fn invalid_and_delayed_spikes_never_emit() {
        assert!(faceted_crystal(CrystalForm{height:f32::NAN,..form(6)}).is_none());
        assert!(faceted_crystal(form(2)).is_none());
        assert!(crystal_rising(form(6),5.0,8.0,12.0).is_none());
        let mesh=crystal_rising(form(6),20.0,8.0,12.0).unwrap();
        assert!(mesh.vertices.iter().any(|v|v.position[1]>=7.2));
    }
}
