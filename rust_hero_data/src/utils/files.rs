use std::{env, fs, path::Path};

pub fn get_exe_folder_path() -> String {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.parent().unwrap().to_str().unwrap();
    format!("{}", path)
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
