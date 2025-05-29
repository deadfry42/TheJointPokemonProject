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
pub enum Type {
    Normal,
    Water,
    Fire,
    Grass,
    Psychic,
    Ground,
    Rock,
    Bug,
    Steel,
    Dark,
    Ice,
    Dragon,
    Fairy,
    Flying,
    Ghost,
    Fighting,
    Poison,
    Electric,
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
