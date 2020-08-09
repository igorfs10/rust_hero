pub enum FlagName {
    UpgradePocao,
    UpgradeAtaque
}

pub struct Flags {
    flags: [bool; 2]
}

impl Flags {
    pub fn new() -> Flags {
        return Flags {flags: [false; 2]};
    }

    pub fn check_flag(&self, flag_name: FlagName) -> bool {
        return self.flags[flag_name as usize];
    }

    pub fn set_flag(&mut self, flag_name: FlagName) {
        self.flags[flag_name as usize] = true;
    }

    pub fn clear_flag(&mut self, flag_name: FlagName) {
        self.flags[flag_name as usize] = false;
    }
}