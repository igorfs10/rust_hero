pub enum Equipamentos {
    Nenhum,
    Espada,
    Escudo,
    Bastao
}

pub struct Equipamento<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub ataque: u8,
    pub defesa: u8
}

pub const EQUIPAMENTOS: [Equipamento; 4] = [
    Equipamento {
        id: Equipamentos::Nenhum as usize,
        nome: "",
        ataque: 1,
        defesa: 1
    },
    Equipamento {
        id: Equipamentos::Espada as usize,
        nome: "Espada",
        ataque: 3,
        defesa: 1
    },
    Equipamento {
        id: Equipamentos::Escudo as usize,
        nome: "Escudo",
        ataque: 1,
        defesa: 3
    },
    Equipamento {
        id: Equipamentos::Bastao as usize,
        nome: "Bastão",
        ataque: 2,
        defesa: 2
    }
];