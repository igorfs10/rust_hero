//Remove tela de linha de comando
//#![windows_subsystem = "windows"]

mod ui;
pub mod utils;

use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

// GUI
use fltk::{app::*, menu::*, prelude::WidgetExt};

use rust_hero_data::{
    data::weapons::{Weapon, Weapons},
    structs::save::Save,
    utils::random::Seed,
};
use utils::file::load_game;

use crate::utils::file::new_game;

pub fn main() {
    let seed = Seed::generate_seed();
    let save = Rc::new(RefCell::new(Save::new(&seed)));
    let save_clone = save.clone();
    let app = App::default();
    let mut ui = ui::RustHeroUI::make_window();
    let mut character_class = ui.character_class.clone();
    let mut character_class_clone = character_class.clone();

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

    app.run().unwrap();
}
