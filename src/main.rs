use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use pkmncore::{
    constants::{
        enums::{Gender, Language, StatusCondition},
        items::Item,
        levels::*,
        moves::MoveType,
        pokemon::*,
    },
    rng::*,
    trainer::*,
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

    let plr = Player {
        trainer: Trainer {
            info: OTInformation {
                sid: generate_trainer_id(),
                id: generate_trainer_id(),
                lang: Language::English,
                gender: Gender::Male,
                name: "Yes",
            },
        },
        money: 0,
        party: [None, None, None, None, None, None],
    };

    let wooper = generate_wild_pokemon(Pokemon::Wooper, 69, &plr);

    println!(
        "lvl {} ({} exp)",
        wooper
            .base
            .get_base()
            .levelling_curve
            .exp_to_levels(wooper.exp),
        wooper.exp,
    );
    println!(
        "Personality: 0x{} (0b{})",
        hex::decimal_to_hex(wooper.pid),
        hex::decimal_to_binary(wooper.pid)
    );
    // println!(
    //     "mettime: {:?} ({})",
    //     chrono::Utc.timestamp_opt(wooper.mettime, 0).unwrap(),
    //     wooper.mettime
    // );
    // println!(
    //     "pokeball: {}",
    //     wooper.pokeball.unwrap_or(Pokeball::Pokeball)
    // );
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
                item.1.as_ref().unwrap().base.get_base().name,
                item.1.as_ref().unwrap().pp,
                item.1.as_ref().unwrap().base.get_base().move_pp
            );
        } else {
            println!("   -");
        }
    }
}
