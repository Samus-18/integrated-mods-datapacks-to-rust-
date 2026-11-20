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
