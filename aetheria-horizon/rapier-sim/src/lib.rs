//! Real Rapier 3D simulation for Aetheria *world effect objects*.
//!
//! This is an isolated native physics adapter, not a graphics engine.
//! Render meshes and WGSL shaders still come from Horizon/wgpu; this module
//! computes collider trajectories for debris, ice shards, and physical impacts.
//! IMPORTANT: server world blocks/entities are not modified here; integrating
//! it into Horizon must happen via verified server-authoritative hooks.
use rapier3d::prelude::*;
const MAX_BODIES: usize = 96;
const GROUND_HALF:f32=48.0;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum FantasyBody {
    IceShard,StoneDebris,WindTossedDebris,EmberStone
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum SimError{
    BodyBudget,InvalidPosition,InvalidVelocity,InvalidRadius,
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct BodySnapshot{
    pub kind:FantasyBody,
    pub position:[f32;3],
    pub velocity:[f32;3],
}
pub struct RapierAetheria {
    world:PhysicsWorld,
    parts:Vec<(RigidBodyHandle,FantasyBody)>
}
impl Default for RapierAetheria{
    fn default()->Self {Self::new()}
}
impl RapierAetheria{
    pub fn new()->Self{
        let mut world=PhysicsWorld::new();
        world.gravity=Vector::new(0.0,-9.81,0.0);
        world.insert(
            RigidBodyBuilder::fixed().translation(Vector::new(0.0,-1.0,0.0)),
            ColliderBuilder::cuboid(GROUND_HALF,1.0,GROUND_HALF)
        );
        // Rapier spatial queries require a completed collision/broadphase
        // update after initial static collider insertion. Without this first
        // step, the raycast sees an empty acceleration structure.
        world.step();
        Self{world,parts:Vec::new()}
    }
    pub fn spawn(
        &mut self,kind:FantasyBody,position:[f32;3],
        velocity:[f32;3],radius:f32
    )->Result<usize,SimError>{
        if self.parts.len()>=MAX_BODIES {return Err(SimError::BodyBudget)}
        if !position.iter().all(|x|x.is_finite()&&x.abs()<40.0){
            return Err(SimError::InvalidPosition)
        }
        if !velocity.iter().all(|x|x.is_finite()&&x.abs()<120.0){
            return Err(SimError::InvalidVelocity)
        }
        if !radius.is_finite() || !(0.05..=2.0).contains(&radius){
            return Err(SimError::InvalidRadius)
        }
        let (density,restitution,damping)=match kind {
            FantasyBody::IceShard=>(0.7,0.15,0.05),
            FantasyBody::StoneDebris=>(2.5,0.10,0.17),
            FantasyBody::WindTossedDebris=>(0.25,0.40,0.02),
            FantasyBody::EmberStone=>(1.3,0.26,0.08)
        };
        let body=RigidBodyBuilder::dynamic()
            .translation(Vector::new(position[0],position[1],position[2]))
            .linvel(Vector::new(velocity[0],velocity[1],velocity[2]))
            .linear_damping(damping)
            .ccd_enabled(true);
        let collider=ColliderBuilder::ball(radius)
            .density(density).restitution(restitution);
        let (handle,_) = self.world.insert(body,collider);
        self.parts.push((handle,kind));
        Ok(self.parts.len()-1)
    }
    pub fn step(&mut self,frames:usize){
        for _ in 0..frames.min(240){
            self.world.step();
        }
    }
    pub fn positions(&self)->Vec<BodySnapshot>{
        self.parts.iter().filter_map(|(handle,kind)|{
            let body=self.world.bodies.get(*handle)?;
            let p=body.translation();
            let v=body.linvel();
            Some(BodySnapshot {
                kind:*kind,
                position:[p.x,p.y,p.z],
                velocity:[v.x,v.y,v.z]
            })
        }).collect()
    }
    pub fn count(&self)->usize{self.parts.len()}
    /// A simple gameplay-agnostic collision probe for aiming physical magic
    /// through the fixed Rapier collider world, not a block-edit operation.
    pub fn floor_hit_distance(&self,from:[f32;3],distance:f32)->Option<f32>{
        if !from.iter().all(|v|v.is_finite())
            ||!distance.is_finite()||distance<=0.0 {return None}
        let ray=Ray::new(
            Vector::new(from[0],from[1],from[2]),
            Vector::new(0.0,-1.0,0.0)
        );
        self.world.cast_ray(&ray,distance,true,QueryFilter::default())
            .map(|(_,toi)|toi)
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn real_3d_shards_fall_hit_ground_and_remain_finite(){
        let mut sim=RapierAetheria::new();
        sim.spawn(FantasyBody::IceShard,[0.,4.,0.],[1.,1.,0.],0.25).unwrap();
        sim.spawn(FantasyBody::StoneDebris,[1.,5.,0.],[-0.5,2.,0.],0.3).unwrap();
        let before=sim.positions();
        assert_eq!(before.len(),2);
        sim.step(120);
        let after=sim.positions();
        assert_eq!(after.len(),2);
        assert!(after[0].position[1]<before[0].position[1]);
        assert!(after.iter().all(|p|p.position.iter().all(|x|x.is_finite())));
        assert!(after.iter().all(|p|p.position[1]>=0.1));
    }
    #[test]fn collision_raycast_targets_real_ground(){
        let sim=RapierAetheria::new();
        let hit=sim.floor_hit_distance([0.,8.,0.],20.).unwrap();
        assert!((hit-8.0).abs()<0.3);
        assert_eq!(sim.floor_hit_distance([0.,8.,0.],3.),None);
    }
    #[test]fn input_guard_and_hard_body_budget(){
        let mut sim=RapierAetheria::new();
        assert_eq!(sim.spawn(FantasyBody::IceShard,[f32::NAN,0.,0.],[0.;3],0.2),
                   Err(SimError::InvalidPosition));
        assert_eq!(sim.spawn(FantasyBody::IceShard,[0.,1.,0.],[f32::INFINITY,0.,0.],0.2),
                   Err(SimError::InvalidVelocity));
        assert_eq!(sim.spawn(FantasyBody::IceShard,[0.,1.,0.],[0.;3],-1.),
                   Err(SimError::InvalidRadius));
        for i in 0..MAX_BODIES{
            sim.spawn(FantasyBody::WindTossedDebris,
                [i as f32*0.1,10.0,0.0],[0.;3],0.07).unwrap();
        }
        assert_eq!(sim.count(),MAX_BODIES);
        assert_eq!(sim.spawn(FantasyBody::IceShard,[0.,10.,0.],[0.;3],0.1),
                   Err(SimError::BodyBudget));
    }
    #[test]fn reproducible_positions_at_fixed_step_count(){
        let mut a=RapierAetheria::new();
        let mut b=RapierAetheria::new();
        for s in [&mut a,&mut b] {
            s.spawn(FantasyBody::EmberStone,[0.,3.,0.],[3.,4.,0.],0.2).unwrap();
            s.step(32);
        }
        assert_eq!(a.positions(),b.positions());
    }
}
