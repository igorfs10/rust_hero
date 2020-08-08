pub struct Equipamento<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub ataque: u8,
    pub defesa: u8
}

pub const EQUIPAMENTOS: [Equipamento; 4] = [
    Equipamento {
        id: 0,
        nome: "Nenhum",
        ataque: 1,
        defesa: 1
    },
    Equipamento {
        id: 1,
        nome: "Espada",
        ataque: 3,
        defesa: 1
    },
    Equipamento {
        id: 2,
        nome: "Escudo",
        ataque: 1,
        defesa: 3
    },
    Equipamento {
        id: 3,
        nome: "Bastão",
        ataque: 2,
        defesa: 2
    }
];