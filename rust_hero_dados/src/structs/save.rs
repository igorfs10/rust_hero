use nanorand::{WyRand, RNG};

use crate::dados::{equipamentos::Equipamentos, flags::Flags};
use crate::jogo::{des_criptografar, TipoFlag};
use crate::structs::personagem::Personagem;

// Arquivo para criação do sistema de save com tempo em segundos
pub struct Save {
    pub chave: u32,
    pub jogador: Personagem,
    pub item_pocao: u8,
    pub item_ataque: u8,
    pub item_defesa: u8,
    pub item_experiencia: u8,
    pub equipamento: Option<Equipamentos>,
    pub flags: TipoFlag,
    pub tempo: u32,
}

impl Save {
    pub fn novo(seed: &u64) -> Self {
        let mut rng = WyRand::new_seed(*seed);
        let chave: u64 = rng.generate();
        let chave = chave as u32;
        Save {
            chave,
            jogador: Personagem::default(),
            item_pocao: 5,
            item_ataque: 5,
            item_defesa: 5,
            item_experiencia: 5,
            equipamento: None,
            flags: des_criptografar(&0, &chave),
            tempo: 0,
        }
    }

    pub fn check_flag(&self, flag: Flags) -> bool {
        let permissao_descriptografada = des_criptografar(&self.flags, &self.chave);
        flag as TipoFlag & permissao_descriptografada == flag as TipoFlag
    }

    pub fn set_flag(&mut self, flag: Flags) {
        let permissao_descriptografada = des_criptografar(&self.flags, &self.chave);
        let permissao = flag as TipoFlag | permissao_descriptografada;
        self.flags = des_criptografar(&permissao, &self.chave);
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        let permissao_descriptografada = des_criptografar(&self.flags, &self.chave);
        let permissao = !(flag as TipoFlag) & permissao_descriptografada;
        self.flags = des_criptografar(&permissao, &self.chave);
    }
}
