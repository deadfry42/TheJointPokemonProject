use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum GameLocation {
    MysteryZone,
}

impl fmt::Display for GameLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameLocation::MysteryZone => write!(f, "Mystery Zone"),
        }
    }
}
