use crate::traits::dados_trait::DadosTrait;

pub struct Item {
    pub id: usize,
    pub nome: &'static str,
    pub descricao: &'static str,
    pub efeito: u8,
}

impl DadosTrait for Item {
    fn get_dados(&self) -> String {
        format!(
            "{}\nId: {}\nDescrição: {}",
            self.nome, self.id, self.descricao
        )
    }
}
