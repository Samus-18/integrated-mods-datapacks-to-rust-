use std::collections::BTreeMap;
#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
pub enum Status {Wet, Burning, Chilled, Rooted, Conductive}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Element{Fire,Ice,Wind,Earth,Water,Nature,Lightning,Astral,Time,Soul}
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Reaction{
    None,
    ConductiveSurge{damage_scale:f32},
    ThermalBurst{radius:f32},
    WildfireGust{radius:f32},
    RootCrush{duration_ticks:u16},
}
#[derive(Debug,Default)]
pub struct StatusField{statuses:BTreeMap<(u128,Status),u64>}
impl StatusField{
    pub fn apply(&mut self,target:u128,status:Status,now:u64,duration_ticks:u64){
        self.statuses.insert((target,status),now.saturating_add(duration_ticks));
    }
    pub fn active(&self,target:u128,status:Status,tick:u64)->bool{
        self.statuses.get(&(target,status)).is_some_and(|until|tick<*until)
    }
    pub fn impact(&mut self,target:u128,element:Element,tick:u64)->Reaction{
        match element {
            Element::Water=>{self.apply(target,Status::Wet,tick,100);Reaction::None}
            Element::Ice=>{self.apply(target,Status::Chilled,tick,80);Reaction::None}
            Element::Nature=>{self.apply(target,Status::Rooted,tick,70);Reaction::None}
            Element::Lightning if self.active(target,Status::Wet,tick)=>{
                self.statuses.remove(&(target,Status::Wet));
                self.apply(target,Status::Conductive,tick,35);
                Reaction::ConductiveSurge{damage_scale:1.5}
            }
            Element::Fire if self.active(target,Status::Chilled,tick)=>{
                self.statuses.remove(&(target,Status::Chilled));
                self.apply(target,Status::Burning,tick,60);
                Reaction::ThermalBurst{radius:3.5}
            }
            Element::Wind if self.active(target,Status::Burning,tick)=>{
                self.statuses.remove(&(target,Status::Burning));
                Reaction::WildfireGust{radius:5.0}
            }
            Element::Earth if self.active(target,Status::Rooted,tick)=>{
                self.statuses.remove(&(target,Status::Rooted));
                Reaction::RootCrush{duration_ticks:40}
            }
            Element::Fire=>{self.apply(target,Status::Burning,tick,60);Reaction::None}
            _=>Reaction::None
        }
    }
    pub fn clear_expired(&mut self,now:u64){
        self.statuses.retain(|_,until|*until>now);
    }
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn water_plus_lightning_triggers_conductive_surge(){
    let mut effects=StatusField::default();
    assert_eq!(effects.impact(7,Element::Water,100),Reaction::None);
    assert_eq!(effects.impact(7,Element::Lightning,101),Reaction::ConductiveSurge{damage_scale:1.5});
    assert!(!effects.active(7,Status::Wet,101));
    assert_eq!(effects.impact(7,Element::Lightning,102),Reaction::None);
 }
 #[test]fn expired_effect_does_not_react(){
    let mut effects=StatusField::default();
    effects.apply(8,Status::Chilled,10,10);
    assert_eq!(effects.impact(8,Element::Fire,20),Reaction::None);
 }
 #[test]fn combos_are_target_specific(){
    let mut effects=StatusField::default();effects.impact(10,Element::Water,0);
    assert_eq!(effects.impact(11,Element::Lightning,1),Reaction::None);
 }
 #[test]fn fire_then_wind_combines(){
    let mut effects=StatusField::default();effects.impact(2,Element::Fire,0);
    assert_eq!(effects.impact(2,Element::Wind,20),Reaction::WildfireGust{radius:5.0});
 }
}
