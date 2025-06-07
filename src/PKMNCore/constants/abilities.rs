use crate::I18NCore::keys::*;
use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Ability {
    Damp,
    WaterAbsorb,
    Unaware,
    Overgrow,
    Chlorophyll,
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ability::Damp => write!(
                f,
                "{}",
                TranslationKey::new("abilities/damp/name").convert_to_string()
            ),
            Ability::WaterAbsorb => write!(
                f,
                "{}",
                TranslationKey::new("abilities/water_absorb/name").convert_to_string()
            ),
            Ability::Unaware => write!(
                f,
                "{}",
                TranslationKey::new("abilities/unaware/name").convert_to_string()
            ),
            Ability::Overgrow => write!(
                f,
                "{}",
                TranslationKey::new("abilities/overgrow/name").convert_to_string()
            ),
            Ability::Chlorophyll => write!(
                f,
                "{}",
                TranslationKey::new("abilities/chlorophyll/name").convert_to_string()
            ),
        }
    }
}
