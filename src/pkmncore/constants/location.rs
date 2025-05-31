use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum Location {
    MysteryZone,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::MysteryZone => write!(f, "Mystery Zone"),
        }
    }
}
