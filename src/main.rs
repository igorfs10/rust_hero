mod equipamento;
mod jogador;

use std::io;
use equipamento::equipamento::{EQUIPAMENTOS};
use jogador::jogador::Jogador;

fn main(){
    let jogador:Jogador;
    println!("Digite o número para escolher o equipamento:");
    println!("");
    for equipamento in &EQUIPAMENTOS{
        println!("{}: {} ", equipamento.id, equipamento.nome);
    }
    println!("");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>(){
                Ok(numero) => {
                    jogador = Jogador {
                        nome: "Teste".to_string(),
                        equipamento: EQUIPAMENTOS[numero].nome.to_string(),
                        vida_total: 100,
                        vida_atual: 100,
                        ataque: EQUIPAMENTOS[numero].ataque,
                        defesa: EQUIPAMENTOS[numero].defesa,
                        experiencia: 0
                    };
                    println!("-------------------");
                    println!("Status:");
                    println!("Nome: {}", jogador.nome);
                    println!("Equipamento: {}", jogador.equipamento);
                    println!("Vida: {}/{}", jogador.vida_atual, jogador.vida_total);
                    println!("Ataque: {}", jogador.ataque);
                    println!("Defesa: {}", jogador.defesa);
                    println!("Experiência: {}", jogador.experiencia);
                    println!("-------------------");
                }
                Err(error) => println!("error: {}", error)
            }
        }
        Err(error) => println!("error: {}", error)
    }  
}