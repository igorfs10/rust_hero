mod structs;

use structs::personagem::Personagem;

fn main(){
    const PERSONAGENS: [Personagem; 2] = [Personagem{id: 1, nome: "Guerreiro", vida: 100, ataque: 2, defesa: 1},
        Personagem{id: 2, nome: "Arqueiro", vida: 100, ataque: 1, defesa: 2}];
    println! ("Id: {}", PERSONAGENS[0].id);
}