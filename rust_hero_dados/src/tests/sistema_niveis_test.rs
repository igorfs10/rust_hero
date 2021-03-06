//! Teste das funções do sistema de níveis.

#[cfg(test)]
pub mod tests {
    use crate::dados::niveis::Nivel;
    use crate::structs::personagem::Personagem;

    #[test]
    pub fn nivel_2() {
        let mut personagem = Personagem::default();
        personagem.experiencia = 39;
        let nivel = 2;
        assert_eq!(Nivel::pegar_nivel(&personagem.experiencia), nivel);
    }

    #[test]
    pub fn nivel_1() {
        let mut personagem = Personagem::default();
        personagem.experiencia = 0;
        let nivel = 1;
        assert_eq!(Nivel::pegar_nivel(&personagem.experiencia), nivel);
    }
}
