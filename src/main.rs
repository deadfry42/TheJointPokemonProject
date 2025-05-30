use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
mod pkmncore;

use pkmncore::constants::pokemon::*;

fn main() {
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );

    let wooper = Pokemon::Bulbasaur.get_base();

    // println!("{}", wooper.base_stats.atk);
    // println!("{}", wooper.abilities.ability1);
    // println!("{}", wooper.levelling_curve);
    println!("{}", wooper.summarise());
    // println!("{}", wooper.get_gender(0.5));
    // println!("{}", wooper.types.egg_group1)
}
