pub struct Item {
    pub id: usize,
    pub nome: &'static str,
    pub descricao: &'static str,
    pub efeito: u8
}

pub enum Itens {
    Nenhum,
    Pocao,
    Ataque,
    Defesa,
    Experiencia
}

// Usar const trait quando lançar na versão estável
impl Itens {
    // Id dos itens
    pub const fn get_id(&self) -> usize {
        match self {
            Itens::Nenhum => 0,
            Itens::Pocao => 1,
            Itens::Ataque => 2,
            Itens::Defesa => 3,
            Itens::Experiencia => 4
        }
    }

    // Nome dos itens
    const fn get_nome(&self) -> &'static str {
        match self {
            Itens::Nenhum => "",
            Itens::Pocao => "Poção",
            Itens::Ataque => "Mais Ataque",
            Itens::Defesa => "Mais Defesa",
            Itens::Experiencia => "Mais Experiência"
        }
    }

    //Descrições dos itens
    const fn get_descricao(&self) -> &'static str {
        match self {
            Itens::Nenhum => "",
            Itens::Pocao => "Recupera 30% da vida.",
            Itens::Ataque => "Aumenta o ataque por 1 minuto.",
            Itens::Defesa => "Aumenta a defesa por 1 minuto.",
            Itens::Experiencia => "Dobra a experiência ganha por 1 minuto."
        }
    }

    //Monta o item
    const fn make(&self) -> Item {
        Item {
            id: self.get_id(),
            nome: self.get_nome(),
            descricao: self.get_descricao(),
            efeito: 0
        }
    }
}

pub const ITENS: [Item; 5] = [
    Itens::Nenhum.make(),
    Itens::Pocao.make(),
    Itens::Ataque.make(),
    Itens::Defesa.make(),
    Itens::Experiencia.make()
];