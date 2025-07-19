use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use std::{
    env::*,
    fs::{self, *},
    path::PathBuf,
};

use crate::utils::strings::concatenate_strings;

pub struct AssetPath {
    pub base_folder: &'static str,

    pub localisation_folder: &'static str,

    pub textures_folder: &'static str,
    pub texture_image_folder: &'static str,
    pub texture_mapping_folder: &'static str,
}

impl AssetPath {
    pub const fn new() -> AssetPath {
        AssetPath {
            base_folder: "./assets",
            localisation_folder: "/localisation",
            textures_folder: "/textures",
            texture_image_folder: "/images",
            texture_mapping_folder: "/mappings",
        }
    }

    pub fn get_asset_folder_path(&self) -> &'static str {
        self.base_folder
    }

    pub fn get_localisation_folder_path(&self) -> &'static str {
        concatenate_strings(self.base_folder, self.localisation_folder)
    }

    pub fn get_textures_folder_path(&self) -> &'static str {
        concatenate_strings(self.base_folder, self.textures_folder)
    }

    pub fn get_texture_images_folder_path(&self) -> &'static str {
        concatenate_strings(self.get_textures_folder_path(), self.texture_image_folder)
    }

    pub fn get_texture_mappings_folder_path(&self) -> &'static str {
        concatenate_strings(self.get_textures_folder_path(), self.texture_mapping_folder)
    }
}

pub const ASSET_PATHS: AssetPath = AssetPath::new();

pub fn assets_available() -> bool {
    let assets_folder = get_assets_folder();
    assets_folder.is_ok()
}

pub fn get_working_directory() -> std::io::Result<PathBuf> {
    current_dir()
}

pub fn get_assets_folder() -> std::io::Result<ReadDir> {
    std::fs::read_dir(format!("{}/", ASSET_PATHS.get_asset_folder_path()))
}

pub fn get_asset(path: &str) -> std::io::Result<File> {
    File::open(format!("{}/{}", ASSET_PATHS.get_asset_folder_path(), path))
}

pub fn get_asset_folder(path: &str) -> std::io::Result<ReadDir> {
    fs::read_dir(format!("{}/{}", ASSET_PATHS.get_asset_folder_path(), path))
}

pub fn get_compiling_platform() -> &'static str {
    COMPILED_ON
}

pub fn get_current_platform() -> &'static str {
    CURRENT_PLATFORM
}
