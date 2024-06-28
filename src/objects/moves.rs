pub enum MoveType {
    Physical,
    Magical,
    Vital
}

pub struct Move {
    name: String,
    description: String,
    movetype: MoveType,
    cost: u16,
}

impl Move{
    pub fn show_name(&self) -> &str {
        &self.name
    }

    pub fn show_description(&self) -> &str {
        &self.description
    }

    pub fn move_type(&self) -> &MoveType {
        &self.movetype
    }

    pub fn cost(&self) -> u16 {
        self.cost
    }
}