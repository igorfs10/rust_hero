use crate::item::Itens;

pub struct Inimigo {
    pub id: usize,
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: usize
}

pub enum Inimigos {
    Nenhum,
    Rato,
    Coelho,
    Cobra,
    Lobo
}

// Usar const trait quando lançar na versão estável
impl Inimigos {
    // Id dos inimigos
    pub const fn get_id(&self) -> usize {
        match self {
            Inimigos::Nenhum => 0,
            Inimigos::Rato => 1,
            Inimigos::Coelho => 2,
            Inimigos::Cobra => 3,
            Inimigos::Lobo => 4
        }
    }

    // Nome dos inimigos
    const fn get_nome(&self) -> &'static str {
        match self {
            Inimigos::Nenhum => "",
            Inimigos::Rato => "Rato",
            Inimigos::Coelho => "Coelho",
            Inimigos::Cobra => "Cobra",
            Inimigos::Lobo => "Lobo"
        }
    }

    // Vida dos inimigos
    const fn get_vida(&self) -> u8 {
        match self {
            Inimigos::Nenhum => 0,
            Inimigos::Rato => 5,
            Inimigos::Coelho => 8,
            Inimigos::Cobra => 12,
            Inimigos::Lobo => 12
        }
    }

    // Ataque dos inimigos
    const fn get_ataque(&self) -> u8 {
        match self {
            Inimigos::Nenhum => 0,
            Inimigos::Rato => 1,
            Inimigos::Coelho => 2,
            Inimigos::Cobra => 3,
            Inimigos::Lobo => 4
        }
    }

    // Defesa dos inimigos
    const fn get_defesa(&self) -> u8 {
        match self {
            Inimigos::Nenhum => 0,
            Inimigos::Rato => 1,
            Inimigos::Coelho => 2,
            Inimigos::Cobra => 4,
            Inimigos::Lobo => 3
        }
    }

    // Experiência dos inimigos
    const fn get_experiencia(&self) -> u16 {
        match self {
            Inimigos::Nenhum => 0,
            Inimigos::Rato => 2,
            Inimigos::Coelho => 4,
            Inimigos::Cobra => 8,
            Inimigos::Lobo => 8
        }
    }

    // Item dos inimigos
    const fn get_item(&self) -> usize {
        match self {
            Inimigos::Nenhum => Itens::Nenhum.get_id(),
            Inimigos::Rato => Itens::Defesa.get_id(),
            Inimigos::Coelho => Itens::Ataque.get_id(),
            Inimigos::Cobra => Itens::Pocao.get_id(),
            Inimigos::Lobo => Itens::Experiencia.get_id()
        }
    }

    //Monta o inimigo
    const fn make(&self) -> Inimigo {
        Inimigo {
            id: self.get_id(),
            nome: self.get_nome(),
            vida: self.get_vida(),
            ataque: self.get_ataque(),
            defesa: self.get_defesa(),
            experiencia: self.get_experiencia(),
            item: self.get_item()
        }
    }
}

pub const INIMIGOS: [Inimigo; 5] = [
    Inimigos::Nenhum.make(),
    Inimigos::Rato.make(),
    Inimigos::Coelho.make(),
    Inimigos::Cobra.make(),
    Inimigos::Lobo.make(),
];