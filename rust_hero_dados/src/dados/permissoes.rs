#[derive(Clone, Copy)]
pub enum Permissoes {
    SaveEditor = 0b00001,
    EquipamentosEnciclopedia = 0b00010,
    ItensEnciclopedia = 0b00100,
    LugaresEnciclopedia = 0b01000,
    InimigosEnciclopedia = 0b10000,
}

impl Permissoes {
    pub fn possui_permissao(self, permissao_usuario: u8) -> Option<Self> {
        if self as u8 & permissao_usuario == self as u8 {
            Some(self)
        } else {
            None
        }
    }

    pub fn nome_permissao(self) -> &'static str {
        match self {
            Permissoes::SaveEditor => "Editor de Save",
            Permissoes::EquipamentosEnciclopedia => "Lista de Equipamentos",
            Permissoes::ItensEnciclopedia => "Lista de Itens",
            Permissoes::LugaresEnciclopedia => "Lista de Lugares",
            Permissoes::InimigosEnciclopedia => "Lista de Inimigos",
        }
    }

    pub fn adiciona_permissao(self, permissoes: &mut u8) {
        *permissoes += self as u8;
    }
}
