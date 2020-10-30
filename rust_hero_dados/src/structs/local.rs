use crate::dados::inimigos::Inimigos;
use crate::traits::dados_trait::DadosTrait;

pub struct Local {
    pub id: usize,
    pub nome: &'static str,
    pub inimigos: [Inimigos; 4],
}

impl DadosTrait for Local {
    fn mostrar_dados(&self) -> String {
        let mut nome_inimigos = String::from("");
        for inimigo in self.inimigos.iter() {
            match inimigo {
                Inimigos::Nenhum => {}
                _ => {
                    nome_inimigos.push_str(&format!("\n{}", inimigo.get_inimigo().nome));
                }
            }
        }
        format!("{}\nID: {}\nInimigos:{}", self.nome, self.id, nome_inimigos)
    }
}
