//! Battle System - Module containing the functions for the Battle System

use crate::data::locations::{Location, Locations};
use crate::jogo::*;
use crate::structs::character::Character;

fn _start_battle(enemy: &mut Character, flag: &mut bool) {
    *flag = true;
    define_enemy(
        enemy,
        pick_location_enemy(&Location::get_location(&Locations::Forest), &0).unwrap(),
    );
}
