use elements::race::Race;

enum Pool {
    Mana,
    Health,
    Stamina
}

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
    fn create_character(name: &str, sex: bool, race: Race, age: u16) {
        Character {
            name,
            level: 1,
            sex,
            race,
            stats: Race.attributes(),
            age,
            health: set_pool_max(Pool::Health,Race.attributes()),
            mana: set_pool_max(Pool::Mana,Race.attributes()),
            stamina: set_pool_max(Pool::Stamina,Race.attributes())
        }
    }

    fn spent_points(&self, points: u16) -> Stats {

    }

    pub fn level_up(&self) -> () {
        self.level += 1;
        self.stats += spent_points(5);
        self.health =  set_pool_max(Pool::Health,self.stats);
        self.mana =  set_pool_max(Pool::Mana,self.stats);
        self.stamina =  set_pool_max(Pool::Stamina,self.stats);
        ()
    }

    pub fn age_up(&self) -> () {
        self.age += 1;
        ()
    }

    fn set_pool_max(&self, pool: Pool,stats: Stats)-> u16{
        match pool {
            Pool::Health => 100 + 10*stats.constitution + 5*stats.strength,
            Pool::Stamina => 100 + 10*stats.constitution + 5*stats.dexterity,
            Pool::Mana => 100 + 10*stats.wisdom + 5*stats.intelligence,
        }
    }

    pub fn update_pool(&self,pool: Pool, qunatity: u16){
        match pool {
            Pool::Health => self.health += qunatity,
            Pool::Stamina => self.stamina += qunatity,
            Pool::Mana => self.mana += qunatity,
        }
    }

    pub fn equip_item(self, item: Item){
        self.gear.equip(self, item);
        self.traits = self.gear.get_gear_traits();
    }

}