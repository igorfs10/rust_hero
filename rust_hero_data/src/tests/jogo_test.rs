//! Teste das funções de jogo.

#[cfg(test)]
pub mod tests {
    use crate::data::locations::{Location, Locations};
    use crate::jogo::*;
    use crate::structs::character::Character;

    struct SortearInimigo {
        lugar: Option<Location>,
        seed: u64,
        esperado: Option<Character>,
    }

    impl SortearInimigo {
        fn novo() -> Self {
            SortearInimigo {
                lugar: None,
                seed: 0,
                esperado: None,
            }
        }

        fn definir_lugar(mut self, lugar: Location) -> Self {
            self.lugar = Some(lugar);
            self
        }

        fn definir_seed(mut self, seed: u64) -> Self {
            self.seed = seed;
            self
        }

        fn espera(mut self, esperado: Character) -> Self {
            self.esperado = Some(esperado);
            self
        }

        fn testar(self) {
            let mut oponente = Character::default();
            let inimigo_possivel = pick_location_enemy(&self.lugar.unwrap(), &0);
            if let Some(inimigo) = inimigo_possivel {
                define_enemy(&mut oponente, inimigo);
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
            .definir_lugar(Location::get_location(&Locations::Town))
            .definir_seed(0)
            .espera(Character::default())
            .testar();
    }
}
