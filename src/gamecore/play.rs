use crate::{
    gamecore::load::{can_run, try_load},
    pkmncore::{
        boxes::pc::PC,
        constants::{enums::*, items::*, moves::*, pokemon::*},
        rng::*,
        trainer::*,
    },
    run_window,
    utils::hex,
};

pub fn play() {
    if can_run() {
        try_load();
        run_window();
    }
}

pub fn test() {
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
}
