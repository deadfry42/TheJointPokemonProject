use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
mod pkmncore;

use pkmncore::{
    constants::{
        abilities::Ability,
        enums::{Gender, Language, Pokeball},
        items::Item,
        levels::*,
        moves::{Move, MoveType},
        pokemon::*,
    },
    moves::MoveData,
    pokemon::{EVs, IVs, PokemonData},
    trainer::OTInformation,
};

const GAME_VERSION: &'static str = "v0.0-beta";

fn main() {
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );

    let wooper = PokemonData {
        nickname: Some("Sigma"),
        ot: OTInformation {
            id: 69420,
            sid: 96420,
            lang: Language::English,
            gender: Gender::Female,
            name: "Sigma",
        },
        base: Pokemon::Wooper.get_base(),
        evs: EVs {
            health: None,
            speed: None,
            atk: None,
            def: None,
            spatk: None,
            spdef: None,
        },
        ivs: IVs {
            health: 0,
            speed: 0,
            atk: 0,
            def: 0,
            spatk: 0,
            spdef: 0,
        },
        moves: [
            Some(MoveData {
                base: Move::Tackle.get_base(),
                pp: 0,
                pp_ups_used: 0,
            }),
            None,
            None,
            None,
        ],
        mettime: 1000,
        exp: 69420,
        ability: Ability::Chlorophyll,
        pid: 1,
        isegg: false,
        friendship: 100,
        pokeball: Pokeball::Masterball,
        pokerus: true,
        marking: None,
        condition: None,
        helditem: Some(Item::LuckyEgg),
    };

    println!("{}", wooper.mettime);
    println!("{}", LevellingCurve::Erratic.levels_to_min_exp(7));
    println!("{}", LevellingCurve::Erratic.exp_to_levels(637));
}
