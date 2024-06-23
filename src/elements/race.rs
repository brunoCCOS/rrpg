pub enum Race {
    Human,
    Elf,
    Dwarf,
    Orc,
    Undead,
    Halfling,
    Fairy,
    Dragonborn,
    Demon,
    Giant,
    Trolls,
    Driades,
}

impl Race {
    pub fn attributes(&self) -> Stats {
        match self {
            Race::Human => RaceAttributes {
                health_bonus: 10,
                mana_bonus: 5,
                strength_bonus: 3,
                defense_bonus: 2,
            },
            Race::Elf => RaceAttributes {
                health_bonus: 5,
                mana_bonus: 10,
                strength_bonus: 2,
                defense_bonus: 3,
            },
            Race::Dwarf => RaceAttributes {
                health_bonus: 15,
                mana_bonus: 2,
                strength_bonus: 5,
                defense_bonus: 5,
            },
            Race::Orc => RaceAttributes {
                health_bonus: 12,
                mana_bonus: 3,
                strength_bonus: 6,
                defense_bonus: 4,
            },
        }
    }
}
