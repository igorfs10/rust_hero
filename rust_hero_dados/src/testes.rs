#[cfg(test)]
mod testes {
    use crate::dados::lugares::LUGARES;
    use crate::jogo::*;
    use crate::sistema_batalha::atacar;
    use crate::structs::lugar::Lugar;
    use crate::structs::personagem::Personagem;

    #[test]
    fn ataque_nao_critico() {
        let atacante = Personagem::default();
        let mut defensor = Personagem::default();

        assert_eq!(false, atacar(&atacante, &mut defensor, &1));
    }

    #[test]
    fn ataque_critico() {
        let atacante = Personagem::default();
        let mut defensor = Personagem::default();

        assert_eq!(true, atacar(&atacante, &mut defensor, &0));
    }

    #[test]
    fn ataque_igual_defesa() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &1);
        assert_eq!(9, defensor.vida_atual);
    }

    #[test]
    fn ataque_igual_defesa_critico() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &0);
        assert_eq!(8, defensor.vida_atual);
    }

    #[test]
    fn ataque_menor_defesa() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 3,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &1);
        assert_eq!(9, defensor.vida_atual);
    }

    #[test]
    fn ataque_menor_defesa_critico() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 3,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &0);
        assert_eq!(8, defensor.vida_atual);
    }

    #[test]
    fn ataque_maior_defesa() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 3,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &1);
        assert_eq!(8, defensor.vida_atual);
    }

    #[test]
    fn ataque_maior_defesa_critico() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 3,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &0);
        assert_eq!(6, defensor.vida_atual);
    }

    #[test]
    fn dano_maior_vida() {
        let atacante = Personagem {
            nome: "".to_string(),
            ataque: 6,
            defesa: 1,
            vida_atual: 10,
            vida_total: 10,
            experiencia: 0,
        };

        let mut defensor = Personagem {
            nome: "".to_string(),
            ataque: 1,
            defesa: 1,
            vida_atual: 5,
            vida_total: 10,
            experiencia: 0,
        };

        let _ = atacar(&atacante, &mut defensor, &0);
        assert_eq!(0, defensor.vida_atual);
    }

    #[test]
    fn sorteio_inimigo_1() {
        let mut oponente = Personagem::default();
        let lugar: Lugar = LUGARES[1].get_local();
        //1 inimigo 1
        //3 inimigo 2
        //15 inimigo 3
        //19 inimigo 4
        let inimigo = sortear_inimigo_lugar(&lugar, &1).unwrap();
        definir_inimigo(&mut oponente, inimigo);
        println!("nome:{}", oponente.nome);
    }
}
