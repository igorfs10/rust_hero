//! Enemys - Data and structs related to items.

pub enum ItemType {
    None,
    Health,
    Attack,
    Defense,
    Experience,
}

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
    pub effect: u8,
}

impl Item {
    // do we need to send in a character to apply it to, or is this for UI
    pub const fn get_item(item: &ItemType) -> Self {
        match item {
            ItemType::None => Self {
                name: "None",
                description: "No effect",
                effect: 0,
            },
            ItemType::Health => Self {
                name: "Health Potion",
                description: "Recover 30% HP.",
                effect: 0,
            },
            ItemType::Attack => Self {
                name: "Attack Potion",
                description: "Increase attack for a minute.",
                effect: 0,
            },
            ItemType::Defense => Self {
                name: "Defense Potion",
                description: "Increase defense for a minute.",
                effect: 0,
            },
            ItemType::Experience => Self {
                name: "Experience Potion",
                description: "Double received experience for a minute.",
                effect: 0,
            },
        }
    }
}
