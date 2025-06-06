use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use std::{env::*, fs::*, path::PathBuf};

pub fn check_for_assets() -> bool {
    let assets_folder = get_assets_folder();
    assets_folder.is_ok()
}

pub fn get_working_directory() -> std::io::Result<PathBuf> {
    Ok(current_dir()?)
}

pub fn get_assets_folder() -> std::io::Result<ReadDir> {
    Ok(std::fs::read_dir("./assets/")?)
}

pub fn get_asset(path: &str) -> std::io::Result<File> {
    Ok(File::open(format!("./assets/{}", path))?)
}

pub fn get_compiling_platform() -> &'static str {
    COMPILED_ON
}

pub fn get_current_platform() -> &'static str {
    CURRENT_PLATFORM
}
