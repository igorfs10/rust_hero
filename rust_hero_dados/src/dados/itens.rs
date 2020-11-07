use crate::structs::item::Item;

#[derive(Copy, Clone)]
pub enum Itens {
    Pocao,
    Ataque,
    Defesa,
    Experiencia,
}

// Usar const trait quando lançar na versão estável
impl Itens {
    // Id dos itens
    pub const fn get_id(self) -> usize {
        self as usize
    }

    // Monta o item
    pub const fn get_item(self) -> Item {
        match self {
            Itens::Pocao => Item {
                id: self.get_id(),
                nome: "Poção",
                descricao: "Recupera 30% da vida.",
                efeito: 0,
            },
            Itens::Ataque => Item {
                id: self.get_id(),
                nome: "Mais Ataque",
                descricao: "Aumenta o ataque por 1 minuto.",
                efeito: 0,
            },
            Itens::Defesa => Item {
                id: self.get_id(),
                nome: "Mais Defesa",
                descricao: "Aumenta a defesa por 1 minuto.",
                efeito: 0,
            },
            Itens::Experiencia => Item {
                id: self.get_id(),
                nome: "Mais Experiência",
                descricao: "Dobra a experiência ganha por 1 minuto.",
                efeito: 0,
            },
        }
    }
}

pub const ITENS: &[Itens] = &[
    Itens::Pocao,
    Itens::Ataque,
    Itens::Defesa,
    Itens::Experiencia,
];
