use std::{ops::Deref, str::Split};

use crate::{
    I18NCore::{
        localisation::{I18NData, Localisation, SectionData},
        parsing::get_localisation,
    },
    Utils::logger::Logger,
};

pub struct TranslationKey {
    pub path: Vec<&'static str>,
}

impl TranslationKey {
    pub fn new(path: &'static str) -> TranslationKey {
        let splits: Split<'_, &'static str> = path.split("/");

        TranslationKey {
            path: splits.collect::<Vec<&'static str>>(),
        }
    }

    pub fn convert_to_string(&self) -> &'static str {
        // pub fn i18n_value(
        //     key: &TranslationKey,
        //     section: Box<&dyn I18NData>,
        // ) -> Option<&'static str> {
        //     let last_key: Option<&&'static str> = key.path.get(key.path.len() - 1);

        //     if last_key.is_none() {
        //         // somethings probably fucked up, lol
        //         Logger::warn(format!("fucked up @ {}", key.path.join("/")));
        //         return None;
        //     }

        //     None
        // }

        // pub fn section(key: &TranslationKey, index: usize) -> Option<&'static str> {
        //     let locale: &Localisation = get_localisation();

        //     let index_predicate: Option<&&str> = key.path.get(index);

        //     if index_predicate.is_none() {
        //         return None;
        //     }

        //     let section_obj: Box<&dyn SectionData> = locale.index(index_predicate.unwrap());

        //     let i18n_potential = section_obj
        //         .as_ref()
        //         .as_any()
        //         .downcast_ref::<Box<dyn I18NData>>();

        //     if i18n_potential.is_some() {
        //         i18n_value(key, Box::new(i18n_potential.unwrap().deref()))
        //     } else {
        //         // TODO I FORGOT INDEXING
        //         section(key, index + 1) // recursive
        //     }
        // }

        // pub fn intern(key: &TranslationKey) -> Option<&'static str> {
        //     if key.path.len() < 1 {
        //         return None;
        //     } else if key.path.len() == 1 {
        //         let locale: &Localisation = get_localisation();

        //     }

        //     if key.path.get(0).is_none() {
        //         return None;
        //     }

        //     println!("Test");

        //     section(key, 1)
        // }

        // intern(self).unwrap_or("???")
        //
        "???"
    }
}
