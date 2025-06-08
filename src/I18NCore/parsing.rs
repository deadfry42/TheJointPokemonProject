use crate::I18NCore::{langs, langs::json_generate::parse_json_files, localisation::*};
use std::io::Result;
use std::sync::{LazyLock, Mutex};

pub struct AvailableLocales {
    pub locales: Vec<Locale>,
    pub current_locale_index: usize,
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

    // let mut count: usize = 0;

    // for locale in AVAILABLE_LOCALES.lock().unwrap().locales.iter() {
    //     println!("{}", locale.name);

    //     AVAILABLE_LOCALES
    //         .lock()
    //         .unwrap()
    //         .set_active_locale_index(count);
    //     count += 1;
    // }
}

pub fn get_localisation() -> &'static Locale {
    // TODO: Support changing localisation
    // langs::en_GB::LOCALISATION // base localisation
    //
    // let current: &mut Option<&Localisation> = Box::leak(Box::new(
    //     AVAILABLE_LOCALES.lock().unwrap().get_active_locale(),
    // ));
    // if current.is_none() {
    //     &langs::en_GB::LOCALISATION // return base localisation
    // } else {
    //     current.unwrap()
    // }

    // let boxed = Box::new(current.unwrap());
    // let a: &'static Localisation = Box::leak(boxed);
    // a
    //

    // let val = langs::en_GB::LOCALISATION["pokemon"]["ivysaur"]["name"];

    &langs::en_GB::LOCALISATION
}
