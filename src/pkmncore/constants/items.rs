use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Item {
    LuckyEgg,
    ConnectionWire,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::LuckyEgg => write!(f, "Lucky Egg"),
            Item::ConnectionWire => write!(f, "Connection Wire"),
        }
    }
}
