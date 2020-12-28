use nanorand::{WyRand, RNG};

use crate::dados::equipamentos::Equipamentos;
use crate::structs::{inimigo::Inimigo, lugar::Lugar, personagem::Personagem};

pub type TipoPermissao = u8;

pub const MULTIPLICADOR_CRITICO: u8 = 2;
pub const CHAVE_CRIPTOGRAFIA: TipoPermissao = 0b10101011;

pub fn des_criptografar(valor: TipoPermissao) -> TipoPermissao {
    valor ^ CHAVE_CRIPTOGRAFIA
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
            let mut rng = WyRand::new_seed(*seed);
            let rng_inimigo = rng.generate_range::<u64>(1, 11);

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
    personagem.nome = inimigo.nome.to_string();
    personagem.ataque = inimigo.ataque;
    personagem.defesa = inimigo.defesa;
    personagem.vida_total = inimigo.vida;
    personagem.vida_atual = inimigo.vida;
    personagem.experiencia = inimigo.experiencia;
}
