pub struct Inimigo<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub vida: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
    pub item: usize
}