use crate::{i18ncore::loaded::LoadedLocales, pkmncore::pokedex::Pokedex};

pub struct GameData {
    pub loaded_locales: LoadedLocales,
    pub dex: Pokedex,
}

impl GameData {}
