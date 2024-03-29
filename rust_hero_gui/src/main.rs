//Remove tela de linha de comando
//#![windows_subsystem = "windows"]

pub mod data;
mod ui;
pub mod utils;

use std::fs;
use std::str::FromStr;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use fltk::output::Output;
// GUI
use fltk::{app::*, image::*, menu::*, prelude::*, *};
use rust_hero_data::data::base_character::BaseCharacter;
use rust_hero_data::data::locations::BaseLocation;

use crate::data::enums::{Action, Direction};
use crate::utils::file::load_game;
use crate::utils::file::new_game;
use rust_hero_data::{
    data::locations::{Location, Locations},
    data::weapons::{Weapon, Weapons},
    jogo::pick_location_enemy,
    structs::character::Character,
    structs::save::Save,
    utils::random::Seed,
};

pub fn main() {
    let loc_test = BaseLocation::get_location(0);
    println!("{}", loc_test.name);
    let enemy = BaseCharacter::get_character(1);
    println!("{}", enemy.name);

    let time = Instant::now();

    let seed = Seed::generate_seed();
    let save = Rc::new(RefCell::new(Save::new(&seed)));
    let save_clone = save.clone();

    let mut ui = ui::RustHeroUI::make_window();
    let mut character_class = ui.character_class.clone();
    let mut character_class_clone = character_class.clone();
    let character: Character = Character::default();

    let game_time = ui.game_time;

    let (send_action, receive_action) = app::channel::<Action>();
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
                character_class.set_label(Weapon::get_weapon(&save.borrow_mut().weapon).name);
            }
        }
    });

    ui.load_button.set_callback(move |_| {
        load_game(&mut save_clone.borrow_mut());
        character_class_clone.set_label(Weapon::get_weapon(&save_clone.borrow_mut().weapon).name);
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
    // Character image
    // background image
    let hero_image_filename: String = match fs::canonicalize(character.image.as_str()) {
        Ok(image_filename) => image_filename.to_str().unwrap().to_owned(),
        Err(e) => {
            println!("ERROR: {:?}", e);
            String::from("")
        }
    };
    if !hero_image_filename.is_empty() {
        let image = SharedImage::load(hero_image_filename.as_str());
        if image.is_ok() {
            let image = image.ok().unwrap();
            ui.hero.set_image(Some(image));
        }
    }

    // map
    let map = [
        Locations::Forest,
        Locations::Forest,
        Locations::Forest,
        Locations::Forest,
        Locations::Forest,
        Locations::Forest,
        Locations::Town,
        Locations::Cave,
        Locations::Cave,
        Locations::Cave,
        Locations::Cave,
        Locations::Cave,
        Locations::Cave,
        Locations::Town,
        Locations::Desert,
        Locations::Desert,
        Locations::Desert,
        Locations::Desert,
        Locations::Town,
        Locations::Forest,
        Locations::Swamp,
        Locations::Swamp,
        Locations::Forest,
        Locations::Town,
    ];
    // location
    let mut location: Location = Location::get_location(&map[0]);
    ui.location.set_label(location.name);

    // background image
    let bg_image_filename: String = match fs::canonicalize(location.image) {
        Ok(image_filename) => image_filename.to_str().unwrap().to_owned(),
        Err(e) => {
            println!("ERROR: {:?}", e);
            String::from("")
        }
    };
    if !bg_image_filename.is_empty() {
        let bg_image = SharedImage::load(bg_image_filename.as_str());
        if bg_image.is_ok() {
            let image = bg_image.ok().unwrap();
            ui.image_box.set_image(Some(image));
        }
    }
    // change location
    ui.forward
        .emit(send_action, Action::Move(Direction::Forward));
    ui.backward
        .emit(send_action, Action::Move(Direction::Backward));

    let mut current_place: usize = 0;
    while app.wait() {
        // update the current state
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Action::Move(direction) => {
                    ui.enemy.hide();
                    match direction {
                        Direction::Forward => {
                            current_place += 1;
                            if current_place >= map.len() {
                                // restart?
                                current_place = 0;
                            }
                        }
                        Direction::Backward => {
                            if current_place > 0 {
                                current_place -= 1;
                            }
                        }
                    }
                    location = Location::get_location(&map[current_place]);
                    if map[current_place] == Locations::Town {
                        // heal or whatever
                    } else {
                        // Get the enemy
                        let enemy = pick_location_enemy(&location, &Seed::generate_seed());
                        if let Some(enemy) = enemy {
                            // get the image filename
                            let enemy_image_filename: String = match fs::canonicalize(enemy.image) {
                                Ok(enemy_image_filename) => {
                                    enemy_image_filename.to_str().unwrap().to_owned()
                                }
                                Err(e) => {
                                    println!("ERROR: {:?}", e);
                                    String::from("")
                                }
                            };
                            if !enemy_image_filename.is_empty() {
                                let enemy_image = SharedImage::load(enemy_image_filename.as_str());
                                // put the image on the box
                                if enemy_image.is_ok() {
                                    let image = enemy_image.ok().unwrap();
                                    ui.enemy.show();
                                    ui.enemy.set_image(Some(image.to_owned()));
                                }
                            }
                        }
                    }

                    ui.location.set_label(location.name);
                    // background image
                    let bg_image_filename: String = match fs::canonicalize(location.image) {
                        Ok(image_filename) => image_filename.to_str().unwrap().to_owned(),
                        Err(e) => {
                            println!("ERROR: {:?}", e);
                            String::from("")
                        }
                    };
                    if !bg_image_filename.is_empty() {
                        let bg_image = SharedImage::load(bg_image_filename.as_str());
                        if bg_image.is_ok() {
                            let image = bg_image.ok().unwrap();
                            ui.image_box.set_image(Some(image.to_owned()));
                        }
                    }
                }
            }
        }
        ui.win.redraw();
    }
}

fn show_time(mut element: Output, time: Instant) {
    element.set_value(&time.elapsed().as_secs().to_string());
    app::add_timeout(1.0, move || show_time(element.clone(), time));
}

#[cfg(test)]
mod tests {
    use rust_hero_data::{
        data::enemies::{Enemies, Enemy},
        data::locations::{Location, Locations},
        //structs::save::Save,
        //utils::random::Seed,
        //data::weapons::{Weapon, Weapons},
        structs::character::Character,
    };
    #[test]
    // Ok lets do a basic location test
    fn test_locations() {
        // can we make a character
        let _character: Character = Character::default();
        // can we make a loation?
        let location: Location = Location::get_location(&Locations::Forest);
        // get or make an enemy
        let _enemy: Enemy = match location.enemies {
            Some(enemy) => Enemy::get_enemy(&enemy[0]),
            None => Enemy::get_enemy(&Enemies::Rat),
        };
    }
}
