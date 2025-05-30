use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use pkmncore::{
    constants::{
        enums::{Gender, Language},
        levels::*,
        pokemon::*,
    },
    rng::*,
    trainer::OTInformation,
};

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
        "{} (lvl {})",
        wooper.exp,
        wooper.base.levelling_curve.exp_to_levels(wooper.exp)
    );
    println!("{}", wooper.nature);
    println!("{}", wooper.shiny);
}
