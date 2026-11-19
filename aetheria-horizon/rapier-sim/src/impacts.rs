//! First 10 original fantasy spell *impact debris* recipes, using Rapier
//! rigid-body trajectories. These are secondary physical fragments alongside
//! custom 3D spell meshes; Rapier doesn't render glow, refraction or the spell.
//! Damage/terrain editing must come from the Horizon authoritative server.
use super::{FantasyBody,RapierAetheria,SimError};
use std::f32::consts::TAU;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ImpactSchool{
    Pyromancer,Cryomancer,Aeromancer,Geomancer,Hydromancer,
    Druid,Electromancer,Astral,Chronomancer,Necromancer
}
impl ImpactSchool{
    pub fn from_class(id:u8)->Option<Self>{
        Some(match id{
            0=>Self::Pyromancer,1=>Self::Cryomancer,2=>Self::Aeromancer,
            3=>Self::Geomancer,4=>Self::Hydromancer,5=>Self::Druid,
            6=>Self::Electromancer,7=>Self::Astral,8=>Self::Chronomancer,
            9=>Self::Necromancer,_=>return None,
        })
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ImpactSummary{pub pieces:usize,pub impulse_hint:f32}
fn jitter(seed:u64)->f32 {
    let mut x=seed.wrapping_add(0x9E3779B97F4A7C15);
    x^=x>>30;x=x.wrapping_mul(0xBF58476D1CE4E5B9);
    x^=x>>27;x=x.wrapping_mul(0x94D049BB133111EB);
    x^=x>>31;
    ((x>>40) as f32/((1_u64<<24) as f32))*2.-1.
}
/// Adds *only* secondary physical pieces to an isolated Rapier world.
/// A real spell's main articulated/volumetric mesh must remain unique.
pub fn emit_impact(
    sim:&mut RapierAetheria,school:ImpactSchool,
    origin:[f32;3],seed:u64
)->Result<ImpactSummary,SimError>{
    if !origin.iter().all(|v|v.is_finite()&&v.abs()<24.)
        ||origin[1]<0. || origin[1]>22. {
        return Err(SimError::InvalidPosition)
    }
    let (count,kind,speed,lift,radius,tangential)=match school{
        ImpactSchool::Pyromancer=>(12,FantasyBody::EmberStone,8.8,9.0,0.17,0.10),
        ImpactSchool::Cryomancer=>(9,FantasyBody::IceShard,5.6,5.6,0.28,0.24),
        ImpactSchool::Aeromancer=>(6,FantasyBody::WindTossedDebris,6.3,9.2,0.14,0.95),
        ImpactSchool::Geomancer=>(7,FantasyBody::StoneDebris,4.2,3.0,0.52,0.14),
        ImpactSchool::Hydromancer=>(8,FantasyBody::WindTossedDebris,7.9,5.1,0.11,0.5),
        ImpactSchool::Druid=>(6,FantasyBody::WindTossedDebris,3.0,4.6,0.21,0.55),
        ImpactSchool::Electromancer=>(8,FantasyBody::StoneDebris,11.5,7.7,0.15,0.25),
        ImpactSchool::Astral=>(6,FantasyBody::IceShard,3.9,10.8,0.34,0.78),
        ImpactSchool::Chronomancer=>(4,FantasyBody::StoneDebris,2.7,2.1,0.25,0.35),
        ImpactSchool::Necromancer=>(9,FantasyBody::StoneDebris,5.3,8.2,0.17,0.33),
    };
    if sim.count()+count>super::MAX_BODIES {
        return Err(SimError::BodyBudget)
    }
    let base_index=sim.count();
    for i in 0..count {
        let angle=TAU*i as f32/count as f32+jitter(seed ^ i as u64)*0.12;
        let radial=1.0+0.2*jitter(seed.rotate_left(17) ^ i as u64);
        let spin=angle.sin_cos();
        let (sa,ca)=spin;
        let dx=ca*speed*radial-tangential*sa*speed;
        let dz=sa*speed*radial+tangential*ca*speed;
        let vy=lift*(0.68+0.23*jitter(seed^((i+5) as u64*31)).abs());
        let position=[
            origin[0]+ca*0.27,
            origin[1]+0.45+(i%3) as f32*0.07,
            origin[2]+sa*0.27,
        ];
        let velocity=[dx,vy,dz];
        sim.spawn(kind,position,velocity,radius)?;
    }
    Ok(ImpactSummary {
        pieces:sim.count()-base_index,
        impulse_hint:(speed*0.8+lift*0.2),
    })
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn all_ten_schools_have_distinct_velocity_and_debris_patterns(){
        let mut fingerprints=Vec::new();
        for class in 0..10 {
            let mut sim=RapierAetheria::new();
            let school=ImpactSchool::from_class(class).unwrap();
            let out=emit_impact(&mut sim,school,[0.,1.5,0.],1337).unwrap();
            assert!(out.pieces>=4 && out.pieces<=12);
            assert_eq!(sim.count(),out.pieces);
            sim.step(20);
            let poses=sim.positions();
            assert_eq!(poses.len(),out.pieces);
            assert!(poses.iter().all(|p|p.position.iter().all(|x|x.is_finite())));
            let v=poses[0].velocity;
            let fingerprint=(out.pieces,(v[0]*10.).round() as i32,
                             (v[1]*10.).round() as i32);
            assert!(!fingerprints.contains(&fingerprint),"same pattern");
            fingerprints.push(fingerprint);
        }
        assert!(ImpactSchool::from_class(10).is_none());
    }
    #[test]fn invalid_hit_and_budget_are_atomic(){
        let mut sim=RapierAetheria::new();
        assert_eq!(emit_impact(&mut sim,ImpactSchool::Cryomancer,
                       [f32::INFINITY,0.,0.],1),Err(SimError::InvalidPosition));
        assert_eq!(sim.count(),0);
        for _ in 0..8{
            emit_impact(&mut sim,ImpactSchool::Pyromancer,[0.,1.0,0.],777).unwrap();
        }
        assert_eq!(sim.count(),96);
        assert_eq!(emit_impact(&mut sim,ImpactSchool::Cryomancer,[0.,1.0,0.],777),
                   Err(SimError::BodyBudget));
        assert_eq!(sim.count(),96);
    }
    #[test]fn same_seed_produces_same_physical_motion(){
        let mut a=RapierAetheria::new();let mut b=RapierAetheria::new();
        for sim in [&mut a,&mut b] {
            emit_impact(sim,ImpactSchool::Geomancer,[2.,2.,-1.],55).unwrap();
            sim.step(42);
        }
        assert_eq!(a.positions(),b.positions());
    }
}
