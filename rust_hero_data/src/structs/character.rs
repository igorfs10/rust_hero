//! Character - Struct used to store player and enemy data and use in battle

use crate::data::classes::{Class, Classes};
use crate::jogo::CRITICAL_MULTIPLIER;
use crate::utils::random::{RandomTrait, RandomValue};
use serde::{Deserialize, Serialize};
/// The different classes a character can be
//https://aknightsstory2.fandom.com/wiki/Main_Page

/// The `Character` contains everything we need to battle!
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Character {
    /// The name of the character
    pub name: String,
    /// The current health amount
    pub health: u8,
    /// The maximum health amount
    pub max_health: u8,
    /// The current mana amount
    pub mana: u8,
    /// The maximum mana amount
    pub max_mana: u8,
    /// The amount of damage the character can do
    pub attack: u8,
    /// The amount of defense the character has
    pub defense: u8,
    /// The amount of damage the character can do with mana
    pub mana_attack: u8,
    /// The amount of mana defense the character has
    pub mana_defense: u8,
    /// The total experience of the character
    pub experience: u16,
    /// The current level the character has reached
    pub level: u8,
    /// The specific type of character we have.
    pub class: Classes,
    /// The image filename
    pub image: String,
}

impl Character {
    /// Create a `Character` from a `Class`
    pub fn from_class(name: String, class: Classes) -> Self {
        let Class {
            hp,
            mp,
            atk,
            def,
            m_atk,
            m_def,
            image,
            ..
        } = Class::get_class(&class);
        // 150 total points based off of https://www.soulraver.net/sukie/AKS2/old/

        Character {
            name,
            health: hp,
            max_health: hp,
            mana: mp,
            max_mana: mp,
            attack: atk,
            defense: def,
            mana_attack: m_atk,
            mana_defense: m_def,
            experience: 0,
            level: 0,
            class,
            image: String::from(image),
        }
    }

    /// The basic attack formula for our characters
    pub fn attack(&self, defending_character: &mut Self, seed: &u64) -> (bool, u8, bool) {
        let mut damage;
        let mut defeated: bool = false;
        let critical_hit = RandomValue::<bool>::get_random_value(seed, 25);

        // If our attack is less than defense default to 1 damage
        if self.attack <= defending_character.defense {
            damage = 1;
        } else {
            damage = self.attack - defending_character.defense;
        }

        // Multiply our damage by our critical muliplier
        if critical_hit {
            damage *= CRITICAL_MULTIPLIER;
        }

        // if they have more health than our attack, damage them
        if defending_character.health > damage {
            defending_character.health -= damage;
        } else {
            // they were defeated, return the remainder of their health
            damage = defending_character.health;
            defending_character.health = 0;
            defeated = true;
        }
        (critical_hit, damage, defeated)
    }
}

impl Default for Character {
    /// Default to Rusty the Knight for our hero
    fn default() -> Self {
        Self::from_class(String::from("Rusty"), Classes::Knight)
    }
}
