//! Jogo - Module containing functions and data for a the game play

use crate::data::enemies::Enemy;
use crate::data::locations::Location;
use crate::structs::character::Character;
use crate::utils::random::{RandomTrait, RandomValue};

pub type TipoFlag = u32;

/// The critical hit multiplier
pub const CRITICAL_MULTIPLIER: u8 = 2;

pub fn des_criptografar(valor: &u32, chave: &u32) -> u32 {
    *valor ^ *chave
}

/// Pick an enemy based on the location
pub fn pick_location_enemy(location: &Location, seed: &u64) -> Option<Enemy> {
    match &location.enemies {
        Some(enemies) => {
            let enemy_range = RandomValue::<u8>::get_random_value(seed, 1..=10);

            if enemy_range <= 4 {
                Some(Enemy::get_enemy(&enemies[0]))
            } else if enemy_range <= 7 {
                Some(Enemy::get_enemy(&enemies[1]))
            } else if enemy_range <= 9 {
                Some(Enemy::get_enemy(&enemies[2]))
            } else {
                Some(Enemy::get_enemy(&enemies[3]))
            }
        }
        None => None,
    }
}

/// Define a character based on an enemy
pub fn define_enemy(character: &mut Character, enemy: Enemy) {
    character.name = enemy.name.to_owned();
    character.attack = enemy.attack;
    character.defense = enemy.defense;
    character.max_health = enemy.health;
    character.health = enemy.health;
    character.experience = enemy.experience;
}
