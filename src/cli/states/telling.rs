#[derive(Debug)]
pub enum TellingCommands {
    Speak { message: String },
    Examine { item: String },
    MoveTo { location: String },
    EquipItem { item_name: String },
    UnequipItem { item_name: String },
    EquipSpell { spell_name: String },
    UnequipSpell { spell_name: String },
    SpendLevelPoints { points: u32 },
    DropItem { item_name: String },
    PickupItem { item_name: String },
}

#[derive(Debug,Clone)]
pub struct TellingState {
    pub characters: Vec<Character>,
    pub narrative: String,
}

impl TellingState {
    pub fn new(characters: Vec<Character>, narrative: String) -> Self {
        TellingState { characters, narrative }
    }

    pub fn execute_command(&mut self, command: TellingCommands) {
        match command {
            TellingCommands::Speak { message } => {
                println!("Speaking: {}", message);
                // Implement speaking logic
            }
            TellingCommands::Examine { item } => {
                println!("Examining {}", item);
                // Implement examining logic
            }
            TellingCommands::MoveTo { location } => {
                println!("Moving to {}", location);
                // Implement movement logic
            }
            TellingCommands::EquipItem { item_name } => {
                println!("Equipping item: {}", item_name);
                // Implement equip item logic
            }
            TellingCommands::UnequipItem { item_name } => {
                println!("Unequipping item: {}", item_name);
                // Implement unequip item logic
            }
            TellingCommands::EquipSpell { spell_name } => {
                println!("Equipping spell: {}", spell_name);
                // Implement equip spell logic
            }
            TellingCommands::UnequipSpell { spell_name } => {
                println!("Unequipping spell: {}", spell_name);
                // Implement unequip spell logic
            }
            TellingCommands::SpendLevelPoints { points } => {
                println!("Spending {} level points", points);
                // Implement spending level points logic
            }
            TellingCommands::DropItem { item_name } => {
                println!("Dropping item: {}", item_name);
                // Implement drop item logic
            }
            TellingCommands::PickupItem { item_name } => {
                println!("Picking up item: {}", item_name);
                // Implement pickup item logic
            }
        }
    }
}
