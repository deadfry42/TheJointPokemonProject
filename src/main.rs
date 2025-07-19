use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::assetcore::gamedata::GameData;
use crate::gamecore::play::play;
use crate::i18ncore::loaded::LoadedLocales;

use crate::pkmncore::pokedex::Pokedex;
use crate::utils::logger::Logger;

extern crate lazy_static;

pub mod assetcore;
pub mod gamecore;
pub mod i18ncore;
pub mod pkmncore;
pub mod sdlcore;
pub mod utils;

#[allow(dead_code)]
pub const GAME_VERSION: &str = "v0.0-beta";
pub const GAME_TITLE: &str = "TJPP";
pub const GAME_VERBOSITY: bool = true;

lazy_static! {
    pub static ref GAME_DATA: Mutex<GameData> = Mutex::new(GameData {
        loaded_locales: LoadedLocales::new(),
        dex: Pokedex::create_regional_dex(),
        player: None,
    });
}

pub fn get_game_data() -> Option<std::sync::MutexGuard<'static, GameData>> {
    // TODO: verify this is a good way to do this :)
    Some(GAME_DATA.lock().unwrap())
}

fn main() {
    Logger::log(format!("Starting {}, {}!", GAME_TITLE, GAME_VERSION));
    play();
}
