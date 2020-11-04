use rand::prelude::*;
use rand::{Rng, SeedableRng};

use crate::consts::MULTIPLICADOR_CRITICO;
use crate::structs::personagem::Personagem;

pub fn atacar(atacante: &Personagem, defensor: &mut Personagem, numero_rng: &u64) -> bool {
    let mut rng: StdRng = SeedableRng::seed_from_u64(*numero_rng);
    let mut dano;
    let critico = rng.gen_ratio(1, 3);

    if atacante.ataque <= defensor.defesa {
        dano = 1;
    } else {
        dano = atacante.ataque - defensor.defesa;
    }

    if critico {
        dano *= MULTIPLICADOR_CRITICO;
    }

    if defensor.vida_atual > dano {
        defensor.vida_atual -= dano;
    } else {
        defensor.vida_atual = 0;
    }

    critico
}
