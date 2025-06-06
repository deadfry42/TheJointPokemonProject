pub struct MoveTranslationData {
    pub tackle: MoveI18n,
    pub growl: MoveI18n,
}

pub struct MoveI18n {
    pub name: &'static str,
    pub desc: &'static str,
}
