mod equipamento;
mod item;
mod inimigo;
mod structs;
mod erros;
mod flags;

use std::io;
use equipamento::dados::EQUIPAMENTOS;
use inimigo::dados::INIMIGOS;
use item::dados::ITENS;
use structs::{Jogador, Oponente};
use flags::*;

fn main(){
    println!("{}, {}", FLAGS::UpgradePocao as usize, FLAGS::UpgradeAtaque as usize);
    let mut flags_jogo:[bool; 5] = [false;5];
    set_flag(&mut flags_jogo, FLAGS::UpgradePocao);
    set_flag(&mut flags_jogo, FLAGS::UpgradeAtaque);
    clear_flag(&mut flags_jogo, FLAGS::UpgradePocao);
    println!("Flag pocao: {}\nFlag_ataque: {}", check_flag(&flags_jogo, FLAGS::UpgradePocao), check_flag(&flags_jogo, FLAGS::UpgradeAtaque));
    let mut jogador:Jogador = Jogador {
        nome: "Teste".to_string(),
        equipamento: 0,
        vida_total: 20,
        vida_atual: 20,
        ataque: 0,
        defesa: 0,
        experiencia: 0
    };
    let oponente:Oponente;
    oponente = escolher_inimigo();
    println!("Digite o número para escolher o equipamento:");
    println!("");
    for equipamento in &EQUIPAMENTOS {
        println!("{}: {} ", equipamento.id, equipamento.nome);
    }
    jogador.equipamento = escolher_equipamento();
    jogador.ataque = EQUIPAMENTOS[jogador.equipamento].ataque;
    jogador.defesa = EQUIPAMENTOS[jogador.equipamento].defesa;

    println!("Jogador: {}", jogador.nome);
    println!("Inimigo: {}", oponente.nome);
    let ola: fn() = ITENS[0].efeito;
    ola();
    // Testar e Arrumar
    (ITENS[0].efeito)();
    let test = &escolher_equipamento;
    let nume_teste = test();
    println!("fdff");
    println!("Pointer: {:p}\nVar: {:p}", &escolher_equipamento, test);
    println!("{}", nume_teste);
}

fn escolher_equipamento() -> usize {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>() {
                Ok(numero) => {
                    match erros::item_nao_existe(numero, EQUIPAMENTOS.len()){
                        Ok(_) => {
                            return numero;
                        }
                        Err(error) => {
                            println!("error: {}", error);
                            return escolher_equipamento();
                        }
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                    return escolher_equipamento();
                }
            }
        }
        Err(error) => {
            println!("error: {}", error);
            return escolher_equipamento();
        }
    }
}

fn escolher_inimigo() -> Oponente{
    let oponente: Oponente = Oponente{
        nome: INIMIGOS[1].nome.to_string(),
        vida_total: INIMIGOS[1].vida,
        vida_atual: INIMIGOS[1].vida,
        ataque: INIMIGOS[1].ataque,
        defesa: INIMIGOS[1].defesa,
        experiencia: INIMIGOS[1].experiencia,
        item: INIMIGOS[1].item
    };
    return oponente;
}
