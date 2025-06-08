use std::any::Any;

use crate::I18NCore::localisation::{DataSection, SectionType};

pub struct ItemLocaleContainer {
    pub lucky_egg: ItemLocale,
    pub connection_wire: ItemLocale,
}

impl DataSection for ItemLocaleContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::I18NCore::localisation::SectionType {
        SectionType::Container
    }

    fn run_container_index(&self, path: &'static str) -> Option<Box<&dyn DataSection>> {
        Some(match path {
            "lucky_egg" => Box::new(&self.lucky_egg),
            _ => Box::new(&self.connection_wire),
        })
    }

    fn run_data_index(&self, _: &'static str) -> Option<&'static str> {
        None
    }
}

pub struct ItemLocale {
    pub name: &'static str,
    pub desc: &'static str,
}

impl DataSection for ItemLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "desc" => &self.desc,
            _ => &self.name,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
