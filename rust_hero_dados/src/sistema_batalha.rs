//! Sistema de batalha - Módulo que contém funções para realizar o sistema de batalha.

use crate::dados::lugares::{Lugar, Lugares};
use crate::jogo::*;
use crate::structs::personagem::Personagem;

fn _comecar_batalha(inimigo: &mut Personagem, flag: &mut bool) {
    *flag = true;
    definir_inimigo(
        inimigo,
        sortear_inimigo_lugar(&Lugar::get_lugar(&Lugares::Floresta), &0).unwrap(),
    );
}
