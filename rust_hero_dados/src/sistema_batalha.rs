use rand::prelude::*;
use rand::{Rng, SeedableRng};

use crate::consts::MULTIPLICADOR_CRITICO;

pub fn atacar(ataque: &u8, defesa: &u8, vida: &mut u8, numero_rng: &u64) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(*numero_rng);
    let critico = rng.gen_ratio(1, 3);
    if critico {
        println!("Golpe crítico");
    }
    let mut dano;
    if *ataque <= *defesa {
        dano = 1;
    } else {
        dano = *ataque - *defesa;
    }

    if critico {
        dano *= MULTIPLICADOR_CRITICO;
    }

    if *vida > dano {
        *vida -= dano;
    } else {
        *vida = 0;
    }
}
