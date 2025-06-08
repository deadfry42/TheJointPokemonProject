use crate::i18ncore::{langs, langs::json_generate::parse_json_files, localisation::*};
use std::io::Result;
use std::sync::{LazyLock, Mutex};

pub struct AvailableLocales {
    pub locales: Vec<Locale>,
    pub current_locale_index: usize,
}

impl Default for AvailableLocales {
    fn default() -> Self {
        Self::new()
    }
}

impl AvailableLocales {
    pub const fn new() -> AvailableLocales {
        AvailableLocales {
            locales: vec![],
            current_locale_index: 0,
        }
    }

    pub fn add_locale(&mut self, locale: Locale) {
        self.locales.push(locale)
    }

    pub fn set_active_locale_index(&mut self, i: usize) {
        self.current_locale_index = i;
    }

    pub fn get_active_locale(&self) -> Option<&Locale> {
        self.locales.get(self.current_locale_index)
    }
}

pub static AVAILABLE_LOCALES: LazyLock<Mutex<AvailableLocales>> =
    LazyLock::new(|| Mutex::new(AvailableLocales::new()));

pub fn load_localisation() {
    let locales: Result<Vec<Locale>> = parse_json_files();
    if locales.is_err() {
        AVAILABLE_LOCALES
            .lock()
            .unwrap()
            .add_locale(langs::en_GB::LOCALISATION);
    } else {
        for locale in locales.unwrap() {
            AVAILABLE_LOCALES.lock().unwrap().add_locale(locale);
        }
    }
}

pub fn get_localisation() -> &'static Locale {
    &langs::en_GB::LOCALISATION
}
