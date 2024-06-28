use crate::elements::{
        gear::Gear,
        inventory::Inventory,
        stats::Stats,
        traits::Traits
    };

use crate::objects::race::Race;

const POINTS_PER_LEVEL:u8 = 5;
enum Pool {
    Mana,
    Health,
    Stamina
}

#[derive(Debug,Clone)]
pub struct Character {
    name: String,
    level: u16,
    sex: bool,
    race: Race,
    stats: Stats,
    age: u16,
    health: u16,
    mana: u16,
    stamina: u16,
    invetory: Inventory,
    gear: Gear,
    traits: Traits
}

impl Character{
    fn create_character(name: String, sex: bool, race: Race, age: u16) -> Character{
        Character {
            name,
            level: 1,
            sex,
            race,
            stats: race.attributes(),
            age,
            health: Character::set_pool_max(Pool::Health,&race.attributes()),
            mana: Character::set_pool_max(Pool::Mana,&race.attributes()),
            stamina: Character::set_pool_max(Pool::Stamina,&race.attributes()),
            invetory: Inventory::new(10), //TODO implement a mechanism to calculate strength
            gear: Gear::new(),
            traits: Traits::new()
        }
    }

    pub fn spend_level_points(&mut self, points: Stats) {
        self.level += points.get_level();
        self.stats += points;
        self.health =  Character::set_pool_max(Pool::Health,&self.stats);
        self.mana = Character::set_pool_max(Pool::Mana,&self.stats);
        self.stamina =  Character::set_pool_max(Pool::Stamina,&self.stats);
            // Implement logic to apply points to stats or abilities
        }

    pub fn age_up(&mut self) -> () {
        self.age += 1;
    }

    fn set_pool_max( pool: Pool,stats: &Stats)-> u16{
        match pool {
            Pool::Health => 100 + 10*stats.constitution + 5*stats.strength,
            Pool::Stamina => 100 + 10*stats.constitution + 5*stats.dexterity,
            Pool::Mana => 100 + 10*stats.wisdom + 5*stats.intelligence,
        }
    }

    pub fn update_pool(&mut self,pool: Pool, qunatity: u16){
        match pool {
            Pool::Health => self.health += qunatity,
            Pool::Stamina => self.stamina += qunatity,
            Pool::Mana => self.mana += qunatity,
        }
    }

    pub fn equip_item(&mut self, item: usize){
        self.gear.equip(item);
        self.traits = self.gear.get_gear_traits();
    }

}