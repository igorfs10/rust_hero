//! Character - Struct used to store player and enemy data and use in battle

use crate::jogo::CRITICAL_MULTIPLIER;
use crate::utils::random::{RandomTrait, RandomValue};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
/// The different classes a character can be
//https://aknightsstory2.fandom.com/wiki/Main_Page
pub enum Class {
    Adept,
    Archer,
    Knight,
    Monk,
    Necromancer,
    Priest,
    Soldier,
    Thief,
    Valkyrie,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// The `Character` contains everything we need to battle!
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
    pub class:Class,
}

impl Character {
    /// Manually create a new `Character`
    const fn _new(name: String, health: u8, mana:u8, attack: u8, defense: u8, mana_attack:u8, mana_defense:u8, experience: u16, level:u8, class:Class) -> Self {
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
            class:class,
        }
    }
    /// Create a `Character` from a `Class`
    pub const fn from_class(name:String, class:Class) -> Self {
        let hp;
        let mp;
        let atk;
        let def;
        let m_attack;
        let m_def;
        // 150 total points based off of https://www.soulraver.net/sukie/AKS2/old/
        match class {
            Class::Adept => {
                hp = 40;
                mp = 16;
                atk = 14;
                def = 30;
                m_attack = 20;
                m_def = 30;
            },
            Class::Archer => {
                hp = 50;
                mp = 25;
                atk = 15;
                def = 10;
                m_attack = 15;
                m_def = 35;
            },
            Class::Knight => {
                hp = 50;
                mp = 20;
                atk = 20;
                def = 20;
                m_attack = 20;
                m_def = 20;
            },
            Class::Monk => {
                hp = 40;
                mp = 40;
                atk = 10;
                def = 15;
                m_attack = 5;
                m_def = 40;
            },
            Class::Necromancer => {
                hp = 70;
                mp = 40;
                atk = 1;
                def = 8;
                m_attack = 30;
                m_def = 1;
            },
            Class::Priest => {
                hp = 60;
                mp = 10;
                atk = 20;
                def = 10;
                m_attack = 10;
                m_def = 40;
            },
            Class::Soldier => {
                hp = 90;
                mp = 0;
                atk = 30;
                def = 12;
                m_attack = 0;
                m_def = 18;
            },
            Class::Thief => {
                hp = 40;
                mp = 70;
                atk = 15;
                def = 9;
                m_attack = 11;
                m_def = 30;
            },
            Class::Valkyrie => {
                hp = 50;
                mp = 10;
                atk = 20;
                def = 20;
                m_attack = 20;
                m_def = 30;
            },
        }
        Character {
            name:name,
            health:hp,
            max_health:hp,
            mana:mp,
            max_mana:mp,
            attack:atk,
            defense:def,
            mana_attack:m_attack,
            mana_defense:m_def,
            experience:0,
            level:0,
            class:class,
        }
    }

    /// The basic attack formula for our characters
    //TODO send in a stat struct from the enemy?
    pub fn attack(&self, defending_character: &mut Self, seed: &u64) -> (bool, u8, bool) {
        let mut damage;
        let mut defeated:bool = false;
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
        Self::from_class(String::from("Rusty"), Class::Knight)
    }
}
