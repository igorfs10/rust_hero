use super::itens::Itens;
use crate::structs::inimigo::Inimigo;

#[derive(Copy, Clone)]
pub enum Inimigos {
    Nenhum,
    Rato,
    Coelho,
    Cobra,
    Lobo,
}

// Usar const trait quando lançar na versão estável
impl Inimigos {
    // Id dos inimigos
    pub const fn get_id(self) -> usize {
        self as usize
    }

    // Monta o inimigo
    pub const fn get_inimigo(self) -> Inimigo {
        match self {
            Inimigos::Nenhum => Inimigo {
                id: self.get_id(),
                nome: "???",
                vida: 0,
                ataque: 0,
                defesa: 0,
                experiencia: 0,
                item: None,
            },
            Inimigos::Rato => Inimigo {
                id: self.get_id(),
                nome: "Rato",
                vida: 5,
                ataque: 1,
                defesa: 1,
                experiencia: 2,
                item: Some(Itens::Defesa),
            },
            Inimigos::Coelho => Inimigo {
                id: self.get_id(),
                nome: "Coelho",
                vida: 8,
                ataque: 2,
                defesa: 2,
                experiencia: 4,
                item: Some(Itens::Ataque),
            },
            Inimigos::Cobra => Inimigo {
                id: self.get_id(),
                nome: "Cobra",
                vida: 12,
                ataque: 3,
                defesa: 4,
                experiencia: 8,
                item: Some(Itens::Pocao),
            },
            Inimigos::Lobo => Inimigo {
                id: self.get_id(),
                nome: "Lobo",
                vida: 12,
                ataque: 4,
                defesa: 3,
                experiencia: 8,
                item: Some(Itens::Experiencia),
            },
        }
    }
}

pub const INIMIGOS: &[Inimigos] = &[
    Inimigos::Nenhum,
    Inimigos::Rato,
    Inimigos::Coelho,
    Inimigos::Cobra,
    Inimigos::Lobo,
];
