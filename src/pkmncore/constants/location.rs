use std::fmt::{self};

use crate::i18ncore::keys::TranslationKey;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum GameLocation {
    MysteryZone,
}

impl fmt::Display for GameLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameLocation::MysteryZone => {
                write!(
                    f,
                    "{}",
                    TranslationKey::new("location/mystery_zone").convert_to_string()
                )
            }
        }
    }
}
