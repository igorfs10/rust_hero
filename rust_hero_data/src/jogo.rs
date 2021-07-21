//! Jogo - Módulo que contém funções e dados para funcionamento do jogo.

use crate::data::enemys::Enemy;
use crate::data::locations::Location;
use crate::structs::personagem::Personagem;
use crate::utils::random::{RandomTrait, RandomValue};

pub type TipoFlag = u32;

pub const MULTIPLICADOR_CRITICO: u8 = 2;

pub fn des_criptografar(valor: &u32, chave: &u32) -> u32 {
    *valor ^ *chave
}

pub fn sortear_inimigo_lugar(location: &Location, seed: &u64) -> Option<Enemy> {
    match &location.enemys {
        Some(inimigos) => {
            let rng_inimigo = RandomValue::<u8>::get_random_value(seed, 1..=10);

            if rng_inimigo <= 4 {
                Some(Enemy::get_enemy(&inimigos[0]))
            } else if rng_inimigo <= 7 {
                Some(Enemy::get_enemy(&inimigos[1]))
            } else if rng_inimigo <= 9 {
                Some(Enemy::get_enemy(&inimigos[2]))
            } else {
                Some(Enemy::get_enemy(&inimigos[3]))
            }
        }
        None => None,
    }
}

pub fn definir_inimigo(personagem: &mut Personagem, inimigo: Enemy) {
    personagem.nome = inimigo.name.to_owned();
    personagem.ataque = inimigo.attack;
    personagem.defesa = inimigo.defense;
    personagem.vida_total = inimigo.health;
    personagem.vida_atual = inimigo.health;
    personagem.experiencia = inimigo.experience;
}
