use crate::dados::flags::Flags;
use crate::traits::flags_trait::FlagsTrait;

pub struct Flag {
    flags: [bool; 3],
}

impl FlagsTrait for Flag {
    fn check_flag(&self, flag_name: Flags) -> bool {
        self.flags[flag_name as usize]
    }

    fn set_flag(&mut self, flag_name: Flags) {
        self.flags[flag_name as usize] = true;
    }

    fn clear_flag(&mut self, flag_name: Flags) {
        self.flags[flag_name as usize] = false;
    }
}

impl Default for Flag {
    fn default() -> Self {
        Flag { flags: [false; 3] }
    }
}
