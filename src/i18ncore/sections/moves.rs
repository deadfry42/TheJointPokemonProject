use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

#[derive(Clone, Copy)]
pub struct MoveLocaleContainer {
    pub tackle: MoveLocale,
    pub growl: MoveLocale,
}

impl DataSection for MoveLocaleContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Container
    }

    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        Some(match path {
            "tackle" => Box::new(&self.tackle),
            _ => Box::new(&self.growl),
        })
    }

    fn run_data_index(&self, _: &'static str) -> Option<&'static str> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct MoveLocale {
    pub name: &'static str,
    pub desc: &'static str,
}

impl DataSection for MoveLocale {
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

pub struct MoveCategoryTranslationData {
    pub physical: &'static str,
    pub special: &'static str,
    pub status: &'static str,
}
