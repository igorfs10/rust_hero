use crate::item::Itens;

pub struct Inimigo {
    pub id: usize,
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: usize,
}

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
    const fn get_inimigo(self) -> Inimigo {
        match self {
            Inimigos::Nenhum => Inimigo {
                id: self.get_id(),
                nome: "",
                vida: 0,
                ataque: 0,
                defesa: 0,
                experiencia: 0,
                item: Itens::Nenhum.get_id(),
            },
            Inimigos::Rato => Inimigo {
                id: self.get_id(),
                nome: "Rato",
                vida: 5,
                ataque: 1,
                defesa: 1,
                experiencia: 2,
                item: Itens::Defesa.get_id(),
            },
            Inimigos::Coelho => Inimigo {
                id: self.get_id(),
                nome: "Coelho",
                vida: 8,
                ataque: 2,
                defesa: 2,
                experiencia: 4,
                item: Itens::Ataque.get_id(),
            },
            Inimigos::Cobra => Inimigo {
                id: self.get_id(),
                nome: "Cobra",
                vida: 12,
                ataque: 3,
                defesa: 4,
                experiencia: 8,
                item: Itens::Pocao.get_id(),
            },
            Inimigos::Lobo => Inimigo {
                id: self.get_id(),
                nome: "Lobo",
                vida: 12,
                ataque: 4,
                defesa: 3,
                experiencia: 8,
                item: Itens::Experiencia.get_id(),
            },
        }
    }
}

pub const INIMIGOS: &[Inimigo] = &[
    Inimigos::Nenhum.get_inimigo(),
    Inimigos::Rato.get_inimigo(),
    Inimigos::Coelho.get_inimigo(),
    Inimigos::Cobra.get_inimigo(),
    Inimigos::Lobo.get_inimigo(),
];
