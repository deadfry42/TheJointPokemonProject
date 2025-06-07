use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use pkmncore::{
    boxes::pc::*,
    constants::{enums::*, items::Item, moves::MoveType, pokemon::*},
    rng::*,
    trainer::*,
};
use utils::hex;

use crate::files::assets::check_for_assets;
use crate::i18n::parsing::*;

pub mod files;
pub mod i18n;
pub mod pkmncore;
pub mod utils;

#[allow(dead_code)]
const GAME_VERSION: &'static str = "v0.0-beta";

fn main() {
    check_for_assets();

    load_localisation();

    // let mut file: std::fs::File = get_asset("localisation/en_GB.json").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();

    // let json = serde_json::from_str(&data).unwrap();
    // println!("{}", json.find_path(&["Address", "Street"]).unwrap());

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
        pc: PC::new_dummy(),
        money: 0,
        party: [None, None, None, None, None, None],
    };

    let wooper = generate_wild_pokemon(Pokemon::Ivysaur, 69, &plr);

    println!("health evs: {}", wooper.get_ev(&Stat::Health));
    println!("health ivs: {}", wooper.get_iv(&Stat::Health));
    println!("nature: {}", wooper.nature);
    println!("health stat: {}", wooper.calculate_stat(&Stat::Health));
    println!("lvl {} ({} exp)", wooper.get_level(), wooper.exp,);
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

    println!("evolevel: {}", wooper.base.get_evolution_level().unwrap());

    println!("moves: ");
    for item in wooper.moves.into_iter().enumerate() {
        if item.1.is_some() {
            let move_data = item.1.as_ref().unwrap();
            println!(
                "   {} : {}/{}",
                move_data.base.get_base().name.convert_to_string(),
                move_data.pp,
                move_data.get_max_pp()
            );
            println!(
                "      --> {}",
                move_data.base.get_base().desc.convert_to_string()
            )
        } else {
            println!("   -");
        }
    }

    // std::thread::sleep(std::time::Duration::from_secs(100));
}
