#![windows_subsystem = "windows"]

mod local;
mod equipamento;
mod item;
mod inimigo;
mod structs;
mod erros;
mod flags;
mod consts;
mod save;
mod login_form;
mod traits;

use std::io;
use console::Term;
use rand::prelude::*;
use rand::{ Rng, SeedableRng };
use chrono::prelude::*;

use equipamento::EQUIPAMENTOS;
use inimigo::INIMIGOS;
use item::ITENS;
use local::LOCAIS;
use structs::{ Jogador, Oponente };
use erros::*;
use flags::*;
use consts::*;
use save::*;
use login_form::*;

// UI
use orbtk::prelude::*;


const COMMAND_LINE_INTERFACE:bool = false;

fn main(){
    let save = Save::default();
    let utc: DateTime<Utc> = Utc::now();
    if COMMAND_LINE_INTERFACE {
        limpar_terminal();
        println!("{}", utc.second());

        // Aleatório com seed definido
        let mut nops: StdRng = SeedableRng::seed_from_u64(5);
        // Retornar um numero com todo o range do u8
        println!("{}", nops.gen::<u8>());
        // O seed é redefinido a cada uso do aleatório, então temos que definir novamente para gerar o mesmo resultado
        nops = SeedableRng::seed_from_u64(5);
        println!("{}", nops.gen_range(0, 255));

        // Brincando com o rng
        let mut rng = thread_rng();
        // O tipo não é obrigatório no loop. Se não for definido ele será i32. Usa "=" no ultimo elemento se quiser incluir ele.
        for x in 1..=10 as u8{
            // Sorteia o número sem incluir o maior, no caso de 0, 10 ele sorteará de 0 à 9
            let y: u8 = rng.gen_range(0, 10 + 1);
            println!("{}: {}", x, y)
        }
        // We can also interact with iterators and slices:
        let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
        println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());
        let mut nums = [1, 2, 3, 4, 5];
        nums.shuffle(&mut rng);
        println!("I shuffled my {:?}", nums);

        let mut flags_jogo:Flags = Flags::default();
        flags_jogo.set_flag(FlagName::UpgradePocao);
        flags_jogo.set_flag(FlagName::UpgradeAtaque);
        flags_jogo.clear_flag(FlagName::UpgradePocao);
        println!("Flag pocao: {}\nFlag ataque: {}", flags_jogo.check_flag(FlagName::UpgradePocao), flags_jogo.check_flag(FlagName::UpgradeAtaque));
        
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

        println!("-----LOCAIS-----");
        for local in LOCAIS.iter() {
            println!("Nome: {}\n\n", local.nome);
        }

        println!("-----INIMIGOS-----");
        for inimigo in INIMIGOS.iter() {
            println!("{}\n", inimigo.nome);
            println!("Vida: {}", inimigo.vida);
            println!("Ataque: {}", inimigo.ataque);
            println!("Defesa: {}\n\n", inimigo.defesa);
        }

        println!("-----EQUIPAMENTOS-----");
        for equipamento in EQUIPAMENTOS.iter() {
            println!("{}\n", equipamento.nome);
            println!("Ataque: {}", equipamento.ataque);
            println!("Ataque crítico: {}", equipamento.ataque * MULTIPLICADOR_CRITICO);
            println!("Defesa: {}\n\n", equipamento.defesa);
        }

        println!("-----ITENS-----");
        for item in ITENS.iter() {
            println!("{}\n", item.nome);
            println!("Descrição: {}\n\n", item.descricao);
        }

        println!("{}", save.jogador.nome);

    } else {
        Application::new()
        .window(|ctx| {
            Window::new()
                .title(INIMIGOS[1].nome)
                .position((125.0, 125.0))
                .size(300.0, 300.0)
                .resizeable(false)
                .child(LoginForm::new().build(ctx))
                .build(ctx)
        })
        .run();
    }
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

fn escolher_inimigo() -> Oponente {
    let oponente: Oponente = Oponente {
        nome: INIMIGOS[1].nome.to_string(),
        vida_total: INIMIGOS[1].vida,
        vida_atual: INIMIGOS[1].vida,
        ataque: INIMIGOS[1].ataque,
        defesa: INIMIGOS[1].defesa,
        experiencia: INIMIGOS[1].experiencia,
    };
    return oponente;
}

fn limpar_terminal(){
    let term = Term::stdout();
    let _ = term.clear_screen();
}

#[cfg(test)]
mod test {

    #[test]
    fn tes() {
        assert_eq!(2 + 2, 4);
    }
    
}