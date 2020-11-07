use crate::dados::equipamentos::Equipamentos;
use crate::dados::inimigos::Inimigos;
use crate::structs::personagem::Personagem;

pub fn match_equipamento(id_equipamento: usize) -> Option<Equipamentos> {
    match id_equipamento {
        0 => Some(Equipamentos::Espada),
        1 => Some(Equipamentos::Escudo),
        2 => Some(Equipamentos::Bastao),
        _ => None,
    }
}

pub fn escolher_inimigo(inimigo: &Inimigos) -> Personagem {
    let mut personagem = Personagem::default();
    let inimigo = inimigo.get_inimigo();
    personagem.nome = inimigo.nome.to_string();
    personagem.ataque = inimigo.ataque;
    personagem.defesa = inimigo.defesa;
    personagem.vida_total = inimigo.vida;
    personagem.vida_atual = inimigo.vida;
    personagem.experiencia = inimigo.experiencia;
    personagem
}
