#[derive(Debug,Clone,Copy)]
pub struct SoundCue{pub charge:&'static str,pub cast:&'static str,pub impact:&'static str}
pub fn default_layers()->SoundCue{
 SoundCue{charge:"charge_harmonic",cast:"spell_transient",impact:"impact_world_reverb"}
}
// TODO(horizon-audio): implement 3D sound positioning with licensed/original waveforms.
