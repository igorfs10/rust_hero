use crate::traits::dados_trait::DadosTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Equipamentos {
    Nenhum,
    Espada,
    Escudo,
    Bastao,
}

pub struct Equipamento {
    pub nome: &'static str,
    pub ataque: u8,
    pub defesa: u8,
}

impl Equipamento {
    pub const fn get_equipamento(equipamento: &Equipamentos) -> Equipamento {
        match equipamento {
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

impl DadosTrait for Equipamento {
    fn get_dados(&self) -> String {
        format!(
            "{}\
            \nAtaque: {}\
            \nDefesa: {}",
            self.nome, self.ataque, self.defesa
        )
    }
}

pub const EQUIPAMENTOS: &[Equipamentos] = &[
    Equipamentos::Nenhum,
    Equipamentos::Espada,
    Equipamentos::Escudo,
    Equipamentos::Bastao,
];
