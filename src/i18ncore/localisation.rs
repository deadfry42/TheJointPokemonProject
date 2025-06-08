use std::any::Any;

use crate::i18ncore::sections::{
    abilities::AbilityLocaleContainer,
    enums::{GenderLocale, OtherLanguageLocale, StatLocale, TypesLocale},
    items::ItemLocaleContainer,
    moves::MoveLocaleContainer,
    natures::NatureLocale,
    pokemon::PokemonLocaleContainer,
};

pub struct Locale {
    pub code_name: SingleValuedData,
    pub name: SingleValuedData,

    pub moves: MoveLocaleContainer,
    pub pokemon: PokemonLocaleContainer,
    pub abilities: AbilityLocaleContainer,
    pub items: ItemLocaleContainer,
    pub other_langs: OtherLanguageLocale,
    pub nature: NatureLocale,
    pub gender: GenderLocale,
    pub stats: StatLocale,
    pub types: TypesLocale,
}

#[allow(dead_code)]
impl Locale {
    pub fn index(&self, path: &'static str) -> Box<&dyn DataSection> {
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

pub struct SingleValuedData {
    pub value: &'static str,
}

impl SingleValuedData {
    pub fn new(value: &'static str) -> SingleValuedData {
        SingleValuedData { value }
    }
}
#[allow(unused_variables)]
impl DataSection for SingleValuedData {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_section_type(&self) -> SectionType {
        SectionType::SingleValued
    }
    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(self.value)
    }
    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}

pub trait DataSection {
    fn as_any(&self) -> &dyn Any;
    fn get_section_type(&self) -> SectionType;
    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>>;
    fn run_data_index(&self, path: &'static str) -> Option<&'static str>;
}

#[derive(PartialEq, Eq)]
pub enum SectionType {
    SingleValued,
    Container,
    Data,
}
