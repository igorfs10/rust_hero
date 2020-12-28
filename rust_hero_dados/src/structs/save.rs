use crate::dados::equipamentos::Equipamentos;
use crate::jogo::des_criptografar;
use crate::jogo::TipoPermissao;
use crate::structs::{flag::Flag, personagem::Personagem};

// Arquivo para criação do sistema de save com tempo em segundos
pub struct Save {
    pub jogador: Personagem,
    pub item_pocao: u8,
    pub item_ataque: u8,
    pub item_defesa: u8,
    pub item_experiencia: u8,
    pub equipamento: Option<Equipamentos>,
    pub flags: Flag,
    pub tempo: u64,
    pub permissoes: TipoPermissao,
}

impl Default for Save {
    fn default() -> Self {
        Save {
            jogador: Personagem::default(),
            item_pocao: 5,
            item_ataque: 5,
            item_defesa: 5,
            item_experiencia: 5,
            equipamento: None,
            flags: Flag::default(),
            tempo: 0,
            permissoes: des_criptografar(0),
        }
    }
}
