use crate::dados::niveis::NIVEIS;
use crate::structs::personagem::Personagem;

pub fn pegar_nivel(personagem: &Personagem) -> u8 {
    let mut nivel = 0;
    loop {
        if personagem.experiencia < NIVEIS[nivel] {
            break nivel as u8;
        }
        nivel += 1;
    }
}
