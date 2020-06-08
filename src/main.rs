mod equipamento;
mod item;
mod inimigo;
mod structs;
mod erros;

use std::io;
use equipamento::equipamento::EQUIPAMENTOS;
use inimigo::dados::INIMIGOS;
use item::dados::ITENS;
use structs::{Jogador, Oponente};

fn main(){
    let jogador:Jogador;
    let oponente:Oponente;
    println!("Digite o número para escolher o equipamento:");
    println!("");
    for equipamento in &EQUIPAMENTOS {
        println!("{}: {} ", equipamento.id, equipamento.nome);
    }
    println!("");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>() {
                Ok(numero) => {
                    match erros::item_nao_existe(numero, EQUIPAMENTOS.len()){
                        Ok(_) => {

                            jogador = Jogador {
                                nome: "Teste".to_string(),
                                equipamento: EQUIPAMENTOS[numero].id,
                                vida_total: 100,
                                vida_atual: 100,
                                ataque: EQUIPAMENTOS[numero].ataque,
                                defesa: EQUIPAMENTOS[numero].defesa,
                                experiencia: 0
                            };

                            oponente = Oponente {
                                nome: INIMIGOS[1].nome.to_string(),
                                vida_total: INIMIGOS[1].vida,
                                vida_atual: INIMIGOS[1].vida,
                                ataque: INIMIGOS[1].ataque,
                                defesa: INIMIGOS[1].defesa,
                                experiencia: INIMIGOS[1].experiencia,
                                item: INIMIGOS[1].item
                            };

                            println!("-------------------");
                            println!("Status:");
                            println!("Nome: {}", jogador.nome);
                            println!("Equipamento: {}", EQUIPAMENTOS[jogador.equipamento].nome);
                            println!("Vida: {}/{}", jogador.vida_atual, jogador.vida_total);
                            println!("Ataque: {}", jogador.ataque);
                            println!("Defesa: {}", jogador.defesa);
                            println!("Experiência: {}", jogador.experiencia);
                            println!("-------------------");
                            println!("Inimigo: {}", oponente.nome);
                            println!("-------------------");
                            let ola: fn() = ITENS[0].efeito;
                            ola();
                            println!("-------------------");
                        }
                        Err(error) => {
                            println!("error: {}", error);
                        }
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                }
            }
        }
        Err(error) => println!("error: {}", error)
    }
}