use crate::I18NCore::sections::{
    abilities::AbilityTranslationData,
    enums::{GenderTranslationData, OtherLanguageData, StatTranslationData, TypesTranslationData},
    items::ItemTranslationData,
    moves::MoveTranslationData,
    natures::NatureTranslationData,
    pokemon::PokemonTranslationData,
};
use std::any::Any;

pub struct Localisation {
    pub code_name: SingleValued,
    pub name: SingleValued,

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

#[allow(dead_code)]
impl Localisation {
    pub fn index(&self, path: &'static str) -> Box<&dyn SectionData> {
        match path {
            "code_name" => Box::new(&self.code_name),
            "name" => Box::new(&self.name),
            "moves" => Box::new(&self.moves),
            "nature" => Box::new(&self.nature),
            "abilities" => Box::new(&self.abilities),
            "gender" => Box::new(&self.gender),
            "items" => Box::new(&self.items),
            "stats" => Box::new(&self.stats),
            "types" => Box::new(&self.types),
            "other_langs" => Box::new(&self.other_langs),
            _ => Box::new(&self.pokemon),
        }
    }
}

pub struct SingleValued {
    pub value: &'static str,
}

impl SingleValued {
    pub fn new(value: &'static str) -> SingleValued {
        SingleValued { value: value }
    }
}
#[allow(unused_variables)]
impl I18NData for SingleValued {
    fn index(&self, path: &'static str) -> &'static str {
        &self.value
    }
}
impl SectionData for SingleValued {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait TranslationData {
    fn index(&self, path: &'static str) -> Box<&dyn SectionData>;
}
pub trait I18NData {
    fn index(&self, path: &'static str) -> &'static str;
}
pub trait SectionData {
    fn as_any(&self) -> &dyn Any;
}

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
