use crate::I18NCore::localisation::{I18NData, SectionData, TranslationData};
use std::any::Any;

pub struct AbilityTranslationData {
    pub damp: AbilityI18n,
    pub water_absorb: AbilityI18n,
    pub unaware: AbilityI18n,
    pub overgrow: AbilityI18n,
    pub chlorophyll: AbilityI18n,
}

impl TranslationData for AbilityTranslationData {
    fn index(&self, path: &'static str) -> Box<&dyn SectionData> {
        match path {
            "damp" => Box::new(&self.damp),
            "water_absorb" => Box::new(&self.water_absorb),
            "unaware" => Box::new(&self.unaware),
            "overgrow" => Box::new(&self.overgrow),
            _ => Box::new(&self.chlorophyll),
        }
    }
}
impl SectionData for AbilityTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AbilityI18n {
    pub name: &'static str,
    pub desc: &'static str,
}

impl I18NData for AbilityI18n {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "desc" => &self.desc,
            _ => &self.name,
        }
    }
}
impl SectionData for AbilityI18n {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
