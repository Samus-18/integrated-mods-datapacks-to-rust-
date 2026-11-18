#[derive(Debug,Clone,Copy)]
pub struct ParticleBudget{pub max_per_frame:u32,pub distance_falloff:f32}
impl Default for ParticleBudget{
 fn default()->Self{Self{max_per_frame:1200,distance_falloff:96.0}}
}
impl ParticleBudget{
 pub fn allowed(&self,requested:u32,distance:f32)->u32{
    let amount=(1.0-distance.max(0.0)/self.distance_falloff).clamp(0.0,1.0);
    requested.min(self.max_per_frame).min((self.max_per_frame as f32*amount) as u32)
 }
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn far_effects_stop_emitting(){
    let b=ParticleBudget::default();
    assert_eq!(b.allowed(100,0.0),100);
    assert_eq!(b.allowed(10000,999.0),0);
 }
}
