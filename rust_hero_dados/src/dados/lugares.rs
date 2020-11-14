use super::inimigos::Inimigos;
use crate::structs::lugar::Lugar;

#[derive(Copy, Clone)]
pub enum Lugares {
    Cidade,
    Floresta,
}

impl Lugares {
    // Id dos lugares
    pub const fn get_id(self) -> usize {
        self as usize
    }

    // Monta o Lugar
    pub const fn get_local(self) -> Lugar {
        match self {
            Lugares::Cidade => Lugar {
                id: self.get_id(),
                nome: "Cidade",
                inimigos: None,
            },
            Lugares::Floresta => Lugar {
                id: self.get_id(),
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
