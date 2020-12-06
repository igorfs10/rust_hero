#[cfg(test)]
pub mod tests {
    use crate::dados::lugares::LUGARES;
    use crate::jogo::*;
    use crate::structs::lugar::Lugar;
    use crate::structs::personagem::Personagem;

    #[test]
    fn sorteio_inimigo_1() {
        let mut oponente = Personagem::default();
        let lugar: Lugar = LUGARES[1].get_local();
        //0 inimigo 1
        //2 inimigo 2
        //1 inimigo 3
        //4 inimigo 4
        let inimigo = sortear_inimigo_lugar(&lugar, &0).unwrap();
        definir_inimigo(&mut oponente, inimigo);
        println!("nome:{}", oponente.nome);
    }
}
