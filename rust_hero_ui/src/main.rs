//Remove tela de linha de comando
// #![windows_subsystem = "windows"]

mod cl_ui;

use std::io;

use chrono::prelude::*;
use rand::prelude::*;
use rand::{Rng, SeedableRng};

use crate::cl_ui::limpar_terminal;
use rust_hero_dados::consts::*;
use rust_hero_dados::dados::equipamentos::EQUIPAMENTOS;
use rust_hero_dados::dados::flags::Flags;
use rust_hero_dados::dados::inimigos::INIMIGOS;
use rust_hero_dados::dados::itens::ITENS;
use rust_hero_dados::dados::locais::LOCAIS;
use rust_hero_dados::erros::*;
use rust_hero_dados::save::*;
use rust_hero_dados::structs::flag::*;
use rust_hero_dados::structs::{jogador::Jogador, oponente::Oponente};
use rust_hero_dados::traits::dados_trait::DadosTrait;
use rust_hero_dados::traits::flags_trait::FlagsTrait;

// UI
use fltk::{app::*, button::*, frame::*, menu::*, window::*};

const COMMAND_LINE_INTERFACE: bool = true;
const TESTE: &str = include_str!("conteudo/teste.txt");

fn main() {
    limpar_terminal();
    let ve = TESTE.lines().nth(1).unwrap();
    println!("{}", TESTE);
    let save = Save::default();
    let utc: DateTime<Utc> = Utc::now();
    if COMMAND_LINE_INTERFACE {
        println!("{}", ve);
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
        for x in 1..=10 as u8 {
            // Sorteia o número sem incluir o maior, no caso de 0, 10 ele sorteará de 0 à 9
            let y: u8 = rng.gen_range(0, 10 + 1);
            println!("{}: {}", x, y)
        }
        // We can also interact with iterators and slices:
        let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
        println!(
            "Lets go in this direction: {}",
            arrows_iter.choose(&mut rng).unwrap()
        );
        let mut nums = [1, 2, 3, 4, 5];
        nums.shuffle(&mut rng);
        println!("I shuffled my {:?}", nums);

        let mut flags_jogo: Flag = Flag::default();
        flags_jogo.set_flag(Flags::UpgradePocao);
        flags_jogo.set_flag(Flags::UpgradeAtaque);
        flags_jogo.clear_flag(Flags::UpgradePocao);
        println!(
            "Flag pocao: {}\nFlag ataque: {}",
            flags_jogo.check_flag(Flags::UpgradePocao),
            flags_jogo.check_flag(Flags::UpgradeAtaque)
        );

        let mut jogador: Jogador = Jogador {
            nome: String::from(TESTE.lines().nth(1).unwrap()),
            equipamento: 0,
            vida_total: 20,
            vida_atual: 20,
            ataque: 1,
            defesa: 1,
            experiencia: 0,
        };

        let oponente: Oponente;
        oponente = escolher_inimigo();
        jogador.equipamento = escolher_equipamento();
        jogador.ataque += EQUIPAMENTOS[jogador.equipamento].get_equipamento().ataque;
        jogador.defesa += EQUIPAMENTOS[jogador.equipamento].get_equipamento().defesa;

        println!("Jogador: {}", jogador.nome);
        println!("Inimigo: {}", oponente.nome);

        println!("-----LOCAIS-----");
        for local in LOCAIS.iter() {
            println!("{}\n", local.get_local().mostrar_dados());
        }

        println!("-----INIMIGOS-----");
        for inimigo in INIMIGOS.iter() {
            println!("{}\n", inimigo.get_inimigo().mostrar_dados());
        }

        println!("Ataque critico: {}\n\n", MULTIPLICADOR_CRITICO);

        println!("-----EQUIPAMENTOS-----");
        for equipamento in EQUIPAMENTOS.iter() {
            println!("{}\n", equipamento.get_equipamento().mostrar_dados());
        }

        println!("-----ITENS-----");
        for item in ITENS.iter() {
            println!("{}\n", item.get_item().mostrar_dados());
        }

        println!("{}", save.jogador.nome);
    } else {
        let app = App::default().with_scheme(AppScheme::Plastic);
        let mut janela = Window::default()
            .with_size(300, 500)
            .center_screen()
            .with_label("Rust Hero");

        let mut frame = Frame::default().with_size(300, 50).with_label("Rust Hero");

        frame.set_color(Color::from_u32(0xaaaaaa));
        frame.set_label_size(30);
        frame.set_frame(FrameType::DownBox);

        let mut botao = Button::default()
            .with_size(100, 50)
            .below_of(&frame, 0)
            .with_label("Novo jogo");

        botao.set_color(Color::from_u32(0x2e7d32));

        let mut botao_salvar = Button::default()
            .size_of(&botao)
            .right_of(&botao, 0)
            .with_label("Carregar");

        botao_salvar.set_color(Color::from_u32(0x1565c032));

        botao.set_callback(Box::new(|| {
            let v = vec!["1st val", "2nd val", "3rd val"];
            let mut x = MenuItem::new(&v);
            match x.popup(100, 100) {
                None => println!("No value was chosen!"),
                Some(val) => println!("{}", val.label().unwrap()),
            }
        }));

        janela.make_resizable(false);
        janela.end();
        janela.show();
        app.run().unwrap();
    }
}

fn escolher_equipamento() -> usize {
    limpar_terminal();
    println!(
        "Digite um número para escolher o equipamento (0-{}):\n",
        EQUIPAMENTOS.len() - 1
    );

    for equipamento in EQUIPAMENTOS {
        println!(
            "{}: {} ",
            equipamento.get_equipamento().id,
            equipamento.get_equipamento().nome
        );
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(numero) => match item_nao_existe(numero, EQUIPAMENTOS.len()) {
                Ok(_) => numero,
                Err(error) => {
                    println!("error: {}", error);
                    escolher_equipamento()
                }
            },
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

fn escolher_inimigo() -> Oponente {
    Oponente {
        nome: INIMIGOS[1].get_inimigo().nome.to_string(),
        vida_total: INIMIGOS[1].get_inimigo().vida,
        vida_atual: INIMIGOS[1].get_inimigo().vida,
        ataque: INIMIGOS[1].get_inimigo().ataque,
        defesa: INIMIGOS[1].get_inimigo().defesa,
        experiencia: INIMIGOS[1].get_inimigo().experiencia,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn tes() {
        assert_eq!(2 + 2, 4);
    }
}
