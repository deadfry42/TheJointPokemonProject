use super::enums::Stat;
use crate::i18n::get_localisation;
use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
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
    fn get_stat_multiplier(&self, stat: &Stat) -> f64;
    fn index_nature(index: i8) -> Nature;
}

impl NatureData for Nature {
    fn index_nature(index: i8) -> Nature {
        match index {
            0 => Nature::Hardy,
            1 => Nature::Lonely,
            2 => Nature::Brave,
            3 => Nature::Adamant,
            4 => Nature::Naughty,
            5 => Nature::Bold,
            6 => Nature::Docile,
            7 => Nature::Relaxed,
            8 => Nature::Impish,
            9 => Nature::Lax,
            10 => Nature::Timid,
            11 => Nature::Hasty,
            12 => Nature::Serious,
            13 => Nature::Jolly,
            14 => Nature::Naive,
            15 => Nature::Modest,
            16 => Nature::Mild,
            17 => Nature::Quiet,
            18 => Nature::Bashful,
            19 => Nature::Rash,
            20 => Nature::Calm,
            21 => Nature::Gentle,
            22 => Nature::Sassy,
            23 => Nature::Careful,
            24 => Nature::Quirky,
            _ => Nature::Hardy,
        }
    }

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

    fn get_stat_multiplier(&self, stat: &Stat) -> f64 {
        if self.get_decreased_stat() == *stat {
            0.9
        } else if self.get_increased_stat() == *stat {
            1.1
        } else {
            1.0
        }
    }
}

impl fmt::Display for Nature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nature::Hardy => write!(f, "{}", get_localisation().nature.hardy),
            Nature::Lonely => write!(f, "{}", get_localisation().nature.lonely),
            Nature::Brave => write!(f, "{}", get_localisation().nature.brave),
            Nature::Adamant => write!(f, "{}", get_localisation().nature.adamant),
            Nature::Naughty => write!(f, "{}", get_localisation().nature.naughty),
            Nature::Bold => write!(f, "{}", get_localisation().nature.bold),
            Nature::Docile => write!(f, "{}", get_localisation().nature.docile),
            Nature::Relaxed => write!(f, "{}", get_localisation().nature.relaxed),
            Nature::Impish => write!(f, "{}", get_localisation().nature.impish),
            Nature::Lax => write!(f, "{}", get_localisation().nature.lax),
            Nature::Timid => write!(f, "{}", get_localisation().nature.timid),
            Nature::Hasty => write!(f, "{}", get_localisation().nature.hasty),
            Nature::Serious => write!(f, "{}", get_localisation().nature.serious),
            Nature::Jolly => write!(f, "{}", get_localisation().nature.jolly),
            Nature::Naive => write!(f, "{}", get_localisation().nature.naive),
            Nature::Modest => write!(f, "{}", get_localisation().nature.modest),
            Nature::Mild => write!(f, "{}", get_localisation().nature.mild),
            Nature::Quiet => write!(f, "{}", get_localisation().nature.quiet),
            Nature::Bashful => write!(f, "{}", get_localisation().nature.bashful),
            Nature::Rash => write!(f, "{}", get_localisation().nature.rash),
            Nature::Calm => write!(f, "{}", get_localisation().nature.calm),
            Nature::Gentle => write!(f, "{}", get_localisation().nature.gentle),
            Nature::Sassy => write!(f, "{}", get_localisation().nature.sassy),
            Nature::Careful => write!(f, "{}", get_localisation().nature.careful),
            Nature::Quirky => write!(f, "{}", get_localisation().nature.quirky),
        }
    }
}
