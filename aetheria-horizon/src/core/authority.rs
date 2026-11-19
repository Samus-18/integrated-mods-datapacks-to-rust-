//! Server-side **portable** Aetheria command processor.
//! This module DOES NOT authorize a client or mutate Horizon entities itself.
//! Its owner must be the authenticated Horizon server player + world save.
//! Only that owner may call this with input verified by the server adapter.
//!
//! Offers one transaction boundary for 4–8-slot books, leveling/unlocks,
//! mana, cooldowns and the first 20 implemented portable spell actions.
//! It is not a server plugin or completed multiplayer integration yet.
use super::player_data::PlayerMagic;
use crate::magic::{
    casting::{cast,CastError,CastResult},
    progression::{PlayerProgression,LearnError},
    spellbook::{Spellbook,BookError},
};
use crate::ui::constellation_menu::{constellation_menu,ConstellationMenu};

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum PlayerCommand {
    Learn{spell_id:u16},
    Equip{slot:u8,spell_id:u16},
    Select{slot:u8},
    Cast{charge:f32},
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ClientRequest {
    /// Monotonic sequence per authenticated player, never a login token.
    pub sequence:u64,
    pub command:PlayerCommand,
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum CommandResult {
    Learned{spell_id:u16},
    Equipped{spell_id:u16,slot:u8},
    Selected{spell_id:Option<u16>,slot:u8},
    Cast(CastResult),
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CommandError {
    ReplayedSequence, OutOfOrderSequence, EmptySlot,
    InvalidBook(BookError), InvalidSpell(LearnError), Cast(CastError),
}
#[derive(Debug)]
pub struct AuthoritativePlayer {
    pub magic:PlayerMagic,
    pub progression:PlayerProgression,
    pub book:Spellbook,
    /// Monotonically increasing, server-owned sequence, reset on secure login.
    last_sequence:Option<u64>,
}
impl AuthoritativePlayer {
    pub fn new(class_id:u8)->Self {
        let magic=PlayerMagic::new(class_id);
        let mut book=Spellbook::new(4).expect("valid default book size");
        book.equip(0,u16::from(class_id)*20+1,&magic)
            .expect("starter spell must be valid and unlocked");
        Self{magic,progression:PlayerProgression::default(),book,last_sequence:None}
    }
    pub fn gui(&self)->ConstellationMenu {
        constellation_menu(&self.magic,&self.progression)
    }
    pub fn regain_mana(&mut self,elapsed_ticks:u32){
        self.magic.regenerate(elapsed_ticks.min(20*60));
    }
    /// Accepts *already server-authenticated* command payloads only.
    ///
    /// The sequence is consumed even on a refused well-formed action; resends
    /// cannot trigger a later success after inventory changes. This prevents
    /// a replayed unlock request from charging resources twice.
    pub fn apply(
        &mut self,request:ClientRequest,tick:u64
    )->Result<CommandResult,CommandError> {
        if let Some(last)=self.last_sequence {
            if request.sequence<=last {return Err(CommandError::ReplayedSequence);}
            if request.sequence!=last.saturating_add(1) {
                return Err(CommandError::OutOfOrderSequence);
            }
        }
        self.last_sequence=Some(request.sequence);
        match request.command {
            PlayerCommand::Learn{spell_id}=>{
                self.progression.learn(&mut self.magic,spell_id)
                    .map_err(CommandError::InvalidSpell)?;
                Ok(CommandResult::Learned{spell_id})
            }
            PlayerCommand::Equip{slot,spell_id}=>{
                self.book.equip(slot as usize,spell_id,&self.magic)
                    .map_err(CommandError::InvalidBook)?;
                Ok(CommandResult::Equipped{slot,spell_id})
            }
            PlayerCommand::Select{slot}=>{
                self.book.select(slot as usize)
                    .map_err(CommandError::InvalidBook)?;
                Ok(CommandResult::Selected{slot,
                    spell_id:self.book.selected_spell()})
            }
            PlayerCommand::Cast{charge}=>{
                let id=self.book.selected_spell().ok_or(CommandError::EmptySlot)?;
                let result=cast(&mut self.magic,id,tick,charge)
                    .map_err(CommandError::Cast)?;
                Ok(CommandResult::Cast(result))
            }
        }
    }
    /// For server save/reload. Never trust client-provided player snapshots.
    pub fn last_sequence(&self)->Option<u64>{self.last_sequence}
}
#[cfg(test)]
mod tests{
    use super::*;
    use std::collections::BTreeSet;
    fn req(sequence:u64,command:PlayerCommand)->ClientRequest{
        ClientRequest{sequence,command}
    }
    #[test]
    fn default_spellbook_holds_one_class_starter(){
        for c in 0..10 {
            let p=AuthoritativePlayer::new(c);
            assert_eq!(p.book.capacity(),4);
            assert_eq!(p.book.selected_spell(),Some(u16::from(c)*20+1));
            assert_eq!(p.gui().nodes.len(),20);
        }
    }
    #[test]
    fn full_learn_equip_and_cast_is_transactional(){
        let mut p=AuthoritativePlayer::new(1);
        p.progression.arcane_level=100;
        p.progression.supply.shards=40;
        p.progression.supply.pages=BTreeSet::from([22]);
        assert_eq!(p.apply(req(1,PlayerCommand::Learn{spell_id:22}),100),
                   Ok(CommandResult::Learned{spell_id:22}));
        assert!(p.magic.unlocked_nodes.contains(&22));
        assert_eq!(p.progression.supply.shards,34);
        assert_eq!(p.apply(req(2,PlayerCommand::Equip{slot:1,spell_id:22}),100),
                   Ok(CommandResult::Equipped{slot:1,spell_id:22}));
        assert_eq!(p.apply(req(3,PlayerCommand::Select{slot:1}),100),
                   Ok(CommandResult::Selected{slot:1,spell_id:Some(22)}));
        let r=p.apply(req(4,PlayerCommand::Cast{charge:0.6}),100).unwrap();
        match r {CommandResult::Cast(out)=>{
            assert_eq!(out.spell_id,22);
            assert!(out.mana_spent>0.0);
            assert!(out.next_allowed_tick>100);
        },_=>panic!("expected cast")}
        let after=p.magic.mana;
        assert_eq!(p.apply(req(5,PlayerCommand::Cast{charge:0.1}),101),
                   Err(CommandError::Cast(CastError::Cooldown)));
        assert_eq!(p.magic.mana,after);
        assert_eq!(p.last_sequence(),Some(5));
    }
    #[test]
    fn forged_client_requests_cannot_unlock_or_overspend(){
        let mut p=AuthoritativePlayer::new(0);
        let initial=p.magic.mana;
        assert_eq!(p.apply(req(1,PlayerCommand::Equip{slot:2,spell_id:2}),90),
                   Err(CommandError::InvalidBook(BookError::Locked)));
        assert_eq!(p.apply(req(2,PlayerCommand::Learn{spell_id:2}),90),
                   Err(CommandError::InvalidSpell(LearnError::LevelTooLow)));
        assert_eq!(p.apply(req(3,PlayerCommand::Cast{charge:f32::NAN}),90),
                   Err(CommandError::Cast(CastError::InvalidInput)));
        assert_eq!(p.magic.mana,initial);
        assert_eq!(p.progression.supply.shards,0);
        assert_eq!(p.book.slot(2),None);
    }
    #[test]
    fn replayed_or_skipped_sequence_has_no_effect(){
        let mut p=AuthoritativePlayer::new(0);
        p.apply(req(100,PlayerCommand::Cast{charge:0.0}),100).unwrap();
        let after=p.magic.mana;
        assert_eq!(p.apply(req(100,PlayerCommand::Cast{charge:0.0}),120),
                   Err(CommandError::ReplayedSequence));
        assert_eq!(p.apply(req(102,PlayerCommand::Cast{charge:0.0}),120),
                   Err(CommandError::OutOfOrderSequence));
        assert_eq!(p.magic.mana,after);
        assert_eq!(p.last_sequence(),Some(100));
    }
    #[test]
    fn no_unimplemented_spell_action_can_spend_mana(){
        let mut p=AuthoritativePlayer::new(0);
        p.magic.unlock(3);
        p.book.equip(1,3,&p.magic).unwrap();
        p.apply(req(1,PlayerCommand::Select{slot:1}),15).unwrap();
        assert_eq!(p.apply(req(2,PlayerCommand::Cast{charge:1.0}),15),
                   Err(CommandError::Cast(CastError::NotYetImplemented)));
        assert_eq!(p.magic.mana,100.0);
        assert!(p.magic.cooldown_until_tick.is_empty());
    }
}
