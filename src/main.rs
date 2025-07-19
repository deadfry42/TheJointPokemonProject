use std::sync::{LazyLock, Mutex};

use crate::gamecore::content::camera::Camera;
use crate::gamecore::engine::play::play;
use crate::i18ncore::loaded::LoadedLocales;

use crate::pkmncore::pokedex::Pokedex;
use crate::utils::logger::Logger;

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

pub static GAME_CAMERA: LazyLock<Mutex<Camera>> = LazyLock::new(|| Mutex::new(Camera::new()));
pub static GAME_LOCALES: LazyLock<Mutex<LoadedLocales>> =
    LazyLock::new(|| Mutex::new(LoadedLocales::new()));

fn main() {
    Logger::log(format!("Starting {}, {}!", GAME_TITLE, GAME_VERSION));
    play();
}
