use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
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
            Ability::Damp => write!(f, "Damp"),
            Ability::WaterAbsorb => write!(f, "Water Absorb"),
            Ability::Unaware => write!(f, "Unaware"),
            Ability::Overgrow => write!(f, "Overgrow"),
            Ability::Chlorophyll => write!(f, "Chlorophyll"),
        }
    }
}
