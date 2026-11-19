//! UI-independent skill tree and spell-wheel view model.
//! The game renderer will draw this using its verified GUI API later.
use crate::{
    core::player_data::PlayerMagic,
    magic::{progression::{PlayerProgression,LearnError,SpellTier},
            registry::{by_id,CLASS_NAMES}},
};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum NodeState { Known, Available, Locked(LearnError) }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct NodeView {
    pub spell_id:u16,
    pub name:&'static str,
    pub tier:SpellTier,
    pub min_level:u8,
    pub shards:u16,
    pub state:NodeState,
}
#[derive(Debug,Clone,PartialEq)]
pub struct ConstellationMenu {
    pub class_name:&'static str,
    pub level:u8,
    pub mana_fraction:f32,
    pub nodes:Vec<NodeView>,
}
pub fn constellation_menu(player:&PlayerMagic,progress:&PlayerProgression)->ConstellationMenu{
    let mut nodes=Vec::with_capacity(20);
    for index in 1..=20 {
        let id=u16::from(player.selected_class)*20+index;
        if let Some(spec)=by_id(id) {
            let cost=PlayerProgression::cost(spec);
            let state=if player.unlocked_nodes.contains(&id) {NodeState::Known}
                 else {match progress.can_learn(player,id) {
                    Ok(_)=>NodeState::Available, Err(reason)=>NodeState::Locked(reason)
                 }};
            nodes.push(NodeView {
                spell_id:id,name:spec.name,tier:PlayerProgression::tier(spec),
                min_level:cost.min_level,shards:cost.shards,state
            });
        }
    }
    let fill=if player.max_mana.is_finite()&&player.max_mana>0.0&&player.mana.is_finite() {
        (player.mana/player.max_mana).clamp(0.0,1.0)
    } else {0.0};
    ConstellationMenu{
        class_name:CLASS_NAMES[player.selected_class as usize],
        level:progress.arcane_level,mana_fraction:fill,nodes,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]fn complete_20_node_view_per_class() {
        for c in 0..10{
            let player=PlayerMagic::new(c);
            let progress=PlayerProgression::default();
            let view=constellation_menu(&player,&progress);
            assert_eq!(view.nodes.len(),20);
            assert_eq!(view.nodes[0].state,NodeState::Known);
            assert_eq!(view.nodes[1].state,NodeState::Locked(LearnError::LevelTooLow));
            assert!((0.0..=1.0).contains(&view.mana_fraction));
        }
    }
}