use crate::{
    files::assets::get_asset_folder,
    i18n::{
        localisation::*,
        sections::{abilities::*, enums::*, items::*, moves::*, natures::*, pokemon::*},
    },
};
use serde_json::Value;
use std::fs;

pub fn get_json_value<'a>(json: &'a Value, path: &str) -> Option<&'a str> {
    let mut split = path.split('/');

    let mut itr = json;

    for section in split {
        itr = &itr[section]
    }

    let string = itr.as_str();
    if string.is_none() {
        return None;
    }

    Some(&string.unwrap()[..])
}

pub fn parse_json_files<'b>() -> std::io::Result<Vec<Localisation>> {
    let paths = get_asset_folder("localisation")?;
    let mut locales: Vec<Localisation> = vec![];
    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());
        let data = fs::read_to_string(&path.unwrap().path()).unwrap();
        let v: Value = serde_json::from_str(&data)?;

        locales.push(Localisation {
            code_name: get_json_value(&v, "code_name").unwrap_or("Unknown"),
            name: get_json_value(&v, "name").unwrap_or("Unknown"),

            pokemon: PokemonTranslationData {
                bulbasaur: PokemonI18n {
                    name: get_json_value(&v, "pokemon/bulbasaur/name").unwrap_or("Bulbasaur"),
                    dex: get_json_value(&v, "pokemon/bulbasaur/dex").unwrap_or("Unknown"),
                    species: get_json_value(&v, "pokemon/bulbasaur/dex").unwrap_or("Seed"),
                },
                ivysaur: PokemonI18n {
                    name: get_json_value(&v, "pokemon/ivysaur/name").unwrap_or("Ivysaur"),
                    dex: get_json_value(&v, "pokemon/ivysaur/dex").unwrap_or("Unknown"),
                    species: get_json_value(&v, "pokemon/ivysaur/dex").unwrap_or("Seed"),
                },
                venusaur: PokemonI18n {
                    name: get_json_value(&v, "pokemon/venusaur/name").unwrap_or("Venusaur"),
                    dex: get_json_value(&v, "pokemon/venusaur/dex").unwrap_or("Unknown"),
                    species: get_json_value(&v, "pokemon/venusaur/dex").unwrap_or("Seed"),
                },
                wooper: PokemonI18n {
                    name: get_json_value(&v, "pokemon/wooper/name").unwrap_or("Wooper"),
                    dex: get_json_value(&v, "pokemon/wooper/dex").unwrap_or("Unknown"),
                    species: get_json_value(&v, "pokemon/wooper/dex").unwrap_or("Water Fish"),
                },
            },

            moves: MoveTranslationData {
                tackle: MoveI18n {
                    name: get_json_value(&v, "moves/tackle/name").unwrap_or("Tackle"),
                    desc: get_json_value(&v, "moves/tackle/desc").unwrap_or("A physical attack in which the user charges and slams into the target with its whole body."),
                },
                growl: MoveI18n {
                    name: get_json_value(&v, "moves/growl/name").unwrap_or("Growl"),
                    desc: get_json_value(&v, "moves/growl/desc").unwrap_or("The user growls in an endearing way, making opposing Pokémon less wary. This lowers their Attack stats."),
                },
            },

            nature: NatureTranslationData {
                hardy: get_json_value(&v, "natures/hardy").unwrap_or("Hardy"),
                lonely: get_json_value(&v, "natures/lonely").unwrap_or("Lonely"),
                brave: get_json_value(&v, "natures/brave").unwrap_or("Brave"),
                adamant: get_json_value(&v, "natures/adamant").unwrap_or("Adamant"),
                naughty: get_json_value(&v, "natures/naughty").unwrap_or("Naughty"),
                bold: get_json_value(&v, "natures/bold").unwrap_or("Bold"),
                docile: get_json_value(&v, "natures/docile").unwrap_or("Docile"),
                relaxed: get_json_value(&v, "natures/relaxed").unwrap_or("Relaxed"),
                impish: get_json_value(&v, "natures/impish").unwrap_or("Impish"),
                lax: get_json_value(&v, "natures/lax").unwrap_or("Lax"),
                timid: get_json_value(&v, "natures/timid").unwrap_or("Timid"),
                hasty: get_json_value(&v, "natures/hasty").unwrap_or("Hasty"),
                serious: get_json_value(&v, "natures/serious").unwrap_or("Serious"),
                jolly: get_json_value(&v, "natures/jolly").unwrap_or("Jolly"),
                naive: get_json_value(&v, "natures/naive").unwrap_or("Naive"),
                modest: get_json_value(&v, "natures/modest").unwrap_or("Modest"),
                mild: get_json_value(&v, "natures/mild").unwrap_or("Mild"),
                quiet: get_json_value(&v, "natures/quiet").unwrap_or("Quiet"),
                bashful: get_json_value(&v, "natures/bashful").unwrap_or("Bashful"),
                rash: get_json_value(&v, "natures/rash").unwrap_or("Rash"),
                calm: get_json_value(&v, "natures/calm").unwrap_or("Calm"),
                gentle: get_json_value(&v, "natures/gentle").unwrap_or("Gentle"),
                sassy: get_json_value(&v, "natures/sassy").unwrap_or("Sassy"),
                careful: get_json_value(&v, "natures/careful").unwrap_or("Careful"),
                quirky: get_json_value(&v, "natures/quirky").unwrap_or("Quirky"),
            },

            abilities: AbilityTranslationData {
                damp: AbilityI18n {
                    name: get_json_value(&v, "abilities/damp/name").unwrap_or("Damp"),
                    desc: get_json_value(&v, "abilities/damp/desc").unwrap_or("Test"),
                },
                water_absorb: AbilityI18n {
                    name: get_json_value(&v, "abilities/water_absorb/name").unwrap_or("Water Absorb"),
                    desc: get_json_value(&v, "abilities/water_absorb/desc").unwrap_or("Test"),
                },
                unaware: AbilityI18n {
                    name: get_json_value(&v, "abilities/unaware/name").unwrap_or("Unaware"),
                    desc: get_json_value(&v, "abilities/unaware/desc").unwrap_or("Test"),
                },
                overgrow: AbilityI18n {
                    name: get_json_value(&v, "abilities/overgrow/name").unwrap_or("Overgrow"),
                    desc: get_json_value(&v, "abilities/overgrow/desc").unwrap_or("Test"),
                },
                chlorophyll: AbilityI18n {
                    name: get_json_value(&v, "abilities/chlorophyll/name").unwrap_or("Chlorophyll"),
                    desc: get_json_value(&v, "abilities/chlorophyll/desc").unwrap_or("Test"),
                },
            },

            gender: GenderTranslationData {
                male: get_json_value(&v, "genders/male").unwrap_or("Male"),
                female: get_json_value(&v, "genders/female").unwrap_or("Female"),
                unknown: get_json_value(&v, "genders/unknown").unwrap_or("Unknown"),
            },

            other_langs: OtherLanguageData {
                english: get_json_value(&v, "other_langs/english").unwrap_or("English"),
            },

            items: ItemTranslationData {
                lucky_egg: ItemI18n {
                    name: get_json_value(&v, "items/lucky_egg/name").unwrap_or("Lucky Egg"),
                    desc: get_json_value(&v, "items/lucky_egg/desc").unwrap_or("Test"),
                },
                connection_wire: ItemI18n {
                    name: get_json_value(&v, "items/connection_wire/name").unwrap_or("Connection Wire"),
                    desc: get_json_value(&v, "items/connection_wire/desc").unwrap_or("Test"),
                },
            },

            stats: StatTranslationData {
                health: get_json_value(&v, "stats/health").unwrap_or("Health"),
                speed: get_json_value(&v, "stats/speed").unwrap_or("Speed"),
                attack: get_json_value(&v, "stats/attack").unwrap_or("Attack"),
                defense: get_json_value(&v, "stats/defense").unwrap_or("Defense"),
                special_attack: get_json_value(&v, "stats/special_attack").unwrap_or("Sp. Attack"),
                special_defense: get_json_value(&v, "stats/special_defense").unwrap_or("Sp. Defense"),
            },

            types: TypesTranslationData {
                normal: get_json_value(&v, "types/normal").unwrap_or("Normal"),
                water: get_json_value(&v, "types/water").unwrap_or("Water"),
                fire: get_json_value(&v, "types/fire").unwrap_or("Fire"),
                grass: get_json_value(&v, "types/grass").unwrap_or("Grass"),
                psychic: get_json_value(&v, "types/psychic").unwrap_or("Psychic"),
                ground: get_json_value(&v, "types/ground").unwrap_or("Ground"),
                rock: get_json_value(&v, "types/rock").unwrap_or("Rock"),
                bug: get_json_value(&v, "types/bug").unwrap_or("Bug"),
                steel: get_json_value(&v, "types/steel").unwrap_or("Steel"),
                dark: get_json_value(&v, "types/dark").unwrap_or("Dark"),
                ice: get_json_value(&v, "types/ice").unwrap_or("Ice"),
                dragon: get_json_value(&v, "types/dragon").unwrap_or("Dragon"),
                fairy: get_json_value(&v, "types/fairy").unwrap_or("Fairy"),
                flying: get_json_value(&v, "types/flying").unwrap_or("Flying"),
                ghost: get_json_value(&v, "types/ghost").unwrap_or("Ghost"),
                fighting: get_json_value(&v, "types/fighting").unwrap_or("Fighting"),
                poison: get_json_value(&v, "types/poison").unwrap_or("Poison"),
                electric: get_json_value(&v, "types/electric").unwrap_or("Electric"),
            },
        })
    }
    Ok(locales)
}
