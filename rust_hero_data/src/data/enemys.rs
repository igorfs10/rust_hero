//! Enemys - Data and structs related to enemys.

use super::items::Items;

pub enum Enemys {
    Rat,
    Rabbit,
    Snake,
    Wolf,
}

pub struct Enemy {
    pub name: &'static str,
    pub health: u8,
    pub attack: u8,
    pub defense: u8,
    pub experience: u16,
    pub item: Items,
}

impl Enemy {
    pub const fn get_enemy(enemy: &Enemys) -> Self {
        match enemy {
            Enemys::Rat => Enemy {
                name: "Rat",
                health: 5,
                attack: 1,
                defense: 1,
                experience: 2,
                item: Items::DefensePotion,
            },
            Enemys::Rabbit => Enemy {
                name: "Rabbit",
                health: 8,
                attack: 2,
                defense: 2,
                experience: 4,
                item: Items::AttackPotion,
            },
            Enemys::Snake => Enemy {
                name: "Snake",
                health: 12,
                attack: 3,
                defense: 4,
                experience: 8,
                item: Items::HealthPotion,
            },
            Enemys::Wolf => Enemy {
                name: "Wolf",
                health: 12,
                attack: 4,
                defense: 3,
                experience: 8,
                item: Items::ExperiencePotion,
            },
        }
    }
}
