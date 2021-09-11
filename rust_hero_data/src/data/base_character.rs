use serde::{Deserialize, Serialize};

use crate::utils::files::{get_exe_folder_path, get_file_content};

#[derive(Serialize, Deserialize, Default)]
pub struct BaseCharacterFile {
    pub id: u8,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BaseCharacterFileList {
    pub list: Vec<BaseCharacterFile>,
}

impl BaseCharacterFileList {
    pub fn get_characters_list() -> Self {
        let full_path = format!(
            "{}/assets/data/characters/list.toml",
            &get_exe_folder_path()
        );

        toml::from_slice(&get_file_content(full_path)).unwrap()
    }
}

/// Struct to be used to load the base stat for a character and calculate its stats and given experience
#[derive(Serialize, Deserialize, Default)]
pub struct BaseCharacter {
    pub name: String,
    pub hp: u16,
    pub mp: u16,
    pub atk: u8,
    pub def: u8,
    pub m_atk: u8,
    pub m_def: u8,
    pub exp: u16, // Base xp used to calculate the received xp after player defeats an enemy
    pub item: Option<u8>, // Id of item that will be dropped by enemy
    pub img_idle: String,
    pub img_walking: String,
    pub img_attacking: String,
    pub playable: bool,
}

impl BaseCharacter {
    pub fn get_character(id: usize) -> Self {
        let files = BaseCharacterFileList::get_characters_list();
        if id > files.list.len() - 1 {
            panic!("Character id doesn't exist.");
        } else {
            let full_path = format!(
                "{}/assets/data/characters/{}",
                &get_exe_folder_path(),
                files.list[id].file_name
            );
            toml::from_slice(&get_file_content(full_path)).unwrap()
        }
    }
}
