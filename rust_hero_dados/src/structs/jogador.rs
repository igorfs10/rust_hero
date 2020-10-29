pub struct Jogador {
    pub nome: String,
    pub equipamento: usize,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
}

impl Default for Jogador {
    fn default() -> Self {
        Jogador {
            nome: String::from(""),
            equipamento: 0,
            vida_total: 10,
            vida_atual: 10,
            ataque: 0,
            defesa: 0,
            experiencia: 0,
        }
    }
}
