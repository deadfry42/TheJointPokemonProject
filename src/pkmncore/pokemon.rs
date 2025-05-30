use std::fmt::{self};

#[allow(dead_code)]
pub struct Pokemon {
    pub name: &'static str,
    pub types: TypeSet,
    pub pokedex: PokedexInfo,
    pub base_stats: BaseStats,
    pub catch_rate: i16,
    pub base_friendship: i16,
    pub gender_ratio: Option<f32>,
    pub egg_cycles: i16,
    pub abilities: AbilitySet,
    pub levelling_curve: LevellingCurve,
    pub base_exp: i16,
    pub evyeild: EVYield,
}

#[allow(dead_code)]
pub struct AbilitySet {
    pub ability1: Ability,
    pub ability2: Option<Ability>,
    pub hability: Ability,
}

#[allow(dead_code)]
pub struct TypeSet {
    pub type1: Type,
    pub type2: Option<Type>,
    pub egg_group1: EggGroup,
    pub egg_group2: Option<EggGroup>,
}

#[allow(dead_code)]
pub enum Ability {
    Damp,
    WaterAbsorb,
    Unaware,
}

#[allow(dead_code)]
pub struct EVYield {
    pub health: Option<i16>,
    pub speed: Option<i16>,
    pub atk: Option<i16>,
    pub def: Option<i16>,
    pub spatk: Option<i16>,
    pub spdef: Option<i16>,
}

#[allow(dead_code)]
pub struct PokedexInfo {
    pub index: i32,
    pub height: f64,
    pub weight: f64,
    pub species: &'static str,
    pub entry: &'static str,
}

// pub struct BoxPokemon {
//     pub pkmn: Pokemon,
// }

#[allow(dead_code)]
pub enum LevellingCurve {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

impl fmt::Display for LevellingCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LevellingCurve::Erratic => write!(f, "Erratic"),
            LevellingCurve::Fast => write!(f, "Fast"),
            LevellingCurve::MediumFast => write!(f, "MediumFast"),
            LevellingCurve::MediumSlow => write!(f, "MediumSlow"),
            LevellingCurve::Slow => write!(f, "Slow"),
            LevellingCurve::Fluctuating => write!(f, "Fluctuating"),
        }
    }
}

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

#[allow(dead_code)]
pub trait Typing {
    fn get_type_multiplier(&self, opposing_type: Type) -> f64;
}

#[allow(dead_code)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl Typing for Type {
    fn get_type_multiplier(&self, opposing_type: Type) -> f64 {
        match self {
            Type::Normal => match opposing_type {
                Type::Rock => 0.5,
                Type::Ghost => 0.0,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Fire => match opposing_type {
                Type::Fire => 0.5,
                Type::Water => 0.5,
                Type::Grass => 2.0,
                Type::Ice => 2.0,
                Type::Bug => 2.0,
                Type::Rock => 0.5,
                Type::Dragon => 0.5,
                Type::Steel => 2.0,
                _ => 1.0,
            },
            Type::Water => match opposing_type {
                Type::Fire => 2.0,
                Type::Water => 0.5,
                Type::Grass => 0.5,
                Type::Ground => 2.0,
                Type::Rock => 2.0,
                Type::Dragon => 0.5,
                _ => 1.0,
            },
            Type::Electric => match opposing_type {
                Type::Water => 2.0,
                Type::Electric => 0.5,
                Type::Grass => 0.5,
                Type::Ground => 0.0,
                Type::Flying => 2.0,
                Type::Dragon => 0.5,
                _ => 1.0,
            },
            Type::Grass => match opposing_type {
                Type::Fire => 0.5,
                Type::Water => 2.0,
                Type::Grass => 0.5,
                Type::Poison => 0.5,
                Type::Ground => 2.0,
                Type::Flying => 0.5,
                Type::Bug => 0.5,
                Type::Rock => 2.0,
                Type::Dragon => 0.5,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Ice => match opposing_type {
                Type::Fire => 0.5,
                Type::Water => 0.5,
                Type::Grass => 2.0,
                Type::Ice => 0.5,
                Type::Ground => 2.0,
                Type::Flying => 2.0,
                Type::Dragon => 2.0,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Fighting => match opposing_type {
                Type::Normal => 2.0,
                Type::Ice => 2.0,
                Type::Poison => 0.5,
                Type::Flying => 0.5,
                Type::Psychic => 0.5,
                Type::Bug => 0.5,
                Type::Rock => 2.0,
                Type::Ghost => 0.0,
                Type::Dark => 2.0,
                Type::Steel => 2.0,
                Type::Fairy => 0.5,
                _ => 1.0,
            },
            Type::Poison => match opposing_type {
                Type::Grass => 2.0,
                Type::Poison => 0.5,
                Type::Ground => 0.5,
                Type::Rock => 0.5,
                Type::Ghost => 0.5,
                Type::Steel => 0.0,
                Type::Fairy => 2.0,
                _ => 1.0,
            },
            Type::Ground => match opposing_type {
                Type::Fire => 2.0,
                Type::Electric => 2.0,
                Type::Grass => 0.5,
                Type::Poison => 2.0,
                Type::Flying => 0.0,
                Type::Bug => 0.5,
                Type::Rock => 2.0,
                Type::Steel => 2.0,
                _ => 1.0,
            },
            Type::Flying => match opposing_type {
                Type::Electric => 0.5,
                Type::Grass => 2.0,
                Type::Fighting => 2.0,
                Type::Bug => 2.0,
                Type::Rock => 0.5,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Psychic => match opposing_type {
                Type::Fighting => 2.0,
                Type::Poison => 2.0,
                Type::Psychic => 0.5,
                Type::Dark => 0.0,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Bug => match opposing_type {
                Type::Fire => 0.5,
                Type::Grass => 2.0,
                Type::Fighting => 0.5,
                Type::Poison => 0.5,
                Type::Flying => 0.5,
                Type::Psychic => 2.0,
                Type::Ghost => 0.5,
                Type::Dark => 2.0,
                Type::Steel => 0.5,
                Type::Fairy => 0.5,
                _ => 1.0,
            },
            Type::Rock => match opposing_type {
                Type::Fire => 2.0,
                Type::Ice => 2.0,
                Type::Fighting => 0.5,
                Type::Ground => 0.5,
                Type::Flying => 2.0,
                Type::Bug => 2.0,
                Type::Steel => 0.5,
                _ => 1.0,
            },
            Type::Ghost => match opposing_type {
                Type::Normal => 0.0,
                Type::Psychic => 2.0,
                Type::Ghost => 2.0,
                Type::Dark => 0.5,
                _ => 1.0,
            },
            Type::Dragon => match opposing_type {
                Type::Dragon => 2.0,
                Type::Steel => 0.5,
                Type::Fairy => 0.0,
                _ => 1.0,
            },
            Type::Dark => match opposing_type {
                Type::Fighting => 0.5,
                Type::Psychic => 2.0,
                Type::Ghost => 2.0,
                Type::Dark => 0.5,
                Type::Fairy => 0.5,
                _ => 1.0,
            },
            Type::Steel => match opposing_type {
                Type::Fire => 0.5,
                Type::Water => 0.5,
                Type::Electric => 0.5,
                Type::Ice => 2.0,
                Type::Rock => 2.0,
                Type::Steel => 0.5,
                Type::Fairy => 2.0,
                _ => 1.0,
            },
            Type::Fairy => match opposing_type {
                Type::Fire => 0.5,
                Type::Fighting => 2.0,
                Type::Poison => 0.5,
                Type::Dragon => 2.0,
                Type::Dark => 2.0,
                Type::Steel => 0.5,
                _ => 1.0,
            },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Normal => write!(f, "Normal"),
            Type::Water => write!(f, "Water"),
            Type::Fire => write!(f, "Fire"),
            Type::Grass => write!(f, "Grass"),
            Type::Psychic => write!(f, "Psychic"),
            Type::Ground => write!(f, "Ground"),
            Type::Rock => write!(f, "Rock"),
            Type::Bug => write!(f, "Bug"),
            Type::Steel => write!(f, "Steel"),
            Type::Dark => write!(f, "Dark"),
            Type::Ice => write!(f, "Ice"),
            Type::Dragon => write!(f, "Dragon"),
            Type::Fairy => write!(f, "Fairy"),
            Type::Flying => write!(f, "Flying"),
            Type::Ghost => write!(f, "Ghost"),
            Type::Fighting => write!(f, "Fighting"),
            Type::Poison => write!(f, "Poison"),
            Type::Electric => write!(f, "Electric"),
        }
    }
}

#[allow(dead_code)]
pub enum EggGroup {
    Mineral,
    Amorphous,
    Grass,
    Water1,
    Water2,
    Water3,
    Bug,
    Dragon,
    Flying,
    Field,
    HumanLike,
    Fairy,
    Monster,
}

impl fmt::Display for EggGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EggGroup::Mineral => write!(f, "Mineral"),
            EggGroup::Amorphous => write!(f, "Amorphous"),
            EggGroup::Grass => write!(f, "Grass"),
            EggGroup::Water1 => write!(f, "Water1"),
            EggGroup::Water2 => write!(f, "Water2"),
            EggGroup::Water3 => write!(f, "Water3"),
            EggGroup::Bug => write!(f, "Bug"),
            EggGroup::Dragon => write!(f, "Dragon"),
            EggGroup::Flying => write!(f, "Flying"),
            EggGroup::Field => write!(f, "Field"),
            EggGroup::HumanLike => write!(f, "HumanLike"),
            EggGroup::Fairy => write!(f, "Fairy"),
            EggGroup::Monster => write!(f, "Monster"),
        }
    }
}

#[allow(dead_code)]
impl Pokemon {
    pub fn summarise(&self) -> String {
        format!(
            "Pokemon {}, {}, eggCycles: {}",
            self.name, self.pokedex.entry, self.egg_cycles
        )
    }

    pub fn get_gender(&self, gender_float: f32) -> Gender {
        if self.gender_ratio.is_none() {
            return Gender::Unknown;
        } else {
            if gender_float >= self.gender_ratio.unwrap() {
                return Gender::Male;
            } else {
                return Gender::Female;
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct BaseStats {
    pub health: i16,
    pub speed: i16,
    pub atk: i16,
    pub def: i16,
    pub spatk: i16,
    pub spdef: i16,
}
