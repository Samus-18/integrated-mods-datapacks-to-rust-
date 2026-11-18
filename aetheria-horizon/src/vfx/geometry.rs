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
