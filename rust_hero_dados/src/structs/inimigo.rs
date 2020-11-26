use crate::dados::itens::Itens;
use crate::traits::dados_trait::DadosTrait;

#[derive(Default, Debug, Clone)]
pub struct Inimigo {
    pub id: usize,
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: Option<Itens>,
}

impl DadosTrait for Inimigo {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_dados(&self) -> String {
        let mut dados = format!(
            "{}\nID: {}\nVida: {}\nAtaque: {}\nDefesa: {}\nExperiência: {}",
            self.nome, self.id, self.vida, self.ataque, self.defesa, self.experiencia,
        );
        match self.item {
            Some(item) => {
                dados.push_str(&format!("\nItem: {}", item.get_item().nome));
                dados
            }
            None => dados,
        }
    }
}
