/// Struct to be used to load the base stat for a character and calculate its stats and given experience
pub struct BaseCharacter{
    pub id: u8,
    pub name: String,
    pub hp: u16,
    pub mp: u16,
    pub atk: u8,
    pub def: u8,
    pub m_atk: u8,
    pub m_def: u8,
    pub exp: u16,               // Base xp used to calculate the received xp after player defeats an enemy
    pub item: Option<u8>,       // Id of item that will be dropped by enemy
    pub img_idle: String,
    pub img_walking: String,
    pub img_attacking: String,
    pub playable: bool,
}