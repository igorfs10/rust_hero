#[derive(PartialEq)]
pub struct Personagem {
    pub nome: String,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
}

impl Personagem {
    const fn new(
        nome: String,
        vida_total: u8,
        vida_atual: u8,
        ataque: u8,
        defesa: u8,
        experiencia: u16,
    ) -> Self {
        Personagem {
            nome,
            vida_total,
            vida_atual,
            ataque,
            defesa,
            experiencia,
        }
    }
}

impl Default for Personagem {
    fn default() -> Self {
        Personagem::new(String::from(""), 10, 10, 1, 1, 0)
    }
}
