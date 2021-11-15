//! Locations - Data and structs related to locations.
use super::items::ItemType;
use crate::{
    data::enemies::Enemies,
    utils::files::{get_exe_folder_path, get_file_content, BaseFileDataList, ListType},
};
use serde::{Deserialize, Serialize};

/// The locations we can travel to in a region
#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn from_string(location: &str) -> Locations {
        match location {
            "Town" => Locations::Town,
            "Forest" => Locations::Forest,
            "Cave" => Locations::Cave,
            "Swamp" => Locations::Swamp,
            "Desert" => Locations::Desert,
            _ => Locations::Town, // Default to a safe place
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

pub struct EnemyData {
    pub character: u8, //Character id
    pub level: u8,
}

/// The Location the character is currently in
#[derive(Serialize, Deserialize)]
pub struct BaseLocation {
    /// The name of the location
    pub name: String,
    /// Location description
    pub description: String,
    /// The enemies that live in the location
    pub enemies_id: Option<[u8; 4]>,
    /// The item in this location
    pub item_id: Option<u8>,
    /// The image filename
    pub background_img: String,
}

impl BaseLocation {
    pub fn get_location(id: usize) -> Self {
        let files = BaseFileDataList::get_file_list(ListType::Locations);
        if id > files.list.len() - 1 {
            panic!("Location id doesn't exist.");
        } else {
            let full_path = format!(
                "{}/assets/data/locations/{}",
                &get_exe_folder_path(),
                files.list[id].file_name
            );
            toml::from_slice(&get_file_content(full_path)).unwrap()
        }
    }
}
