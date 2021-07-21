//! Sistema de batalha - Módulo que contém funções para realizar o sistema de batalha.

use crate::data::locations::{Location, Locations};
use crate::jogo::*;
use crate::structs::personagem::Personagem;

fn _start_battle(enemy: &mut Personagem, flag: &mut bool) {
    *flag = true;
    definir_inimigo(
        enemy,
        sortear_inimigo_lugar(&Location::get_location(&Locations::Forest), &0).unwrap(),
    );
}
