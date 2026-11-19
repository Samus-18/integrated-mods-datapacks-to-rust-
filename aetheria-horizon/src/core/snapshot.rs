//! M10 full-state save snapshot for the portable Aetheria RPG engine.
//!
//! This is NOT a Horizon world-save hook. Only an authenticated game-server
//! adapter may persist these bytes. The FNV-1a checksum detects accidental
//! corruption; it is NOT a MAC and does not authenticate client data.
//!
//! V1 migration preserves magic/unlocks and intentionally resets legacy
//! absolute cooldown ticks, which would otherwise be wrong in a new world.
//! V2 cooldowns save remaining duration, not absolute tick numbers.
use std::collections::{BTreeMap,BTreeSet};
use super::{authority::AuthoritativePlayer,player_data::PlayerMagic,persistence};
use crate::magic::{progression::PlayerProgression,spellbook::Spellbook};

pub const STATE_VERSION:u8=2;
const MAX_BYTES:usize=16_384;
const MAX_COOLDOWN:u64=72_000;
const FIELDS:[&str;17]=[
    "class","mana","max","regen","level","experience",
    "shards","sigils","essence","known","pages","trials",
    "book_tier","book_capacity","book_selected","book_entries","cooldowns",
];
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum SnapshotError{
    TooLarge,BadVersion,MissingField(&'static str),BadField(&'static str),
    IntegrityMismatch,LegacyInvalid,
}
fn invalid(name:&'static str)->SnapshotError{SnapshotError::BadField(name)}
fn checksum(bytes:&[u8])->u64{
    let mut hash=0xcbf29ce484222325u64;
    for &b in bytes {
        hash^=u64::from(b);
        hash=hash.wrapping_mul(0x100000001b3);
    }
    hash
}
fn ids(list:&BTreeSet<u16>)->String {
    list.iter().map(u16::to_string).collect::<Vec<_>>().join(",")
}
fn list_ids(raw:&str,field:&'static str,limit:usize)->Result<BTreeSet<u16>,SnapshotError>{
    let mut result=BTreeSet::new();
    if raw.is_empty(){return Ok(result)}
    for item in raw.split(','){
        if result.len()>=limit{return Err(invalid(field))}
        let id:u16=item.parse().map_err(|_|invalid(field))?;
        if !(1..=200).contains(&id)||!result.insert(id) {
            return Err(invalid(field));
        }
    }
    Ok(result)
}
fn number<T:std::str::FromStr>(raw:&str,field:&'static str)->Result<T,SnapshotError>{
    raw.parse().map_err(|_|invalid(field))
}
fn positive_finite(value:f32)->bool {value.is_finite() && value>0.0}
fn valid_player(p:&PlayerMagic)->Result<(),SnapshotError>{
    if p.selected_class>=10{return Err(invalid("class"))}
    if !positive_finite(p.max_mana)||p.max_mana>100_000.0
        ||!p.mana.is_finite()||p.mana<0.0||p.mana>p.max_mana
        ||!p.mana_regen_per_second.is_finite()
        ||p.mana_regen_per_second<0.0||p.mana_regen_per_second>1000.0{
        return Err(invalid("mana/max/regen"));
    }
    let starter=u16::from(p.selected_class)*20+1;
    if !p.unlocked_nodes.contains(&starter)
        ||p.unlocked_nodes.len()>20
        ||p.unlocked_nodes.iter().any(|&id| !(1..=200).contains(&id)
            ||(id-1)/20!=u16::from(p.selected_class)) {
        return Err(invalid("known"));
    }
    if p.cooldown_until_tick.len()>20
        ||p.cooldown_until_tick.keys().any(|&id| !(1..=200).contains(&id)
            ||(id-1)/20!=u16::from(p.selected_class)) {
        return Err(invalid("cooldowns"));
    }
    Ok(())
}
/// Encode a full, deterministic portable player snapshot. Save on the game
/// server only, never using data supplied by the client as authoritative.
pub fn encode_snapshot(state:&AuthoritativePlayer,now_tick:u64)
    ->Result<String,SnapshotError>{
    let p=&state.magic;
    valid_player(p)?;
    if !(1..=100).contains(&state.progression.arcane_level){
        return Err(invalid("level"))
    }
    if !(1..=5).contains(&state.book.tier) || !(4..=8).contains(&state.book.capacity())
        ||state.book.selected_index()>=state.book.capacity(){
        return Err(invalid("book"));
    }
    if state.progression.supply.pages.iter().chain(state.progression.completed_trials.iter())
        .any(|&id|!(1..=200).contains(&id)){
        return Err(invalid("pages/trials"))
    }
    let mut equipped=BTreeSet::new();
    let mut slots=Vec::with_capacity(state.book.capacity());
    for i in 0..state.book.capacity() {
        let id=state.book.slot(i).unwrap_or(0);
        if id!=0&&(!p.unlocked_nodes.contains(&id)||!equipped.insert(id)){
            return Err(invalid("book_entries"))
        }
        slots.push(id.to_string());
    }
    let cooldowns=p.cooldown_until_tick.iter().filter_map(|(id,&until)|{
        let remaining=until.saturating_sub(now_tick).min(MAX_COOLDOWN);
        (remaining>0).then_some(format!("{id}:{remaining}"))
    }).collect::<Vec<_>>().join(",");
    let values=[
        p.selected_class.to_string(),p.mana.to_string(),
        p.max_mana.to_string(),p.mana_regen_per_second.to_string(),
        state.progression.arcane_level.to_string(),state.progression.experience.to_string(),
        state.progression.supply.shards.to_string(),
        state.progression.supply.awakening_sigils.to_string(),
        state.progression.supply.primal_essence.to_string(),
        ids(&p.unlocked_nodes),ids(&state.progression.supply.pages),
        ids(&state.progression.completed_trials),
        state.book.tier.to_string(),state.book.capacity().to_string(),
        state.book.selected_index().to_string(),slots.join(","),cooldowns
    ];
    let mut data=format!("aetheria-state={STATE_VERSION}\n");
    for (key,value) in FIELDS.iter().zip(values.iter()){
        data.push_str(key);
        data.push('=');
        data.push_str(value);
        data.push('\n');
    }
    if data.len()+26>MAX_BYTES{return Err(SnapshotError::TooLarge)}
    let hash=checksum(data.as_bytes());
    data.push_str(&format!("checksum={hash:016x}\n"));
    Ok(data)
}
/// Restore without taking inventory or unlocks from a client packet.
/// The supplied tick belongs to the target world's current authoritative time.
pub fn decode_snapshot(text:&str,now_tick:u64)->Result<AuthoritativePlayer,SnapshotError>{
    if text.len()>MAX_BYTES {return Err(SnapshotError::TooLarge)}
    if text.starts_with("aetheria-save=1\n") {
        let mut legacy=persistence::decode(text)
            .map_err(|_|SnapshotError::LegacyInvalid)?;
        let class=legacy.selected_class;
        let mut state=AuthoritativePlayer::new(class);
        legacy.unlocked_nodes.insert(u16::from(class)*20+1);
        // Old snapshots had absolute ticks. Do not preserve invalid cooldowns.
        legacy.cooldown_until_tick.clear();
        valid_player(&legacy)?;
        state.magic=legacy;
        return Ok(state)
    }
    let lines=text.lines().collect::<Vec<_>>();
    if lines.len()!=FIELDS.len()+2
        ||lines[0]!="aetheria-state=2" {
        return Err(SnapshotError::BadVersion);
    }
    let raw_hash=lines.last().unwrap().strip_prefix("checksum=")
        .ok_or(SnapshotError::MissingField("checksum"))?;
    if raw_hash.len()!=16||!raw_hash.bytes().all(|c|c.is_ascii_hexdigit()){
        return Err(invalid("checksum"))
    }
    let expected=u64::from_str_radix(raw_hash,16).map_err(|_|invalid("checksum"))?;
    let mut payload=lines[..lines.len()-1].join("\n");
    payload.push('\n');
    if checksum(payload.as_bytes())!=expected {
        return Err(SnapshotError::IntegrityMismatch);
    }
    let mut values=Vec::with_capacity(FIELDS.len());
    for (index,key) in FIELDS.iter().enumerate(){
        let v=lines[index+1].strip_prefix(&format!("{key}="))
            .ok_or(SnapshotError::MissingField(key))?;
        values.push(v);
    }
    let class:u8=number(values[0],"class")?;
    if class>=10{return Err(invalid("class"))}
    let mut state=AuthoritativePlayer::new(class);
    state.magic.mana=number(values[1],"mana")?;
    state.magic.max_mana=number(values[2],"max")?;
    state.magic.mana_regen_per_second=number(values[3],"regen")?;
    state.progression.arcane_level=number(values[4],"level")?;
    if !(1..=100).contains(&state.progression.arcane_level){
        return Err(invalid("level"))
    }
    state.progression.experience=number(values[5],"experience")?;
    state.progression.supply.shards=number(values[6],"shards")?;
    state.progression.supply.awakening_sigils=number(values[7],"sigils")?;
    state.progression.supply.primal_essence=number(values[8],"essence")?;
    state.magic.unlocked_nodes=list_ids(values[9],"known",20)?;
    state.progression.supply.pages=list_ids(values[10],"pages",200)?;
    state.progression.completed_trials=list_ids(values[11],"trials",200)?;
    valid_player(&state.magic)?;
    let tier:u8=number(values[12],"book_tier")?;
    let capacity:usize=number(values[13],"book_capacity")?;
    let selected:usize=number(values[14],"book_selected")?;
    if !(1..=5).contains(&tier)||!(4..=8).contains(&capacity)||selected>=capacity {
        return Err(invalid("book"));
    }
    let entries=values[15].split(',').collect::<Vec<_>>();
    if entries.len()!=capacity {return Err(invalid("book_entries"))}
    let mut book=Spellbook::new(capacity).map_err(|_|invalid("book_capacity"))?;
    book.tier=tier;
    for (slot,value) in entries.into_iter().enumerate(){
        let id:u16=number(value,"book_entries")?;
        if id!=0 {
            book.equip(slot,id,&state.magic).map_err(|_|invalid("book_entries"))?;
        }
    }
    book.select(selected).map_err(|_|invalid("book_selected"))?;
    state.book=book;
    let mut cds=BTreeMap::new();
    if !values[16].is_empty(){
        for entry in values[16].split(','){
            let (raw_id,raw_time)=entry.split_once(':')
                .ok_or(invalid("cooldowns"))?;
            let id:u16=number(raw_id,"cooldowns")?;
            let remaining:u64=number(raw_time,"cooldowns")?;
            if !(1..=200).contains(&id)
                ||(id-1)/20!=u16::from(class)
                ||remaining==0||remaining>MAX_COOLDOWN
                ||cds.insert(id,now_tick.saturating_add(remaining)).is_some() {
                return Err(invalid("cooldowns"));
            }
        }
    }
    state.magic.cooldown_until_tick=cds;
    Ok(state)
}
#[cfg(test)]
mod tests{
    use super::*;
    use crate::{core::authority::{ClientRequest,PlayerCommand},
                magic::spellbook::ArcaneIngredients};
    fn full_state(class:u8)->AuthoritativePlayer{
        let mut state=AuthoritativePlayer::new(class);
        state.book=Spellbook::new(6).unwrap();
        state.book.equip(0,u16::from(class)*20+1,&state.magic).unwrap();
        let second=u16::from(class)*20+2;
        state.magic.unlock(second);
        state.magic.mana=23.75;
        state.magic.max_mana=180.;
        state.magic.mana_regen_per_second=7.5;
        state.magic.cooldown_until_tick.insert(second,1_355);
        state.progression.arcane_level=46;
        state.progression.experience=899;
        state.progression.supply.shards=216;
        state.progression.supply.awakening_sigils=3;
        state.progression.supply.primal_essence=2;
        state.progression.supply.pages.insert(second+1);
        state.progression.completed_trials.insert(second+13);
        state.book.tier=3;
        state.book.equip(1,second,&state.magic).unwrap();
        state.book.select(1).unwrap();
        state
    }
    #[test] fn round_trip_all_ten_classes_full_state_and_relative_cooldowns(){
        for class in 0..10 {
            let p=full_state(class);
            let bytes=encode_snapshot(&p,1000).unwrap();
            let q=decode_snapshot(&bytes,18_000).unwrap();
            assert_eq!(q.magic.selected_class,p.magic.selected_class);
            assert_eq!(q.magic.mana,p.magic.mana);
            assert_eq!(q.magic.max_mana,p.magic.max_mana);
            assert_eq!(q.magic.unlocked_nodes,p.magic.unlocked_nodes);
            assert_eq!(q.magic.cooldown_until_tick.get(&(u16::from(class)*20+2)),
                       Some(&18_355));
            assert_eq!(q.progression.arcane_level,46);
            assert_eq!(q.progression.experience,899);
            assert_eq!(q.progression.supply.pages,p.progression.supply.pages);
            assert_eq!(q.progression.completed_trials,p.progression.completed_trials);
            assert_eq!(q.progression.supply.shards,216);
            assert_eq!(q.book.tier,3);
            assert_eq!(q.book.capacity(),6);
            assert_eq!(q.book.selected_spell(),Some(u16::from(class)*20+2));
            assert_eq!(q.book.selected_index(),1);
            assert!(bytes.len()<MAX_BYTES);
        }
    }
    #[test] fn snapshot_is_stable_and_detects_tampering(){
        let p=full_state(2);
        let bytes=encode_snapshot(&p,1000).unwrap();
        assert_eq!(bytes,encode_snapshot(&p,1000).unwrap());
        let corrupted=bytes.replace("mana=23.75","mana=99999");
        assert_eq!(decode_snapshot(&corrupted,0).err(),
                   Some(SnapshotError::IntegrityMismatch));
        assert_eq!(decode_snapshot(&"z".repeat(MAX_BYTES+1),0).err(),
                   Some(SnapshotError::TooLarge));
    }
    #[test] fn invalid_internal_player_cannot_be_serialized(){
        let mut p=full_state(0);
        p.magic.mana=f32::NAN;
        assert_eq!(encode_snapshot(&p,10).err(),
                   Some(SnapshotError::BadField("mana/max/regen")));
        p.magic.mana=10.;
        p.magic.unlocked_nodes.clear();
        assert_eq!(encode_snapshot(&p,10).err(),
                   Some(SnapshotError::BadField("known")));
    }
    #[test] fn old_v1_migration_preserves_unlocks_but_resets_stale_timers(){
        let mut old=PlayerMagic::new(4);
        old.unlock(85);
        old.mana=44.25;
        old.cooldown_until_tick.insert(81,72_000);
        let v1=persistence::encode(&old);
        let v2=decode_snapshot(&v1,10).unwrap();
        assert_eq!(v2.magic.mana,44.25);
        assert!(v2.magic.unlocked_nodes.contains(&85));
        assert!(v2.magic.cooldown_until_tick.is_empty());
        assert_eq!(v2.book.selected_spell(),Some(81));
        assert!(v2.progression.supply.pages.is_empty());
        assert_eq!(v2.progression.arcane_level,1);
        assert_eq!(v2.last_sequence(),None);
    }
    #[test] fn invalid_payload_is_rejected_without_changing_active_player(){
        let mut p=full_state(1);
        let before=encode_snapshot(&p,1000).unwrap();
        let damaged=before.replace("book_entries=21,22,0,0,0,0",
                                   "book_entries=21,21,0,0,0,0");
        assert_ne!(damaged,before);
        // Recompute the integrity check: rejection must come from the
        // duplicate equipped spell, not just the modified checksum.
        let lines=damaged.lines().collect::<Vec<_>>();
        let payload=lines[..lines.len()-1].join("\n")+"\n";
        let valid_hash_but_invalid_book=format!(
            "{}checksum={:016x}\n",payload,checksum(payload.as_bytes()));
        assert_eq!(decode_snapshot(&valid_hash_but_invalid_book,1000).err(),
                   Some(SnapshotError::BadField("book_entries")));
        assert_eq!(encode_snapshot(&p,1000).unwrap(),before);
    }
    #[test] fn past_cooldowns_are_not_persisted(){
        let mut p=full_state(1);
        p.magic.cooldown_until_tick.insert(22,200);
        let encoded=encode_snapshot(&p,1000).unwrap();
        assert!(encoded.contains("cooldowns=\n"));
        assert!(decode_snapshot(&encoded,8000).unwrap().magic.cooldown_until_tick.is_empty());
    }
}
