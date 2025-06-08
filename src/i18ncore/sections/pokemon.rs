use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

#[derive(Clone, Copy)]
pub struct PokemonLocaleContainer {
    pub bulbasaur: PokemonLocale,
    pub ivysaur: PokemonLocale,
    pub venusaur: PokemonLocale,
    pub wooper: PokemonLocale,
}

impl DataSection for PokemonLocaleContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Container
    }

    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        Some(match path {
            "bulbasaur" => Box::new(&self.bulbasaur),
            "ivysaur" => Box::new(&self.ivysaur),
            "venusaur" => Box::new(&self.venusaur),
            _ => Box::new(&self.wooper),
        })
    }

    fn run_data_index(&self, _: &'static str) -> Option<&'static str> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct PokemonLocale {
    pub name: &'static str,
    pub dex: &'static str,
    pub species: &'static str,
}

impl DataSection for PokemonLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "species" => self.species,
            "dex" => self.dex,
            _ => self.name,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
