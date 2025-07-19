use std::time::{Duration, Instant};

use native_dialog::{DialogBuilder, MessageLevel};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{
    GAME_TITLE, GAME_VERBOSITY,
    gamecore::load::{can_run, try_load},
    pkmncore::{
        boxes::pc::PC,
        constants::{enums::*, items::*, moves::*, pokemon::*},
        rng::*,
        trainer::*,
    },
    sdlcore::window::GameWindow,
    utils::{hex, logger::Logger, strings::concatenate_strings},
};

pub fn play() {
    Logger::debug_log_literal("Checking if game can be ran..");
    if can_run() {
        Logger::debug_log_literal("Game successfully passed pre-run checks.");
        try_load();
        // TODO:
        // figure out if rust-sdl2 has a way to detect if sdl2 crashes the game
        // cuz a display output isnt available
        // and then show an error

        let builder_timer = Instant::now();

        let game_window_builder: Result<GameWindow, String> = GameWindow::new();

        if game_window_builder.is_err() {
            display_window_error();
            return;
        }

        if GAME_VERBOSITY {
            let elapsed_time = builder_timer.elapsed();
            Logger::log(format!(
                "Successfully built game window (took {}ms)",
                elapsed_time.as_millis()
            ));
        }

        let mut game_window = game_window_builder.unwrap();

        game_window
            .renderer
            .canvas
            .set_draw_color(Color::RGB(0, 0, 0));
        game_window.renderer.canvas.clear();
        game_window.renderer.canvas.present();
        let mut event_pump = game_window.sdl_context.event_pump().unwrap();
        let mut frame = 0;
        'running: loop {
            frame = (frame + 1) % 255;
            game_window
                .renderer
                .canvas
                .set_draw_color(Color::RGB(frame, 0, 0));
            game_window.renderer.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            // canvas.copy(&texture, None, None).unwrap();
            game_window.renderer.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
        }
    }
}

pub fn display_window_error() {
    Logger::warn_literal("Displaying window error.");
    DialogBuilder::message()
        .set_level(MessageLevel::Error)
        .set_title(concatenate_strings(GAME_TITLE, " Error"))
        .set_text("The game encountered an error initialising the window!\nThe game may be build incorrectly, or SDL may be encountering an error!\nCheck stdout for more information.")
        .alert()
        .show()
        .unwrap();
}

pub fn test_pkmn_engine() {
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
