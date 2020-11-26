use crate::traits::dados_trait::DadosTrait;
pub struct Equipamento {
    pub id: usize,
    pub nome: &'static str,
    pub ataque: u8,
    pub defesa: u8,
}

impl DadosTrait for Equipamento {
    fn get_id(&self) -> usize {
        self.id    
    }

    fn get_dados(&self) -> String {
        format!(
            "{}\nID: {}\nAtaque: {}\nDefesa: {}",
            self.nome, self.id, self.ataque, self.defesa
        )
    }
}
