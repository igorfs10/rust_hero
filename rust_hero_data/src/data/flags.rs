//! Flags - Flag data.

#[derive(Clone)]
pub enum Flags {
    HeathPotionUpgrade = 0b1,
    AttackPotionUpgrade = 0b10,
    DefensePotionUpgrade = 0b100,
    ExperiencePotionUpgrade = 0b1000,
    // EnciclopediaEquipamentos = 0b1_0000,
    // EnciclopediaItens = 0b10_0000,
    // EnciclopediaLugares = 0b100_0000,
    // EnciclopediaInimigos = 0b1000_0000,
    // SaveEditor = 0b1_0000_0000,
}
