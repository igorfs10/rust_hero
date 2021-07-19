//! Teste das funções de jogo.

#[cfg(test)]
pub mod tests {
    use crate::data::lugares::{Lugar, LUGARES};
    use crate::jogo::*;
    use crate::structs::personagem::Personagem;

    struct SortearInimigo {
        lugar: Option<Lugar>,
        seed: u64,
        esperado: Option<Personagem>,
    }

    impl SortearInimigo {
        fn novo() -> Self {
            SortearInimigo {
                lugar: None,
                seed: 0,
                esperado: None,
            }
        }

        fn definir_lugar(mut self, lugar: Lugar) -> Self {
            self.lugar = Some(lugar);
            self
        }

        fn definir_seed(mut self, seed: u64) -> Self {
            self.seed = seed;
            self
        }

        fn espera(mut self, esperado: Personagem) -> Self {
            self.esperado = Some(esperado);
            self
        }

        fn testar(self) {
            let mut oponente = Personagem::default();
            let inimigo_possivel = sortear_inimigo_lugar(&self.lugar.unwrap(), &0);
            if let Some(inimigo) = inimigo_possivel {
                definir_inimigo(&mut oponente, inimigo);
            }
            assert_eq!(self.esperado.unwrap(), oponente);
        }
    }

    #[test]
    fn sorteio_inimigo_1() {
        //0 inimigo 1
        //2 inimigo 2
        //1 inimigo 3
        //4 inimigo 4
        SortearInimigo::novo()
            .definir_lugar(Lugar::get_lugar(&LUGARES[0]))
            .definir_seed(0)
            .espera(Personagem::default())
            .testar();
    }
}
