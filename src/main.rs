use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use pkmncore::constants::pokemon::*;

use crate::assetcore::gamedata::GameData;
use crate::i18ncore::loaded::LoadedLocales;
use crate::{assetcore::assets::check_for_assets, i18ncore::keys::TranslationKey};
use crate::{i18ncore::parsing::*, pkmncore::constants::natures::Nature};

pub mod assetcore;
pub mod i18ncore;
pub mod pkmncore;
pub mod utils;

#[allow(dead_code)]
const GAME_VERSION: &str = "v0.0-beta";

fn main() {
    let mut game: GameData = GameData {
        loaded_locales: LoadedLocales::new(),
    };

    check_for_assets();

    load_localisation(&mut game);

    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );

    println!(
        "Translated string (pokemon/bulbasaur/species): {}",
        Pokemon::Bulbasaur.get_base().name.convert_to_string()
    );
    println!("Translated string (nature/quirky): {}", Nature::Quirky);
    println!(
        "Translated string (code_name): {}",
        TranslationKey::new("code_name").convert_to_string()
    );
    println!(
        "Translated string (name): {}",
        TranslationKey::new("name").convert_to_string()
    );

    // let plr = Player {
    //     trainer: Trainer {
    //         info: OTInformation {
    //             sid: generate_trainer_id(),
    //             id: generate_trainer_id(),
    //             lang: Language::English,
    //             gender: Gender::Male,
    //             name: "Yes",
    //         },
    //     },
    //     pc: PC::new_dummy(),
    //     money: 0,
    //     party: [None, None, None, None, None, None],
    // };

    // let wooper = generate_wild_pokemon(Pokemon::Ivysaur, 69, &plr);

    // println!("health evs: {}", wooper.get_ev(&Stat::Health));
    // println!("health ivs: {}", wooper.get_iv(&Stat::Health));
    // println!("nature: {}", wooper.nature);
    // println!("health stat: {}", wooper.calculate_stat(&Stat::Health));
    // println!("lvl {} ({} exp)", wooper.get_level(), wooper.exp,);
    // println!(
    //     "Personality: 0x{} (0b{})",
    //     hex::decimal_to_hex(wooper.pid),
    //     hex::decimal_to_binary(wooper.pid)
    // );
    // // println!(
    // //     "mettime: {:?} ({})",
    // //     chrono::Utc.timestamp_opt(wooper.mettime, 0).unwrap(),
    // //     wooper.mettime
    // // );
    // // println!(
    // //     "pokeball: {}",
    // //     wooper.pokeball.unwrap_or(Pokeball::Pokeball)
    // // );
    // println!(
    //     "helditem: {}",
    //     wooper.helditem.unwrap_or(Item::ConnectionWire)
    // );
    // println!(
    //     "status: {}",
    //     wooper.condition.unwrap_or(StatusCondition::BadlyPoison)
    // );
    // println!("pokerus: {}", wooper.pokerus);
    // println!("nature: {}", wooper.nature);
    // println!("shiny: {}", wooper.shiny);
    // println!("ability: {}", wooper.ability);

    // println!("evolevel: {}", wooper.base.get_evolution_level().unwrap());

    // println!("moves: ");
    // for item in wooper.moves.into_iter().enumerate() {
    //     if item.1.is_some() {
    //         let move_data = item.1.as_ref().unwrap();
    //         println!(
    //             "   {} : {}/{}",
    //             move_data.base.get_base().name.convert_to_string(),
    //             move_data.pp,
    //             move_data.get_max_pp()
    //         );
    //         println!(
    //             "      --> {}",
    //             move_data.base.get_base().desc.convert_to_string()
    //         )
    //     } else {
    //         println!("   -");
    //     }
    // }

    // std::thread::sleep(std::time::Duration::from_secs(100));
}
