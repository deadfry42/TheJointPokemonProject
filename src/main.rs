use chrono::TimeZone;
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use pkmncore::{
    constants::{
        enums::{Gender, Language, Pokeball, StatusCondition},
        items::Item,
        levels::*,
        pokemon::*,
    },
    rng::*,
    trainer::OTInformation,
};
use utils::hex;

mod pkmncore;
mod utils;

#[allow(dead_code)]
const GAME_VERSION: &'static str = "v0.0-beta";

fn main() {
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );

    let wooper = generate_wild_pokemon(
        Pokemon::Wooper,
        69,
        &OTInformation {
            id: generate_trainer_id(),
            sid: generate_trainer_id(),
            lang: Language::English,
            gender: Gender::Male,
            name: "Sigma",
        },
    );

    println!(
        "lvl {} ({} exp)",
        wooper.base.levelling_curve.exp_to_levels(wooper.exp),
        wooper.exp,
    );
    println!(
        "Personality: 0x{} (0b{})",
        hex::decimal_to_hex(wooper.pid),
        hex::decimal_to_binary(wooper.pid)
    );
    println!(
        "mettime: {:?} ({})",
        chrono::Utc.timestamp_opt(wooper.mettime, 0).unwrap(),
        wooper.mettime
    );
    println!(
        "pokeball: {}",
        wooper.pokeball.unwrap_or(Pokeball::Pokeball)
    );
    println!(
        "helditem: {}",
        wooper.helditem.unwrap_or(Item::ConnectionWire)
    );
    println!(
        "status: {}",
        wooper.condition.unwrap_or(StatusCondition::BadlyPoison)
    );
    println!("pokerus: {}", wooper.pokerus);
    println!("nature: {}", wooper.nature);
    println!("shiny: {}", wooper.shiny);
    println!("ability: {}", wooper.ability);

    println!("moves: ");
    for item in wooper.moves.into_iter().enumerate() {
        if item.1.is_some() {
            println!(
                "   {} : {}/{}",
                item.1.as_ref().unwrap().base.name,
                item.1.as_ref().unwrap().pp,
                item.1.as_ref().unwrap().base.move_pp
            );
        } else {
            println!("   -");
        }
    }
}
