/// A local camera cue. Never slow multiplayer server ticks for visual effects.
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CinematicCue{pub seconds:f32,pub shake:f32,pub fov_delta:f32,pub local_time_scale:f32}
impl CinematicCue{
 pub fn ultimate()->Self{
    Self{seconds:1.5,shake:0.65,fov_delta:12.0,local_time_scale:0.15}
 }
}
// TODO(horizon-camera): use actual view/projection camera hook, opt out for motion sensitivity.
