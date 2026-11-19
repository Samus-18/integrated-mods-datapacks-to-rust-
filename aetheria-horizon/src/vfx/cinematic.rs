//! Safe, client-local cinematic choreography for the 20 *planned* ultimates.
//! This is pure Rust pose/FOV/shake intent; it does **not** detach the actual
//! Horizon camera, slow any world/server ticks, or claim finished ultimates.
//! An engine adapter must validate collision, consent, multiplayer and FOV.
use std::f32::consts::TAU;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum SequencePhase{Prepare,Manifest,Impact,Afterglow}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CameraLanguage{
    EmberRise, SolarDive, IceCloseup, WhiteSilence,
    TornadoOrbit, SkywardSweep, FissureTracking, StoneDescent,
    TidalSweep, AbyssUndertow, GroveAscension, WorldTreeCrown,
    LivingStorm, ThunderCathedral, StarRift, FallingStars,
    FrozenMoment, BrokenHours, SoulGathering, CrimsonDusk
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CinematicSettings {
    pub enabled:bool,
    pub reduced_motion:bool,
    pub allow_shake:bool,
    pub allow_fov_changes:bool,
    pub intensity:f32,
}
impl Default for CinematicSettings {
    fn default()->Self {
        Self {enabled:true,reduced_motion:false,allow_shake:true,
              allow_fov_changes:true,intensity:0.7}
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CinematicFrame{
    pub spell_id:u16,
    pub language:CameraLanguage,
    pub phase:SequencePhase,
    /// Cinematic camera offset relative to the inspected world target.
    /// Collision adjustment must happen *in the game* before use.
    pub relative_camera:[f32;3],
    pub fov_delta:f32,
    pub shake_translation:[f32;3],
    pub local_vfx_time_scale:f32,
    pub overlay_opacity:f32,
}
#[derive(Clone,Copy)]
struct Track {
    language:CameraLanguage,
    p0:[f32;3],p1:[f32;3],p2:[f32;3],p3:[f32;3],
    fov:f32,impact:u32,
}
fn track(id:u16)->Option<Track>{
    // IDs 19/20 of every 20-spell class. Two distinct compositions per class.
    let t=match id {
        19=>(CameraLanguage::EmberRise,[-3.,1.,-6.],[-4.,1.,-5.],[0.,8.,-4.],[1.,11.,-7.],14.,39),
        20=>(CameraLanguage::SolarDive,[5.,9.,-8.],[3.,11.,-6.],[-3.,3.,-5.],[0.,1.,-9.],17.,43),
        39=>(CameraLanguage::IceCloseup,[2.,1.,-5.],[1.,0.4,-2.],[-4.,6.,-6.],[0.,8.,-9.],11.,42),
        40=>(CameraLanguage::WhiteSilence,[-6.,5.,-3.],[-1.,6.,-3.],[5.,4.,-4.],[0.,2.,-8.],8.,47),
        59=>(CameraLanguage::TornadoOrbit,[7.,1.,-7.],[1.,9.,-8.],[-7.,13.,-4.],[-8.,4.,6.],18.,40),
        60=>(CameraLanguage::SkywardSweep,[0.,0.3,-3.],[-2.,2.,-4.],[2.,14.,-5.],[6.,17.,-10.],17.,46),
        79=>(CameraLanguage::FissureTracking,[8.,1.,-2.],[5.,0.3,-4.],[-5.,0.6,-4.],[-9.,5.,-6.],13.,42),
        80=>(CameraLanguage::StoneDescent,[-1.,13.,-3.],[-5.,11.,-3.],[3.,4.,-5.],[1.,1.,-8.],12.,45),
        99=>(CameraLanguage::TidalSweep,[-9.,2.,-4.],[-5.,2.,-5.],[4.,4.,-6.],[9.,5.,-10.],14.,43),
        100=>(CameraLanguage::AbyssUndertow,[0.,7.,-7.],[-6.,4.,-4.],[-1.,-2.,-5.],[0.,3.,-8.],10.,49),
        119=>(CameraLanguage::GroveAscension,[-6.,0.,-4.],[-3.,1.,-6.],[2.,12.,-9.],[0.,18.,-13.],12.,53),
        120=>(CameraLanguage::WorldTreeCrown,[1.,16.,-10.],[9.,20.,-9.],[-12.,13.,-10.],[-5.,4.,-11.],10.,56),
        139=>(CameraLanguage::LivingStorm,[-7.,4.,-7.],[0.,10.,-7.],[8.,3.,-8.],[0.,2.,-13.],18.,43),
        140=>(CameraLanguage::ThunderCathedral,[0.,2.,-5.],[-7.,14.,-8.],[6.,16.,-6.],[0.,7.,-12.],16.,49),
        159=>(CameraLanguage::StarRift,[0.,2.,-6.],[-4.,5.,-1.],[2.,1.,-2.],[0.,7.,-13.],13.,52),
        160=>(CameraLanguage::FallingStars,[-10.,13.,-10.],[-6.,16.,-4.],[9.,7.,-7.],[4.,3.,-12.],19.,55),
        179=>(CameraLanguage::FrozenMoment,[4.,3.,-4.],[3.,3.,-2.],[-3.,3.,-2.],[-4.,3.,-7.],5.,43),
        180=>(CameraLanguage::BrokenHours,[-2.,10.,-5.],[2.,14.,-4.],[8.,0.5,-7.],[-5.,1.,-11.],14.,48),
        199=>(CameraLanguage::SoulGathering,[0.,1.,-4.],[-6.,3.,-3.],[2.,7.,-4.],[0.,6.,-9.],11.,49),
        200=>(CameraLanguage::CrimsonDusk,[-9.,4.,-8.],[-3.,9.,-7.],[8.,9.,-4.],[0.,1.,-12.],16.,51),
        _=>return None
    };
    Some(Track{language:t.0,p0:t.1,p1:t.2,p2:t.3,p3:t.4,fov:t.5,impact:t.6})
}
fn cubic(a:[f32;3],b:[f32;3],c:[f32;3],d:[f32;3],t:f32)->[f32;3]{
    let inv=1.-t;
    std::array::from_fn(|i|
        inv*inv*inv*a[i]+3.*inv*inv*t*b[i]
        +3.*inv*t*t*c[i]+t*t*t*d[i]
    )
}
fn fade(t:f32)->f32{let t=t.clamp(0.,1.);t*t*(3.-2.*t)}
/// Tick-based 20 cinematic templates. A full clip spans 100 client ticks.
/// Camera displacement is an **intent**, never automatically applied.
pub fn sample_ultimate(
    spell_id:u16,elapsed_ticks:u32,settings:CinematicSettings
)->Option<CinematicFrame>{
    let tr=track(spell_id)?;
    if elapsed_ticks>=100 || !settings.enabled || !settings.intensity.is_finite(){
        return None;
    }
    let phase=if elapsed_ticks<12 {SequencePhase::Prepare}
        else if elapsed_ticks<tr.impact {SequencePhase::Manifest}
        else if elapsed_ticks<tr.impact+9 {SequencePhase::Impact}
        else {SequencePhase::Afterglow};
    let amount=settings.intensity.clamp(0.,1.);
    let t=fade((elapsed_ticks as f32/95.).clamp(0.,1.));
    let blend=if settings.reduced_motion {0.} else {amount};
    let path=cubic(tr.p0,tr.p1,tr.p2,tr.p3,t);
    let release=if elapsed_ticks>65 {
        1.-fade((elapsed_ticks as f32-65.)/35.)
    } else {1.};
    let shock=if elapsed_ticks>=tr.impact {
        let after=(elapsed_ticks-tr.impact) as f32;
        (-after*0.20).exp()*(after*1.8).sin().abs()
    }else{0.};
    let shake=if settings.reduced_motion || !settings.allow_shake {
        [0.;3]
    } else {
        let phase=elapsed_ticks as f32;
        let strength=shock*0.15*amount*release;
        [
            (phase*1.71).sin()*strength,
            (phase*2.03+0.5).cos()*strength,
            (phase*1.32+1.6).sin()*strength*0.5,
        ]
    };
    let fov=if settings.reduced_motion || !settings.allow_fov_changes {0.}
        else {tr.fov*amount*release*(0.35+0.65*fade(elapsed_ticks as f32/20.))};
    // VFX time scaling is local to visual animation: NO global/server ticks.
    let local_time_scale=if settings.reduced_motion {1.}
        else if elapsed_ticks<30 {1.-0.85*amount}
        else {1.};
    Some(CinematicFrame {
        spell_id,language:tr.language,phase,
        relative_camera:std::array::from_fn(|i|tr.p0[i]+(path[i]-tr.p0[i])*blend),
        fov_delta:fov,shake_translation:shake,
        local_vfx_time_scale:local_time_scale,
        overlay_opacity:release*amount*0.26,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]fn twenty_unique_camera_paths_are_finite_and_bounded(){
        let mut identities=HashSet::new();
        let mut paths=HashSet::new();
        for school in 0..10 {
            for pos in [19,20] {
                let id=school*20+pos;
                let f=sample_ultimate(id,47,CinematicSettings::default()).unwrap();
                assert!(identities.insert(f.language as u8));
                let p=f.relative_camera.map(|v|(v*100.).round() as i32);
                assert!(paths.insert(p),"duplicate camera path {id}");
                assert!(f.relative_camera.iter().all(|v|v.is_finite()&&v.abs()<=20.));
                assert!(f.fov_delta<20.);
                assert!(f.local_vfx_time_scale>=0.15);
                assert!(f.overlay_opacity<=0.26);
            }
        }
        assert_eq!(identities.len(),20);
    }
    #[test]fn accessibility_switches_remove_motion_and_fov(){
        let mut settings=CinematicSettings::default();
        settings.reduced_motion=true;
        let f=sample_ultimate(19,44,settings).unwrap();
        assert_eq!(f.shake_translation,[0.;3]);
        assert_eq!(f.fov_delta,0.);
        assert_eq!(f.local_vfx_time_scale,1.);
        assert_eq!(f.relative_camera,track(19).unwrap().p0);
    }
    #[test]fn cue_has_duration_and_no_invalid_inputs(){
        assert!(sample_ultimate(1,25,CinematicSettings::default()).is_none());
        assert!(sample_ultimate(19,100,CinematicSettings::default()).is_none());
        let mut settings=CinematicSettings::default();
        settings.enabled=false;
        assert!(sample_ultimate(19,1,settings).is_none());
        settings.enabled=true;settings.intensity=f32::NAN;
        assert!(sample_ultimate(19,1,settings).is_none());
    }
}
