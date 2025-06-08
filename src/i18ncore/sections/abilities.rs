use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

#[derive(Clone, Copy)]
pub struct AbilityLocaleContainer {
    pub damp: AbilityLocale,
    pub water_absorb: AbilityLocale,
    pub unaware: AbilityLocale,
    pub overgrow: AbilityLocale,
    pub chlorophyll: AbilityLocale,
}

impl DataSection for AbilityLocaleContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Container
    }

    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        Some(match path {
            "damp" => Box::new(&self.damp),
            "water_absorb" => Box::new(&self.water_absorb),
            "unaware" => Box::new(&self.unaware),
            "overgrow" => Box::new(&self.overgrow),
            _ => Box::new(&self.chlorophyll),
        })
    }

    fn run_data_index(&self, _: &'static str) -> Option<&'static str> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct AbilityLocale {
    pub name: &'static str,
    pub desc: &'static str,
}

impl DataSection for AbilityLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "desc" => self.desc,
            _ => self.name,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
