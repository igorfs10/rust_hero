#[derive(Copy, Clone)]
pub enum Flags {
    UpgradePocao = 0b1,
    UpgradeAtaque = 0b10,
    UpgradeDefesa = 0b100,
    UpgradeExperiencia = 0b1_000,
    EnciclopediaEquipamentos = 0b10_000,
    EnciclopediaItens = 0b100_000,
    EnciclopediaLugares = 0b1_000_000,
    EnciclopediaInimigos = 0b10_000_000,
    SaveEditor = 0b100_000_000,
}
