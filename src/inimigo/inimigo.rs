use item::dados::{ITEM_NENHUM, ITEM_POCAO, ITEM_ATAQUE, ITEM_DEFESA, ITEM_EXPERIENCIA};

pub struct Inimigo<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: usize
}

pub const INIMIGOS: [Inimigo; 5] = [
    Inimigo {
        id: 0,
        nome: "Nenhum",
        vida: 1,
        ataque: 1,
        defesa: 1,
        experiencia: 0,
        item: ITEM_NENHUM
    },
    Inimigo {
        id: 1,
        nome: "Rato",
        vida: 5,
        ataque: 1,
        defesa: 1,
        experiencia: 2,
        item: ITEM_DEFESA
    },
    Inimigo {
        id: 2,
        nome: "Coelho",
        vida: 8,
        ataque: 2,
        defesa: 1,
        experiencia: 4,
        item: ITEM_ATAQUE
    },
    Inimigo {
        id: 3,
        nome: "Cobra",
        vida: 12,
        ataque: 2,
        defesa: 3,
        experiencia: 8,
        item: ITEM_POCAO
    },
    Inimigo {
        id: 4,
        nome: "Lobo",
        vida: 12,
        ataque: 3,
        defesa: 2,
        experiencia: 8,
        item: ITEM_EXPERIENCIA
    }
];