//! Equipamentos - Dados e funções relacionados à estrutura de equipamentos.

use crate::traits::dados_trait::DadosTrait;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize)]
pub enum Equipamentos {
    Nenhum,
    Espada,
    Escudo,
    Bastao,
}

/// Struct de equipamento, usado para armazenar dados de equipamento.
///
/// # Exemplo da criação de uma struct com um equipamento.
/// ```
/// let equipamento = Equipamento::get_equipamento(Equipamentos::Espada);
/// ```
pub struct Equipamento {
    pub nome: &'static str,
    pub ataque: u8,
    pub defesa: u8,
}

impl Equipamento {
    pub const fn get_equipamento(equipamento: &Equipamentos) -> Self {
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

pub const EQUIPAMENTOS: &[Equipamento] = &[
    Equipamento::get_equipamento(&Equipamentos::Nenhum),
    Equipamento::get_equipamento(&Equipamentos::Espada),
    Equipamento::get_equipamento(&Equipamentos::Escudo),
    Equipamento::get_equipamento(&Equipamentos::Bastao),
];
