#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SkillNode{pub spell_id:u16,pub x:f32,pub y:f32}
pub fn constellation(class_id:u8)->Vec<SkillNode>{
 if class_id>=10{return Vec::new();}
 (0..20).map(|i|{
    let a=i as f32*2.3999632;let r=(i as f32).sqrt()*0.16;
    SkillNode{spell_id:class_id as u16*20+i+1,x:r*a.cos(),y:r*a.sin()}
 }).collect()
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn node_count_is_twenty(){assert_eq!(constellation(9).len(),20);}
}
