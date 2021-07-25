//! Teste das funções do sistema de níveis.

#[cfg(test)]
pub mod tests {
    use crate::data::levels::Level;
    use crate::structs::character::Character;

    #[test]
    pub fn nivel_2() {
        let mut personagem = Character::default();
        personagem.experience = 39;
        let nivel = 2;
        assert_eq!(Level::get_level(&personagem.experience), nivel);
    }

    #[test]
    pub fn nivel_1() {
        let mut personagem = Character::default();
        personagem.experience = 0;
        let nivel = 1;
        assert_eq!(Level::get_level(&personagem.experience), nivel);
    }
}
