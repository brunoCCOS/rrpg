use crate::objects::items::Kind;
use crate::objects::items::access_item;

use super::traits::Traits;  // Import the ITEMS_BIB variable

struct Gear {
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

    fn get_gear_traits(&self) -> Traits {
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