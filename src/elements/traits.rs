use std::ops::Add;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone,Copy)]
pub struct Traits {
    fire_res: u16,
    cold_res: u16,
    light_res: u16,
    dark_res: u16,
    water_res: u16,
    thunder_res:u16,
    poison_res: u16,
    bonus_health: u16,
    bonus_mana: u16,
    bonus_stamina: u16,
    armor: u16,
    damage: u16,
    religion: u16,
    stealth: u16,
    medicine: u16,
    intimidation: u16,
    fire_dmg: u16,
    cold_dmg: u16,
    light_dmg: u16,
    dark_dmg: u16,
    water_dmg: u16,
    thunder_dmg:u16,
    poison_dmg: u16,
    piercing_dmg: u16,
    spell_dmg: u16,
    spell_res: u16,
    mana_regen: u16,
    heatlh_regen: u16,
    stamina_regen: u16,
    speed: u16,
    crit_chance: u8,
}

impl Add for Traits {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            fire_res: self.fire_res + other.fire_res,
            cold_res: self.cold_res + other.cold_res,
            light_res: self.light_res + other.light_res,
            dark_res: self.dark_res + other.dark_res,
            water_res: self.water_res + other.water_res,
            thunder_res: self.thunder_res + other.thunder_res,
            poison_res: self.poison_res + other.poison_res,
            bonus_health: self.bonus_health + other.bonus_health,
            bonus_mana: self.bonus_mana + other.bonus_mana,
            bonus_stamina: self.bonus_stamina + other.bonus_stamina,
            armor: self.armor + other.armor,
            damage: self.damage + other.damage,
            religion: self.religion + other.religion,
            stealth: self.stealth + other.stealth,
            medicine: self.medicine + other.medicine,
            intimidation: self.intimidation + other.intimidation,
            fire_dmg: self.fire_dmg + other.fire_dmg,
            cold_dmg: self.cold_dmg + other.cold_dmg,
            light_dmg: self.light_dmg + other.light_dmg,
            dark_dmg: self.dark_dmg + other.dark_dmg,
            water_dmg: self.water_dmg + other.water_dmg,
            thunder_dmg: self.thunder_dmg + other.thunder_dmg,
            poison_dmg: self.poison_dmg + other.poison_dmg,
            piercing_dmg: self.piercing_dmg + other.piercing_dmg,
            spell_dmg: self.spell_dmg + other.spell_dmg,
            spell_res: self.spell_res + other.spell_res,
            mana_regen: self.mana_regen + other.mana_regen,
            heatlh_regen: self.heatlh_regen + other.heatlh_regen,
            stamina_regen: self.stamina_regen + other.stamina_regen,
            speed: self.speed + other.speed,
            crit_chance: self.crit_chance + other.crit_chance,
        }
    }
}
