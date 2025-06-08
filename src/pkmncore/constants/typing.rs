use std::fmt::{self};

use crate::i18ncore::keys::TranslationKey;

#[allow(dead_code)]
pub trait Typing {
    fn get_type_multiplier(&self, opposing_type: &Type) -> f64;
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
    fn get_type_multiplier(&self, opposing_type: &Type) -> f64 {
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
            Type::Normal => write!(
                f,
                "{}",
                TranslationKey::new("type/normal").convert_to_string()
            ),
            Type::Water => write!(
                f,
                "{}",
                TranslationKey::new("type/water").convert_to_string()
            ),
            Type::Fire => write!(
                f,
                "{}",
                TranslationKey::new("type/fire").convert_to_string()
            ),
            Type::Grass => write!(
                f,
                "{}",
                TranslationKey::new("type/grass").convert_to_string()
            ),
            Type::Psychic => write!(
                f,
                "{}",
                TranslationKey::new("type/psychic").convert_to_string()
            ),
            Type::Ground => write!(
                f,
                "{}",
                TranslationKey::new("type/ground").convert_to_string()
            ),
            Type::Rock => write!(
                f,
                "{}",
                TranslationKey::new("type/rock").convert_to_string()
            ),
            Type::Bug => write!(f, "{}", TranslationKey::new("type/bug").convert_to_string()),
            Type::Steel => write!(
                f,
                "{}",
                TranslationKey::new("type/steel").convert_to_string()
            ),
            Type::Dark => write!(
                f,
                "{}",
                TranslationKey::new("type/dark").convert_to_string()
            ),
            Type::Ice => write!(f, "{}", TranslationKey::new("type/ice").convert_to_string()),
            Type::Dragon => write!(
                f,
                "{}",
                TranslationKey::new("type/dragon").convert_to_string()
            ),
            Type::Fairy => write!(
                f,
                "{}",
                TranslationKey::new("type/fairy").convert_to_string()
            ),
            Type::Flying => write!(
                f,
                "{}",
                TranslationKey::new("type/flying").convert_to_string()
            ),
            Type::Ghost => write!(
                f,
                "{}",
                TranslationKey::new("type/ghost").convert_to_string()
            ),
            Type::Fighting => write!(
                f,
                "{}",
                TranslationKey::new("type/fighting").convert_to_string()
            ),
            Type::Poison => write!(
                f,
                "{}",
                TranslationKey::new("type/poison").convert_to_string()
            ),
            Type::Electric => write!(
                f,
                "{}",
                TranslationKey::new("type/electric").convert_to_string()
            ),
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
    None,
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
            EggGroup::None => write!(f, "Not discovered"),
        }
    }
}
