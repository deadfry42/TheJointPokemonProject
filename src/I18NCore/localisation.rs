use crate::I18NCore::sections::{
    abilities::AbilityTranslationData,
    enums::{GenderTranslationData, OtherLanguageData, StatTranslationData, TypesTranslationData},
    items::ItemTranslationData,
    moves::MoveTranslationData,
    natures::NatureTranslationData,
    pokemon::PokemonTranslationData,
};
use std::ops::Index;

pub struct Localisation {
    pub code_name: &'static str,
    pub name: &'static str,

    pub moves: MoveTranslationData,
    pub pokemon: PokemonTranslationData,
    pub nature: NatureTranslationData,
    pub abilities: AbilityTranslationData,
    pub gender: GenderTranslationData,
    pub items: ItemTranslationData,
    pub stats: StatTranslationData,
    pub types: TypesTranslationData,
    pub other_langs: OtherLanguageData,
}

// impl Index<&'static str> for Localisation {
//     type Output = Box<dyn TranslationData>;

//     fn index(&self, path: &'static str) -> &Self::Output {
//         match path {
//             "moves" => &Box::new(self.moves),
//             _ => &Box::new(self.pokemon),
//         }
//     }
// }

pub trait TranslationData {}

pub struct AvailableLocales {
    pub locales: Vec<Localisation>,
    pub current_locale_index: usize,
}

impl AvailableLocales {
    pub const fn new() -> AvailableLocales {
        AvailableLocales {
            locales: vec![],
            current_locale_index: 0,
        }
    }

    pub fn add_locale(&mut self, locale: Localisation) {
        self.locales.push(locale)
    }

    pub fn set_active_locale_index(&mut self, i: usize) {
        self.current_locale_index = i;
    }

    pub fn get_active_locale(&self) -> Option<&Localisation> {
        self.locales.get(self.current_locale_index)
    }
}
