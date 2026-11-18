//! This contract separates tested game rules from unverified Horizon internals.
use crate::magic::casting::CastResult;
pub trait EngineBridge {
    fn emit_spell(&mut self,player_id:u128,event:&CastResult);
    fn save_player(&mut self,player_id:u128,encoded:&str);
    // TODO(horizon-api): bind official mod_api::Hooks and ServerHost at exact pinned SDK.
    // TODO(horizon-world): safe interactions and reversible block changes.
    // TODO(horizon-network): authoritative sync, not locally trusted damage.
    // TODO(horizon-render): validated renderer attachment and camera matrices.
}
#[derive(Default,Debug)]
pub struct RecordingBridge {
    pub spell_ids:Vec<(u128,u16)>,
    pub saved_players:Vec<(u128,String)>,
}
impl EngineBridge for RecordingBridge {
    fn emit_spell(&mut self,id:u128,event:&CastResult){self.spell_ids.push((id,event.spell_id));}
    fn save_player(&mut self,id:u128,encoded:&str){self.saved_players.push((id,encoded.to_string()));}
}
#[cfg(test)]mod tests{
 use super::*;
 use crate::{core::player_data::PlayerMagic,magic::casting::cast};
 #[test]fn bridge_receives_event(){
    let mut p=PlayerMagic::new(0);
    let result=cast(&mut p,1,100,0.0).unwrap();
    let mut bridge=RecordingBridge::default();bridge.emit_spell(42,&result);
    assert_eq!(bridge.spell_ids,vec![(42,1)]);
 }
}
