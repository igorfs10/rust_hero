pub mod consts;
pub mod dados;
pub mod erros;
pub mod save;
pub mod sistema_batalha;
pub mod structs;
pub mod traits;

#[cfg(test)]
mod test {
    use crate::sistema_batalha::atacar;

    #[test]
    fn ataque_igual_defesa() {
        let ataque = 1;
        let defesa = 1;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &1);
        assert_eq!(9, vida);
    }

    #[test]
    fn ataque_igual_defesa_critico() {
        let ataque = 1;
        let defesa = 1;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &0);
        assert_eq!(8, vida);
    }

    #[test]
    fn ataque_menor_defesa(){
        let ataque = 1;
        let defesa = 3;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &1);
        assert_eq!(9, vida);
    }

    #[test]
    fn ataque_menor_defesa_critico(){
        let ataque = 1;
        let defesa = 3;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &0);
        assert_eq!(8, vida);
    }

    #[test]
    fn ataque_maior_defesa(){
        let ataque = 3;
        let defesa = 1;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &1);
        assert_eq!(8, vida);
    }

    #[test]
    fn ataque_maior_defesa_critico(){
        let ataque = 3;
        let defesa = 1;
        let mut vida = 10;
        atacar(&ataque, &defesa, &mut vida, &0);
        assert_eq!(6, vida);
    }
}
