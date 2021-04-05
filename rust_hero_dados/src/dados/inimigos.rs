use super::itens::Itens;
use crate::structs::inimigo::Inimigo;

#[derive(Clone)]
pub enum Inimigos {
    Rato,
    Coelho,
    Cobra,
    Lobo,
}

// Usar const trait quando lançar na versão estável
impl Inimigos {
    // Monta o inimigo
    pub const fn get_inimigo(&self) -> Inimigo {
        match self {
            Inimigos::Rato => Inimigo {
                nome: "Rato",
                vida: 5,
                ataque: 1,
                defesa: 1,
                experiencia: 2,
                item: Itens::Defesa,
            },
            Inimigos::Coelho => Inimigo {
                nome: "Coelho",
                vida: 8,
                ataque: 2,
                defesa: 2,
                experiencia: 4,
                item: Itens::Ataque,
            },
            Inimigos::Cobra => Inimigo {
                nome: "Cobra",
                vida: 12,
                ataque: 3,
                defesa: 4,
                experiencia: 8,
                item: Itens::Pocao,
            },
            Inimigos::Lobo => Inimigo {
                nome: "Lobo",
                vida: 12,
                ataque: 4,
                defesa: 3,
                experiencia: 8,
                item: Itens::Experiencia,
            },
        }
    }
}

pub const INIMIGOS: &[Inimigos] = &[
    Inimigos::Rato,
    Inimigos::Coelho,
    Inimigos::Cobra,
    Inimigos::Lobo,
];
