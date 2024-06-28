use crate::elements::traits::Traits;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Debug, Deserialize, Serialize,Clone, Copy)]
pub enum Kind {
    Chest,
    Helmet,
    Hands,
    Boots,
    Legs,
    Necklace,
    Ring,
    Weapon,
    OffHand,
    Books,
    Misc,
    Consumables,
    Keys,
    NonConsumables,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub traits: Traits,
    pub kind: Kind,
}

pub static mut ITEMS_BIB: Option<&'static mut [Item]> = None;  // Placeholder for the fixed-size array

impl Item {
    pub fn from_json(file_path: &str) {
        let file_content = fs::read_to_string(file_path).expect("Unable to read file");
        let items: Vec<Item> = from_str(&file_content).expect("Unable to parse JSON");

        let item_count = items.len();
        let mut item_array: Vec<Item> = Vec::with_capacity(item_count);
        item_array.extend(items);

        unsafe {
            ITEMS_BIB = Some(Box::leak(item_array.into_boxed_slice()));
        }
    }
}

pub fn access_item(item_id: usize) -> Option<&'static Item> {
    unsafe {
        ITEMS_BIB.as_ref().and_then(|items| items.get(item_id))
    }
}
