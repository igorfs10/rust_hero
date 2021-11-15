//! Items - Data and structs related to items.
use serde::{Deserialize, Serialize};

/// The type of item
#[derive(Serialize, Deserialize)]
pub enum ItemType {
    /// No item
    None,
    /// An Item to boost health
    Health,
    /// An Item to boost attack
    Attack,
    /// An Item to boost defense
    Defense,
    /// An Item to boost experience
    Experience,
}

pub struct Item {
    /// The name of the item
    pub name: &'static str,
    /// The human-readable description
    pub description: &'static str,
    /// The effect the item has see: `data::flags::Flags` for more info
    pub effect: u8,
}

impl Item {
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
