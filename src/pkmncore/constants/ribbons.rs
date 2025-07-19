use crate::i18ncore::keys::*;
use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Ribbon {
    Champion,
}

impl fmt::Display for Ribbon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ribbon::Champion => write!(
                f,
                "{}",
                TranslationKey::new("ribbons/champion/name").convert_to_string()
            ),
        }
    }
}
