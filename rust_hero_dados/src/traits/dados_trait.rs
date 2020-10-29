// Implementar quando lançarem traits constantes
pub trait Dados {
    fn get_id(self) -> usize;
    fn get_nome(&self) -> &str;
}
