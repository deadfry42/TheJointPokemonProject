use crate::i18n::localisation::TranslationData;

pub struct MoveTranslationData {
    pub tackle: MoveI18n,
    pub growl: MoveI18n,
}

impl TranslationData for MoveTranslationData {}

pub struct MoveI18n {
    pub name: &'static str,
    pub desc: &'static str,
}

pub struct MoveCategoryTranslationData {
    pub physical: &'static str,
    pub special: &'static str,
    pub status: &'static str,
}
