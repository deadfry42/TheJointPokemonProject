use crate::pkmncore::constants::enums::*;
use crate::pkmncore::pokemon::*;

use super::abilities::Ability;
use super::items::Item;
use super::levels::*;
use super::typing::*;

#[allow(dead_code)]
pub trait PokemonType {
    fn get_base(&self) -> PokemonBase;
    fn get_base_stat(&self, stat: &Stat) -> i16;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Pokemon {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Wooper,
}

impl PokemonType for Pokemon {
    fn get_base(&self) -> PokemonBase {
        match self {
            Pokemon::Bulbasaur => PokemonBase {
                name: "Bulbasaur",
                pkmn: Pokemon::Bulbasaur,
                pokedex: PokedexInfo {
                    index: 1,
                    height: 0.7,
                    weight: 6.9,
                    species: "Seed Pokemon",
                    entry: "This pokemon is extra gay",
                },
                ev_yield: EVs {
                    health: None,
                    speed: None,
                    atk: None,
                    def: None,
                    spatk: Some(1),
                    spdef: None,
                },
                abilities: AbilitySet {
                    ability1: Ability::Overgrow,
                    ability2: None,
                    hability: Ability::Chlorophyll,
                },
                base_stats: BaseStats {
                    health: 45,
                    atk: 49,
                    def: 49,
                    spatk: 65,
                    spdef: 65,
                    speed: 45,
                },
                types: TypeSet {
                    type1: Type::Grass,
                    type2: Some(Type::Poison),
                    egg_group1: EggGroup::Grass,
                    egg_group2: Some(EggGroup::Monster),
                },
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 64,
            },
            Pokemon::Ivysaur => PokemonBase {
                name: "Ivysaur",
                pkmn: Pokemon::Ivysaur,
                pokedex: PokedexInfo {
                    index: 2,
                    height: 1.0,
                    weight: 13.0,
                    species: "Seed Pokemon",
                    entry: "This pokemon is extra gay",
                },
                ev_yield: EVs {
                    health: None,
                    speed: None,
                    atk: None,
                    def: None,
                    spatk: Some(1),
                    spdef: Some(1),
                },
                abilities: AbilitySet {
                    ability1: Ability::Overgrow,
                    ability2: None,
                    hability: Ability::Chlorophyll,
                },
                base_stats: BaseStats {
                    health: 60,
                    atk: 62,
                    def: 63,
                    spatk: 80,
                    spdef: 80,
                    speed: 60,
                },
                types: TypeSet {
                    type1: Type::Grass,
                    type2: Some(Type::Poison),
                    egg_group1: EggGroup::Grass,
                    egg_group2: Some(EggGroup::Monster),
                },
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 142,
            },
            Pokemon::Venusaur => PokemonBase {
                name: "Venusaur",
                pkmn: Pokemon::Venusaur,
                pokedex: PokedexInfo {
                    index: 3,
                    height: 2.0,
                    weight: 100.0,
                    species: "Seed Pokemon",
                    entry: "This pokemon is extra gay",
                },
                ev_yield: EVs {
                    health: None,
                    speed: None,
                    atk: None,
                    def: None,
                    spatk: Some(2),
                    spdef: Some(1),
                },
                abilities: AbilitySet {
                    ability1: Ability::Overgrow,
                    ability2: None,
                    hability: Ability::Chlorophyll,
                },
                base_stats: BaseStats {
                    health: 80,
                    atk: 82,
                    def: 83,
                    spatk: 100,
                    spdef: 100,
                    speed: 80,
                },
                types: TypeSet {
                    type1: Type::Grass,
                    type2: Some(Type::Poison),
                    egg_group1: EggGroup::Grass,
                    egg_group2: Some(EggGroup::Monster),
                },
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 236,
            },
            Pokemon::Wooper => PokemonBase {
                name: "Wooper",
                pkmn: Pokemon::Wooper,
                pokedex: PokedexInfo {
                    index: 194,
                    height: 0.4,
                    weight: 8.5,
                    species: "Water Fish",
                    entry: "This pokemon is gay",
                },
                ev_yield: EVs {
                    health: Some(1),
                    speed: None,
                    atk: None,
                    def: None,
                    spatk: None,
                    spdef: None,
                },
                abilities: AbilitySet {
                    ability1: Ability::Damp,
                    ability2: Some(Ability::WaterAbsorb),
                    hability: Ability::Unaware,
                },
                base_stats: BaseStats {
                    health: 55,
                    atk: 45,
                    def: 45,
                    spatk: 25,
                    spdef: 25,
                    speed: 15,
                },
                types: TypeSet {
                    type1: Type::Water,
                    type2: Some(Type::Ground),
                    egg_group1: EggGroup::Field,
                    egg_group2: Some(EggGroup::Water1),
                },
                held_items: Some(vec![PokemonHeldItem {
                    item: Item::LuckyEgg,
                    chance: 0.5,
                }]),
                levelling_curve: LevellingCurve::MediumFast,
                catch_rate: 255,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.5),
                base_exp: 42,
            },
        }
    }

    fn get_base_stat(&self, stat: &Stat) -> i16 {
        match stat {
            Stat::Health => self.get_base().base_stats.health,
            Stat::Speed => self.get_base().base_stats.speed,
            Stat::Attack => self.get_base().base_stats.atk,
            Stat::Defense => self.get_base().base_stats.def,
            Stat::SpecialAttack => self.get_base().base_stats.spatk,
            Stat::SpecialDefence => self.get_base().base_stats.spdef,
        }
    }
}

#[allow(dead_code)]
pub trait EvolutionByLevel {
    fn get_evolution(&self) -> Option<Pokemon>;
    fn get_evolution_level(&self) -> Option<i8>;
}

#[allow(dead_code)]
pub trait EvolutionByItem {
    fn get_evolution(&self) -> Option<Pokemon>;
    fn get_evolution_item(&self) -> Option<Item>;
}

#[allow(dead_code)]
pub trait EvolutionByTrade {
    fn get_evolution(&self) -> Option<Pokemon>;
}

#[allow(dead_code)]
pub trait EvolutionByTradeWithItem {
    fn get_evolution(&self) -> Option<Pokemon>;
    fn get_evolution_item(&self) -> Option<Item>;
}

impl EvolutionByLevel for Pokemon {
    fn get_evolution(&self) -> Option<Pokemon> {
        match self {
            Pokemon::Bulbasaur => Some(Pokemon::Ivysaur),
            Pokemon::Ivysaur => Some(Pokemon::Venusaur),
            _ => None,
        }
    }

    fn get_evolution_level(&self) -> Option<i8> {
        match self {
            Pokemon::Bulbasaur => Some(16),
            Pokemon::Ivysaur => Some(32),
            _ => None,
        }
    }
}

impl EvolutionByItem for Pokemon {
    fn get_evolution(&self) -> Option<Pokemon> {
        match self {
            _ => None,
        }
    }

    fn get_evolution_item(&self) -> Option<Item> {
        match self {
            _ => None,
        }
    }
}

impl EvolutionByTrade for Pokemon {
    fn get_evolution(&self) -> Option<Pokemon> {
        match self {
            _ => None,
        }
    }
}

impl EvolutionByTradeWithItem for Pokemon {
    fn get_evolution(&self) -> Option<Pokemon> {
        match self {
            _ => None,
        }
    }

    fn get_evolution_item(&self) -> Option<Item> {
        match self {
            _ => None,
        }
    }
}
