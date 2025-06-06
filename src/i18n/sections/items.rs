pub struct ItemTranslationData {
    pub lucky_egg: ItemI18n,
    pub connection_wire: ItemI18n,
}

pub struct ItemI18n {
    pub name: &'static str,
    pub desc: &'static str,
}
