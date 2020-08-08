mod equipamento;
mod item;
mod inimigo;
mod structs;
mod erros;
mod flags;

use std::io;
use console::Term;

use equipamento::EQUIPAMENTOS;
use inimigo::INIMIGOS;
use item::ITENS;
use structs::{Jogador, Oponente};
use erros::*;
use flags::*;

fn main(){
    limpar_terminal();
    let mut flags_jogo:[bool; 5] = [false;5];
    set_flag(&mut flags_jogo, FLAGS::UpgradePocao);
    set_flag(&mut flags_jogo, FLAGS::UpgradeAtaque);
    clear_flag(&mut flags_jogo, FLAGS::UpgradePocao);
    println!("Flag pocao: {}\nFlag_ataque: {}", check_flag(&flags_jogo, FLAGS::UpgradePocao), check_flag(&flags_jogo, FLAGS::UpgradeAtaque));
    
    let mut jogador:Jogador = Jogador {
        nome: String::from("Teste"),
        equipamento: 0,
        vida_total: 20,
        vida_atual: 20,
        ataque: 0,
        defesa: 0,
        experiencia: 0
    };

    let oponente:Oponente;
    oponente = escolher_inimigo();
    jogador.equipamento = escolher_equipamento();
    jogador.ataque = EQUIPAMENTOS[jogador.equipamento].ataque;
    jogador.defesa = EQUIPAMENTOS[jogador.equipamento].defesa;

    println!("Jogador: {}", jogador.nome);
    println!("Inimigo: {}", oponente.nome);

    println!("{}", ITENS[0].nome);
}

fn escolher_equipamento() -> usize {
    #[cfg(not(debug_assertions))]
    limpar_terminal();
    println!("Digite um número para escolher o equipamento (0-{}):\n", EQUIPAMENTOS.len() - 1);
    for equipamento in &EQUIPAMENTOS {
        println!("{}: {} ", equipamento.id, equipamento.nome);
    }
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>() {
                Ok(numero) => {
                    match item_nao_existe(numero, EQUIPAMENTOS.len()){
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

fn limpar_terminal(){
    let term = Term::stdout();
    let _ = term.clear_screen();
}