use crate::flags;
use crate::structs;

use flags::Flags;
use structs::Jogador;

// Arquivo para criação do sistema de save
pub struct Save {
    pub jogador: Jogador,
    pub item_pocao: u8,
    pub item_ataque: u8,
    pub item_defesa: u8,
    pub item_experiencia: u8,
    pub flags: Flags,
}

impl Default for Save {
    fn default() -> Self {
        Save {
            jogador: Jogador::default(),
            item_pocao: 5,
            item_ataque: 5,
            item_defesa: 5,
            item_experiencia: 5,
            flags: Flags::default(),
        }
    }
}
