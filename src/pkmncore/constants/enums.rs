use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Language {
    // język
    English,
}

#[allow(dead_code)]
pub enum StatusCondition {
    Poison,
    BadlyPoison,
    Burn,
    Freeze,
    Paralysis,
    Sleep,
}

#[allow(dead_code)]
pub enum BattleConditions {
    LeechSeed,
    Confused,
}

#[allow(dead_code)]
pub enum Pokeball {
    Pokeball,
    Greatball,
    Ultraball,
    Masterball,
}

#[allow(dead_code)]
pub enum Marking {
    Circle,
    Triangle,
    Square,
    Heart,
    Star,
    Diamond,
}

#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Unknown => write!(f, "Unknown"),
        }
    }
}
