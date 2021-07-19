//! Weapons - Data and structs related to weapons.

use crate::traits::dados_trait::DadosTrait;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize)]
pub enum Weapons {
    None,
    Sword,
    Shield,
    Spear,
}

/// Weapon's struct.
///
/// # Example to get a weapon using the weapon's enum.
/// ```
/// let weapon = Weapon::get_weapon(Weapons::Sword);
/// ```
pub struct Weapon {
    pub name: &'static str,
    pub attack: u8,
    pub defense: u8,
}

impl Weapon {
    pub const fn get_weapon(weapon: &Weapons) -> Self {
        match weapon {
            Weapons::None => Self {
                name: "None",
                attack: 0,
                defense: 0,
            },
            Weapons::Sword => Self {
                name: "Sword",
                attack: 3,
                defense: 1,
            },
            Weapons::Shield => Self {
                name: "Shield",
                attack: 1,
                defense: 3,
            },
            Weapons::Spear => Self {
                name: "Spear",
                attack: 2,
                defense: 2,
            },
        }
    }
}

impl DadosTrait for Weapon {
    fn get_dados(&self) -> String {
        format!(
            "{}\
            \nAttack: {}\
            \nDefense: {}",
            self.name, self.attack, self.defense
        )
    }
}

pub const WEAPONS: &[Weapon] = &[
    Weapon::get_weapon(&Weapons::None),
    Weapon::get_weapon(&Weapons::Sword),
    Weapon::get_weapon(&Weapons::Shield),
    Weapon::get_weapon(&Weapons::Spear),
];
