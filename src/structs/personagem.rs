pub struct Personagem<'a> {
    pub id: u8,
    pub nome: &'a str,
    pub vida: u16,
    pub ataque: u8,
    pub defesa: u8,
}