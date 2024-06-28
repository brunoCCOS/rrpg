use std::ops::{Add, AddAssign};

#[derive(Debug,Clone)]
pub struct Stats {
    pub strength: u16,
    pub dexterity: u16,
    pub charisma: u16,
    pub intelligence: u16,
    pub constitution: u16,
    pub wisdom: u16,
    pub lucky: u16,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            charisma: self.charisma + other.charisma,
            intelligence: self.intelligence + other.intelligence,
            constitution: self.constitution + other.constitution,
            wisdom: self.wisdom + other.wisdom,
            lucky: self.lucky + other.lucky,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            charisma: self.charisma + other.charisma,
            intelligence: self.intelligence + other.intelligence,
            constitution: self.constitution + other.constitution,
            wisdom: self.wisdom + other.wisdom,
            lucky: self.lucky + other.lucky,
        };
    }}

impl Stats {

    pub fn get_level(&self) -> u16 {
        self.strength +
        self.dexterity +
        self.charisma +
        self.intelligence +
        self.constitution +
        self.wisdom +
        self.lucky
    }
}