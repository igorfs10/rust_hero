use crate::traits::dados_trait::DadosTrait;

#[derive(Clone)]
pub enum Itens {
    Nenhum,
    Pocao,
    Ataque,
    Defesa,
    Experiencia,
}

pub struct Item {
    pub nome: &'static str,
    pub descricao: &'static str,
    pub efeito: u8,
}

impl Item {
    // Monta o item
    pub const fn get_item(item: &Itens) -> Item {
        match item {
            Itens::Nenhum => Item {
                nome: "Nenhum",
                descricao: "Sem efeito",
                efeito: 0,
            },
            Itens::Pocao => Item {
                nome: "Poção",
                descricao: "Recupera 30% da vida.",
                efeito: 0,
            },
            Itens::Ataque => Item {
                nome: "Mais Ataque",
                descricao: "Aumenta o ataque por 1 minuto.",
                efeito: 0,
            },
            Itens::Defesa => Item {
                nome: "Mais Defesa",
                descricao: "Aumenta a defesa por 1 minuto.",
                efeito: 0,
            },
            Itens::Experiencia => Item {
                nome: "Mais Experiência",
                descricao: "Dobra a experiência ganha por 1 minuto.",
                efeito: 0,
            },
        }
    }
}

impl DadosTrait for Item {
    fn get_dados(&self) -> String {
        format!(
            "{}\
            \nDescrição: {}",
            self.nome, self.descricao
        )
    }
}

pub const ITENS: &[Itens] = &[
    Itens::Nenhum,
    Itens::Pocao,
    Itens::Ataque,
    Itens::Defesa,
    Itens::Experiencia,
];
