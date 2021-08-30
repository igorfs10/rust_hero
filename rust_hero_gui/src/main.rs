//Remove tela de linha de comando
//#![windows_subsystem = "windows"]

mod ui;
pub mod utils;

use std::str::FromStr;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use fltk::output::Output;
// GUI
use fltk::{app::*, menu::*, prelude::*, *};

use rust_hero_data::{
    data::weapons::{Weapon, Weapons},
    structs::character::Character,
    structs::save::Save,
    utils::random::Seed,
};
use utils::file::load_game;

use crate::utils::file::new_game;

pub fn main() {
    let time = Instant::now();

    let seed = Seed::generate_seed();
    let save = Rc::new(RefCell::new(Save::new(&seed)));
    let save_clone = save.clone();

    let mut ui = ui::RustHeroUI::make_window();
    let mut character_class = ui.character_class.clone();
    let mut character_class_clone = character_class.clone();
    let character:Character = Character::default();

    let game_time = ui.game_time;

    let app = App::default();
    app::add_timeout(1.0, move || {
        show_time(game_time.clone(), time);
    });

    ui.new_button.set_label("Test");

    ui.new_button.set_callback(move |this| {
        let options = vec![
            Weapon::get_weapon(&Weapons::None).name,
            Weapon::get_weapon(&Weapons::Sword).name,
            Weapon::get_weapon(&Weapons::Shield).name,
            Weapon::get_weapon(&Weapons::Spear).name,
        ];
        let menu = MenuItem::new(&options);
        match menu.popup(this.x() + this.width(), this.y() + this.height()) {
            None => println!("No value was chosen!"),
            Some(selection) => {
                let selected_weapon = Weapons::from_str(&selection.label().unwrap()).unwrap();
                new_game(&mut save.borrow_mut(), &seed, &selected_weapon);
                println!("{}", Weapon::get_weapon(&save.borrow_mut().weapon).name);
                character_class.set_label(&Weapon::get_weapon(&save.borrow_mut().weapon).name);
            }
        }
    });

    ui.load_button.set_callback(move |_| {
        load_game(&mut save_clone.borrow_mut());
        character_class_clone.set_label(&Weapon::get_weapon(&save_clone.borrow_mut().weapon).name);
    });

    // Character stats
    ui.atk.set_value(character.attack as f64);
    ui.def.set_value(character.defense as f64);
    ui.mp.set_value(character.mana as f64);
    ui.hp.set_value(character.health as f64);
    ui.m_def.set_value(character.mana_defense as f64);
    ui.m_atk.set_value(character.mana_attack as f64);
    ui.xp.set_value(character.experience as f64);
    ui.level.set_value(character.level as f64);
    ui.character_class.set_label(character.name.as_str());

    while app.wait() {
        // update the current state
    }
}

fn show_time(mut element: Output, time: Instant) {
    element.set_value(&time.elapsed().as_secs().to_string());
    app::add_timeout(1.0, move || show_time(element.clone(), time));
}
