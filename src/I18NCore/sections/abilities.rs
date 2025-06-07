pub struct AbilityTranslationData {
    pub damp: AbilityI18n,
    pub water_absorb: AbilityI18n,
    pub unaware: AbilityI18n,
    pub overgrow: AbilityI18n,
    pub chlorophyll: AbilityI18n,
}

pub struct AbilityI18n {
    pub name: &'static str,
    pub desc: &'static str,
}
