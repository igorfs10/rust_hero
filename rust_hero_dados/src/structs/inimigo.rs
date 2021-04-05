use crate::dados::itens::Itens;
use crate::traits::dados_trait::DadosTrait;

#[derive(Clone)]
pub struct Inimigo {
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: Itens,
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
            self.nome, self.vida, self.ataque, self.defesa, self.experiencia, self.item.clone().get_item().nome,
        )
    }
}
