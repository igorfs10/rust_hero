pub enum Itens {
    Nenhum,
    Pocao,
    Ataque,
    Defesa,
    Experiencia
}

pub struct Item<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub descricao: &'a str,
    pub efeito: u8
}

pub const ITENS: [Item; 5] = [
    Item {
        id: Itens::Nenhum as usize,
        nome: "",
        descricao: "",
        efeito: 0
    },
    Item {
        id: Itens::Pocao as usize,
        nome: "Poção",
        descricao: "Recupera 30% da vida.",
        efeito: 0
    },
    Item {
        id: Itens::Ataque as usize,
        nome: "Mais Ataque",
        descricao: "Aumenta o ataque por 1 minuto.",
        efeito: 0
    },
    Item {
        id: Itens::Defesa as usize,
        nome: "Mais Defesa",
        descricao: "Aumenta a defesa por 1 minuto.",
        efeito: 0
    },
    Item {
        id: Itens::Experiencia as usize,
        nome: "Mais Experiência",
        descricao: "Dobra a experiência ganha por 1 minuto.",
        efeito: 0
    }
];