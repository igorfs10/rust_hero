pub struct Equipamento {
    pub id: usize,
    pub nome: &'static str,
    pub ataque: u8,
    pub defesa: u8,
}

pub enum Equipamentos {
    Nenhum,
    Espada,
    Escudo,
    Bastao,
}

// Usar const trait quando lançar na versão estável
impl Equipamentos {
    // Id dos equipamentos
    pub const fn get_id(self) -> usize {
        self as usize
    }

    //Monta o equipamento
    pub const fn get_equipamento(self) -> Equipamento {
        match self {
            Equipamentos::Nenhum => Equipamento {
                id: self.get_id(),
                nome: "Nenhum",
                ataque: 0,
                defesa: 0,
            },
            Equipamentos::Espada => Equipamento {
                id: self.get_id(),
                nome: "Espada",
                ataque: 3,
                defesa: 1,
            },
            Equipamentos::Escudo => Equipamento {
                id: self.get_id(),
                nome: "Escudo",
                ataque: 1,
                defesa: 3,
            },
            Equipamentos::Bastao => Equipamento {
                id: self.get_id(),
                nome: "Bastão",
                ataque: 2,
                defesa: 2,
            },
        }
    }
}

pub const EQUIPAMENTOS: &[Equipamento] = &[
    Equipamentos::Nenhum.get_equipamento(),
    Equipamentos::Espada.get_equipamento(),
    Equipamentos::Escudo.get_equipamento(),
    Equipamentos::Bastao.get_equipamento(),
];
