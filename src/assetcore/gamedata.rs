use crate::{
    i18ncore::loaded::LoadedLocales,
    pkmncore::{pokedex::Pokedex, trainer::Player},
};

pub struct GameData {
    pub loaded_locales: LoadedLocales,
    pub dex: Pokedex,

    pub player: Option<Player>,
}

impl GameData {}
