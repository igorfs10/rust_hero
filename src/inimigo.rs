use crate::item::Itens;

pub enum Inimigos {
    Nenhum,
    Rato,
    Coelho,
    Cobra,
    Lobo
}

// Deixe a string com o tempo de vida da struct
pub struct Inimigo<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: Itens
}

pub const INIMIGOS: [Inimigo; 5] = [
    Inimigo {
        id: Inimigos::Nenhum as usize,
        nome: "",
        vida: 0,
        ataque: 0,
        defesa: 0,
        experiencia: 0,
        item: Itens::Nenhum
    },
    Inimigo {
        id: Inimigos::Rato as usize,
        nome: "Rato",
        vida: 5,
        ataque: 1,
        defesa: 1,
        experiencia: 2,
        item: Itens::Defesa
    },
    Inimigo {
        id: Inimigos::Coelho as usize,
        nome: "Coelho",
        vida: 8,
        ataque: 2,
        defesa: 1,
        experiencia: 4,
        item: Itens::Ataque
    },
    Inimigo {
        id: Inimigos::Cobra as usize,
        nome: "Cobra",
        vida: 12,
        ataque: 2,
        defesa: 3,
        experiencia: 8,
        item: Itens::Pocao
    },
    Inimigo {
        id: Inimigos::Lobo as usize,
        nome: "Lobo",
        vida: 12,
        ataque: 3,
        defesa: 2,
        experiencia: 8,
        item: Itens::Experiencia
    }
];