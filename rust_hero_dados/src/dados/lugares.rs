use super::inimigos::Inimigos;
use crate::structs::lugar::Lugar;

#[derive(Clone)]
pub enum Lugares {
    Cidade,
    Floresta,
}

impl Lugares {
    // Monta o Lugar
    pub const fn get_lugar(self) -> Lugar {
        match self {
            Lugares::Cidade => Lugar {
                nome: "Cidade",
                inimigos: None,
            },
            Lugares::Floresta => Lugar {
                nome: "Floresta",
                inimigos: Some([
                    Inimigos::Rato,
                    Inimigos::Coelho,
                    Inimigos::Cobra,
                    Inimigos::Lobo,
                ]),
            },
        }
    }
}

pub const LUGARES: &[Lugares] = &[Lugares::Cidade, Lugares::Floresta];
