#[derive(Debug,Clone,Copy,PartialEq)]
pub struct WheelSlot{pub spell_id:u16,pub angle_radians:f32}
pub fn layout(ids:&[u16])->Vec<WheelSlot>{
 let count=ids.len().min(8);
 (0..count).map(|i|WheelSlot{
    spell_id:ids[i],angle_radians:std::f32::consts::TAU*i as f32/count as f32
        -std::f32::consts::FRAC_PI_2
 }).collect()
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn caps_wheel_at_eight(){assert_eq!(layout(&[1,2,3,4,5,6,7,8,9]).len(),8);}
 #[test]fn empty_wheel_is_safe(){assert_eq!(layout(&[]).len(),0);}
}
