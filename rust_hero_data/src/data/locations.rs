//! Locations - Data and structs related to locations.
use crate::data::enemys::Enemys;

pub enum Locations {
    Town,
    Forest,
}

pub struct Location {
    pub name: &'static str,
    pub enemys: Option<[Enemys; 4]>,
}

impl Location {
    pub const fn get_location(location: &Locations) -> Self {
        match location {
            Locations::Town => Self {
                name: "Town",
                enemys: None,
            },
            Locations::Forest => Self {
                name: "Forest",
                enemys: Some([Enemys::Rat, Enemys::Rabbit, Enemys::Snake, Enemys::Wolf]),
            },
        }
    }
}
