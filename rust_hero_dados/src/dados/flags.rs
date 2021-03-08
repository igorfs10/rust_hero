#[derive(Copy, Clone)]
pub enum Flags {
    UpgradePocao = 0b1,
    UpgradeAtaque = 0b10,
    UpgradeDefesa = 0b100,
    UpgradeExperiencia = 0b1000,
    EnciclopediaEquipamentos = 0b1_0000,
    EnciclopediaItens = 0b10_0000,
    EnciclopediaLugares = 0b100_0000,
    EnciclopediaInimigos = 0b1000_0000,
    SaveEditor = 0b1_0000_0000,
}
