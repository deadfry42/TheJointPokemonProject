use std::collections::HashMap;

use crate::{i18ncore::localisation::Locale, utils::logger::Logger};

pub struct LoadedLocales {
    pub locales: HashMap<&'static str, Locale>,
    pub selected: Option<&'static str>,
}

impl LoadedLocales {
    pub fn new() -> LoadedLocales {
        LoadedLocales {
            locales: HashMap::new(),
            selected: None,
        }
    }

    pub fn add_locale(&mut self, locale: Locale) {
        if self.locales.contains_key(locale.code_name.value) {
            Logger::warn(format!("Adding locale that already exists!"));
        }
        self.locales.insert(locale.code_name.value, locale);
    }

    pub fn get_locale(&self, name: &'static str) -> Option<&Locale> {
        self.locales.get(name)
    }
}
