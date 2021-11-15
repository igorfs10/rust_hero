use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

pub fn get_exe_folder_path() -> String {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.parent().unwrap().to_str().unwrap();
    path.to_string()
}

pub fn get_file_content(full_path: String) -> Vec<u8> {
    if Path::new(&full_path).exists() {
        match fs::read(&full_path) {
            Ok(file_content) => file_content,
            Err(error) => panic!("{}", error.to_string()),
        }
    } else {
        panic!("Can't find file at: {}", full_path);
    }
}

pub enum ListType {
    Characters,
    Locations,
}

impl ListType {
    pub fn get_type_folder_name(self) -> String {
        match self {
            ListType::Characters => "characters".to_string(),
            ListType::Locations => "locations".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BaseFileData {
    pub id: u8,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BaseFileDataList {
    pub list: Vec<BaseFileData>,
}

impl BaseFileDataList {
    pub fn get_file_list(list_type: ListType) -> Self {
        let full_path = format!(
            "{}/assets/data/{}/list.toml",
            &get_exe_folder_path(),
            list_type.get_type_folder_name()
        );

        toml::from_slice(&get_file_content(full_path)).unwrap()
    }
}
