// src/state/combat.rs
#[derive(Debug)]
pub enum CombatCommands {
    Attack { target: String },
    Move { x: i32, y: i32 },
    UseItem { item_name: String },
    Flee,
}


#[derive(Debug,Clone)]
pub struct CombatState {
    pub characters: Vec<Character>,
    pub enemies: Vec<Character>, // Assuming enemies are also represented by the Character struct
}

impl CombatState {
    pub fn new(characters: Vec<Character>, enemies: Vec<Character>) -> Self {
        CombatState { characters, enemies }
    }

    pub fn execute_command(&mut self, command: CombatCommands) {
        match command {
            CombatCommands::Attack { target } => {
                println!("Attacking {}", target);
                // Implement attack logic
            }
            CombatCommands::Move { x, y } => {
                println!("Moving to ({}, {})", x, y);
                // Implement move logic
            }
            CombatCommands::UseItem { item_name } => {
                println!("Using item {}", item_name);
                // Implement item usage logic
            }
            CombatCommands::Flee => {
                println!("Fleeing combat");
                // Implement flee logic
            }
        }
    }
}