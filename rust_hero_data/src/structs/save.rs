//! Save - Save struct and system
use crate::data::{flags::Flags, weapons::Weapons};
use crate::jogo::{des_criptografar, TipoFlag};
use crate::structs::character::Character;
use crate::utils::random::{RandomTrait, RandomValue};
use serde::{Deserialize, Serialize};

// Save struct
#[derive(Serialize, Deserialize)]
pub struct Save {
    key: u32,
    pub player: Character,
    amount_health_potion: u8,
    amount_attack_potion: u8,
    amount_defense_potion: u8,
    amount_experience_potion: u8,
    pub weapon: Weapons,
    flags: TipoFlag,
    pub time: u64,
}

impl Save {
    pub fn new(seed: &u64) -> Self {
        let key = RandomValue::<u32>::get_random_value(seed, u32::MIN..=u32::MAX);
        Save {
            key,
            player: Character::default(),
            amount_health_potion: 5,
            amount_attack_potion: 5,
            amount_defense_potion: 5,
            amount_experience_potion: 5,
            weapon: Weapons::None,
            flags: des_criptografar(&0, &key),
            time: 0,
        }
    }

    pub fn check_flag(&self, flag: Flags) -> bool {
        let decrypted_flags = des_criptografar(&self.flags, &self.key);
        flag.clone() as TipoFlag & decrypted_flags == flag as TipoFlag
    }

    pub fn set_flag(&mut self, flag: Flags) {
        let decrypted_flags = des_criptografar(&self.flags, &self.key);
        let u_flag = flag as TipoFlag | decrypted_flags;
        self.flags = des_criptografar(&u_flag, &self.key);
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        let decrypted_flags = des_criptografar(&self.flags, &self.key);
        let u_flag = !(flag as TipoFlag) & decrypted_flags;
        self.flags = des_criptografar(&u_flag, &self.key);
    }
}
