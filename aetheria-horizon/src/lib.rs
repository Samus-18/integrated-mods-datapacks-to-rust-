//! Aetheria: standalone, verifiable Rust magic rules and visual contracts.
//! Horizon integration is deliberately behind EngineBridge, NOT yet wired to the game.
pub mod core;
pub mod magic;
pub mod ui;
pub mod vfx;
pub mod sfx;
pub use magic::casting::{cast, CastError, CastResult, SpellAction};
pub use magic::registry::{by_id, SpellSpec, SPELLS};

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    fn every_class_has_a_functioning_core_action() {
        for c in 0..10u8 {
            let mut p=core::player_data::PlayerMagic::new(c);
            assert!(cast(&mut p, c as u16*20+1, 100, 0.7).is_ok(),"class {c}");
        }
    }
    #[test]
    fn catalog_is_exactly_200_unique_entries() {
        use std::collections::HashSet;
        assert_eq!(SPELLS.len(),200);
        assert_eq!(SPELLS.iter().map(|s|s.id).collect::<HashSet<_>>().len(),200);
        assert_eq!(SPELLS.iter().map(|s|s.name).collect::<HashSet<_>>().len(),200);
        for c in 0..10 {
            assert_eq!(SPELLS.iter().filter(|s|s.class_id==c).count(),20);
            assert_eq!(SPELLS.iter().filter(|s|s.class_id==c&&s.role=="Standard").count(),14);
            assert_eq!(SPELLS.iter().filter(|s|s.class_id==c&&s.role=="Awakening").count(),4);
            assert_eq!(SPELLS.iter().filter(|s|s.class_id==c&&s.role=="Ultimate").count(),2);
        }
    }
}
