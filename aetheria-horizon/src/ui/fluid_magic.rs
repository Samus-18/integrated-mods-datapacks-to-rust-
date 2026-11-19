//! M10 portable view model for the **fluid** Mana HUD and radial spell wheel.
//!
//! Deliberately renderer-independent: verified, deterministic values for a
//! future Horizon shader/GUI adapter. Not a claim that the UI is on-screen yet.
//! No network access, world edits, Vanilla sprites or GPU allocations here.
use std::f32::consts::TAU;
use crate::{
    core::player_data::PlayerMagic,
    magic::{registry::{by_id,CLASS_NAMES},spellbook::Spellbook},
};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct FluidOptions {
    /// Replaces wobble and ripples with stable geometry for accessibility.
    pub reduced_motion:bool,
    pub animation_intensity:f32,
}
impl Default for FluidOptions {
    fn default()->Self{Self{reduced_motion:false,animation_intensity:0.72}}
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ManaLiquid {
    pub fill:f32,
    /// Normalized liquid surface height at x (0..1).
    pub surface:f32,
    /// How much the liquid's surface reacts to an almost-empty mana pool.
    pub amplitude:f32,
    pub glow:f32,
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SpellWheelEntry {
    pub slot:usize,
    pub spell_id:Option<u16>,
    pub spell_name:Option<&'static str>,
    pub angle_radians:f32,
    /// Fraction of cooldown still remaining (1 = just cast; 0 = ready).
    pub cooldown_remaining:f32,
    pub cooldown_ticks:u64,
    pub selected:bool,
    pub castable:bool,
    pub mana_cost:f32,
}
#[derive(Debug,Clone,PartialEq)]
pub struct FluidMagicHud {
    pub class_name:&'static str,
    /// The format is a visual colour intent, not an engine-defined tint.
    pub accent_rgb:[f32;3],
    pub liquid:ManaLiquid,
    pub slots:Vec<SpellWheelEntry>,
}
const CLASS_PALETTE:[[f32;3];10]=[
    [0.98,0.39,0.12], // fire
    [0.39,0.71,0.92], // ice
    [0.49,0.85,0.88], // wind
    [0.68,0.52,0.84], // stone
    [0.17,0.63,0.91], // water
    [0.31,0.79,0.46], // verdant
    [0.58,0.64,0.97], // storm
    [0.60,0.37,0.81], // void
    [0.94,0.71,0.36], // time
    [0.72,0.31,0.49], // soul
];
fn fraction(value:f32,max:f32)->f32 {
    if max.is_finite()&&max>0.&&value.is_finite(){
        (value/max).clamp(0.,1.)
    }else{0.}
}
/// Normalized ripple sample for SDF liquid shader on a rounded mana glass.
/// Keeps the material readable when almost empty/full, even under huge ticks.
pub fn sample_liquid(mana:f32,max_mana:f32,x:f32,tick:u64,options:FluidOptions)
    ->ManaLiquid {
    let fill=fraction(mana,max_mana);
    let intensity=if options.reduced_motion {0.}
        else if options.animation_intensity.is_finite(){
            options.animation_intensity.clamp(0.,1.)
        }else{0.};
    let x=if x.is_finite(){x.clamp(0.,1.)}else{0.5};
    let t=(tick%2000) as f32 *0.09;
    let amplitude=(0.004+0.024*(1.-fill))*intensity;
    let ripple=(TAU*x*1.6+t).sin()*amplitude
        + (TAU*x*2.9-t*0.67).cos()*amplitude*0.28;
    ManaLiquid {
        fill,surface:(fill+ripple).clamp(0.,1.),
        amplitude,glow:(0.2+fill*0.63).clamp(0.,1.),
    }
}
pub fn fluid_magic_hud(
    player:&PlayerMagic,book:&Spellbook,tick:u64,options:FluidOptions
)->Option<FluidMagicHud>{
    let class=usize::from(player.selected_class);
    let (class_name,&accent)=
        (CLASS_NAMES.get(class)?,CLASS_PALETTE.get(class)?);
    let count=book.capacity();
    if !(4..=8).contains(&count){return None}
    let mut slots=Vec::with_capacity(count);
    for index in 0..count {
        let id=book.slot(index);
        let spec=id.and_then(by_id);
        let mana_cost=spec.map(|s|s.mana_cost).unwrap_or(0.);
        let duration=spec.map(|s|u64::from(s.cooldown_ticks)).unwrap_or(0);
        let until=id.and_then(|spell|player.cooldown_until_tick.get(&spell).copied())
            .unwrap_or(0);
        let remaining=until.saturating_sub(tick);
        let cooldown_remaining=if duration>0{
            (remaining as f32/duration as f32).clamp(0.,1.)
        }else{0.};
        let has_mana=player.mana.is_finite()&&player.mana>=mana_cost;
        let is_known=id.map(|spell|player.unlocked_nodes.contains(&spell))
            .unwrap_or(false);
        let belongs=spec.map(|s|s.class_id==player.selected_class).unwrap_or(false);
        slots.push(SpellWheelEntry {
            slot:index,spell_id:id,
            spell_name:spec.map(|s|s.name),
            angle_radians:TAU*index as f32/count as f32-TAU/4.,
            cooldown_remaining,
            cooldown_ticks:remaining,
            selected:index==book.selected_index(),
            castable:id.is_some()&&is_known&&belongs&&has_mana&&remaining==0,
            mana_cost,
        });
    }
    Some(FluidMagicHud{
        class_name,
        accent_rgb:accent,
        liquid:sample_liquid(player.mana,player.max_mana,0.5,tick,options),
        slots,
    })
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn ten_classes_get_correct_readable_wheel_and_orb(){
        for class in 0..10 {
            let p=PlayerMagic::new(class);
            let mut b=Spellbook::new(4).unwrap();
            b.equip(0,u16::from(class)*20+1,&p).unwrap();
            let hud=fluid_magic_hud(&p,&b,35,FluidOptions::default()).unwrap();
            assert_eq!(hud.class_name,CLASS_NAMES[usize::from(class)]);
            assert_eq!(hud.slots.len(),4);
            assert_eq!(hud.slots[0].selected,true);
            assert!(hud.slots[0].castable);
            assert_eq!(hud.slots[0].cooldown_remaining,0.);
            assert_eq!(hud.slots[1].spell_id,None);
            assert!(!hud.slots[1].castable);
            assert_eq!(hud.liquid.fill,1.);
        }
    }
    #[test]fn real_cast_cost_and_cooldown_affect_wheel(){
        let mut p=PlayerMagic::new(0);
        let mut b=Spellbook::new(5).unwrap();
        b.equip(0,1,&p).unwrap();
        p.mana=0.;
        assert!(!fluid_magic_hud(&p,&b,20,FluidOptions::default()).unwrap()
            .slots[0].castable);
        p.mana=99.;
        p.cooldown_until_tick.insert(1,90);
        let hud=fluid_magic_hud(&p,&b,20,FluidOptions::default()).unwrap();
        assert!(!hud.slots[0].castable);
        assert_eq!(hud.slots[0].cooldown_ticks,70);
        assert_eq!(hud.slots[0].cooldown_remaining,1.);
        assert!(fluid_magic_hud(&p,&b,91,FluidOptions::default()).unwrap()
            .slots[0].castable);
    }
    #[test]fn reduced_motion_and_invalid_float_produce_safe_ui(){
        let options=FluidOptions{reduced_motion:true,animation_intensity:f32::NAN};
        let a=sample_liquid(25.,100.,0.,1,options);
        let b=sample_liquid(25.,100.,1.,1999,options);
        assert_eq!(a.surface,0.25);
        assert_eq!(a,b);
        let bad=sample_liquid(f32::NAN,-1.,f32::INFINITY,u64::MAX,
                              FluidOptions::default());
        assert!(bad.surface.is_finite());
        assert_eq!(bad.fill,0.);
        assert!((0.0..=1.0).contains(&bad.surface));
    }
    #[test]fn eight_slots_have_finite_distinct_angles(){
        let p=PlayerMagic::new(2);
        let b=Spellbook::new(8).unwrap();
        let hud=fluid_magic_hud(&p,&b,0,FluidOptions::default()).unwrap();
        assert_eq!(hud.slots.len(),8);
        for window in hud.slots.windows(2) {
            assert!((window[1].angle_radians-window[0].angle_radians-TAU/8.)
                    .abs()<1e-5);
        }
    }
}
