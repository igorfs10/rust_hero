use crate::item::{ITEM_ATAQUE, ITEM_DEFESA, ITEM_EXPERIENCIA, ITEM_NENHUM, ITEM_POCAO};

pub struct Inimigo<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: usize
}

pub const INIMIGO_NENHUM:   usize = 0;
pub const INIMIGO_RATO:     usize = 1;
pub const INIMIGO_COELHO:   usize = 2;
pub const INIMIGO_COBRA:    usize = 3;
pub const INIMIGO_LOBO:     usize = 4;

pub const INIMIGOS: [Inimigo; 5] = [
    Inimigo {
        id: INIMIGO_NENHUM,
        nome: "Nenhum",
        vida: 1,
        ataque: 1,
        defesa: 1,
        experiencia: 0,
        item: ITEM_NENHUM
    },
    Inimigo {
        id: INIMIGO_RATO,
        nome: "Rato",
        vida: 5,
        ataque: 1,
        defesa: 1,
        experiencia: 2,
        item: ITEM_DEFESA
    },
    Inimigo {
        id: INIMIGO_COELHO,
        nome: "Coelho",
        vida: 8,
        ataque: 2,
        defesa: 1,
        experiencia: 4,
        item: ITEM_ATAQUE
    },
    Inimigo {
        id: INIMIGO_COBRA,
        nome: "Cobra",
        vida: 12,
        ataque: 2,
        defesa: 3,
        experiencia: 8,
        item: ITEM_POCAO
    },
    Inimigo {
        id: INIMIGO_LOBO,
        nome: "Lobo",
        vida: 12,
        ataque: 3,
        defesa: 2,
        experiencia: 8,
        item: ITEM_EXPERIENCIA
    }
];