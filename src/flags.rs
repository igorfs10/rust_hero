pub enum FlagName {
    UpgradePocao,
    UpgradeAtaque,
}

pub struct Flags {
    flags: [bool; 2],
}

pub trait FlagsMethods {
    fn check_flag(&self, flag_name: FlagName) -> bool;
    fn set_flag(&mut self, flag_name: FlagName);
    fn clear_flag(&mut self, flag_name: FlagName);
}

impl FlagsMethods for Flags {
    fn check_flag(&self, flag_name: FlagName) -> bool {
        return self.flags[flag_name as usize];
    }

    fn set_flag(&mut self, flag_name: FlagName) {
        self.flags[flag_name as usize] = true;
    }

    fn clear_flag(&mut self, flag_name: FlagName) {
        self.flags[flag_name as usize] = false;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags { flags: [false; 2] }
    }
}
