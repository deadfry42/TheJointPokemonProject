use super::constants::abilities::Ability;
use super::constants::levels::LevellingCurveCalc;
use super::constants::moves::{Move, MoveType};
use super::constants::natures::*;
use super::constants::pokemon::{Pokemon, PokemonType};
use super::moves::MoveData;
use super::{pokemon::*, trainer::OTInformation};
use rand::{Rng, RngCore};

use chrono;

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
    Nature::index_nature((personality % 25) as i8)
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
        mettime: chrono::Utc::now().timestamp(), // TODO
        helditem: None,                          // TODO
        pokerus: false,                          // TODO
        moves: [
            // TODO
            Some(MoveData {
                base: Move::Tackle.get_base(),
                pp: Move::Tackle.get_base().move_pp,
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
