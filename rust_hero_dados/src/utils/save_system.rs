//! Funções para armazenamento do save.

use crate::structs::save::Save;

pub fn load_save(byte_data: Vec<u8>) -> Save {
    let save: Save;
    save = bincode::deserialize(&byte_data[..]).unwrap();
    save
}

pub fn new_save(seed: &u64) -> Save {
    Save::new(seed)
}

pub fn generate_save_data(save: &Save) -> Vec<u8> {
    bincode::serialize(save).expect("It's not possible to generate save file.")
}
