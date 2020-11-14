use crate::dados::inimigos::Inimigos;
use crate::traits::dados_trait::DadosTrait;

pub struct Lugar {
    pub id: usize,
    pub nome: &'static str,
    pub inimigos: Option<[Inimigos; 4]>,
}

impl DadosTrait for Lugar {
    fn get_dados(&self) -> String {
        let mut dados = format!("{}\nID: {}", self.nome, self.id);
        match self.inimigos {
            Some(inimigos) => {
                let mut nome_inimigos = String::from("\nInimigos:");
                for inimigo in inimigos.iter() {
                    nome_inimigos.push_str(&format!("\n{}", inimigo.get_inimigo().nome));
                }
                dados.push_str(&nome_inimigos);
                dados
            }
            None => dados,
        }
    }
}
