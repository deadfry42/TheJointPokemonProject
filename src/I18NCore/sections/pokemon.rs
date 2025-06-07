use crate::I18NCore::localisation::{I18NData, SectionData, TranslationData};
use std::any::Any;

pub struct PokemonTranslationData {
    pub bulbasaur: PokemonI18n,
    pub ivysaur: PokemonI18n,
    pub venusaur: PokemonI18n,
    pub wooper: PokemonI18n,
}

impl TranslationData for PokemonTranslationData {
    fn index(&self, path: &'static str) -> Box<&dyn SectionData> {
        match path {
            "bulbasaur" => Box::new(&self.bulbasaur),
            "ivysaur" => Box::new(&self.ivysaur),
            "venusaur" => Box::new(&self.venusaur),
            _ => Box::new(&self.wooper),
        }
    }
}
impl SectionData for PokemonTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct PokemonI18n {
    pub name: &'static str,
    pub dex: &'static str,
    pub species: &'static str,
}

impl I18NData for PokemonI18n {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "species" => &self.species,
            "dex" => &self.dex,
            _ => &self.name,
        }
    }
}
impl SectionData for PokemonI18n {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
