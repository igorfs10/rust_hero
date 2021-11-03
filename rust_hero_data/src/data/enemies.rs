//! Enemies - Data and structs related to enemies.
use super::items::ItemType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Enemies {
    /// The very prevalent rodent
    Rat,
    /// The fairly common forest creature
    Rabbit,
    /// The hard to find forest snake
    Snake,
    /// The ultra rare Forest creature
    Wolf,
    /// These are rare to find in a swamp, thankfully
    Crocodile,
}

/// Our basic enemy
pub struct Enemy {
    /// The enemy name (for display)
    pub name: &'static str,
    /// The enemy's health
    pub health: u8,
    /// The enemy's attack
    pub attack: u8,
    /// The enemy's defense
    pub defense: u8,
    /// The experience you get
    pub experience: u16,
    /// The item the enemy carries
    pub item: ItemType,
    /// The image filename
    pub image: &'static str,
}
//TODO move stats to struct?
//TODO enemy will need an image filename
impl Enemy {
    /// Get the enemy from an `Enemies` enum
    pub const fn get_enemy(enemy: &Enemies) -> Self {
        match enemy {
            Enemies::Rat => Enemy {
                name: "Rat",
                health: 5,
                attack: 1,
                defense: 1,
                experience: 2,
                item: ItemType::Defense,
                image: "assets/enemies/rat.png",
            },
            Enemies::Rabbit => Enemy {
                name: "Rabbit",
                health: 8,
                attack: 2,
                defense: 2,
                experience: 4,
                item: ItemType::Attack,
                image: "assets/enemies/rabbit.png",
            },
            Enemies::Snake => Enemy {
                name: "Snake",
                health: 12,
                attack: 3,
                defense: 4,
                experience: 8,
                item: ItemType::Health,
                image: "assets/enemies/snake.png",
            },
            Enemies::Wolf => Enemy {
                name: "Wolf",
                health: 12,
                attack: 4,
                defense: 3,
                experience: 8,
                item: ItemType::Experience,
                image: "assets/enemies/wolf.png",
            },
            Enemies::Crocodile => Enemy {
                name: "Crocodile",
                health: 12,
                attack: 4,
                defense: 3,
                experience: 8,
                item: ItemType::Experience,
                image: "assets/enemies/crocodile.png",
            },
        }
    }
}
