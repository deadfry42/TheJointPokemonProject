use crate::{i18ncore::keys::*, pkmncore::constants::enums::BagCategory};
use std::fmt::{self};

pub trait ItemBase {
    fn get_category(&self) -> BagCategory;
}

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
                TranslationKey::new("item/lucky_egg/name").convert_to_string()
            ),
            Item::ConnectionWire => write!(
                f,
                "{}",
                TranslationKey::new("item/connection_wire/name").convert_to_string()
            ),
        }
    }
}

impl ItemBase for Item {
    fn get_category(&self) -> BagCategory {
        match self {
            Item::LuckyEgg => BagCategory::Pocket,
            Item::ConnectionWire => BagCategory::Pocket,
        }
    }
}
