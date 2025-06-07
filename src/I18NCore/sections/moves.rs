use crate::I18NCore::localisation::{I18NData, SectionData, TranslationData};
use std::any::Any;

pub struct MoveTranslationData {
    pub tackle: MoveI18n,
    pub growl: MoveI18n,
}

impl TranslationData for MoveTranslationData {
    fn index(&self, path: &'static str) -> Box<&dyn SectionData> {
        match path {
            "tackle" => Box::new(&self.tackle),
            _ => Box::new(&self.growl),
        }
    }
}
impl SectionData for MoveTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MoveI18n {
    pub name: &'static str,
    pub desc: &'static str,
}

impl I18NData for MoveI18n {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "desc" => &self.desc,
            _ => &self.name,
        }
    }
}
impl SectionData for MoveI18n {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MoveCategoryTranslationData {
    pub physical: &'static str,
    pub special: &'static str,
    pub status: &'static str,
}
