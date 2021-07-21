//! Levels - Data and functions related to levels.

const LEVELS: &[u16] = &[0, 15, 40, 90, 170, 300, 480];

pub struct Level;

impl Level {
    pub const fn get_level(experience: &u16) -> u8 {
        let mut level = 0;
        loop {
            if *experience < LEVELS[level] {
                break level as u8;
            }
            level += 1;
        }
    }
}
