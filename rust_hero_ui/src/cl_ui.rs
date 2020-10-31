use console::Term;
use rust_hero_dados::traits::dados_trait::DadosTrait;

pub fn limpar_terminal() {
    let term = Term::stdout();
    let _ = term.clear_screen();
}

pub fn mostrar_dados(dados: impl DadosTrait) {
    println!("{}", dados.get_dados());
    println!();
}
