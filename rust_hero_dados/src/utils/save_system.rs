use std::{
    env,
    fs::{read, File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::structs::save::Save;

pub fn get_save_location() -> String {
    // Pega o caminho do executável para criar a configuração e pasta de conversão
    let caminho_executavel = env::current_exe().unwrap();
    let nome_save = caminho_executavel.file_stem().unwrap().to_str().unwrap();
    let caminho = caminho_executavel.parent().unwrap().to_str().unwrap();
    format!("{}/{}.sav", caminho, nome_save)
}

pub fn get_save() -> Save {
    let save: Save;
    let caminho_save = get_save_location();

    if Path::new(&caminho_save).exists() {
        println!("Arquivo de configurações encontrado. Carregando configurações...");
        let arquivo = read(&caminho_save).expect("Não foi possível ler o arquivo de configuração");

        save = bincode::deserialize(&arquivo[..]).unwrap();
        save
    } else {
        println!("Arquivo de configurações não encontrado. Criando arquivo de save novo...");
        save = Save::novo(&0);
        let mut arquivo =
            File::create(caminho_save).expect("Não foi possível criar o arquivo de save");

        let conteudo = bincode::serialize(&save).unwrap();

        arquivo
            .write_all(&conteudo[..])
            .expect("Não foi possível salvar o arquivo de save");

        save
    }
}

pub fn save_game(save: &Save) {
    let caminho_save = get_save_location();
    let conteudo = bincode::serialize(save).unwrap();

    let mut arquivo = OpenOptions::new().write(true).open(caminho_save).unwrap();
    arquivo
        .write_all(&conteudo[..])
        .expect("Não foi possível salvar o arquivo de save");
}
