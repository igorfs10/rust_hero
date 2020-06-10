pub struct Equipamento<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub ataque: u8,
    pub defesa: u8
}