use crate::structs::equipamento::Equipamento;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Equipamentos {
    Nenhum,
    Espada,
    Escudo,
    Bastao,
}

// Usar const trait quando lançar na versão estável
impl Equipamentos {
    //Monta o equipamento
    pub const fn get_equipamento(&self) -> Equipamento {
        match self {
            Equipamentos::Nenhum => Equipamento {
                nome: "Nenhum",
                ataque: 0,
                defesa: 0,
            },
            Equipamentos::Espada => Equipamento {
                nome: "Espada",
                ataque: 3,
                defesa: 1,
            },
            Equipamentos::Escudo => Equipamento {
                nome: "Escudo",
                ataque: 1,
                defesa: 3,
            },
            Equipamentos::Bastao => Equipamento {
                nome: "Bastão",
                ataque: 2,
                defesa: 2,
            },
        }
    }
}

pub const EQUIPAMENTOS: &[Equipamentos] = &[
    Equipamentos::Nenhum,
    Equipamentos::Espada,
    Equipamentos::Escudo,
    Equipamentos::Bastao,
];
