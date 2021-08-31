//! Locations - Data and structs related to locations.
use crate::data::enemies::Enemies;

/// The locations we can travel to in a region
pub enum Locations {
    /// The place to rest and purchase things
    Town,
    /// Dense undergrowth has lead to many creatures living here
    Forest,
    /// Inaccessable to many, the cave is full of lurking monsters
    Cave,
    /// 
    Swamp,
    /// 
    Desert,
}

pub struct Location {
    pub name: &'static str,
    pub enemies: Option<[Enemies; 4]>,
}

impl Location {
    /// Get a `Location` struct from a `Locations` enum
    pub const fn get_location(location: &Locations) -> Self {
        match location {
            Locations::Town => Self {
                name: "Town",
                enemies: None,
            },
            Locations::Forest => Self {
                name: "Forest",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
            },
            Locations::Cave => Self {
                name: "Cave",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
            },
            Locations::Swamp => Self {
                name: "Swamp",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Crocodile]),
            },
            Locations::Desert => Self {
                name: "Desert",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
            },
        }
    }
}
