use native_dialog::{DialogBuilder, MessageLevel};

use crate::{
    GAME_LOCALES, GAME_TITLE,
    assetcore::assets::assets_available,
    i18ncore::parsing::load_localisation,
    utils::{logger::Logger, strings::concatenate_strings},
};

pub fn try_load() {
    Logger::debug_log_literal("Loading locale..");
    load_localisation();
    GAME_LOCALES
        .lock()
        .unwrap()
        .set_selected_locale("en_lolcat");
}

pub fn can_run() -> bool {
    if !assets_available() {
        display_asset_error();
        return false;
    }

    true
}

pub fn display_asset_error() {
    Logger::warn_literal("Displaying asset error.");
    DialogBuilder::message()
        .set_level(MessageLevel::Error)
        .set_title(concatenate_strings(GAME_TITLE, " Error"))
        .set_text("The assets folder cannot be found! The game will not run.\nTry making sure that the assets folder is present in the game directory.")
        .alert()
        .show()
        .unwrap();
}
