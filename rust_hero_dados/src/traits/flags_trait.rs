use crate::dados::flags::Flags;

pub trait FlagsTrait {
    fn check_flag(&self, flag_name: Flags) -> bool;
    fn set_flag(&mut self, flag_name: Flags);
    fn clear_flag(&mut self, flag_name: Flags);
}
