use crate::dados::inimigos::Inimigos;

pub struct Local {
    pub id: usize,
    pub nome: &'static str,
    pub inimigos: [Inimigos; 4],
}
