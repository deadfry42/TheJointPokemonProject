use crate::i18n::get_localisation;
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
            Ability::Damp => write!(f, "{}", get_localisation().abilities.damp.name),
            Ability::WaterAbsorb => write!(f, "{}", get_localisation().abilities.water_absorb.name),
            Ability::Unaware => write!(f, "{}", get_localisation().abilities.unaware.name),
            Ability::Overgrow => write!(f, "{}", get_localisation().abilities.overgrow.name),
            Ability::Chlorophyll => write!(f, "{}", get_localisation().abilities.chlorophyll.name),
        }
    }
}
