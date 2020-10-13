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
    pub const fn get_id(&self) -> usize {
        match self {
            Equipamentos::Nenhum => 0,
            Equipamentos::Espada => 1,
            Equipamentos::Escudo => 2,
            Equipamentos::Bastao => 3,
        }
    }

    // Nome dos equipamentos
    const fn get_nome(&self) -> &'static str {
        match self {
            Equipamentos::Nenhum => "",
            Equipamentos::Espada => "Espada",
            Equipamentos::Escudo => "Escudo",
            Equipamentos::Bastao => "Bastão",
        }
    }

    //Ataque dos equipamentos
    const fn get_ataque(&self) -> u8 {
        match self {
            Equipamentos::Nenhum => 1,
            Equipamentos::Espada => 3,
            Equipamentos::Escudo => 1,
            Equipamentos::Bastao => 2,
        }
    }

    //Defesa dos equipamentos
    const fn get_defesa(&self) -> u8 {
        match self {
            Equipamentos::Nenhum => 1,
            Equipamentos::Espada => 1,
            Equipamentos::Escudo => 3,
            Equipamentos::Bastao => 2,
        }
    }

    //Monta o equipamento
    const fn make(&self) -> Equipamento {
        Equipamento {
            id: self.get_id(),
            nome: self.get_nome(),
            ataque: self.get_ataque(),
            defesa: self.get_defesa(),
        }
    }
}

pub const EQUIPAMENTOS: [Equipamento; 4] = [
    Equipamentos::Nenhum.make(),
    Equipamentos::Espada.make(),
    Equipamentos::Escudo.make(),
    Equipamentos::Bastao.make(),
];
