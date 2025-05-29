use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
mod pkmncore;

use pkmncore::pokemon::*;

fn main() {
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );

    let wooper = Pokemon {
        name: "Wooper",
        pokedex: PokedexInfo {
            index: 0,
            height: 0.4,
            weight: 8.5,
            species: "Water Fish",
            entry: "This pokemon is gay",
        },
        evyeild: EVYield {
            health: Some(1),
            speed: None,
            atk: None,
            def: None,
            spatk: None,
            spdef: None,
        },
        abilities: AbilitySet {
            ability1: Ability::Damp,
            ability2: Some(Ability::WaterAbsorb),
            hability: Ability::Unaware,
        },
        base_stats: BaseStats {
            health: 55,
            atk: 45,
            def: 45,
            spatk: 25,
            spdef: 25,
            speed: 15,
        },
        types: TypeSet {
            type1: Type::Water,
            type2: Some(Type::Ground),
            egg_group1: EggGroup::Field,
            egg_group2: Some(EggGroup::Water1),
        },
        levelling_curve: LevellingCurve::MediumFast,
        catch_rate: 255,
        base_friendship: 50,
        egg_cycles: 20,
        gender_ratio: Some(0.5),
        base_exp: 42,
    };

    println!("{}", wooper.base_stats.atk);
    println!("{}", wooper.summarise());
    println!("{}", wooper.get_gender(0.5));
    println!("{}", wooper.types.egg_group1)
}
