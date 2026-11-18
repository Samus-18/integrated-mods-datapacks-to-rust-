//! Input-independent hold/release mechanics for charged and channeled spells.
use crate::core::player_data::PlayerMagic;
use super::casting::{cast,CastError,CastResult};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ChargeState{pub spell_id:u16,pub started_at_tick:u64,pub max_charge_ticks:u32}
impl ChargeState{
 pub fn new(spell_id:u16,start:u64,max_charge_ticks:u32)->Self {
    Self{spell_id,started_at_tick:start,max_charge_ticks:max_charge_ticks.max(1)}
 }
 pub fn charge_fraction(&self,now:u64)->f32 {
    (now.saturating_sub(self.started_at_tick) as f32/self.max_charge_ticks as f32).clamp(0.0,1.0)
 }
 pub fn release(&self,p:&mut PlayerMagic,now:u64)->Result<CastResult,CastError> {
    cast(p,self.spell_id,now,self.charge_fraction(now))
 }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ChannelState {
 pub spell_id:u16,
 pub last_tick:u64,
 pub mana_per_second:f32,
 pub active:bool,
}
impl ChannelState {
 pub fn new(spell_id:u16,now:u64,mana_per_second:f32)->Self{
    Self{spell_id,last_tick:now,mana_per_second:mana_per_second.max(0.0),active:true}
 }
 /// Drains mana once per server time advance. Returns false when channel ended.
 pub fn update(&mut self,p:&mut PlayerMagic,now:u64)->bool{
    if !self.active {return false;}
    let ticks=now.saturating_sub(self.last_tick);
    let cost=ticks as f32*self.mana_per_second/20.0;
    if p.mana<cost {self.active=false;return false;}
    p.mana-=cost;
    self.last_tick=now;
    true
 }
 pub fn stop(&mut self){self.active=false;}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn charge_clamps_and_releases() {
    let c=ChargeState::new(1,20,40);let mut p=PlayerMagic::new(0);
    assert_eq!(c.charge_fraction(0),0.0);
    assert_eq!(c.charge_fraction(40),0.5);
    assert_eq!(c.charge_fraction(100),1.0);
    assert_eq!(c.release(&mut p,40).unwrap().intensity,1.5);
 }
 #[test]fn channel_pays_per_tick_once() {
    let mut p=PlayerMagic::new(0);
    let mut s=ChannelState::new(1,100,20.0);
    assert!(s.update(&mut p,120));assert_eq!(p.mana,80.0);
    assert!(s.update(&mut p,120));assert_eq!(p.mana,80.0);
    assert!(!s.update(&mut p,500));assert_eq!(p.mana,80.0);
 }
}
