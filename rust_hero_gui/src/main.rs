//Remove tela de linha de comando
#![windows_subsystem = "windows"]

mod counter;

use crate::counter::*;

// Biblioteca interface
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    let mut configuracoes = Settings::default();
    configuracoes.window.resizable = false;
    configuracoes.window.size = (300, 300);
    Counter::run(configuracoes)
}
