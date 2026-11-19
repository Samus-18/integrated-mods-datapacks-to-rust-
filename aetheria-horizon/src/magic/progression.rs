//! Deterministic, transactional spell acquisition rules.
//! This portable core **does not** grant loot or modify saves in Horizon yet.
//! The authoritative server adapter must validate actual drops/trials/XP.
use std::collections::BTreeSet;
use crate::core::player_data::PlayerMagic;
use super::registry::{by_id, SpellSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearnError {
    UnknownSpell, WrongClass, AlreadyKnown, LevelTooLow, MissingPage,
    TrialNotCompleted, InsufficientShards, InsufficientSigils,
    InsufficientEssence,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTier { Standard, Awakening, Ultimate }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnlockCost {
    pub min_level:u8,
    pub shards:u16,
    pub sigils:u8,
    pub essence:u8,
    pub requires_page:bool,
    pub requires_trial:bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArcaneSupply {
    /// Discovered unique pages from exploration, quests and crafting.
    pub pages:BTreeSet<u16>,
    pub shards:u16,
    pub awakening_sigils:u8,
    pub primal_essence:u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerProgression {
    pub arcane_level:u8,
    pub experience:u32,
    /// Earned through boss/temple trials, keyed to exact spell.
    pub completed_trials:BTreeSet<u16>,
    pub supply:ArcaneSupply,
}
impl Default for PlayerProgression {
    fn default()->Self {
        Self {arcane_level:1,experience:0,completed_trials:BTreeSet::new(),
            supply:ArcaneSupply::default()}
    }
}
impl PlayerProgression {
    pub fn earn_experience(&mut self, points:u32) {
        self.experience=self.experience.saturating_add(points);
        while self.arcane_level<100 {
            let need=50+u32::from(self.arcane_level)*25;
            if self.experience<need {break;}
            self.experience-=need;
            self.arcane_level+=1;
        }
    }
    pub fn cost(spec:&SpellSpec)->UnlockCost {
        let rank=(spec.id-1)%20;
        match rank {
            0..=13=>UnlockCost {
                min_level:2+rank as u8*2,
                shards:4+rank*2, sigils:0, essence:0,
                requires_page:true, requires_trial:false
            },
            14..=17=>UnlockCost {
                min_level:40+(rank as u8-14)*7,
                shards:40+5*(rank-14),sigils:1+(rank as u8-14),
                essence:0,requires_page:true,requires_trial:true
            },
            _=>UnlockCost {
                min_level:75+(rank as u8-18)*15,
                shards:80+20*(rank-18),sigils:3,
                essence:2+(rank as u8-18),
                requires_page:true,requires_trial:true
            },
        }
    }
    pub fn tier(spec:&SpellSpec)->SpellTier {
        match (spec.id-1)%20 { 0..=13=>SpellTier::Standard,
                              14..=17=>SpellTier::Awakening,
                              _=>SpellTier::Ultimate }
    }
    pub fn can_learn(&self, player:&PlayerMagic, spell_id:u16)->Result<UnlockCost,LearnError>{
        let spec=by_id(spell_id).ok_or(LearnError::UnknownSpell)?;
        if spec.class_id!=player.selected_class {return Err(LearnError::WrongClass);}
        if player.unlocked_nodes.contains(&spell_id){return Err(LearnError::AlreadyKnown);}
        let cost=Self::cost(spec);
        if self.arcane_level<cost.min_level {return Err(LearnError::LevelTooLow);}
        if cost.requires_page && !self.supply.pages.contains(&spell_id) {
            return Err(LearnError::MissingPage);
        }
        if cost.requires_trial && !self.completed_trials.contains(&spell_id) {
            return Err(LearnError::TrialNotCompleted);
        }
        if self.supply.shards<cost.shards {return Err(LearnError::InsufficientShards);}
        if self.supply.awakening_sigils<cost.sigils {
            return Err(LearnError::InsufficientSigils);
        }
        if self.supply.primal_essence<cost.essence {
            return Err(LearnError::InsufficientEssence);
        }
        Ok(cost)
    }
    /// Call on the authoritative server after validating real inventory and milestones.
    /// All checks run before any material is spent or a new node is written.
    pub fn learn(&mut self,player:&mut PlayerMagic,id:u16)->Result<UnlockCost,LearnError>{
        let cost=self.can_learn(player,id)?;
        if !player.unlock(id) {return Err(LearnError::WrongClass);}
        self.supply.shards-=cost.shards;
        self.supply.awakening_sigils-=cost.sigils;
        self.supply.primal_essence-=cost.essence;
        if cost.requires_page {self.supply.pages.remove(&id);}
        Ok(cost)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    fn provision(id:u16)->(PlayerMagic,PlayerProgression) {
        let mut p=PlayerMagic::new(((id-1)/20) as u8);
        let mut xp=PlayerProgression::default();
        xp.arcane_level=100;
        xp.supply.pages.insert(id);
        xp.completed_trials.insert(id);
        xp.supply.shards=500;
        xp.supply.awakening_sigils=10;
        xp.supply.primal_essence=10;
        // First spell comes unlocked by default; can learn from second onward.
        assert!(p.unlocked_nodes.len()==1);
        (p,xp)
    }
    #[test] fn every_class_has_20_valid_unlock_costs(){
        for c in 0..10u16 {
            for local in 1..=20 {
                let id=c*20+local;
                let spec=by_id(id).unwrap();
                let cost=PlayerProgression::cost(spec);
                assert!(cost.min_level<=100 && cost.shards>0);
                assert_eq!(PlayerProgression::tier(spec),match local {
                    1..=14=>SpellTier::Standard,
                    15..=18=>SpellTier::Awakening,
                    _=>SpellTier::Ultimate
                });
                if local>1 {
                    let (mut player,mut progress)=provision(id);
                    assert!(progress.learn(&mut player,id).is_ok(),"spell {id}");
                    assert!(player.unlocked_nodes.contains(&id));
                    assert_eq!(progress.learn(&mut player,id),Err(LearnError::AlreadyKnown));
                }
            }
        }
    }
    #[test] fn no_materials_spent_on_failed_attempts(){
        let (mut player,mut state)=provision(20);
        state.arcane_level=20;
        let before=state.clone();
        assert_eq!(state.learn(&mut player,20),Err(LearnError::LevelTooLow));
        assert_eq!(state,before);
        state.arcane_level=100;
        state.completed_trials.clear();
        let before=state.clone();
        assert_eq!(state.learn(&mut player,20),Err(LearnError::TrialNotCompleted));
        assert_eq!(state,before);
        assert!(!player.unlocked_nodes.contains(&20));
    }
    #[test] fn class_and_page_requirements_enforced() {
        let (mut player,mut state)=provision(2);
        state.supply.pages.clear();
        assert_eq!(state.learn(&mut player,2),Err(LearnError::MissingPage));
        assert_eq!(state.learn(&mut player,22),Err(LearnError::WrongClass));
        assert_eq!(state.learn(&mut player,201),Err(LearnError::UnknownSpell));
    }
    #[test] fn world_and_inventory_are_not_touched_when_progression_lacks_resources() {
        let (mut p,mut x)=provision(19);
        x.supply.primal_essence=0;
        let before=x.clone();
        assert_eq!(x.learn(&mut p,19),Err(LearnError::InsufficientEssence));
        assert_eq!(x,before);
        assert_eq!(p.unlocked_nodes.len(),1);
    }
    #[test] fn xp_levels_have_stable_integer_arithmetic() {
        let mut x=PlayerProgression::default();
        x.earn_experience(74);
        assert_eq!(x.arcane_level,1);
        x.earn_experience(1);
        assert_eq!(x.arcane_level,2);
        x.earn_experience(u32::MAX);
        assert_eq!(x.arcane_level,100);
    }
}