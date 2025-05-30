use super::constants::abilities::Ability;
use super::constants::levels::LevellingCurveCalc;
use super::constants::moves::{Move, MoveType};
use super::constants::natures::Nature;
use super::constants::pokemon::{Pokemon, PokemonType};
use super::moves::MoveData;
use super::{pokemon::*, trainer::OTInformation};
use rand::{Rng, RngCore};

pub fn generate_personality() -> u32 {
    rand::rng().next_u32()
}

pub fn generate_trainer_id() -> u16 {
    rand::rng().next_u32() as u16
}

pub fn generate_iv() -> i8 {
    rand::rng().random_range(0..31)
}

pub fn determine_nature(personality: &u32) -> Nature {
    let nature_index: i8 = (personality % 25) as i8;

    match nature_index {
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

#[allow(dead_code)]
pub fn determine_ability(personality: &u32, pkmn: &PokemonBase) -> Ability {
    let ability1: &Ability = &pkmn.abilities.ability1;
    let ability2: &Option<Ability> = &pkmn.abilities.ability2;

    if ability2.is_none() {
        ability1.clone()
    } else {
        if ((personality / 65536) % 2) == 0 {
            ability1.clone()
        } else {
            ability2.as_ref().unwrap().clone()
        }
    }
}

#[allow(dead_code)]
pub fn determine_shininess(personality: &u32, ot: &OTInformation) -> bool {
    // using bulbapedia impl. S=IDTrainer⊕IDSecret⊕p1⊕p2
    // TODO: verify this works

    let p1 = personality / 65536;
    let p2 = personality % 65536;

    let shinyval = ot.id ^ ot.sid ^ p1 as u16 ^ p2 as u16;

    shinyval < 8 // 1/8192 allegedly
    // in xy+ it's shinyval < 16, but i like the harder odds
}

#[allow(dead_code)]
pub fn generate_wild_pokemon(pkmn: Pokemon, lvl: i8, ot: &OTInformation) -> PokemonData {
    let personality = generate_personality();

    PokemonData {
        pid: personality,
        base: pkmn.get_base(),
        exp: pkmn.get_base().levelling_curve.levels_to_min_exp(lvl),
        ability: determine_ability(&personality, &pkmn.get_base()),
        shiny: determine_shininess(&personality, &ot),
        friendship: pkmn.get_base().base_friendship,
        nature: determine_nature(&personality),
        isegg: false,
        nickname: None,
        ot: None,
        pokeball: None,
        marking: None,
        condition: None,
        mettime: 0,     // TODO
        helditem: None, // TODO
        pokerus: false, // TODO
        moves: [
            // TODO
            Some(MoveData {
                base: Move::Tackle.get_base(),
                pp: 0,
                pp_ups_used: 0,
            }),
            None,
            None,
            None,
        ],
        evs: EVs {
            health: Some(0),
            speed: Some(0),
            atk: Some(0),
            def: Some(0),
            spatk: Some(0),
            spdef: Some(0),
        },
        ivs: IVs {
            health: generate_iv(),
            speed: generate_iv(),
            atk: generate_iv(),
            def: generate_iv(),
            spatk: generate_iv(),
            spdef: generate_iv(),
        },
    }
}
