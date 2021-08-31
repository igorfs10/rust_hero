//! Locations - Data and structs related to locations.
use crate::data::enemies::Enemies;

pub enum Locations {
    Town,
    Forest,
    Cave,
    Swamp,
    Desert,
}

pub struct Location {
    pub name: &'static str,
    pub enemies: Option<[Enemies; 4]>,
}

impl Location {
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
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
            },
            Locations::Desert => Self {
                name: "Desert",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
            },
        }
    }
}
