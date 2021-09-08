//! Locations - Data and structs related to locations.
use super::items::ItemType;
use crate::data::enemies::Enemies;

/// The locations we can travel to in a region
pub enum Locations {
    /// The place to rest and purchase things
    Town,
    /// Dense undergrowth has lead to many creatures living here
    Forest,
    /// Inaccessable to many, the cave is full of lurking monsters
    Cave,
    /// The long hard treks through the swampy muck are greeted with plentiful creatures.
    Swamp,
    /// Dry sands prevade the deserts, with many hardy monsters around
    Desert,
}
impl Locations {
    /// convert a string into a Location
    pub fn from_string(location:String) -> Locations {
        if location == "Town" {
            return Locations::Town
        } else if location == "Forest" {
            return Locations::Forest
        } else if location == "Cave" {
            return Locations::Cave
        } else if location == "Swamp" {
            return Locations::Swamp
        } else if location == "Desert" {
            return Locations::Desert
        }
        else {
            // Default to a safe place
            return Locations::Town
        }
    
    }
}

/// The Location the character is currently in
pub struct Location {
    /// The name of the location
    pub name: &'static str,
    /// The enemies that live in the location
    pub enemies: Option<[Enemies; 4]>,
    /// The item in this location
    pub item: Option<ItemType>,
    /// The image filename
    pub image: &'static str,
}

impl Location {

    /// Get a `Location` struct from a `Locations` enum
    pub const fn get_location(location: &Locations) -> Self {
        match location {
            Locations::Town => Self {
                name: "Town",
                enemies: None,
                item: None,
                image: "assets/backgrounds/town.png",
            },
            Locations::Forest => Self {
                name: "Forest",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
                item: Some(ItemType::Health),
                image: "assets/backgrounds/forest.png",
            },
            Locations::Cave => Self {
                name: "Cave",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
                item: None,
                image: "assets/backgrounds/cave.png",
            },
            Locations::Swamp => Self {
                name: "Swamp",
                enemies: Some([
                    Enemies::Rat,
                    Enemies::Rabbit,
                    Enemies::Snake,
                    Enemies::Crocodile,
                ]),
                item: None,
                image: "assets/backgrounds/swamp.png",
            },
            Locations::Desert => Self {
                name: "Desert",
                enemies: Some([Enemies::Rat, Enemies::Rabbit, Enemies::Snake, Enemies::Wolf]),
                item: None,
                image: "assets/backgrounds/desert.png",
            },
        }
    }
}
