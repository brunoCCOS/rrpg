
#[derive(Debug, Clone)]
pub enum TellingCommands {
    EquipItem { item_name: String },
    UnequipItem { item_name: String },
    EquipMove { spell_name: String },
    UnequipMove { spell_name: String },
    SpendLevelPoints { points: u32 },
    DropItem { item_name: String },
    PickupItem { item_name: String },
}

#[derive(Debug, Clone)]
pub struct TellingState {}

impl TellingState {
    pub fn new() -> Self {
        TellingState {}
    }

    pub fn execute_command(&self, command: TellingCommands) {
        match command {
            TellingCommands::EquipItem { item_name } => {
                println!("Equipping item: {}", item_name);
                // Implement equip item logic
            }
            TellingCommands::UnequipItem { item_name } => {
                println!("Unequipping item: {}", item_name);
                // Implement unequip item logic
            }
            TellingCommands::EquipMove { spell_name } => {
                println!("Equipping spell: {}", spell_name);
                // Implement equip spell logic
            }
            TellingCommands::UnequipMove { spell_name } => {
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
