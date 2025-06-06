use crate::i18n::get_localisation;
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
            Item::LuckyEgg => write!(f, "{}", get_localisation().items.lucky_egg.name),
            Item::ConnectionWire => write!(f, "{}", get_localisation().items.connection_wire.name),
        }
    }
}
