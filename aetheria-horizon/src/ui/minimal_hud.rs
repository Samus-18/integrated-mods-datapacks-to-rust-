//! M11: compact, Iron-style **quick selection** without a permanent big wheel.
//! Portable layout/animation contracts. The Horizon client renderer owns pixels
//! and server state; this crate never fabricates vital values or casts spells.
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HudVitals {
    pub health:f32,
    pub max_health:f32,
    pub absorption:f32,
    pub armor:f32,
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Bars {
    pub health_fill:f32,
    pub absorption_fill:f32,
    pub armor_fill:f32,
}
impl HudVitals {
    pub fn normalized(self)->Option<Bars> {
        if !self.health.is_finite()||!self.max_health.is_finite()
            ||!self.armor.is_finite()||!self.absorption.is_finite()
            ||self.max_health<=0.0 {return None}
        Some(Bars {
            health_fill:(self.health/self.max_health).clamp(0.0,1.0),
            absorption_fill:(self.absorption/self.max_health).clamp(0.0,1.0),
            armor_fill:(self.armor/20.0).clamp(0.0,1.0),
        })
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CompactHudSettings {
    /// When true, only mana and the selected spell are drawn. Existing vanilla
    /// health/armor/food/hotbar stay visible (no duplicate statistics).
    pub preserve_vanilla_health:bool,
    pub reduced_motion:bool,
    /// Fraction of GUI-scaled width allowed for Aetheria, leaving view clear.
    pub max_width:f32,
}
impl Default for CompactHudSettings {
    fn default()->Self {
        Self{preserve_vanilla_health:true,reduced_motion:false,max_width:222.0}
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct GuiRect {pub x:f32,pub y:f32,pub w:f32,pub h:f32}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct MinimalHudLayout {
    pub mana:GuiRect,
    pub spell:GuiRect,
    /// Only exposed for a renderer explicitly replacing the vanilla layer.
    pub vitals:Option<GuiRect>,
    pub expanded:Option<GuiRect>,
}
/// Anchor a compact indicator *above* the vanilla hotbar, never at the
/// crosshair. Do not cover the center of the gameplay viewport.
pub fn layout(gui:[f32;2],selection_open:bool,settings:CompactHudSettings)
    ->Option<MinimalHudLayout> {
    let [w,h]=gui;
    if !w.is_finite()||!h.is_finite()||w<240.0||h<160.0
        ||!settings.max_width.is_finite()||settings.max_width<116.0 {
        return None
    }
    let max=settings.max_width.min(w*0.45).max(116.0);
    let mana_w=(max*0.73).min(150.0);
    let center=w*0.5;
    // Stay beside/above the hotbar rather than overlaying its icons.
    let y=(h-72.0).max(30.0);
    let spell=GuiRect{x:center+92.0,y:y+13.0,w:max-mana_w+15.0,h:17.0};
    let mana=GuiRect{x:center-mana_w*0.5,y:y-7.0,w:mana_w,h:9.0};
    let expanded=selection_open.then_some(GuiRect{
        x:(center-max*0.5).max(6.0),y:(y-64.0).max(14.0),
        w:max,h:49.0
    });
    let vitals=(!settings.preserve_vanilla_health).then_some(GuiRect{
        x:(center-91.0-max).max(4.0),y:y,w:max*0.60,h:25.0
    });
    Some(MinimalHudLayout{mana,spell,vitals,expanded})
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum QuickMove {Previous,Next}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct QuickSelector {
    pub selected:usize,
    pub visible_until_tick:u64,
}
impl Default for QuickSelector {
    fn default()->Self{Self{selected:0,visible_until_tick:0}}
}
impl QuickSelector{
    /// Skip empty slots; never choose a spell the player has not equipped.
    /// Returns true only when the selected slot actually changes.
    pub fn move_selection(&mut self,slots:&[Option<u16>],direction:QuickMove,
                          tick:u64)->bool {
        let len=slots.len().min(8);
        if len==0 {return false}
        if !slots[..len].iter().any(Option::is_some){return false}
        let old=self.selected.min(len-1);
        let mut target=old;
        for _ in 0..len{
            target=match direction {
                QuickMove::Next=>(target+1)%len,
                QuickMove::Previous=>(target+len-1)%len
            };
            if slots[target].is_some(){break}
        }
        self.selected=target;
        self.visible_until_tick=tick.saturating_add(50);
        old!=target
    }
    pub fn open(&mut self,tick:u64){self.visible_until_tick=tick.saturating_add(50);}
    pub fn visible(&self,tick:u64,held:bool)->bool {
        held||tick<self.visible_until_tick
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct MenuMotion {
    pub star_x:f32,
    pub star_y:f32,
    pub rune_rotation:f32,
    pub pulse:f32,
    pub link_charge:f32,
}
/// Slow deterministic drift and hover shine; background changes every frame
/// without generating any images or copying foreign art.
pub fn menu_motion(tick:u64,partial:f32,node_id:u16,reduced_motion:bool)->MenuMotion{
    if reduced_motion{return MenuMotion{
        star_x:0.,star_y:0.,rune_rotation:0.,pulse:0.5,link_charge:0.0
    }}
    let time=((tick%72_000) as f32+if partial.is_finite(){partial.clamp(0.,1.)}else{0.})/20.;
    let phase=time*0.24+node_id as f32*0.37;
    MenuMotion{
        star_x:(phase*0.3).sin()*1.6,
        star_y:(phase*0.24).cos()*1.1,
        rune_rotation:(time*0.10)%TAU,
        pulse:0.45+0.25*(phase*1.2).sin(),
        link_charge:(time*0.25+node_id as f32*0.05).fract()
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn health_armor_and_absorption_are_real_values(){
        let out=HudVitals{health:7.0,max_health:20.0,absorption:4.0,armor:16.0}
            .normalized().unwrap();
        assert_eq!(out.health_fill,0.35);
        assert_eq!(out.absorption_fill,0.2);
        assert_eq!(out.armor_fill,0.8);
        assert!(HudVitals{health:f32::NAN,..HudVitals{
            health:7.0,max_health:20.0,absorption:0.0,armor:5.0
        }}.normalized().is_none());
    }
    #[test]fn quick_selector_skips_empties_wraps_and_times_out(){
        let mut wheel=QuickSelector::default();
        let slots=[Some(2),None,Some(6),None,Some(8),None];
        assert!(wheel.move_selection(&slots,QuickMove::Previous,100));
        assert_eq!(wheel.selected,4);
        assert!(wheel.move_selection(&slots,QuickMove::Next,105));
        assert_eq!(wheel.selected,0);
        assert!(wheel.visible(150,false));
        assert!(!wheel.visible(156,false));
        assert!(wheel.visible(156,true));
        assert!(!wheel.move_selection(&[None,None],QuickMove::Next,200));
    }
    #[test]fn layout_preserves_view_and_vanilla(){
        let settings=CompactHudSettings::default();
        let hud=layout([854.,480.],false,settings).unwrap();
        assert!(hud.vitals.is_none());
        assert!(hud.expanded.is_none());
        assert!(hud.mana.w<=150.);
        assert!(hud.spell.x>854./2.);
        assert!(hud.mana.y>480./2.);
        assert!(layout([854.,480.],true,settings).unwrap().expanded.is_some());
        assert!(layout([f32::NAN,300.],true,settings).is_none());
    }
    #[test]fn menu_animation_is_bounded_and_accessible(){
        for tick in [0,1,2,300,50_000,u64::MAX]{
            let a=menu_motion(tick,0.4,12,false);
            assert!((0.2..=0.7).contains(&a.pulse));
            assert!((0.0..1.0).contains(&a.link_charge));
            assert!(a.rune_rotation.is_finite());
        }
        let a=menu_motion(0,0.0,9,true);
        let b=menu_motion(100,0.7,9,true);
        assert_eq!(a,b);
    }
}
