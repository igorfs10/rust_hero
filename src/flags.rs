pub enum FLAGS {
    UpgradePocao,
    UpgradeAtaque
}

pub fn set_flag(flags: &mut [bool; 5], flag_name: FLAGS){
    flags[flag_name as usize] = true;
}

pub fn clear_flag(flags: &mut [bool; 5], flag_name: FLAGS){
    flags[flag_name as usize] = false;
}