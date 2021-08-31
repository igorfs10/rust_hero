//! Weapons - Data and structs related to weapons.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize)]
/// The enumeration of the different Weapons we have in the game
pub enum Weapons {
    /// No Weapon
    None,
    /// A basic Sword
    Sword,
    /// A basic Shield
    Shield,
    /// A basic Spear
    Spear,
}

impl FromStr for Weapons {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sword" => Ok(Weapons::Sword),
            "Shield" => Ok(Weapons::Shield),
            "Spear" => Ok(Weapons::Spear),
            _ => Ok(Weapons::None),
        }
    }
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
    /// The function to get the weapon details from a `Weapons` enum
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
