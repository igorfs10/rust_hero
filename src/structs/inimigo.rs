use crate::dados::itens::Itens;

pub struct Inimigo {
    pub id: usize,
    pub nome: &'static str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: Itens,
}
