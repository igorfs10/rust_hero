use crate::{
    dados::equipamentos::EQUIPAMENTOS,
    utils::random::{RandomTrait, RandomValue},
};

use crate::dados::equipamentos::Equipamentos;
use crate::dados::inimigos::Inimigo;
use crate::dados::lugares::Lugar;
use crate::structs::personagem::Personagem;

pub type TipoFlag = u32;

pub const MULTIPLICADOR_CRITICO: u8 = 2;

pub fn des_criptografar(valor: &u32, chave: &u32) -> u32 {
    *valor ^ *chave
}

pub fn match_equipamento(id_equipamento: usize) -> Equipamentos {
    if id_equipamento >= EQUIPAMENTOS.len() {
        EQUIPAMENTOS[0].clone()
    } else {
        EQUIPAMENTOS[id_equipamento].clone()
    }
}

pub fn sortear_inimigo_lugar(lugar: &Lugar, seed: &u64) -> Option<Inimigo> {
    match lugar.inimigos.clone() {
        Some(inimigos) => {
            let rng_inimigo = RandomValue::<u8>::get_random_value(seed, 1..=10);

            if rng_inimigo <= 4 {
                Some(Inimigo::get_inimigo(&inimigos[0]))
            } else if rng_inimigo <= 7 {
                Some(Inimigo::get_inimigo(&inimigos[1]))
            } else if rng_inimigo <= 9 {
                Some(Inimigo::get_inimigo(&inimigos[2]))
            } else {
                Some(Inimigo::get_inimigo(&inimigos[3]))
            }
        }
        None => None,
    }
}

pub fn definir_inimigo(personagem: &mut Personagem, inimigo: Inimigo) {
    personagem.nome = inimigo.nome.to_owned();
    personagem.ataque = inimigo.ataque;
    personagem.defesa = inimigo.defesa;
    personagem.vida_total = inimigo.vida;
    personagem.vida_atual = inimigo.vida;
    personagem.experiencia = inimigo.experiencia;
}
