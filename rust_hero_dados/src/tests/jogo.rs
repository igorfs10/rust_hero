use crate::dados::lugares::LUGARES;
use crate::jogo::*;
use crate::structs::lugar::Lugar;
use crate::structs::personagem::Personagem;

#[test]
fn sorteio_inimigo_1() {
    let mut oponente = Personagem::default();
    let lugar: Lugar = LUGARES[1].get_local();
    //1 inimigo 1
    //3 inimigo 2
    //15 inimigo 3
    //19 inimigo 4
    let inimigo = sortear_inimigo_lugar(&lugar, &1).unwrap();
    definir_inimigo(&mut oponente, inimigo);
    println!("nome:{}", oponente.nome);
}
