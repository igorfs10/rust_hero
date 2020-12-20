#[cfg(test)]
pub mod tests {
    use crate::sistema_niveis::pegar_nivel;
    use crate::structs::personagem::Personagem;

    #[test]
    pub fn nivel_2() {
        let mut personagem = Personagem::default();
        personagem.experiencia = 40;
        let nivel = 3;
        assert_eq!(pegar_nivel(&personagem), nivel);
    }

    #[test]
    pub fn nivel_1() {
        let mut personagem = Personagem::default();
        personagem.experiencia = 0;
        let nivel = 1;
        assert_eq!(pegar_nivel(&personagem), nivel);
    }
}
