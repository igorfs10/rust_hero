//! Flags - Flag data.

#[derive(Clone)]
/// The flags used to determine upgrades
pub enum Flags {
    /// The health item upgrade
    HeathPotionUpgrade = 0b1,
    /// The attack item upgrade
    AttackPotionUpgrade = 0b10,
    /// The defense item upgrade
    DefensePotionUpgrade = 0b100,
    /// The experience item upgrade
    ExperiencePotionUpgrade = 0b1000,
    // EnciclopediaEquipamentos = 0b1_0000,
    // EnciclopediaItens = 0b10_0000,
    // EnciclopediaLugares = 0b100_0000,
    // EnciclopediaInimigos = 0b1000_0000,
    // SaveEditor = 0b1_0000_0000,
}
