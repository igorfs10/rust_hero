use crate::jogo::{des_criptografar, TipoPermissao};

#[derive(Clone, Copy)]
pub enum Permissoes {
    SaveEditor = 0b1,
    EquipamentosEnciclopedia = 0b10,
    ItensEnciclopedia = 0b100,
    LugaresEnciclopedia = 0b1000,
    InimigosEnciclopedia = 0b10000,
}

impl Permissoes {
    pub fn possui_permissao(self, permissao_usuario: TipoPermissao) -> Option<Self> {
        let permissao_descriptografada = des_criptografar(permissao_usuario);
        if self as TipoPermissao & permissao_descriptografada == self as TipoPermissao {
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

    pub fn adicionar_permissao(self, permissoes: &mut TipoPermissao) {
        let permissao_descriptografada = des_criptografar(*permissoes);
        let permissao = self as TipoPermissao | permissao_descriptografada;
        *permissoes = des_criptografar(permissao);
    }

    pub fn remover_permissao(self, permissoes: &mut TipoPermissao) {
        let permissao_descriptografada = des_criptografar(*permissoes);
        let permissao = !(self as TipoPermissao) & permissao_descriptografada;
        *permissoes = des_criptografar(permissao);
    }
}
