use crate::dados::equipamentos::Equipamentos;
use crate::dados::inimigos::Inimigos;
use crate::structs::personagem::Personagem;

pub fn match_equipamento(id_equipamento: usize) -> Equipamentos {
    match id_equipamento {
        1 => Equipamentos::Espada,
        2 => Equipamentos::Escudo,
        3 => Equipamentos::Bastao,
        _ => Equipamentos::Nenhum,
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
