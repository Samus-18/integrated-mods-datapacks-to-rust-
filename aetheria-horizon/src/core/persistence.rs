//! Portable v1 text save; world storage and client sync are not yet hooked.
use super::player_data::PlayerMagic;
#[derive(Debug,PartialEq)]
pub enum SaveError{BadVersion,BadValue(&'static str)}
pub fn encode(p:&PlayerMagic)->String {
    let unlocked=p.unlocked_nodes.iter().map(u16::to_string).collect::<Vec<_>>().join(",");
    let cooldowns=p.cooldown_until_tick.iter().map(|(id,t)|format!("{id}:{t}"))
        .collect::<Vec<_>>().join(",");
    format!("aetheria-save=1\nclass={}\nmana={}\nmax={}\nregen={}\nunlocked={}\ncooldowns={}\n",
        p.selected_class,p.mana,p.max_mana,p.mana_regen_per_second,unlocked,cooldowns)
}
pub fn decode(text:&str)->Result<PlayerMagic,SaveError>{
    if text.lines().next()!=Some("aetheria-save=1"){return Err(SaveError::BadVersion);}
    let field=|key:&'static str|->Result<&str,SaveError> {
        text.lines().find_map(|l|l.strip_prefix(&format!("{key}=")))
            .ok_or(SaveError::BadValue(key))
    };
    let class:u8=field("class")?.parse().map_err(|_|SaveError::BadValue("class"))?;
    if class>=10{return Err(SaveError::BadValue("class"));}
    let mut player=PlayerMagic::new(class);
    player.mana=field("mana")?.parse().map_err(|_|SaveError::BadValue("mana"))?;
    player.max_mana=field("max")?.parse().map_err(|_|SaveError::BadValue("max"))?;
    player.mana_regen_per_second=field("regen")?.parse().map_err(|_|SaveError::BadValue("regen"))?;
    if !player.mana.is_finite()||!player.max_mana.is_finite()||!player.mana_regen_per_second.is_finite()||
       player.max_mana<=0.0||player.mana<0.0||player.mana>player.max_mana||player.mana_regen_per_second<0.0
       {return Err(SaveError::BadValue("mana/max/regen"));}
    player.unlocked_nodes.clear();
    for item in field("unlocked")?.split(',').filter(|x|!x.is_empty()) {
        let id:u16=item.parse().map_err(|_|SaveError::BadValue("unlocked"))?;
        if id==0||(id-1)/20!=class as u16 {return Err(SaveError::BadValue("unlocked"));}
        player.unlocked_nodes.insert(id);
    }
    player.cooldown_until_tick.clear();
    for item in field("cooldowns")?.split(',').filter(|x|!x.is_empty()){
        let (id,t)=item.split_once(':').ok_or(SaveError::BadValue("cooldowns"))?;
        let id:u16=id.parse().map_err(|_|SaveError::BadValue("cooldowns"))?;
        if id==0||(id-1)/20!=class as u16{return Err(SaveError::BadValue("cooldowns"));}
        let tick:u64=t.parse().map_err(|_|SaveError::BadValue("cooldowns"))?;
        player.cooldown_until_tick.insert(id,tick);
    }
    Ok(player)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn round_trip(){
    let mut p=PlayerMagic::new(4);
    p.mana=42.5;p.unlock(85);p.cooldown_until_tick.insert(81,1700);
    assert_eq!(decode(&encode(&p)),Ok(p));
 }
 #[test]fn reject_corrupt_data(){assert_eq!(decode("bad"),Err(SaveError::BadVersion));}
 #[test]fn reject_wrong_class(){assert_eq!(decode("aetheria-save=1\nclass=55\n"),Err(SaveError::BadValue("class")));}
}
