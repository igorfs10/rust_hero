//! Character - Struct used to store player and enemy data and use in battle

use crate::jogo::MULTIPLICADOR_CRITICO;
use crate::utils::random::{RandomTrait, RandomValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub max_health: u8,
    pub health: u8,
    pub mana: u8,
    pub max_mana: u8,
    pub attack: u8,
    pub defense: u8,
    pub mana_attack: u8,
    pub mana_defense: u8,
    pub experience: u16,
    pub level: u8,
}

impl Character {
    const fn new(name: String, health: u8, mana:u8, attack: u8, defense: u8, mana_attack:u8, mana_defense:u8, experience: u16, level:u8) -> Self {
        Self {
            name,
            health,
            max_health: health,
            mana,
            max_mana:mana,
            attack,
            defense,
            mana_attack,
            mana_defense,
            experience,
            level,
        }
    }

    pub fn attack(&self, defending_character: &mut Self, seed: &u64) -> (bool, u8, bool) {
        let mut damage;
        let beated;
        let critical_hit = RandomValue::<bool>::get_random_value(seed, 25);

        if self.attack <= defending_character.defense {
            damage = 1;
        } else {
            damage = self.attack - defending_character.defense;
        }

        if critical_hit {
            damage *= MULTIPLICADOR_CRITICO;
        }

        if defending_character.health > damage {
            defending_character.health -= damage;
            beated = false;
        } else {
            damage = defending_character.health;
            defending_character.health = 0;
            beated = true;
        }
        (critical_hit, damage, beated)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new("".to_owned(), 10, 5, 1, 1, 1, 1, 0, 0) // is level 1 better to start than 0?
    }
}
