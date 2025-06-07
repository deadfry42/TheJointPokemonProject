use crate::i18n::{
    langs::json_generate::parse_json_files,
    localisation::{AvailableLocales, Localisation},
};
use std::io::Result;
use std::sync::{LazyLock, Mutex};

pub mod langs;
pub mod localisation;
pub mod sections;

pub static AVAILABLE_LOCALES: LazyLock<Mutex<AvailableLocales>> =
    LazyLock::new(|| Mutex::new(AvailableLocales::new()));

pub fn load_localisation() {
    let locales: Result<Vec<Localisation>> = parse_json_files();
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

    let mut count: usize = 0;

    for locale in AVAILABLE_LOCALES.lock().unwrap().locales.iter() {
        println!("{}", locale.name);

        AVAILABLE_LOCALES
            .lock()
            .unwrap()
            .set_active_locale_index(count);
        count += 1;
    }
}

pub fn get_localisation() -> &'static localisation::Localisation {
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

    &langs::en_GB::LOCALISATION
}
