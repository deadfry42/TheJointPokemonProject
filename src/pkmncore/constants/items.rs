use crate::i18ncore::keys::*;
use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Item {
    LuckyEgg,
    ConnectionWire, // link cable
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::LuckyEgg => write!(
                f,
                "{}",
                TranslationKey::new("item/lucky_egg").convert_to_string()
            ),
            Item::ConnectionWire => write!(
                f,
                "{}",
                TranslationKey::new("item/connection_wire").convert_to_string()
            ),
        }
    }
}
