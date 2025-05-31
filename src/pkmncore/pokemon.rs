use super::{
    constants::{abilities::*, enums::*, items::*, levels::*, natures::*, pokemon::*, typing::*},
    moves::MoveData,
    trainer::*,
};

#[allow(dead_code)]
pub struct PokemonBase {
    pub name: &'static str,
    pub types: TypeSet,
    pub pkmn: Pokemon,
    pub pokedex: PokedexInfo,
    pub base_stats: BaseStats,
    pub catch_rate: u8,
    pub base_friendship: u8,
    pub gender_ratio: Option<f32>,
    pub egg_cycles: u8,
    pub abilities: AbilitySet,
    pub levelling_curve: LevellingCurve,
    pub base_exp: u32,
    pub ev_yield: EVs,
}

#[allow(dead_code)]
impl PokemonBase {
    pub fn summarise(&self) -> String {
        format!(
            "Pokemon {}\nType1: {}\nPokedexInfo: {}, {}\nStats: {}hp, {}spd, {}atk, {}def, {}spatk, {}spdef\nCatch rate: {}\nFriendship: {}\nGender Ratio: {}% to be Male.\nEgg cycles: {}\nAbility1: {}",
            self.name,
            self.types.type1,
            self.pokedex.species,
            self.pokedex.entry,
            self.base_stats.health,
            self.base_stats.speed,
            self.base_stats.atk,
            self.base_stats.def,
            self.base_stats.spatk,
            self.base_stats.spdef,
            self.catch_rate,
            self.base_friendship,
            self.gender_ratio.unwrap() * 100.0,
            self.egg_cycles,
            self.abilities.ability1,
        )
    }

    pub fn get_gender(&self, gender_float: f32) -> Gender {
        if self.gender_ratio.is_none() {
            return Gender::Unknown;
        } else {
            if gender_float < self.gender_ratio.unwrap() {
                return Gender::Male;
            } else {
                return Gender::Female;
            }
        }
    }
}

#[allow(dead_code)]
pub struct PokemonData {
    pub nickname: Option<&'static str>,
    pub ot: Option<OTInformation>,
    pub base: PokemonBase,
    pub evs: EVs,
    pub ivs: IVs,
    pub exp: u32,
    pub ability: Ability,
    pub mettime: i64,
    pub pid: u32,
    pub isegg: bool,
    pub friendship: u8,
    pub pokeball: Option<Pokeball>,
    pub pokerus: bool,
    pub shiny: bool,
    pub marking: Option<Marking>,
    pub condition: Option<StatusCondition>,
    pub helditem: Option<Item>,
    pub nature: Nature,
    pub moves: [Option<MoveData>; 4],
    // TODO:
    // metlocation: Location
}

#[allow(dead_code)]
impl PokemonData {
    pub fn award_xp(&mut self, xp: u32) {
        self.exp += xp;
    }

    pub fn get_level(&self) -> i8 {
        self.base.levelling_curve.exp_to_levels(self.exp)
    }

    pub fn is_holding(&self, i: Item) -> bool {
        self.helditem.as_ref().unwrap().eq(&i)
    }
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
pub struct EVs {
    pub health: Option<i16>,
    pub speed: Option<i16>,
    pub atk: Option<i16>,
    pub def: Option<i16>,
    pub spatk: Option<i16>,
    pub spdef: Option<i16>,
}

#[allow(dead_code)]
pub struct IVs {
    pub health: i8,
    pub speed: i8,
    pub atk: i8,
    pub def: i8,
    pub spatk: i8,
    pub spdef: i8,
}

#[allow(dead_code)]
pub struct PokedexInfo {
    pub index: i32,
    pub height: f64,
    pub weight: f64,
    pub species: &'static str,
    pub entry: &'static str,
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
