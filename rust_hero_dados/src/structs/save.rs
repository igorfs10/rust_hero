use crate::dados::{equipamentos::Equipamentos, flags::Flags};
use crate::jogo::{des_criptografar, TipoFlag};
use crate::structs::personagem::Personagem;
use crate::utils::random::{RandomTrait, RandomValue};
use serde::{Deserialize, Serialize};

// Arquivo para criação do sistema de save com tempo em segundos
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Save {
    pub chave: u32,
    pub jogador: Personagem,
    pub item_pocao: u8,
    pub item_ataque: u8,
    pub item_defesa: u8,
    pub item_experiencia: u8,
    pub equipamento: Option<Equipamentos>,
    pub flags: TipoFlag,
    pub tempo: u64,
}

impl Save {
    pub fn novo(seed: &u64) -> Self {
        let chave = RandomValue::<u32>::get_random_value(seed, u32::MIN..=u32::MAX);
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
