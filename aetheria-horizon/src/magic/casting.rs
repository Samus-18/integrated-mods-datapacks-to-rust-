use crate::core::player_data::PlayerMagic;
use super::registry::by_id;

/// Portable combat intent to apply via Horizon's authoritative Host (future adapter).
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum SpellAction {
 EmberDart{damage:f32,burning_ticks:u16},
 FrostSpike{damage:f32,slow_ticks:u16},
 WindPush{strength:f32,range:f32},
 StoneFist{lift:f32,damage:f32},
 WaterJet{damage:f32,wet_ticks:u16},
 ThornSeed{damage:f32,root_ticks:u16},
 ForkLightning{damage:f32,jumps:u8},
 StarShard{damage:f32,pull:f32},
 ProjectileDelay{delay_ticks:u16,radius:f32},
 BoneSpike{damage:f32,knockup:f32},
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CastError{UnknownSpell,WrongClass,Locked,NotYetImplemented,Cooldown,InsufficientMana}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CastResult{
 pub spell_id:u16,pub caster_class:u8,pub mana_spent:f32,pub next_allowed_tick:u64,
 pub action:SpellAction,pub intensity:f32
}
fn prototype_action(class:u8,power:f32)->SpellAction{
 match class{
  0=>SpellAction::EmberDart{damage:5.0*power,burning_ticks:60},
  1=>SpellAction::FrostSpike{damage:4.0*power,slow_ticks:70},
  2=>SpellAction::WindPush{strength:1.6*power,range:7.0*power},
  3=>SpellAction::StoneFist{lift:1.2*power,damage:6.0*power},
  4=>SpellAction::WaterJet{damage:4.2*power,wet_ticks:110},
  5=>SpellAction::ThornSeed{damage:3.5*power,root_ticks:50},
  6=>SpellAction::ForkLightning{damage:5.5*power,jumps:2},
  7=>SpellAction::StarShard{damage:4.8*power,pull:0.7*power},
  8=>SpellAction::ProjectileDelay{delay_ticks:30,radius:3.0*power},
  _=>SpellAction::BoneSpike{damage:5.0*power,knockup:0.9*power},
 }
}
/// Calculates a spell cast without touching the world. Never charges unimplemented spells.
pub fn cast(p:&mut PlayerMagic,id:u16,tick:u64,charge:f32)->Result<CastResult,CastError>{
 let s=by_id(id).ok_or(CastError::UnknownSpell)?;
 if s.class_id!=p.selected_class {return Err(CastError::WrongClass);}
 if !p.unlocked_nodes.contains(&id){return Err(CastError::Locked);}
 if !s.core_prototype {return Err(CastError::NotYetImplemented);}
 if tick<*p.cooldown_until_tick.get(&id).unwrap_or(&0){return Err(CastError::Cooldown);}
 if p.mana<s.mana_cost{return Err(CastError::InsufficientMana);}
 let intensity=1.0+charge.clamp(0.0,1.0);
 let until=tick.saturating_add(s.cooldown_ticks as u64);
 p.mana-=s.mana_cost;
 p.cooldown_until_tick.insert(id,until);
 Ok(CastResult{spell_id:id,caster_class:s.class_id,mana_spent:s.mana_cost,
    next_allowed_tick:until,action:prototype_action(s.class_id,intensity),intensity})
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn casts_and_spends_mana(){
    let mut p=PlayerMagic::new(0);
    let r=cast(&mut p,1,100,0.5).unwrap();
    assert_eq!(r.mana_spent,10.0);assert_eq!(p.mana,90.0);
    assert_eq!(cast(&mut p,1,101,0.0),Err(CastError::Cooldown));
    assert_eq!(p.mana,90.0);
 }
 #[test]fn incomplete_spells_do_not_spend_mana(){
    let mut p=PlayerMagic::new(0);p.unlock(2);
    assert_eq!(cast(&mut p,2,100,0.0),Err(CastError::NotYetImplemented));
    assert_eq!(p.mana,100.0);
 }
 #[test]fn class_and_resource_checks(){
    let mut p=PlayerMagic::new(0);
    assert_eq!(cast(&mut p,21,100,0.0),Err(CastError::WrongClass));
    p.mana=0.0;
    assert_eq!(cast(&mut p,1,100,0.0),Err(CastError::InsufficientMana));
 }
 #[test]fn charge_is_clamped(){
    let mut a=PlayerMagic::new(0);let mut b=PlayerMagic::new(0);
    assert_eq!(cast(&mut a,1,100,-10.0).unwrap().intensity,1.0);
    assert_eq!(cast(&mut b,1,100,10.0).unwrap().intensity,2.0);
 }
}
