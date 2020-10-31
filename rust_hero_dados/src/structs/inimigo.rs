use crate::dados::itens::Itens;
use crate::traits::dados_trait::DadosTrait;

pub struct Inimigo {
    pub id: usize,
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
            "{}\nID: {}\nVida: {}\nAtaque: {}\nDefesa: {}\nExperiência: {}\nItem: {}",
            self.nome,
            self.id,
            self.vida,
            self.ataque,
            self.defesa,
            self.experiencia,
            self.item.get_item().nome
        )
    }
}
