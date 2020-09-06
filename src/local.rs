pub enum Locais {
    Nenhum,
    Cidade,
    Floresta
}

pub struct Local<'a> {
    pub id: usize,
    pub nome: &'a str
}

pub const LOCAIS: [Local; 3] = [
    Local {
        id: Locais::Nenhum as usize,
        nome: ""
    },
    Local {
        id: Locais::Cidade as usize,
        nome: "Cidade"
    },
    Local {
        id: Locais::Floresta as usize,
        nome: "Floresta"
    }
];