use item::item::{Item, nada};

pub const ITEM_NENHUM:          usize = 0;
pub const ITEM_POCAO:           usize = 1;
pub const ITEM_ATAQUE:          usize = 2;
pub const ITEM_DEFESA:          usize = 3;
pub const ITEM_EXPERIENCIA:     usize = 4;

pub const ITENS: [Item; 5] = [
    Item {
        id: ITEM_NENHUM,
        nome: "Nenhum",
        descricao: "",
        efeito: nada
    },
    Item {
        id: ITEM_POCAO,
        nome: "Poção",
        descricao: "Recupera 30% da vida.",
        efeito: nada
    },
    Item {
        id: ITEM_ATAQUE,
        nome: "Mais Ataque",
        descricao: "Aumenta o ataque por 1 minuto.",
        efeito: nada
    },
    Item {
        id: ITEM_DEFESA,
        nome: "Mais Defesa",
        descricao: "Aumenta a defesa por 1 minuto.",
        efeito: nada
    },
    Item {
        id: ITEM_EXPERIENCIA,
        nome: "Mais Experiência",
        descricao: "Dobra a experiência ganha por 1 minuto.",
        efeito: nada
    }
];