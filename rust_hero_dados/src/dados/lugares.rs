//! Lugares - Dados e funções relacionados à estrutura de lugares.

use crate::dados::inimigos::{Inimigo, Inimigos};
use crate::traits::dados_trait::DadosTrait;

pub enum Lugares {
    Cidade,
    Floresta,
}

pub struct Lugar {
    pub nome: &'static str,
    pub inimigos: Option<[Inimigos; 4]>,
}

impl Lugar {
    // Monta o Lugar
    pub const fn get_lugar(lugar: &Lugares) -> Self {
        match lugar {
            Lugares::Cidade => Lugar {
                nome: "Cidade",
                inimigos: None,
            },
            Lugares::Floresta => Lugar {
                nome: "Floresta",
                inimigos: Some([
                    Inimigos::Rato,
                    Inimigos::Coelho,
                    Inimigos::Cobra,
                    Inimigos::Lobo,
                ]),
            },
        }
    }
}

impl DadosTrait for Lugar {
    fn get_dados(&self) -> String {
        let mut dados = self.nome.to_string();
        match &self.inimigos {
            Some(inimigos) => {
                let mut nome_inimigos = String::from("\nInimigos:");
                for inimigo in inimigos.iter() {
                    nome_inimigos.push_str(&format!("\n{}", Inimigo::get_inimigo(inimigo).nome));
                }
                dados.push_str(&nome_inimigos);
                dados
            }
            None => dados,
        }
    }
}

pub const LUGARES: &[Lugares] = &[Lugares::Cidade, Lugares::Floresta];
