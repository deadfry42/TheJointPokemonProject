use std::fmt::{self};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Language {
    // język
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => write!(f, "English"),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Stat {
    Health,
    Speed,
    Attack,
    Defense,
    SpecialDefence,
    SpecialAttack,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stat::Health => write!(f, "Health"),
            Stat::Speed => write!(f, "Speed"),
            Stat::Attack => write!(f, "Attack"),
            Stat::Defense => write!(f, "Defense"),
            Stat::SpecialDefence => write!(f, "Sp. Defence"),
            Stat::SpecialAttack => write!(f, "Sp. Attack"),
        }
    }
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

impl fmt::Display for StatusCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusCondition::Poison => write!(f, "Poison"),
            StatusCondition::BadlyPoison => write!(f, "Badly Poisoned"),
            StatusCondition::Burn => write!(f, "Burn"),
            StatusCondition::Freeze => write!(f, "Freeze"),
            StatusCondition::Paralysis => write!(f, "Paralysis"),
            StatusCondition::Sleep => write!(f, "Sleep"),
        }
    }
}

#[allow(dead_code)]
pub enum BattleConditions {
    LeechSeed,
    Confused,
}

impl fmt::Display for BattleConditions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BattleConditions::LeechSeed => write!(f, "Leech Seed"),
            BattleConditions::Confused => write!(f, "Confused"),
        }
    }
}

#[allow(dead_code)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

impl fmt::Display for MoveCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveCategory::Physical => write!(f, "Physical"),
            MoveCategory::Special => write!(f, "Special"),
            MoveCategory::Status => write!(f, "Status"),
        }
    }
}

#[allow(dead_code)]
pub enum MoveRange {
    Normal,     //The move affects a selected adjacent target.
    ManyOthers, //Affects all adjacent opponents, but not allies
}

#[allow(dead_code)]
pub enum Pokeball {
    Pokeball,
    Greatball,
    Ultraball,
    Masterball,
}

impl fmt::Display for Pokeball {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pokeball::Pokeball => write!(f, "Pokeball"),
            Pokeball::Greatball => write!(f, "Greatball"),
            Pokeball::Ultraball => write!(f, "Ultraball"),
            Pokeball::Masterball => write!(f, "Masterball"),
        }
    }
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

impl fmt::Display for Marking {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Marking::Circle => write!(f, "Circle"),
            Marking::Triangle => write!(f, "Triangle"),
            Marking::Square => write!(f, "Square"),
            Marking::Heart => write!(f, "Heart"),
            Marking::Star => write!(f, "Star"),
            Marking::Diamond => write!(f, "Diamond"),
        }
    }
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
