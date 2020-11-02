pub struct Personagem {
    pub nome: String,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
}

impl Default for Personagem {
    fn default() -> Self {
        Personagem {
            nome: String::from(""),
            vida_total: 10,
            vida_atual: 10,
            ataque: 1,
            defesa: 1,
            experiencia: 0,
        }
    }
}
