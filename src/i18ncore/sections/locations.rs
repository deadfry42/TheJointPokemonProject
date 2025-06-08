use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

#[derive(Clone, Copy)]
pub struct LocationLocale {
    pub mystery_zone: &'static str,
}

impl DataSection for LocationLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            _ => self.mystery_zone,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
