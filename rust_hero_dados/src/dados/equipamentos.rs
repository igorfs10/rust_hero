use crate::structs::equipamento::Equipamento;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Equipamentos {
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

pub const EQUIPAMENTOS: &[Equipamentos] = &[
    Equipamentos::Espada,
    Equipamentos::Escudo,
    Equipamentos::Bastao,
];
