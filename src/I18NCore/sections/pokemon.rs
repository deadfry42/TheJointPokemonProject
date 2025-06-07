use crate::I18NCore::localisation::TranslationData;

pub struct PokemonTranslationData {
    pub bulbasaur: PokemonI18n,
    pub ivysaur: PokemonI18n,
    pub venusaur: PokemonI18n,
    pub wooper: PokemonI18n,
}

impl TranslationData for PokemonTranslationData {}

pub struct PokemonI18n {
    pub name: &'static str,
    pub dex: &'static str,
    pub species: &'static str,
}
