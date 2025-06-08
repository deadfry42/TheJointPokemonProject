use std::{
    collections::HashMap,
    fmt::{Error, Result},
};

use crate::{i18ncore::localisation::Locale, utils::logger::Logger};

pub struct LoadedLocales {
    locales: HashMap<&'static str, Locale>,
    selected: Option<&'static str>,
}

impl LoadedLocales {
    pub fn new() -> LoadedLocales {
        LoadedLocales {
            locales: HashMap::new(),
            selected: None,
        }
    }

    pub fn get_locale(&self, name: &'static str) -> Option<&Locale> {
        self.locales.get(name)
    }

    pub fn get_selected_locale(&self) -> Option<&Locale> {
        if let Some(selected) = self.selected {
            self.locales.get(selected)
        } else {
            None
        }
    }

    pub fn set_selected_locale(&mut self, name: &'static str) {
        if let Some(_) = self.get_locale(name) {
            self.selected = Some(name);
        }
    }

    pub fn add_locale(&mut self, locale: Locale) {
        if self.locales.contains_key(locale.code_name.value) {
            Logger::warn(format!("Adding locale that already exists!"));
        }
        self.locales.insert(locale.code_name.value, locale);
    }
}
