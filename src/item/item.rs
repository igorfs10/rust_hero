pub struct Item<'a> {
    pub id: usize,
    pub nome: &'a str,
    pub descricao: &'a str,
    pub efeito: fn()
}

pub fn nada () {
    println!("TESTE");
}