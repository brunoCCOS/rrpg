use elements::race::Race;

enum Pool {
    Mana,
    Health,
    Stamina
}

pub struct Character {
    name: String,
    level: u8,
    sex: bool,
    race: Race,
    stats: Stats,
    age: u8,
    health: u8,
    mana: u8,
    stamina: u8,
}

impl Character{
    fn create_character(name: &str, sex: bool, race: Race, age: u8) {
        Character {
            name: name,
            level: 1,
            sex: sex,
            race: race,
            stats: Race.attributes(),
            age: age,
            health: set_pool_max(Pool::Health,Race.attributes()),
            mana: set_pool_max(Pool::Mana,Race.attributes()),
            stamina: set_pool_max(Pool::Stamina,Race.attributes())
        }
    }


    fn spent_points(&self) {

    }

    fn level_up(&self) {

    }

    fn age_up(&self) {

    }

    fn set_pool_max(&self, pool: Pool,stats: Stats)-> u8{
        match pool {
            Pool::Health => 100 + 10*stats.constitution + 5*stats.strength,
            Pool::Stamina => 100 + 10*stats.constitution + 5*stats.dexterity,
            Pool::Mana => 100 + 10*stats.wisdom + 5*stats.intelligence,
        }
    }

    fn update_pool(&self,pool: Pool, qunatity: u8){

    }
}