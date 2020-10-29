use console::Term;

pub fn limpar_terminal() {
    let term = Term::stdout();
    let _ = term.clear_screen();
}
