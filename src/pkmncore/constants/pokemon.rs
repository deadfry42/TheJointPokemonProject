use crate::pkmncore::constants::enums::*;
use crate::pkmncore::constants::moves::Move;
use crate::pkmncore::evolution::*;
use crate::pkmncore::levels::*;
use crate::pkmncore::moves::LevelUpMove;
use crate::pkmncore::pokemon::*;

use super::abilities::Ability;
use super::items::Item;
use super::typing::*;

#[allow(dead_code)]
pub trait PokemonType {
    fn get_base(&self) -> PokemonBase;
    fn get_base_stat(&self, stat: &Stat) -> i16;
    fn get_evolution_level(&self) -> Option<i8>;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Pokemon {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Wooper,
    Foliwli,
}

impl PokemonType for Pokemon {
    fn get_base(&self) -> PokemonBase {
        match self {
            Pokemon::Bulbasaur => PokemonBase {
                translation_path: PokemonTranslation::new("pokemon/bulbasaur/"),
                pkmn: Pokemon::Bulbasaur,
                pokedex_info: PokedexInfo {
                    height: 0.7,
                    weight: 6.9,
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
                    hability: Some(Ability::Chlorophyll),
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
                learned_moves: vec![Box::new(LevelUpMove {
                    base: Move::Tackle,
                    level: 1,
                })],
                evolution: Some(Box::new(LevelUpEvolution {
                    evolution: Pokemon::Ivysaur,
                    level: 16,
                })),
                tm_moves: vec![],
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 64,
            },
            Pokemon::Ivysaur => PokemonBase {
                translation_path: PokemonTranslation::new("pokemon/ivysaur/"),
                pkmn: Pokemon::Ivysaur,
                pokedex_info: PokedexInfo {
                    height: 1.0,
                    weight: 13.0,
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
                    hability: Some(Ability::Chlorophyll),
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
                learned_moves: vec![Box::new(LevelUpMove {
                    base: Move::Tackle,
                    level: 1,
                })],
                evolution: Some(Box::new(LevelUpEvolution {
                    evolution: Pokemon::Venusaur,
                    level: 32,
                })),
                tm_moves: vec![],
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 142,
            },
            Pokemon::Venusaur => PokemonBase {
                translation_path: PokemonTranslation::new("pokemon/venusaur/"),
                pkmn: Pokemon::Venusaur,
                pokedex_info: PokedexInfo {
                    height: 2.0,
                    weight: 100.0,
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
                    hability: Some(Ability::Chlorophyll),
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
                learned_moves: vec![Box::new(LevelUpMove {
                    base: Move::Tackle,
                    level: 1,
                })],
                evolution: None,
                tm_moves: vec![],
                held_items: None,
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 236,
            },
            Pokemon::Wooper => PokemonBase {
                translation_path: PokemonTranslation::new("pokemon/wooper/"),
                pkmn: Pokemon::Wooper,
                pokedex_info: PokedexInfo {
                    height: 0.4,
                    weight: 8.5,
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
                    hability: Some(Ability::Unaware),
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
                evolution: None,
                learned_moves: vec![],
                tm_moves: vec![],
                levelling_curve: LevellingCurve::MediumFast,
                catch_rate: 255,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.5),
                base_exp: 42,
            },
            Pokemon::Foliwli => PokemonBase {
                translation_path: PokemonTranslation::new("pokemon/foliwi/"),
                pkmn: Pokemon::Foliwli,
                pokedex_info: PokedexInfo {
                    height: 0.4,
                    weight: 2.5,
                },
                ev_yield: EVs {
                    health: None,
                    speed: None,
                    atk: Some(1),
                    def: None,
                    spatk: None,
                    spdef: None,
                },
                abilities: AbilitySet {
                    ability1: Ability::Overgrow,
                    ability2: Some(Ability::Chlorophyll),
                    hability: Some(Ability::Drought),
                },
                base_stats: BaseStats {
                    health: 55,
                    speed: 45,
                    atk: 60,
                    def: 60,
                    spatk: 50,
                    spdef: 50,
                },
                types: TypeSet {
                    type1: Type::Grass,
                    type2: None,
                    egg_group1: EggGroup::Grass,
                    egg_group2: Some(EggGroup::Field),
                },
                held_items: None,
                evolution: None,
                learned_moves: vec![],
                tm_moves: vec![],
                levelling_curve: LevellingCurve::MediumSlow,
                catch_rate: 45,
                base_friendship: 50,
                egg_cycles: 20,
                gender_ratio: Some(0.875),
                base_exp: 84,
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

    fn get_evolution_level(&self) -> Option<i8> {
        let evo_opt: Option<Box<dyn Evolution + Send>> = Pokemon::get_base(self).evolution;
        if evo_opt.is_none() {
            None
        } else {
            let evo_potential: Option<&LevelUpEvolution> = evo_opt
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<LevelUpEvolution>();

            evo_potential.map(|value| value.get_level()) // thanks clippy
        }
    }
}
