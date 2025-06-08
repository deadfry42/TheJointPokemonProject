use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use std::{
    env::*,
    fs::{self, *},
    path::PathBuf,
};

pub const ASSETS_PATH: &str = "./assets";

pub fn check_for_assets() -> bool {
    let assets_folder = get_assets_folder();
    assets_folder.is_ok()
}

pub fn get_working_directory() -> std::io::Result<PathBuf> {
    current_dir()
}

pub fn get_assets_folder() -> std::io::Result<ReadDir> {
    std::fs::read_dir(format!("{}/", ASSETS_PATH))
}

pub fn get_asset(path: &str) -> std::io::Result<File> {
    File::open(format!("{}/{}", ASSETS_PATH, path))
}

pub fn get_asset_folder(path: &str) -> std::io::Result<ReadDir> {
    fs::read_dir(format!("{}/{}", ASSETS_PATH, path))
}

pub fn get_compiling_platform() -> &'static str {
    COMPILED_ON
}

pub fn get_current_platform() -> &'static str {
    CURRENT_PLATFORM
}
