use crate::{
    GameData,
    i18ncore::{
        langs::{self, json_generate::parse_json_files},
        localisation::*,
    },
};
use std::io::Result;

pub fn load_localisation(game: &mut GameData) {
    let locales: Result<Vec<Locale>> = parse_json_files();
    if locales.is_ok() {
        for locale in locales.unwrap().iter() {
            game.loaded_locales.add_locale(locale.clone());
        }
    }
}

pub fn get_localisation() -> &'static Locale {
    &langs::en_GB::LOCALISATION
}
