pub struct PokemonTranslationData {
    pub bulbasaur: PokemonI18n,
    pub ivysaur: PokemonI18n,
    pub venusaur: PokemonI18n,
    pub wooper: PokemonI18n,
}

pub struct PokemonI18n {
    pub name: &'static str,
    pub dex: &'static str,
}
