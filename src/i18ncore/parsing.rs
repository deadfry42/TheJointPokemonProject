use crate::{
    GAME_LOCALES,
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
            GAME_LOCALES.lock().unwrap().add_locale(*locale);
        }
    }
}

pub fn get_localisation() -> Locale {
    *GAME_LOCALES
        .lock()
        .unwrap()
        .get_selected_locale()
        .unwrap_or(&en_GB::LOCALISATION)
}
