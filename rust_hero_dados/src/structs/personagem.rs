use crate::jogo::MULTIPLICADOR_CRITICO;
use crate::utils::random::{RandomTrait, RandomValue};

#[derive(Clone, PartialEq, Debug)]
pub struct Personagem {
    pub nome: String,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
}

impl Personagem {
    const fn new(nome: String, vida: u8, ataque: u8, defesa: u8, experiencia: u16) -> Self {
        Personagem {
            nome,
            vida_total: vida,
            vida_atual: vida,
            ataque,
            defesa,
            experiencia,
        }
    }

    pub fn atacar(&self, defensor: &mut Self, seed: &u64) -> (bool, u8, bool) {
        let mut dano;
        let derrotou;
        let critico = RandomValue::<bool>::get_random_value(seed, 25);

        if self.ataque <= defensor.defesa {
            dano = 1;
        } else {
            dano = self.ataque - defensor.defesa;
        }

        if critico {
            dano *= MULTIPLICADOR_CRITICO;
        }

        if defensor.vida_atual > dano {
            defensor.vida_atual -= dano;
            derrotou = false;
        } else {
            dano = defensor.vida_atual;
            defensor.vida_atual = 0;
            derrotou = true;
        }
        (critico, dano, derrotou)
    }
}

impl Default for Personagem {
    fn default() -> Self {
        Personagem::new(String::from(""), 10, 1, 1, 0)
    }
}
