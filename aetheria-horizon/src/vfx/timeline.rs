//! Bounded four-stage VFX data; not yet attached to Horizon GPU or camera.
use crate::magic::registry::by_id;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Stage { Telegraph, Cast, Impact, Afterglow }
#[derive(Debug,Clone,Copy)]
pub struct Phase { pub kind:Stage, pub duration_ticks:u16, pub particle_cap:u16 }
#[derive(Debug,Clone,Copy)]
pub struct Timeline {
 pub spell_id:u16, pub mesh:&'static str, pub shader:&'static str,
 pub sound:&'static str, pub phases:[Phase;4],
}
impl Timeline {
 pub fn stage_at(&self,elapsed:u32)->Option<Stage>{
  let mut t=0;
  for phase in self.phases {t+=u32::from(phase.duration_ticks);if elapsed<t{return Some(phase.kind);}}
  None
 }
 pub fn total_ticks(&self)->u32{self.phases.iter().map(|x|u32::from(x.duration_ticks)).sum()}
}
pub fn for_spell(id:u16)->Option<Timeline>{
 let spec=by_id(id)?;
 if !spec.core_prototype{return None;}
 let (ticks,cap):([u16;4],[u16;4])=match id{
 1=>([3,4,2,12],[18,50,100,24]),
 2=>([4,6,3,15],[24,70,175,36]),
 21=>([2,5,3,18],[12,42,90,22]),
 22=>([8,10,8,160],[12,70,110,20]),
 41=>([2,4,4,10],[15,65,88,12]),
 42=>([3,8,3,11],[18,82,140,16]),
 61=>([6,8,4,24],[20,82,190,38]),
 62=>([6,9,5,180],[15,80,145,18]),
 81=>([2,12,3,19],[16,108,160,48]),
 82=>([3,22,2,24],[25,135,55,30]),
 101=>([4,6,4,21],[24,55,140,36]),
 102=>([8,10,5,45],[22,88,166,45]),
 121=>([1,3,2,13],[12,92,210,28]),
 122=>([6,24,4,18],[30,90,180,42]),
 141=>([3,8,4,17],[16,84,145,22]),
 142=>([7,14,3,36],[24,75,118,30]),
 161=>([3,12,4,30],[15,80,90,15]),
 162=>([1,6,2,17],[14,120,55,35]),
 181=>([5,8,5,29],[25,85,155,48]),
 182=>([7,9,4,200],[20,52,110,18]),
 3=>([6,11,8,39],[22,94,140,45]),    // Magma Lash: coil then crack
 23=>([7,13,7,54],[18,115,150,27]), // Shard Chorus: outward fan
 43=>([5,15,5,48],[12,90,70,24]),  // Updraft Well: ascending spiral
 63=>([9,9,13,45],[20,60,160,30]), // Gravel Jaws: closing strike
 83=>([10,18,8,83],[19,80,50,21]), // Pearl Shield: formation/absorption
 103=>([11,16,6,102],[24,55,80,35]), // Moss Mend: gradual growth
 123=>([2,7,4,25],[14,135,170,24]), // Thunder Needle: fast rupture
 143=>([8,10,3,54],[15,110,44,22]), // Blink Fold: open/close portal
 163=>([12,14,10,91],[24,75,65,30]), // Tick Pulse: temporal contraction
 183=>([9,17,5,80],[18,55,72,40]), // Grave Whisper: prolonged lure

 _=>return None,
 };
 let kinds=[Stage::Telegraph,Stage::Cast,Stage::Impact,Stage::Afterglow];
 let phases=std::array::from_fn(|i|Phase{kind:kinds[i],duration_ticks:ticks[i],particle_cap:cap[i]});
 Some(Timeline{spell_id:id,mesh:spec.mesh,shader:spec.shader_profile,sound:spec.sound_id,phases})
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn every_portable_spell_has_a_bounded_timeline(){
  use std::collections::HashSet;
  let mut signatures=HashSet::new();
  for c in 0..10u16 {for n in [1,2,3]{
   let id=c*20+n;
   let t=for_spell(id).expect("timeline missing");
   assert_eq!(t.spell_id,id);assert!(!t.mesh.is_empty());assert!(!t.shader.is_empty());
   assert!(t.phases.iter().all(|p|p.particle_cap<=1200&&p.duration_ticks>0));
   assert!(signatures.insert(t.phases.map(|p|p.duration_ticks)));
  }}
 }
 #[test] fn stage_transitions_have_end(){
  let t=for_spell(22).unwrap();
  assert_eq!(t.stage_at(0),Some(Stage::Telegraph));
  assert_eq!(t.stage_at(8),Some(Stage::Cast));
  assert_eq!(t.stage_at(t.total_ticks()),None);
  assert!(for_spell(24).is_none());
 }
}

#[cfg(test)]
mod m14_choreography_tests {
    use super::{for_spell, Stage};
    #[test]
    fn all_thirty_actions_have_complete_timeline_and_distinct_phase_speeds(){
        let mut signatures=std::collections::BTreeSet::new();
        for school in 0..10u16 {
            for rank in 1..=3 {
                let id=school*20+rank;
                let t=for_spell(id).expect("every 30-spell action has a timeline");
                let lengths=t.phases.map(|p|p.duration_ticks);
                assert!(signatures.insert(lengths),"duplicated phase choreography for {id}");
                assert!(t.total_ticks()>=12);
                assert!(t.total_ticks()<=240);
                assert!(t.phases.iter().all(|p|p.duration_ticks>0 && p.particle_cap<=1200));
                assert_eq!(t.stage_at(0),Some(Stage::Telegraph));
                assert_eq!(t.stage_at(t.total_ticks()),None);
            }
        }
        assert!(for_spell(4).is_none(),"future spells must remain design only");
        assert_eq!(signatures.len(),30);
    }
}
