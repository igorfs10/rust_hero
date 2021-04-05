use crate::structs::item::Item;

#[derive(Clone)]
pub enum Itens {
    Nenhum,
    Pocao,
    Ataque,
    Defesa,
    Experiencia,
}

// Usar const trait quando lançar na versão estável
impl Itens {
    // Monta o item
    pub const fn get_item(self) -> Item {
        match self {
            Itens::Nenhum => Item {
                nome: "Nenhum",
                descricao: "Sem efeito",
                efeito: 0,
            },
            Itens::Pocao => Item {
                nome: "Poção",
                descricao: "Recupera 30% da vida.",
                efeito: 0,
            },
            Itens::Ataque => Item {
                nome: "Mais Ataque",
                descricao: "Aumenta o ataque por 1 minuto.",
                efeito: 0,
            },
            Itens::Defesa => Item {
                nome: "Mais Defesa",
                descricao: "Aumenta a defesa por 1 minuto.",
                efeito: 0,
            },
            Itens::Experiencia => Item {
                nome: "Mais Experiência",
                descricao: "Dobra a experiência ganha por 1 minuto.",
                efeito: 0,
            },
        }
    }
}

pub const ITENS: &[Itens] = &[
    Itens::Nenhum,
    Itens::Pocao,
    Itens::Ataque,
    Itens::Defesa,
    Itens::Experiencia,
];
