//! Abstract VFX settings, NOT a hooked wgpu pipeline.
#[derive(Debug,Clone,Copy)]
pub struct VisualParams{
 pub emissive:f32,pub distortion:f32,pub particles:u32,pub quality:f32
}
impl VisualParams {
 pub fn safe(mut self)->Self{
    self.emissive=self.emissive.clamp(0.0,6.0);
    self.distortion=self.distortion.clamp(0.0,1.0);
    self.particles=self.particles.min(1200);
    self.quality=self.quality.clamp(0.2,1.0);
    self
 }
}
// TODO(horizon-render): validate WGSL (Naga), choose the correct target and bind-group layouts.
// An emissive number above 1.0 alone will not add HDR bloom.
