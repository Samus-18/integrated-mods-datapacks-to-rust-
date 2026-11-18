use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug,Clone,PartialEq)]
pub struct PlayerMagic {
    pub selected_class:u8,
    pub mana:f32,
    pub max_mana:f32,
    pub mana_regen_per_second:f32,
    pub unlocked_nodes:BTreeSet<u16>,
    pub cooldown_until_tick:BTreeMap<u16,u64>,
}
impl PlayerMagic {
    pub fn new(selected_class:u8)->Self {
        assert!(selected_class<10,"invalid arcane class");
        let mut unlocked_nodes=BTreeSet::new();
        unlocked_nodes.insert(selected_class as u16*20+1);
        Self{selected_class,mana:100.0,max_mana:100.0,
             mana_regen_per_second:4.0,unlocked_nodes,
             cooldown_until_tick:BTreeMap::new()}
    }
    pub fn regenerate(&mut self,elapsed_ticks:u32) {
        let recovered=elapsed_ticks as f32*self.mana_regen_per_second/20.0;
        self.mana=(self.mana+recovered).clamp(0.0,self.max_mana);
    }
    pub fn unlock(&mut self,id:u16)->bool {
        if id==0 || (id-1)/20 != self.selected_class as u16 {return false;}
        self.unlocked_nodes.insert(id)
    }
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn default_unlock_is_class_local(){let p=PlayerMagic::new(3);assert!(p.unlocked_nodes.contains(&61));}
 #[test]fn foreign_or_zero_spell_is_not_unlocked(){let mut p=PlayerMagic::new(0);assert!(!p.unlock(0));assert!(!p.unlock(21));}
 #[test]fn regen_clamps(){let mut p=PlayerMagic::new(1);p.mana=92.0;p.regenerate(1000);assert_eq!(p.mana,100.0);}
}
