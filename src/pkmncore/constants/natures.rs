use super::enums::Stat;
use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Nature {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}

#[allow(dead_code)]
pub trait NatureData {
    fn get_increased_stat(&self) -> Stat;
    fn get_decreased_stat(&self) -> Stat;
}

impl NatureData for Nature {
    fn get_increased_stat(&self) -> Stat {
        match self {
            Nature::Hardy => Stat::Attack,
            Nature::Lonely => Stat::Attack,
            Nature::Brave => Stat::Attack,
            Nature::Adamant => Stat::Attack,
            Nature::Naughty => Stat::Attack,
            Nature::Bold => Stat::Defense,
            Nature::Docile => Stat::Defense,
            Nature::Relaxed => Stat::Defense,
            Nature::Impish => Stat::Defense,
            Nature::Lax => Stat::Defense,
            Nature::Timid => Stat::Speed,
            Nature::Hasty => Stat::Speed,
            Nature::Serious => Stat::Speed,
            Nature::Jolly => Stat::Speed,
            Nature::Naive => Stat::Speed,
            Nature::Modest => Stat::SpecialAttack,
            Nature::Mild => Stat::SpecialAttack,
            Nature::Quiet => Stat::SpecialAttack,
            Nature::Bashful => Stat::SpecialAttack,
            Nature::Rash => Stat::SpecialAttack,
            Nature::Calm => Stat::SpecialDefence,
            Nature::Gentle => Stat::SpecialDefence,
            Nature::Sassy => Stat::SpecialDefence,
            Nature::Careful => Stat::SpecialDefence,
            Nature::Quirky => Stat::SpecialDefence,
        }
    }

    fn get_decreased_stat(&self) -> Stat {
        match self {
            Nature::Hardy => Stat::Attack,
            Nature::Lonely => Stat::Defense,
            Nature::Brave => Stat::Speed,
            Nature::Adamant => Stat::SpecialAttack,
            Nature::Naughty => Stat::SpecialDefence,
            Nature::Bold => Stat::Attack,
            Nature::Docile => Stat::Defense,
            Nature::Relaxed => Stat::Speed,
            Nature::Impish => Stat::SpecialAttack,
            Nature::Lax => Stat::SpecialDefence,
            Nature::Timid => Stat::Attack,
            Nature::Hasty => Stat::Defense,
            Nature::Serious => Stat::Speed,
            Nature::Jolly => Stat::SpecialAttack,
            Nature::Naive => Stat::SpecialDefence,
            Nature::Modest => Stat::Attack,
            Nature::Mild => Stat::Defense,
            Nature::Quiet => Stat::Speed,
            Nature::Bashful => Stat::SpecialAttack,
            Nature::Rash => Stat::SpecialDefence,
            Nature::Calm => Stat::Attack,
            Nature::Gentle => Stat::Defense,
            Nature::Sassy => Stat::Speed,
            Nature::Careful => Stat::SpecialAttack,
            Nature::Quirky => Stat::SpecialDefence,
        }
    }
}

impl fmt::Display for Nature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nature::Hardy => write!(f, "Hardy"),
            Nature::Lonely => write!(f, "Lonely"),
            Nature::Brave => write!(f, "Brave"),
            Nature::Adamant => write!(f, "Adamant"),
            Nature::Naughty => write!(f, "Naughty"),
            Nature::Bold => write!(f, "Bold"),
            Nature::Docile => write!(f, "Docile"),
            Nature::Relaxed => write!(f, "Relaxed"),
            Nature::Impish => write!(f, "Impish"),
            Nature::Lax => write!(f, "Lax"),
            Nature::Timid => write!(f, "Timid"),
            Nature::Hasty => write!(f, "Hasty"),
            Nature::Serious => write!(f, "Serious"),
            Nature::Jolly => write!(f, "Jolly"),
            Nature::Naive => write!(f, "Naive"),
            Nature::Modest => write!(f, "Modest"),
            Nature::Mild => write!(f, "Mild"),
            Nature::Quiet => write!(f, "Quiet"),
            Nature::Bashful => write!(f, "Bashful"),
            Nature::Rash => write!(f, "Rash"),
            Nature::Calm => write!(f, "Calm"),
            Nature::Gentle => write!(f, "Gentle"),
            Nature::Sassy => write!(f, "Sassy"),
            Nature::Careful => write!(f, "Careful"),
            Nature::Quirky => write!(f, "Quirky"),
        }
    }
}
