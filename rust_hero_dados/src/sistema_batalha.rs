use crate::dados::{flags::*, lugares::Lugares};
use crate::jogo::*;
use crate::structs::{flag::Flag, personagem::Personagem};
use crate::traits::flags_trait::FlagsTrait;

fn _comecar_batalha(inimigo: &mut Personagem, flags: &mut Flag) {
    flags.set_flag(Flags::EmBatalha);
    definir_inimigo(
        inimigo,
        sortear_inimigo_lugar(&Lugares::Floresta.get_lugar(), &0).unwrap(),
    );
}
