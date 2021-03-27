use crate::utils::random::{RandomTrait, RandomValue};

use crate::dados::equipamentos::Equipamentos;
use crate::structs::{inimigo::Inimigo, lugar::Lugar, personagem::Personagem};

pub type TipoFlag = u32;

pub const MULTIPLICADOR_CRITICO: u8 = 2;

pub fn des_criptografar(valor: &u32, chave: &u32) -> u32 {
    *valor ^ *chave
}

pub fn match_equipamento(id_equipamento: usize) -> Option<Equipamentos> {
    match id_equipamento {
        0 => Some(Equipamentos::Espada),
        1 => Some(Equipamentos::Escudo),
        2 => Some(Equipamentos::Bastao),
        _ => None,
    }
}

pub fn sortear_inimigo_lugar(lugar: &Lugar, seed: &u64) -> Option<Inimigo> {
    match lugar.inimigos {
        Some(inimigos) => {
            let rng_inimigo = RandomValue::<u8>::get_random_value(seed, 1..=10);

            if rng_inimigo <= 4 {
                Some(inimigos[0].get_inimigo())
            } else if rng_inimigo <= 7 {
                Some(inimigos[1].get_inimigo())
            } else if rng_inimigo <= 9 {
                Some(inimigos[2].get_inimigo())
            } else {
                Some(inimigos[3].get_inimigo())
            }
        }
        None => None,
    }
}

pub fn definir_inimigo(personagem: &mut Personagem, inimigo: Inimigo) {
    personagem.nome = inimigo.nome;
    personagem.ataque = inimigo.ataque;
    personagem.defesa = inimigo.defesa;
    personagem.vida_total = inimigo.vida;
    personagem.vida_atual = inimigo.vida;
    personagem.experiencia = inimigo.experiencia;
}
