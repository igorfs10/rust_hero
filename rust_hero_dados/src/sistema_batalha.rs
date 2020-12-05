use nanorand::{WyRand, RNG};

use crate::consts::MULTIPLICADOR_CRITICO;
use crate::structs::personagem::Personagem;

pub fn atacar((atacante, defensor, seed): (&Personagem, &mut Personagem, &u64)) -> bool {
    let mut rng = WyRand::new_seed(*seed);
    let mut dano;
    let critico = rng.generate_range::<u64>(1, 4) < 2;

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
