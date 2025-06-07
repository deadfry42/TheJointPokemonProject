use crate::I18NCore::localisation::{I18NData, SectionData, TranslationData};
use std::any::Any;

pub struct ItemTranslationData {
    pub lucky_egg: ItemI18n,
    pub connection_wire: ItemI18n,
}

impl TranslationData for ItemTranslationData {
    fn index(&self, path: &'static str) -> Box<&dyn SectionData> {
        match path {
            "lucky_egg" => Box::new(&self.lucky_egg),
            _ => Box::new(&self.connection_wire),
        }
    }
}
impl SectionData for ItemTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ItemI18n {
    pub name: &'static str,
    pub desc: &'static str,
}

impl I18NData for ItemI18n {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "desc" => &self.desc,
            _ => &self.name,
        }
    }
}
impl SectionData for ItemI18n {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
