pub enum Itens {
    ItemNenhum,
    ItemPocao,
    ItemAtaque,
    ItemDefesa,
    ItemExperiencia
}

pub struct Item<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub descricao: &'a str,
    pub efeito: u8
}

pub const ITENS: [Item; 5] = [
    Item {
        id: Itens::ItemNenhum as usize,
        nome: "Nenhum",
        descricao: "",
        efeito: 0
    },
    Item {
        id: Itens::ItemPocao as usize,
        nome: "Poção",
        descricao: "Recupera 30% da vida.",
        efeito: 0
    },
    Item {
        id: Itens::ItemAtaque as usize,
        nome: "Mais Ataque",
        descricao: "Aumenta o ataque por 1 minuto.",
        efeito: 0
    },
    Item {
        id: Itens::ItemDefesa as usize,
        nome: "Mais Defesa",
        descricao: "Aumenta a defesa por 1 minuto.",
        efeito: 0
    },
    Item {
        id: Itens::ItemExperiencia as usize,
        nome: "Mais Experiência",
        descricao: "Dobra a experiência ganha por 1 minuto.",
        efeito: 0
    }
];