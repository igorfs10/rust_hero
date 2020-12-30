use crate::dados::lugares::Lugares;
use crate::jogo::*;
use crate::structs::personagem::Personagem;

fn _comecar_batalha(inimigo: &mut Personagem, flag: &mut bool) {
    *flag = true;
    definir_inimigo(
        inimigo,
        sortear_inimigo_lugar(&Lugares::Floresta.get_lugar(), &0).unwrap(),
    );
}
