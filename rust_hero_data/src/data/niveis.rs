//! Níveis - Dados e funções relacionados à estrutura de níveis.

const NIVEIS: &[u16] = &[0, 15, 40, 90, 170, 300, 480];

pub struct Nivel;

impl Nivel {
    pub const fn pegar_nivel(experiencia: &u16) -> u8 {
        let mut nivel = 0;
        loop {
            if *experiencia < NIVEIS[nivel] {
                break nivel as u8;
            }
            nivel += 1;
        }
    }
}
