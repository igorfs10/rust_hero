use super::itens::{Item, Itens};
use crate::traits::dados_trait::DadosTrait;

pub enum Inimigos {
    Rato,
    Coelho,
    Cobra,
    Lobo,
}

pub struct Inimigo {
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: Itens,
}

impl Inimigo {
    // Monta o inimigo
    pub const fn get_inimigo(inimigo: &Inimigos) -> Self {
        match inimigo {
            Inimigos::Rato => Inimigo {
                nome: "Rato",
                vida: 5,
                ataque: 1,
                defesa: 1,
                experiencia: 2,
                item: Itens::Defesa,
            },
            Inimigos::Coelho => Inimigo {
                nome: "Coelho",
                vida: 8,
                ataque: 2,
                defesa: 2,
                experiencia: 4,
                item: Itens::Ataque,
            },
            Inimigos::Cobra => Inimigo {
                nome: "Cobra",
                vida: 12,
                ataque: 3,
                defesa: 4,
                experiencia: 8,
                item: Itens::Pocao,
            },
            Inimigos::Lobo => Inimigo {
                nome: "Lobo",
                vida: 12,
                ataque: 4,
                defesa: 3,
                experiencia: 8,
                item: Itens::Experiencia,
            },
        }
    }
}

impl DadosTrait for Inimigo {
    fn get_dados(&self) -> String {
        format!(
            "{}\
            \nVida: {}\
            \nAtaque: {}\
            \nDefesa: {}\
            \nExperiência: {}\
            \nItem: {}",
            self.nome,
            self.vida,
            self.ataque,
            self.defesa,
            self.experiencia,
            Item::get_item(&self.item).nome,
        )
    }
}

pub const INIMIGOS: &[Inimigos] = &[
    Inimigos::Rato,
    Inimigos::Coelho,
    Inimigos::Cobra,
    Inimigos::Lobo,
];
