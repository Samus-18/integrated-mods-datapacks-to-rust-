//! Thirty curated test spells: the first three of each of Aetheria's ten schools.
//! Metadata comes from the same 200-spell registry used by the portable rules.
//! These are a CLIENT effect preview until the server-world adapter is wired.
use aetheria_horizon::magic::registry::{by_id,SpellSpec};

pub const SPELL_COUNT:usize=30;
pub const SCHOOL_COUNT:usize=10;
pub const DIRECT_SLOTS:[usize;5]=[0,3,15,18,21];
pub const SCHOOL_COLORS:[u32;10]=[
    0xFFEAA56C,0xFF83D7F2,0xFFC1E4DE,0xFFD2AC83,0xFF62C4D9,
    0xFF85C17A,0xFFADB9F7,0xFFAD87DA,0xFFDFC885,0xFFC5C6E9
];
pub const WORLD_COLORS:[u32;10]=[
    0xCFFB7837,0xC98EDCF5,0xC3C5F4E6,0xCDAA8061,0xC278BDD9,
    0xC389D66C,0xC1A3BFFF,0xC29B83EC,0xCFCBAE70,0xC7B0A8E0
];
pub const SCHOOL_SFX:[[&str;3];10]=[
    ["aetheria_preview:phoenix_charge","aetheria_preview:phoenix_cast","aetheria_preview:phoenix_impact"],
    ["aetheria_preview:glacier_charge","aetheria_preview:glacier_cast","aetheria_preview:glacier_impact"],
    ["aetheria_preview:aero_charge","aetheria_preview:aero_cast","aetheria_preview:aero_impact"],
    ["aetheria_preview:geo_charge","aetheria_preview:geo_cast","aetheria_preview:geo_impact"],
    ["aetheria_preview:hydro_charge","aetheria_preview:hydro_cast","aetheria_preview:hydro_impact"],
    ["aetheria_preview:worldroot_charge","aetheria_preview:worldroot_cast","aetheria_preview:worldroot_impact"],
    ["aetheria_preview:thunder_charge","aetheria_preview:thunder_cast","aetheria_preview:thunder_impact"],
    ["aetheria_preview:astral_charge","aetheria_preview:astral_cast","aetheria_preview:astral_impact"],
    ["aetheria_preview:chrono_charge","aetheria_preview:chrono_cast","aetheria_preview:chrono_impact"],
    ["aetheria_preview:necro_charge","aetheria_preview:necro_cast","aetheria_preview:necro_impact"],
];
pub fn school(slot:usize)->usize {slot.min(SPELL_COUNT-1)/3}
pub fn rank(slot:usize)->usize {slot.min(SPELL_COUNT-1)%3}
pub fn id(slot:usize)->u16 { (school(slot)*20+rank(slot)+1) as u16 }
pub fn spec(slot:usize)->&'static SpellSpec {
    by_id(id(slot)).expect("all first three spells of each school exist")
}
pub fn short_name(slot:usize)->String {
    spec(slot).name.chars().take(17).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curated_set_has_30_distinct_registry_spells_with_three_in_each_school() {
        let ids=(0..SPELL_COUNT).map(id).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(ids.len(),30);
        for s in 0..SCHOOL_COUNT {
            for r in 0..3 {
                let index=s*3+r;
                assert_eq!(spec(index).class_id as usize,s);
                assert_eq!(rank(index),r);
                assert!(spec(index).mana_cost>0.0);
                assert!(spec(index).cooldown_ticks>0);
            }
        }
    }
}
