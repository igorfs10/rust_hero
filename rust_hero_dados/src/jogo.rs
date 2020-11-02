use crate::dados::equipamentos::Equipamentos;

pub fn match_equipamento(id_equipamento: usize) -> Equipamentos {
    match id_equipamento {
        1 => Equipamentos::Espada,
        2 => Equipamentos::Escudo,
        3 => Equipamentos::Bastao,
        _ => Equipamentos::Nenhum,
    }
}
