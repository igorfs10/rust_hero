use crate::traits::dados_trait::DadosTrait;
pub struct Equipamento {
    pub nome: &'static str,
    pub ataque: u8,
    pub defesa: u8,
}

impl DadosTrait for Equipamento {
    fn get_dados(&self) -> String {
        format!(
            "{}\
            \nAtaque: {}\
            \nDefesa: {}",
            self.nome, self.ataque, self.defesa
        )
    }
}
