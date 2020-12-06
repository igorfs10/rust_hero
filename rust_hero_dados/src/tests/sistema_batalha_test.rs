#[cfg(test)]
pub mod tests {
    use crate::sistema_batalha::atacar;
    use crate::structs::personagem::Personagem;

    #[must_use]
    struct Atacar {
        atacante: Personagem,
        defensor: Personagem,
        seed: u64,
        esperado: (bool, u8, bool),
    }

    impl Atacar {
        fn novo() -> Self {
            let personagem_padrao = Personagem::default();
            let seed: u64 = 0;
            let esperado = (false, 0, false);
            Self {
                atacante: personagem_padrao.clone(),
                defensor: personagem_padrao,
                seed,
                esperado,
            }
        }

        fn definir_ataque(mut self, ataque: u8) -> Self {
            self.atacante.ataque = ataque;
            self
        }

        fn definir_defesa(mut self, defesa: u8) -> Self {
            self.defensor.defesa = defesa;
            self
        }

        fn definir_vida(mut self, vida: u8) -> Self {
            self.defensor.vida_atual = vida;
            self
        }

        fn definir_seed(mut self, seed: u64) -> Self {
            self.seed = seed;
            self
        }

        fn espera(mut self, esperado: (bool, u8, bool)) -> Self {
            self.esperado = esperado;
            self
        }

        fn testar(mut self) {
            assert_eq!(
                self.esperado,
                atacar(&self.atacante, &mut self.defensor, &self.seed)
            );
        }
    }

    #[test]
    fn ataque_igual_defesa() {
        let ataque = 1;
        let defesa = 1;
        let seed = 1;
        let espera = (false, 1, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn ataque_igual_defesa_critico() {
        let ataque = 1;
        let defesa = 1;
        let seed = 0;
        let espera = (true, 2, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn ataque_menor_defesa() {
        let ataque = 1;
        let defesa = 2;
        let seed = 1;
        let espera = (false, 1, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn ataque_menor_defesa_critico() {
        let ataque = 1;
        let defesa = 2;
        let seed = 0;
        let espera = (true, 2, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn ataque_maior_defesa() {
        let ataque = 3;
        let defesa = 1;
        let seed = 1;
        let espera = (false, 2, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn ataque_maior_defesa_critico() {
        let ataque = 3;
        let defesa = 1;
        let seed = 0;
        let espera = (true, 4, false);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn dano_maior_vida() {
        let ataque = 3;
        let defesa = 1;
        let vida = 1;
        let seed = 1;
        let espera = (false, 1, true);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_vida(vida)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }

    #[test]
    fn dano_maior_vida_critico() {
        let ataque = 3;
        let defesa = 1;
        let vida = 1;
        let seed = 0;
        let espera = (true, 1, true);
        Atacar::novo()
            .definir_ataque(ataque)
            .definir_defesa(defesa)
            .definir_vida(vida)
            .definir_seed(seed)
            .espera(espera)
            .testar();
    }
}
