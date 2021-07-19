use std::{
    env,
    fs::{read, File, OpenOptions},
    io::Write,
    path::Path,
};

use rust_hero_data::{
    structs::save::Save,
    utils::save_system::{generate_save_data, load_save, new_save},
};

pub fn get_save_location() -> String {
    let exe_path = env::current_exe().unwrap();
    let save_name = exe_path.file_stem().unwrap().to_str().unwrap();
    let path = exe_path.parent().unwrap().to_str().unwrap();
    format!("{}/{}.sav", path, save_name)
}

pub fn new_game(save: &mut Save, seed: &u64) -> String {
    *save = new_save(seed);
    let save_path = get_save_location();

    match File::create(save_path) {
        Ok(mut file) => {
            let save_content = generate_save_data(save);
            match file.write_all(&save_content[..]) {
                Ok(_) => "".to_string(),
                Err(error) => error.to_string(),
            }
        }
        Err(error) => error.to_string(),
    }
}

pub fn save_game(save: &Save) -> String {
    let save_path = get_save_location();
    let file_content = generate_save_data(save);

    let mut file = OpenOptions::new().write(true).open(save_path).unwrap();

    match file.write_all(&file_content[..]) {
        Ok(_) => todo!(),
        Err(error) => error.to_string(),
    }
}

pub fn load_game(save: &mut Save) -> String {
    let save_path = get_save_location();

    if Path::new(&save_path).exists() {
        match read(&save_path) {
            Ok(file_content) => {
                *save = load_save(file_content);
                "".to_string()
            }
            Err(error) => error.to_string(),
        }
    } else {
        "Can't find save file".to_string()
    }
}
