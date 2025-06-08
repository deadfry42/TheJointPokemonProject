use crate::{
    get_game_data,
    i18ncore::{
        langs::{en_GB, json_generate::parse_json_files},
        localisation::*,
    },
};
use std::io::Result;

pub fn load_localisation() {
    let locales: Result<Vec<Locale>> = parse_json_files();
    if locales.is_ok() {
        for locale in locales.unwrap().iter() {
            get_game_data()
                .unwrap()
                .loaded_locales
                .add_locale(locale.clone());
        }
    }
}

pub fn get_localisation() -> Locale {
    *get_game_data()
        .unwrap()
        .loaded_locales
        .get_selected_locale()
        .unwrap_or(&en_GB::LOCALISATION)
}
