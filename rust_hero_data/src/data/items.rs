//! Enemys - Data and structs related to items.

pub enum Items {
    None,
    HealthPotion,
    AttackPotion,
    DefensePotion,
    ExperiencePotion,
}

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
    pub effect: u8,
}

impl Item {
    pub const fn get_item(item: &Items) -> Self {
        match item {
            Items::None => Self {
                name: "None",
                description: "No effect",
                effect: 0,
            },
            Items::HealthPotion => Self {
                name: "Health Potion",
                description: "Recover 30% HP.",
                effect: 0,
            },
            Items::AttackPotion => Self {
                name: "Attack Potion",
                description: "Increase attack for a minute.",
                effect: 0,
            },
            Items::DefensePotion => Self {
                name: "Defense Potion",
                description: "Increase defense for a minute.",
                effect: 0,
            },
            Items::ExperiencePotion => Self {
                name: "Experience Potion",
                description: "Double received experience for a minute.",
                effect: 0,
            },
        }
    }
}
