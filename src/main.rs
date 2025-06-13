use lazy_static::lazy_static;
use pkmncore::constants::pokemon::*;
use std::sync::Mutex;

use crate::assetcore::assets::assets_available;
use crate::assetcore::gamedata::GameData;
use crate::i18ncore::loaded::LoadedLocales;
use crate::i18ncore::parsing::*;
use crate::pkmncore::constants::enums::*;
use crate::pkmncore::constants::items::*;
use crate::pkmncore::constants::moves::MoveType;

use crate::pkmncore::boxes::pc::*;
use crate::pkmncore::pokedex::Pokedex;
use crate::pkmncore::rng::*;
use crate::pkmncore::trainer::*;

use crate::sdlcore::window::run_window;
use crate::utils::logger::Logger;
use crate::utils::*;

extern crate lazy_static;

pub mod assetcore;
pub mod i18ncore;
pub mod pkmncore;
pub mod sdlcore;
pub mod utils;

#[allow(dead_code)]
const GAME_VERSION: &str = "v0.0-beta";

lazy_static! {
    pub static ref GAME_DATA: Mutex<GameData> = Mutex::new(GameData {
        loaded_locales: LoadedLocales::new(),
        dex: Pokedex::create_regional_dex(),
    });
}

pub fn get_game_data() -> Option<std::sync::MutexGuard<'static, GameData>> {
    // TODO: verify this is a good way to do this :)
    Some(GAME_DATA.lock().unwrap())
}

fn main() {
    if !assets_available() {
        Logger::error_literal("The game could not find an assets folder!");
        Logger::log_literal("Exiting game...");
        return;
    }

    load_localisation();

    run_window();

    get_game_data()
        .unwrap()
        .loaded_locales
        .set_selected_locale("en_lolcat");

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

    let wooper = generate_wild_pokemon(Pokemon::Foliwli, 5, &plr);

    println!(
        "name: {}",
        wooper
            .base
            .get_base()
            .translation_path
            .get_name()
            .convert_to_string()
    );
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
    println!("helditem: {}", wooper.helditem.unwrap_or(Item::LuckyEgg));
    println!(
        "status: {}",
        wooper.condition.unwrap_or(StatusCondition::BadlyPoison)
    );
    println!("pokerus: {}", wooper.pokerus);
    println!("nature: {}", wooper.nature);
    println!("shiny: {}", wooper.shiny);
    println!("ability: {}", wooper.ability);

    println!(
        "evolevel: {}",
        wooper.base.get_evolution_level().unwrap_or(0)
    );

    println!("moves: ");
    for item in wooper.moves.into_iter().enumerate() {
        if item.1.is_some() {
            let move_data = item.1.as_ref().unwrap();
            println!(
                "   {} : {}/{}",
                move_data
                    .base
                    .get_base()
                    .translation_path
                    .get_name()
                    .convert_to_string(),
                move_data.pp,
                move_data.get_max_pp()
            );
            println!(
                "      --> {}",
                move_data
                    .base
                    .get_base()
                    .translation_path
                    .get_description()
                    .convert_to_string()
            )
        } else {
            println!("   -");
        }
    }

    // std::thread::sleep(std::time::Duration::from_secs(100));
}
