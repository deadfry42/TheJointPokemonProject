use std::any::Any;

use crate::i18ncore::sections::{
    abilities::AbilityLocaleContainer,
    enums::{GenderLocale, OtherLanguageLocale, StatLocale, TypesLocale},
    items::ItemLocaleContainer,
    locations::LocationLocale,
    moves::MoveLocaleContainer,
    natures::NatureLocale,
    pokemon::PokemonLocaleContainer,
    ribbons::RibbonLocaleContainer,
};

#[derive(Clone, Copy)]
pub struct Locale {
    pub code_name: SingleValuedData,
    pub name: SingleValuedData,

    pub moves: MoveLocaleContainer, // "move"
    pub pokemon: PokemonLocaleContainer,
    pub ability: AbilityLocaleContainer,
    pub item: ItemLocaleContainer,
    pub ribbons: RibbonLocaleContainer,
    pub other_langs: OtherLanguageLocale,
    pub location: LocationLocale,
    pub nature: NatureLocale,
    pub gender: GenderLocale,
    pub stat: StatLocale,
    pub types: TypesLocale, // "type"
}

#[allow(dead_code)]
impl Locale {
    pub fn index(&self, path: &'static str) -> Box<&dyn DataSection> {
        match path {
            "code_name" => Box::new(&self.code_name),
            "location" => Box::new(&self.location),
            "name" => Box::new(&self.name),
            "move" => Box::new(&self.moves),
            "nature" => Box::new(&self.nature),
            "ability" => Box::new(&self.ability),
            "gender" => Box::new(&self.gender),
            "item" => Box::new(&self.item),
            "stat" => Box::new(&self.stat),
            "type" => Box::new(&self.types),
            "other_langs" => Box::new(&self.other_langs),
            "ribbons" => Box::new(&self.ribbons),
            _ => Box::new(&self.pokemon),
        }
    }
}

#[derive(Clone, Copy)]
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
