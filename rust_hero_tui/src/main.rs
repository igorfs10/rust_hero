mod tl_ui;

use std::io;
use std::time::Instant;

use tl_ui::{limpar_terminal, mostrar_dados};

use rust_hero_dados::dados::{
    equipamentos::{Equipamento, Equipamentos, EQUIPAMENTOS},
    flags::Flags,
    inimigos::{Inimigo, INIMIGOS},
    itens::{Item, ITENS},
    lugares::{Lugar, LUGARES},
};
use rust_hero_dados::jogo::*;
use rust_hero_dados::structs::personagem::Personagem;
use rust_hero_dados::utils::save_system::{get_save, save_game};

const TESTE: &str = include_str!("conteudo/teste.txt");

const NOME: &str = env!("CARGO_CRATE_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let agora = Instant::now();
    limpar_terminal();
    println!("Pacote: {}  Versão: {}", NOME, VERSION);
    println!("{}", TESTE);
    let ve = TESTE.lines().nth(1).unwrap();
    println!("{}", ve);
    let mut save = get_save();

    // println!("Save 1:\n{:?}", save);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("error: {}", error);
        }
    }

    let mut jogador: Personagem = Personagem {
        nome: input,
        vida_total: 20,
        vida_atual: 20,
        ataque: 1,
        defesa: 1,
        experiencia: 0,
    };

    save.equipamento = escolher_equipamento();
    let equip = Equipamento::get_equipamento(&save.equipamento);
    jogador.ataque += equip.ataque;
    jogador.defesa += equip.defesa;

    println!("Jogador: {}", jogador.nome);

    save.jogador = jogador;

    // println!("Save 2:\n{:?}", save);

    println!("-----LOCAIS-----");
    for lugar in LUGARES.iter() {
        mostrar_dados(Lugar::get_lugar(lugar));
    }

    println!("-----INIMIGOS-----");
    for inimigo in INIMIGOS.iter() {
        mostrar_dados(Inimigo::get_inimigo(inimigo));
    }

    println!("Ataque critico: {}\n\n", MULTIPLICADOR_CRITICO);

    println!("-----EQUIPAMENTOS-----");
    for equipamento in EQUIPAMENTOS.iter() {
        mostrar_dados(Equipamento::get_equipamento(equipamento));
    }

    println!("-----ITENS-----");
    for item in ITENS.iter() {
        mostrar_dados(Item::get_item(item));
    }

    save.set_flag(Flags::EnciclopediaEquipamentos);
    save.set_flag(Flags::EnciclopediaInimigos);
    save.set_flag(Flags::EnciclopediaItens);
    save.set_flag(Flags::EnciclopediaLugares);
    save.set_flag(Flags::SaveEditor);
    save.set_flag(Flags::UpgradeAtaque);
    save.set_flag(Flags::UpgradeDefesa);
    save.set_flag(Flags::UpgradeExperiencia);
    save.set_flag(Flags::UpgradePocao);

    save.clear_flag(Flags::EnciclopediaEquipamentos);
    save.clear_flag(Flags::EnciclopediaLugares);

    save.set_flag(Flags::EnciclopediaEquipamentos);

    if save.check_flag(Flags::EnciclopediaEquipamentos) {
        println!("Equipamentos");
    }
    if save.check_flag(Flags::EnciclopediaInimigos) {
        println!("Inimigos");
    }
    if save.check_flag(Flags::EnciclopediaItens) {
        println!("Itens");
    }
    if save.check_flag(Flags::EnciclopediaLugares) {
        println!("Lugares");
    }

    if save.check_flag(Flags::UpgradeAtaque) {
        println!("Ataque");
    }

    if save.check_flag(Flags::UpgradeDefesa) {
        println!("Defesa");
    }

    if save.check_flag(Flags::UpgradeExperiencia) {
        println!("EXP");
    }

    if save.check_flag(Flags::UpgradePocao) {
        println!("Pocao");
    }

    if save.check_flag(Flags::SaveEditor) {
        println!("Save");
    }

    println!("o nome do save é {}", save.jogador.nome);
    let tempo_jogado = agora.elapsed().as_secs();
    println!("segundos: {:?}", tempo_jogado);
    save.tempo += tempo_jogado;
    save_game(&save);
}

fn escolher_equipamento() -> Equipamentos {
    println!(
        "Digite um número para escolher o equipamento (0-{}):\n",
        EQUIPAMENTOS.len() - 1
    );

    for (index, equipamento) in EQUIPAMENTOS.iter().enumerate() {
        println!(
            "{}: {} ",
            index,
            Equipamento::get_equipamento(&equipamento).nome
        );
    }
    println!("{} ou mais: Nenhum", EQUIPAMENTOS.len());

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(numero) => match_equipamento(numero),
            Err(error) => {
                println!("error: {}", error);
                escolher_equipamento()
            }
        },
        Err(error) => {
            println!("error: {}", error);
            escolher_equipamento()
        }
    }
}
