use crate::dados;
use crate::structs;

use dados::equipamentos::Equipamentos;
use structs::flag::Flag;
use structs::personagem::Personagem;

// Arquivo para criação do sistema de save
pub struct Save {
    pub jogador: Personagem,
    pub item_pocao: u8,
    pub item_ataque: u8,
    pub item_defesa: u8,
    pub item_experiencia: u8,
    pub equipamento: Equipamentos,
    pub flags: Flag,
}

impl Default for Save {
    fn default() -> Self {
        Save {
            jogador: Personagem::default(),
            item_pocao: 5,
            item_ataque: 5,
            item_defesa: 5,
            item_experiencia: 5,
            equipamento: Equipamentos::Nenhum,
            flags: Flag::default(),
        }
    }
}
