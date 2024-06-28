use crate::objects::items::Kind;
use crate::objects::items::access_item;
use super::traits::Traits;

#[derive(Debug,Clone)]
pub struct Gear {
    helmet: usize,
    chest: usize,
    legs: usize,
    boots: usize,
    hands: usize,
    weapon: usize,
    off_hand: usize,
    ring: usize,
    necklace: usize
}


impl Gear {
    pub fn new() -> Gear {
        Gear {
            helmet: 0,
            chest: 0,
            legs: 0,
            boots: 0,
            hands: 0,
            weapon: 0,
            off_hand: 0,
            ring: 0,
            necklace: 0
        }
    }

    pub fn equip(&mut self,item: usize) -> Option<usize> {
        if let Some(item_obj) = access_item(item){
            let item_kind: Kind = (*item_obj).kind;
            match item_kind {
                Kind::Helmet => {self.helmet = item; Some(item)},
                Kind::Chest => {self.chest = item; Some(item)},
                Kind::Legs => {self.legs = item; Some(item)},
                Kind::Boots => {self.boots = item; Some(item)},
                Kind::Hands => {self.hands = item; Some(item)},
                Kind::Weapon => {self.weapon = item; Some(item)},
                Kind::OffHand => {self.off_hand = item; Some(item)},
                Kind::Ring => {self.ring = item; Some(item)},
                Kind::Necklace => {self.necklace = item; Some(item)},
                _ => {println!("Item not equipable"); Some(item)}
            }
        } else
        {
            println!("There's no such item");
            None
            }
    }

    pub fn get_gear_traits(&self) -> Traits {
        let gear_trait = access_item(self.helmet).unwrap().traits +
        access_item(self.chest).unwrap().traits +
        access_item(self.legs).unwrap().traits +
        access_item(self.boots).unwrap().traits +
        access_item(self.hands).unwrap().traits +
        access_item(self.weapon).unwrap().traits +
        access_item(self.off_hand).unwrap().traits +
        access_item(self.ring).unwrap().traits +
        access_item(self.necklace).unwrap().traits;
        gear_trait
    }
}