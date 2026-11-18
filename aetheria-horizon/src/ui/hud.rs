#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ManaHud{pub fill:f32,pub wave_amplitude:f32}
pub fn view(mana:f32,max_mana:f32)->ManaHud{
 let fill=if max_mana>0.0{(mana/max_mana).clamp(0.0,1.0)}else{0.0};
 ManaHud{fill,wave_amplitude:0.02+(1.0-fill)*0.06}
}
