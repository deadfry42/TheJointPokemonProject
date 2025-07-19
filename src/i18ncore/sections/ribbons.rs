use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

#[derive(Clone, Copy)]
pub struct RibbonLocaleContainer {
    pub champion: RibbonLocale,
}

impl DataSection for RibbonLocaleContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Container
    }

    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        Some(match path {
            _ => Box::new(&self.champion),
        })
    }

    fn run_data_index(&self, _: &'static str) -> Option<&'static str> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct RibbonLocale {
    pub name: &'static str,
    pub desc: &'static str,
    pub title: &'static str,
}

impl DataSection for RibbonLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "desc" => self.desc,
            "title" => self.title,
            _ => self.name,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
