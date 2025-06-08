use crate::{
    GAME_DATA,
    i18ncore::{
        langs::{self, json_generate::parse_json_files},
        localisation::*,
    },
};
use std::io::Result;

pub fn load_localisation() {
    let locales: Result<Vec<Locale>> = parse_json_files();
    if locales.is_ok() {
        for locale in locales.unwrap().iter() {
            GAME_DATA
                .lock()
                .unwrap()
                .loaded_locales
                .add_locale(locale.clone());
        }
    }
}

pub fn get_localisation() -> &'static Locale {
    // let loaded_locales = GAME_DATA.lock().unwrap().loaded_locales;

    // loaded_locales.get_selected_locale().unwrap_or() &

    &langs::en_GB::LOCALISATION
}
