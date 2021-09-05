//! Class - Data and structs related to classes.

use serde::{Deserialize, Serialize};

/// The type of class
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Classes {
    Adept,
    Archer,
    Knight,
    Monk,
    Necromancer,
    Priest,
    Soldier,
    Thief,
    Valkyrie,
}

pub struct Class {
    pub name: &'static str,
    pub hp: u8,
    pub mp: u8,
    pub atk: u8,
    pub def: u8,
    pub m_atk: u8,
    pub m_def: u8,
}

impl Class {
    pub const fn get_class(class: &Classes) -> Self {
        match class {
            Classes::Adept => Class {
                name: "Adept",
                hp: 40,
                mp: 16,
                atk: 14,
                def: 30,
                m_atk: 20,
                m_def: 30,
            },
            Classes::Archer => Class {
                name: "Archer",
                hp: 50,
                mp: 25,
                atk: 15,
                def: 10,
                m_atk: 15,
                m_def: 35,
            },
            Classes::Knight => Class {
                name: "Knight",
                hp: 50,
                mp: 20,
                atk: 20,
                def: 20,
                m_atk: 20,
                m_def: 20,
            },
            Classes::Monk => Class {
                name: "Monk",
                hp: 40,
                mp: 40,
                atk: 10,
                def: 15,
                m_atk: 5,
                m_def: 40,
            },
            Classes::Necromancer => Class {
                name: "Necromancer",
                hp: 70,
                mp: 40,
                atk: 1,
                def: 8,
                m_atk: 30,
                m_def: 1,
            },
            Classes::Priest => Class {
                name: "Priest",
                hp: 60,
                mp: 10,
                atk: 20,
                def: 10,
                m_atk: 10,
                m_def: 40,
            },
            Classes::Soldier => Class {
                name: "Soldier",
                hp: 90,
                mp: 0,
                atk: 30,
                def: 12,
                m_atk: 0,
                m_def: 18,
            },
            Classes::Thief => Class {
                name: "Thief",
                hp: 40,
                mp: 70,
                atk: 15,
                def: 9,
                m_atk: 11,
                m_def: 30,
            },
            Classes::Valkyrie => Class {
                name: "Valkyrie",
                hp: 50,
                mp: 10,
                atk: 20,
                def: 20,
                m_atk: 20,
                m_def: 30,
            },
        }
    }
}
