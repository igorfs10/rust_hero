pub struct Local {
    pub id: usize,
    pub nome: &'static str,
}

pub enum Locais {
    Nenhum,
    Cidade,
    Floresta,
}

impl Locais {
    // Id dos locais
    pub const fn get_id(self) -> usize {
        self as usize
    }

    // Monta o Local
    const fn get_local(self) -> Local {
        match self {
            Locais::Nenhum => Local {
                id: self.get_id(),
                nome: "",
            },
            Locais::Cidade => Local {
                id: self.get_id(),
                nome: "Cidade",
            },
            Locais::Floresta => Local {
                id: self.get_id(),
                nome: "Floresta",
            },
        }
    }
}

pub const LOCAIS: [Local; 3] = [
    Locais::Nenhum.get_local(),
    Locais::Cidade.get_local(),
    Locais::Floresta.get_local(),
];
