use crate::core::player_data::PlayerMagic;
use super::registry::by_id;

/// Portable combat intent to apply via Horizon's authoritative Host (future adapter).
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum SpellAction {
 /// Authored mechanical intent for one of the 170 expanded spell designs.
 /// Exact name, mechanic, form, and cast mode are looked up by ID; engine
 /// world effects MUST implement these rules before calling it playable.
 DesignIntent{spell_id:u16, mechanic:&'static str, form:&'static str,
              cast_type:&'static str, role:&'static str, intensity:f32},
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
 CinderFan{bolts:u8,spread_radians:f32,damage_per_bolt:f32},
 RimeBridge{plates:u8,spacing:f32,lifetime_ticks:u16},
 RazorCrescent{arc_radians:f32,damage:f32,range:f32},
 CrystalTraverse{steps:u8,rise:f32,lifetime_ticks:u16},
 Flowstep{speed:f32,duration_ticks:u16,wet_trail_ticks:u16},
 RootSnare{radius:f32,max_targets:u8,bind_ticks:u16},
 StaticHalo{radius:f32,pulses:u8,arc_damage:f32},
 GravityThread{max_target_mass:f32,pull:f32,duration_ticks:u16},
 MomentSlip{lookback_ticks:u16,max_distance:f32},
 SoulLantern{reveal_radius:f32,duration_ticks:u16,hostile_only:bool},
 MagmaLash{reach:f32,damage:f32,burn_ticks:u16},
 ShardChorus{spikes:u8,spread_radians:f32,damage:f32},
 UpdraftWell{radius:f32,lift:f32,duration_ticks:u16},
 GravelJaws{damage:f32,pin_ticks:u16,radius:f32},
 PearlShield{absorption:f32,duration_ticks:u16},
 MossMend{heal_per_tick:f32,duration_ticks:u16},
 ThunderNeedle{damage:f32,range:f32},
 BlinkFold{range:f32,arrival_guard_ticks:u16},
 TickPulse{radius:f32,slow_factor:f32,duration_ticks:u16},
 GraveWhisper{lure_radius:f32,duration_ticks:u16,undead_only:bool},
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CastError{UnknownSpell,WrongClass,Locked,NotYetImplemented,Cooldown,InsufficientMana,InvalidInput}
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
/// Two behaviorally different gameplay intents per class. World changes are
/// requests to a future authoritative Horizon adapter, not direct block writes.
fn action_for_spell(id:u16,power:f32)->Option<SpellAction>{
 if id>=1 && id<=200 && (id-1)%20==0 {
     return Some(prototype_action(((id-1)/20) as u8,power));
 }
 Some(match id {
 2=>SpellAction::CinderFan{bolts:7,spread_radians:0.9,damage_per_bolt:1.5*power},
 22=>SpellAction::RimeBridge{plates:8,spacing:0.9*power,lifetime_ticks:160},
 42=>SpellAction::RazorCrescent{arc_radians:1.8,damage:4.6*power,range:10.0},
 62=>SpellAction::CrystalTraverse{steps:7,rise:0.7*power,lifetime_ticks:180},
 82=>SpellAction::Flowstep{speed:6.0*power,duration_ticks:22,wet_trail_ticks:60},
 102=>SpellAction::RootSnare{radius:4.5*power,max_targets:4,bind_ticks:45},
 122=>SpellAction::StaticHalo{radius:3.5*power,pulses:6,arc_damage:1.8*power},
 142=>SpellAction::GravityThread{max_target_mass:8.0,pull:1.4*power,duration_ticks:40},
 162=>SpellAction::MomentSlip{lookback_ticks:55,max_distance:8.0*power},
 182=>SpellAction::SoulLantern{reveal_radius:20.0*power,duration_ticks:200,hostile_only:true},
 3=>SpellAction::MagmaLash{reach:8.0*power,damage:4.5*power,burn_ticks:70},
 23=>SpellAction::ShardChorus{spikes:9,spread_radians:1.4,damage:2.1*power},
 43=>SpellAction::UpdraftWell{radius:4.2*power,lift:2.4*power,duration_ticks:70},
 63=>SpellAction::GravelJaws{damage:5.8*power,pin_ticks:30,radius:2.7*power},
 83=>SpellAction::PearlShield{absorption:8.0*power,duration_ticks:120},
 103=>SpellAction::MossMend{heal_per_tick:0.22*power,duration_ticks:110},
 123=>SpellAction::ThunderNeedle{damage:8.0*power,range:17.0*power},
 143=>SpellAction::BlinkFold{range:6.0*power,arrival_guard_ticks:12},
 163=>SpellAction::TickPulse{radius:5.0*power,slow_factor:0.42,duration_ticks:65},
 183=>SpellAction::GraveWhisper{lure_radius:13.0*power,duration_ticks:170,undead_only:true},
 _=>{let spec=by_id(id)?;SpellAction::DesignIntent{
     spell_id:id,mechanic:spec.mechanic,form:spec.mesh,
     cast_type:spec.cast_type,role:spec.role,intensity:power
 }},
 })
}
/// Server adapter safeguard: a DesignIntent is a fully identified *request*,
/// not an implemented world effect. Must never consume authoritative resources.
pub fn world_adapter_ready(id:u16)->bool {
    action_for_spell(id,1.0).map(|action|
        !matches!(action,SpellAction::DesignIntent{..})
    ).unwrap_or(false)
}
/// Calculates a spell cast without touching the world. Never charges unimplemented spells.
pub fn cast(p:&mut PlayerMagic,id:u16,tick:u64,charge:f32)->Result<CastResult,CastError>{
 let s=by_id(id).ok_or(CastError::UnknownSpell)?;
 if s.class_id!=p.selected_class {return Err(CastError::WrongClass);}
 if !p.unlocked_nodes.contains(&id){return Err(CastError::Locked);}
 if !s.core_prototype {return Err(CastError::NotYetImplemented);}
 if !charge.is_finite() || !p.mana.is_finite() || p.mana<0.0 {return Err(CastError::InvalidInput);}
 if tick<*p.cooldown_until_tick.get(&id).unwrap_or(&0){return Err(CastError::Cooldown);}
 if p.mana<s.mana_cost{return Err(CastError::InsufficientMana);}
 let intensity=1.0+charge.clamp(0.0,1.0);
 let action=action_for_spell(id,intensity).ok_or(CastError::NotYetImplemented)?;
 let until=tick.saturating_add(s.cooldown_ticks as u64);
 p.mana-=s.mana_cost;
 p.cooldown_until_tick.insert(id,until);
 Ok(CastResult{spell_id:id,caster_class:s.class_id,mana_spent:s.mana_cost,
    next_allowed_tick:until,action,intensity})
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
 #[test]fn newly_registered_design_intent_spends_only_its_mana(){
    let mut p=PlayerMagic::new(0);p.unlock(4);
    let action=cast(&mut p,4,100,0.0).unwrap().action;
    match action{
        SpellAction::DesignIntent{spell_id,mechanic,form,..}=>{
            assert_eq!(spell_id,4);
            assert_eq!(mechanic,"dash leaves delayed burning footsteps");
            assert!(form.contains("footstep"));
        }
        _=>panic!("no authored design intent for Furnace Step")
    }
    assert_eq!(p.mana,78.0);
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

#[cfg(test)]
mod m2_tests {
 use super::*;
 use std::collections::HashSet;
 use std::mem::discriminant;
 #[test] fn each_class_has_two_distinct_implemented_intents() {
  let mut kinds=HashSet::new();
  for c in 0..10u8 {
   let id=c as u16*20+2;
   let mut p=PlayerMagic::new(c);assert!(p.unlock(id));
   let first=cast(&mut p,id-1,100,0.0).unwrap();
   let second=cast(&mut p,id,100,0.0).unwrap();
   assert_eq!(second.mana_spent,14.0);
   assert_eq!(p.mana,76.0);
   assert!(kinds.insert(discriminant(&first.action)));
   assert!(kinds.insert(discriminant(&second.action)));
  }
  assert_eq!(kinds.len(),20);
 }
 #[test] fn invalid_charge_never_changes_resources() {
  let mut p=PlayerMagic::new(0);
  assert_eq!(cast(&mut p,1,100,f32::NAN),Err(CastError::InvalidInput));
  assert_eq!(cast(&mut p,1,100,f32::INFINITY),Err(CastError::InvalidInput));
  assert_eq!(p.mana,100.0);assert!(p.cooldown_until_tick.is_empty());
 }
 #[test] fn rime_bridge_cast_matches_eight_original_visual_plates() {
  // M34: one physical ice plate per authored Rime Bridge segment;
  // do not silently drift the future server-world spell out of visual sync.
  let mut p=PlayerMagic::new(1);
  assert!(p.unlock(22));
  let result=cast(&mut p,22,100,0.6).unwrap();
  match result.action {
   SpellAction::RimeBridge{plates,spacing,lifetime_ticks}=>{
    assert_eq!(plates,8);
    assert!((spacing-1.44).abs()<0.0001);
    assert_eq!(lifetime_ticks,160);
   },
   _=>panic!("Rime Bridge cast is not the matching authored action"),
  }
 }
 #[test] fn moment_slip_requests_safe_rewind_not_direct_teleport() {
  let mut p=PlayerMagic::new(8);p.unlock(162);
  assert_eq!(cast(&mut p,162,100,1.0).unwrap().action,
   SpellAction::MomentSlip{lookback_ticks:55,max_distance:16.0});
 }
}

#[cfg(test)]
mod thirty_spell_tests {
    use super::*;
    use std::{collections::HashSet,mem::discriminant};
    #[test]
    fn first_three_in_each_school_have_distinct_cast_actions() {
        let mut action_kinds=HashSet::new();
        for school in 0..10u8 {
            let mut player=PlayerMagic::new(school);
            for rank in 1..=3u16 {
                let id=u16::from(school)*20+rank;
                assert!(by_id(id).unwrap().core_prototype,"spell {id} missing core");
                if rank>1 {assert!(player.unlock(id));}
                let result=cast(&mut player,id,300,0.8).expect("cast");
                assert_eq!(result.spell_id,id);
                assert!(action_kinds.insert(discriminant(&result.action)),
                    "duplicate portable action type for {id}");
            }
            assert_eq!(player.mana,58.0);
        }
        assert_eq!(action_kinds.len(),30);
    }
    #[test]
    fn all_two_hundred_spells_cast_as_identifiable_intents(){
        use std::collections::BTreeSet;
        let mut identities=BTreeSet::new();
        for spec in crate::magic::registry::SPELLS {
            let mut player=PlayerMagic::new(spec.class_id);
            if !player.unlocked_nodes.contains(&spec.id){assert!(player.unlock(spec.id));}
            player.mana=100.0;
            let result=cast(&mut player,spec.id,100,0.5).unwrap();
            assert_eq!(result.spell_id,spec.id);
            assert_eq!(result.mana_spent,spec.mana_cost);
            assert!((player.mana-(100.0-spec.mana_cost)).abs()<0.001);
            assert!(identities.insert(result.spell_id));
            if (spec.id-1)%20>=3 {
                match result.action {
                    SpellAction::DesignIntent{spell_id,mechanic,form,cast_type,role,..}=>{
                        assert_eq!(spell_id,spec.id);
                        assert_eq!(mechanic,spec.mechanic);
                        assert_eq!(form,spec.mesh);
                        assert_eq!(cast_type,spec.cast_type);
                        assert_eq!(role,spec.role);
                    },
                    _=>panic!("expected exact authored design action for {}",spec.id),
                }
            }
            assert_eq!(cast(&mut player,spec.id,101,0.0),Err(CastError::Cooldown));
        }
        assert_eq!(identities.len(),200);
    }
    #[test]
    fn wrong_class_and_locked_designs_do_not_consume_resources(){
        let mut p=PlayerMagic::new(0);
        assert_eq!(cast(&mut p,20,99,0.6),Err(CastError::Locked));
        assert_eq!(cast(&mut p,22,99,0.6),Err(CastError::WrongClass));
        assert_eq!(p.mana,100.0);
        assert!(p.cooldown_until_tick.is_empty());
    }

}
