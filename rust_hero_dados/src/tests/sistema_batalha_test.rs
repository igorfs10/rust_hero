#[cfg(test)]
pub mod tests {
    use crate::structs::personagem::Personagem;
    use crate::{sistema_batalha::atacar, tests::struct_test::tests};
    use tests::TestStruct;

    #[test]
    pub fn ataque_nao_critico() {
        let atacante = Personagem::default();
        let mut defensor = Personagem::default();
        TestStruct::novo()
            .espera((false, 1, false))
            .parametriza((&atacante, &mut defensor, &1))
            .funcao(atacar)
            .testar();
    }

    #[test]
    pub fn ataque_critico() {
        let atacante = Personagem::default();
        let mut defensor = Personagem::default();
        TestStruct::novo()
            .espera((true, 2, false))
            .parametriza((&atacante, &mut defensor, &0))
            .funcao(atacar)
            .testar();
    }
}

// #[test]
// pub fn ataque_nao_critico() {
//     let atacante = Personagem::default();
//     let mut defensor = Personagem::default();

//     assert_eq!(false, atacar(&atacante, &mut defensor, &1));
// }

// #[test]
// fn ataque_critico() {
//     let atacante = Personagem::default();
//     let mut defensor = Personagem::default();

//     assert_eq!(true, atacar(&atacante, &mut defensor, &0));
// }

// #[test]
// fn ataque_igual_defesa() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &1);
//     assert_eq!(9, defensor.vida_atual);
// }

// #[test]
// fn ataque_igual_defesa_critico() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &0);
//     assert_eq!(8, defensor.vida_atual);
// }

// #[test]
// fn ataque_menor_defesa() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 3,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &1);
//     assert_eq!(9, defensor.vida_atual);
// }

// #[test]
// fn ataque_menor_defesa_critico() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 3,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &0);
//     assert_eq!(8, defensor.vida_atual);
// }

// #[test]
// fn ataque_maior_defesa() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 3,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &1);
//     assert_eq!(8, defensor.vida_atual);
// }

// #[test]
// fn ataque_maior_defesa_critico() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 3,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &0);
//     assert_eq!(6, defensor.vida_atual);
// }

// #[test]
// fn dano_maior_vida() {
//     let atacante = Personagem {
//         nome: "".to_string(),
//         ataque: 6,
//         defesa: 1,
//         vida_atual: 10,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let mut defensor = Personagem {
//         nome: "".to_string(),
//         ataque: 1,
//         defesa: 1,
//         vida_atual: 5,
//         vida_total: 10,
//         experiencia: 0,
//     };

//     let _ = atacar(&atacante, &mut defensor, &0);
//     assert_eq!(0, defensor.vida_atual);
// }
