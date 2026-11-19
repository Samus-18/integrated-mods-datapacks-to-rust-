//! M19: complete two-hundred-entry selectable Aetheria spell library.
 //! Original metadata is authored in the portable Rust registry. This module
 //! never substitutes the old 30 preview shapes for 170 missing VFX.
use aetheria_horizon::magic::registry::{by_id,SpellSpec};
pub const SPELL_COUNT:usize=200;
pub const SCHOOL_COUNT:usize=10;
pub const SCHOOL_COLORS:[u32;10]=super::catalog30::SCHOOL_COLORS;
pub const WORLD_COLORS:[u32;10]=super::catalog30::WORLD_COLORS;
pub const SCHOOL_SFX:[[&str;3];10]=super::catalog30::SCHOOL_SFX;
pub const DIRECT_SLOTS:[usize;5]=[0,20,100,120,140];
pub fn id(slot:usize)->u16{slot.min(SPELL_COUNT-1) as u16+1}
pub fn school(slot:usize)->usize{slot.min(SPELL_COUNT-1)/20}
pub fn rank(slot:usize)->usize{slot.min(SPELL_COUNT-1)%20}
pub fn spec(slot:usize)->&'static SpellSpec{
    by_id(id(slot)).expect("canonical 200-spell registry")
}
/// Map FIRST THREE spells in every school to the 30 curated VFX renderer.
/// Never accidentally render school-0 spell #10 using some other school's
/// older client preview, or pretend such a fallback is an authored effect.
pub fn curated_slot(slot:usize)->Option<usize>{
    if slot>=200{return None;}
    let local=rank(slot);
    (local<3).then_some(school(slot)*3+local)
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn full_catalog_exactly_matches_authoritative_spell_registry(){
        let mut unique=std::collections::BTreeSet::new();
        for slot in 0..200{
            let s=spec(slot);
            assert_eq!(s.id,id(slot));
            assert_eq!(s.class_id as usize,school(slot));
            assert!(!s.mechanic.is_empty()&&!s.mesh.is_empty());
            assert!(unique.insert(s.name));
            assert_eq!(curated_slot(slot).is_some(),rank(slot)<3);
            if rank(slot)<3{
                assert_eq!(super::super::catalog30::id(curated_slot(slot).unwrap()),id(slot));
            }
        }
        assert_eq!(unique.len(),200);
    }
}
