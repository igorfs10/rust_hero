mod tl_ui;

use std::io;
use std::time::Instant;

use nanorand::{WyRand, RNG};

use tl_ui::{limpar_terminal, mostrar_dados};

use rust_hero_dados::dados::equipamentos::{Equipamentos, EQUIPAMENTOS};
use rust_hero_dados::dados::flags::Flags;
use rust_hero_dados::dados::inimigos::INIMIGOS;
use rust_hero_dados::dados::itens::ITENS;
use rust_hero_dados::dados::lugares::LUGARES;
use rust_hero_dados::jogo::*;
use rust_hero_dados::structs::inimigo::Inimigo;
use rust_hero_dados::structs::personagem::Personagem;
use rust_hero_dados::structs::save::Save;
use rust_hero_dados::traits::dados_trait::BaseRepo;
use rust_hero_dados::traits::dados_trait::Repo;

const TESTE: &str = include_str!("conteudo/teste.txt");

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let agora = Instant::now();
    limpar_terminal();
    println!("{}", VERSION);
    let mut vetor = vec![Inimigo::default(), Inimigo::default()];
    vetor[0].id = 1;
    vetor[0].nome = "EEE";
    let oi = Repo::<Inimigo>::get_by_id(vetor, 1);
    println!("{} TTTTT", oi.unwrap().nome);
    let ve = TESTE.lines().nth(1).unwrap();
    println!("{}", TESTE);
    let mut save = Save::novo(&8520);
    println!("{}", ve);

    // Aleatório com seed definido
    let mut nops = WyRand::new_seed(5);
    // Retorna um numero com todo o range do u8
    println!("{}", nops.generate::<u8>());
    // O seed é redefinido a cada uso do aleatório, então temos que definir novamente para gerar o mesmo resultado
    println!("{}", nops.generate_range::<u8>(0, 255));

    // Brincando com o rng
    let mut rng = WyRand::new();
    // O tipo não é obrigatório no loop. Se não for definido ele será i32. Usa "=" no ultimo elemento se quiser incluir ele.
    for x in 1..=10_u8 {
        // Sorteia o número sem incluir o maior, no caso de 0, 10 ele sorteará de 0 à 9
        let y: u8 = rng.generate_range(0, 10 + 1);
        println!("{}: {}", x, y)
    }
    // We can also interact with iterators and slices:
    let mut arrows_iter: Vec<char> = "➡⬈⬆⬉⬅⬋⬇⬊".chars().collect();
    rng.shuffle(&mut arrows_iter);
    println!("Lets go in this direction: {}", arrows_iter[0]);

    let mut jogador: Personagem = Personagem {
        nome: String::from(TESTE.lines().nth(1).unwrap()),
        vida_total: 20,
        vida_atual: 20,
        ataque: 1,
        defesa: 1,
        experiencia: 0,
    };

    save.equipamento = escolher_equipamento();
    if let Some(equip) = save.equipamento {
        jogador.ataque += equip.get_equipamento().ataque;
        jogador.defesa += equip.get_equipamento().defesa;
    }

    println!("Jogador: {}", jogador.nome);

    println!("-----LOCAIS-----");
    for lugar in LUGARES.iter() {
        mostrar_dados(lugar.get_lugar());
    }

    println!("-----INIMIGOS-----");
    for inimigo in INIMIGOS.iter() {
        mostrar_dados(inimigo.get_inimigo());
    }

    println!("Ataque critico: {}\n\n", MULTIPLICADOR_CRITICO);

    println!("-----EQUIPAMENTOS-----");
    for equipamento in EQUIPAMENTOS.iter() {
        mostrar_dados(equipamento.get_equipamento());
    }

    println!("-----ITENS-----");
    for item in ITENS.iter() {
        mostrar_dados(item.get_item());
    }

    println!("{:b}", save.flags);
    println!("{:b}", des_criptografar(&save.flags, &save.chave));
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
    println!("{:b}", des_criptografar(&save.flags, &save.chave));

    save.set_flag(Flags::EnciclopediaEquipamentos);

    println!("\n\n");
    println!("{:b}", des_criptografar(&save.flags, &save.chave));
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

    println!("\n\n");

    println!("Chave: {}", save.chave);

    println!("{}", save.jogador.nome);
    println!("nanosegundos: {:?}", agora.elapsed().as_nanos());
}

fn escolher_equipamento() -> Option<Equipamentos> {
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
