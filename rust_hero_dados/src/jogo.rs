use nanorand::{WyRand, RNG};

use crate::dados::equipamentos::Equipamentos;
use crate::structs::inimigo::Inimigo;
use crate::structs::lugar::Lugar;
use crate::structs::personagem::Personagem;

pub fn match_equipamento(id_equipamento: usize) -> Option<Equipamentos> {
    match id_equipamento {
        0 => Some(Equipamentos::Espada),
        1 => Some(Equipamentos::Escudo),
        2 => Some(Equipamentos::Bastao),
        _ => None,
    }
}

pub fn sortear_inimigo_lugar(lugar: &Lugar, seed: &u64) -> Option<Inimigo> {
    if let Some(inimigos) = lugar.inimigos {
        let mut rng = WyRand::new_seed(*seed);
        let rng_inimigo = rng.generate_range::<u8>(1, 11);

        if rng_inimigo <= 4 {
            Some(inimigos[0].get_inimigo())
        } else if rng_inimigo <= 7 {
            Some(inimigos[1].get_inimigo())
        } else if rng_inimigo <= 9 {
            Some(inimigos[2].get_inimigo())
        } else {
            Some(inimigos[3].get_inimigo())
        }
    } else {
        None
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
