use crate::core::player_data::PlayerMagic;
use super::registry::by_id;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum BookError{Capacity,InvalidSlot,UnknownSpell,WrongClass,Locked,Duplicate,MaxTier,InsufficientMaterials}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Spellbook{
    slots:Vec<Option<u16>>,
    selected:usize,
    pub tier:u8,
}
impl Spellbook{
    pub fn new(slots:usize)->Result<Self,BookError>{
        if !(4..=8).contains(&slots){return Err(BookError::Capacity);}
        Ok(Self{slots:vec![None;slots],selected:0,tier:1})
    }
    pub fn capacity(&self)->usize{self.slots.len()}
    pub fn selected_spell(&self)->Option<u16>{self.slots[self.selected]}
    pub fn slot(&self,index:usize)->Option<u16>{self.slots.get(index).copied().flatten()}
    pub fn select(&mut self,index:usize)->Result<(),BookError>{
        if index>=self.slots.len(){return Err(BookError::InvalidSlot);}
        self.selected=index;Ok(())
    }
    pub fn equip(&mut self,index:usize,id:u16,player:&PlayerMagic)->Result<(),BookError>{
        if index>=self.slots.len(){return Err(BookError::InvalidSlot);}
        let spec=by_id(id).ok_or(BookError::UnknownSpell)?;
        if spec.class_id!=player.selected_class{return Err(BookError::WrongClass);}
        if !player.unlocked_nodes.contains(&id){return Err(BookError::Locked);}
        if self.slots.iter().enumerate().any(|(i,s)|i!=index&&*s==Some(id)){
            return Err(BookError::Duplicate);
        }
        self.slots[index]=Some(id);Ok(())
    }
    pub fn unequip(&mut self,index:usize)->Result<(),BookError>{
        if index>=self.slots.len(){return Err(BookError::InvalidSlot);}
        self.slots[index]=None;Ok(())
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ArcaneIngredients{pub scrolls:u8,pub crystals:u16}
pub struct InscriptionTable;
impl InscriptionTable{
    /// Tier upgrade I→V; cost is paid transactionally on successful upgrade.
    pub fn upgrade(book:&mut Spellbook,inventory:&mut ArcaneIngredients)->Result<(),BookError>{
        if book.tier>=5{return Err(BookError::MaxTier);}
        let cost_scrolls=book.tier;let cost_crystals=(book.tier as u16)*4;
        if inventory.scrolls<cost_scrolls||inventory.crystals<cost_crystals{
            return Err(BookError::InsufficientMaterials);
        }
        inventory.scrolls-=cost_scrolls;inventory.crystals-=cost_crystals;
        book.tier+=1;Ok(())
    }
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn enforces_four_to_eight_slots(){
    assert_eq!(Spellbook::new(3),Err(BookError::Capacity));
    assert_eq!(Spellbook::new(9),Err(BookError::Capacity));
    assert_eq!(Spellbook::new(6).unwrap().capacity(),6);
 }
 #[test]fn equipment_is_class_locked(){
    let mut p=PlayerMagic::new(0);let mut b=Spellbook::new(4).unwrap();
    b.equip(0,1,&p).unwrap();assert_eq!(b.selected_spell(),Some(1));
    assert_eq!(b.equip(1,21,&p),Err(BookError::WrongClass));
    p.unlock(2);b.equip(1,2,&p).unwrap();
    assert_eq!(b.equip(2,2,&p),Err(BookError::Duplicate));
    b.select(1).unwrap();assert_eq!(b.selected_spell(),Some(2));
 }
 #[test]fn upgrades_do_not_charge_on_failure(){
    let mut b=Spellbook::new(4).unwrap();
    let mut inv=ArcaneIngredients{scrolls:0,crystals:10};
    assert_eq!(InscriptionTable::upgrade(&mut b,&mut inv),Err(BookError::InsufficientMaterials));
    assert_eq!(b.tier,1);assert_eq!(inv.crystals,10);
    inv.scrolls=15;inv.crystals=100;
    for target in 2..=5 {InscriptionTable::upgrade(&mut b,&mut inv).unwrap();assert_eq!(b.tier,target);}
    assert_eq!(InscriptionTable::upgrade(&mut b,&mut inv),Err(BookError::MaxTier));
 }
}
