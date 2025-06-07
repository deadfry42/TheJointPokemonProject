use crate::{
    files::assets::get_asset_folder,
    i18n::{
        langs::en_GB,
        localisation::*,
        sections::{abilities::*, enums::*, items::*, moves::*, natures::*, pokemon::*},
    },
    utils::logger::Logger,
};
use serde_json::Value;
use std::fs;
use std::io::Result;

pub fn parse_json_files() -> Result<Vec<Localisation>> {
    let compatible_versions: Vec<i64> = vec![1];
    let recommended_version: i64 = 1;

    let paths = get_asset_folder("localisation")?;
    let mut locales: Vec<Localisation> = vec![];
    for path in paths {
        // println!("Name: {}", (&path).as_ref().unwrap().path().display());
        let data: String = fs::read_to_string((&path).as_ref().unwrap().path()).unwrap();
        let v: &'static mut Value = Box::leak(Box::new(serde_json::from_str::<Value>(&data)?));

        let code_name = &v["code_name"].as_str();
        let name = &v["name"].as_str();
        let file_ver = &v["version"].as_i64();

        if code_name.is_none() || name.is_none() || file_ver.is_none() {
            // ignore locale files without "name" and/or "code_name" values
            Logger::warn(format!(
                "Locale file {} is invalid, and will not be loaded!",
                (&path).as_ref().unwrap().path().display()
            ));
            continue;
        }

        if compatible_versions.contains(&file_ver.unwrap()) {
            if file_ver.unwrap() != recommended_version {
                Logger::warn(format!(
                    "Locale file {}'s version (v{}) is compatible, but out of date! Consider refactoring it!",
                    (&path).as_ref().unwrap().path().display(),
                    file_ver.unwrap()
                ));
            }
        } else {
            Logger::warn(format!(
                "Locale file {}'s version (v{}) is incompatible with this version, and will not be loaded!",
                (&path).as_ref().unwrap().path().display(),
                file_ver.unwrap()
            ));
            continue;
        }

        let locale = Localisation {
            code_name: code_name.unwrap(),
            name: name.unwrap(),

            pokemon: PokemonTranslationData {
                bulbasaur: PokemonI18n {
                    name: &v["pokemon"]["bulbasaur"]["name"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.name),
                    dex: &v["pokemon"]["bulbasaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.dex),
                    species: &v["pokemon"]["bulbasaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.species),
                },
                ivysaur: PokemonI18n {
                    name: &v["pokemon"]["ivysaur"]["name"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.name),
                    dex: &v["pokemon"]["ivysaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.dex),
                    species: &v["pokemon"]["ivysaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.species),
                },
                venusaur: PokemonI18n {
                    name: &v["pokemon"]["venusaur"]["name"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.name),
                    dex: &v["pokemon"]["venusaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.dex),
                    species: &v["pokemon"]["venusaur"]["dex"].as_str().unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.species),
                },
                wooper: PokemonI18n {
                    name: &v["pokemon"]["wooper"]["name"].as_str().unwrap_or("Wooper"),
                    dex: &v["pokemon"]["wooper"]["dex"].as_str().unwrap_or("Unknown"),
                    species: &v["pokemon"]["wooper"]["dex"].as_str().unwrap_or("Water Fish"),
                },
            },

            moves: MoveTranslationData {
                tackle: MoveI18n {
                    name: &v["moves"]["tackle"]["name"].as_str().unwrap_or("Tackle"),
                    desc: &v["moves"]["tackle"]["desc"].as_str().unwrap_or("A physical attack in which the user charges and slams into the target with its whole body."),
                },
                growl: MoveI18n {
                    name: &v["moves"]["growl"]["name"].as_str().unwrap_or("Growl"),
                    desc: &v["moves"]["growl"]["desc"].as_str().unwrap_or("The user growls in an endearing way, making opposing Pokémon less wary. This lowers their Attack stats."),
                },
            },

            nature: NatureTranslationData {
                hardy: &v["natures"]["hardy"].as_str().unwrap_or("Hardy"),
                lonely: &v["natures"]["lonely"].as_str().unwrap_or("Lonely"),
                brave: &v["natures"]["brave"].as_str().unwrap_or("Brave"),
                adamant: &v["natures"]["adamant"].as_str().unwrap_or("Adamant"),
                naughty: &v["natures"]["naughty"].as_str().unwrap_or("Naughty"),
                bold: &v["natures"]["bold"].as_str().unwrap_or("Bold"),
                docile: &v["natures"]["docile"].as_str().unwrap_or("Docile"),
                relaxed: &v["natures"]["relaxed"].as_str().unwrap_or("Relaxed"),
                impish: &v["natures"]["impish"].as_str().unwrap_or("Impish"),
                lax: &v["natures"]["lax"].as_str().unwrap_or("Lax"),
                timid: &v["natures"]["timid"].as_str().unwrap_or("Timid"),
                hasty: &v["natures"]["hasty"].as_str().unwrap_or("Hasty"),
                serious: &v["natures"]["serious"].as_str().unwrap_or("Serious"),
                jolly: &v["natures"]["jolly"].as_str().unwrap_or("Jolly"),
                naive: &v["natures"]["naive"].as_str().unwrap_or("Naive"),
                modest: &v["natures"]["modest"].as_str().unwrap_or("Modest"),
                mild: &v["natures"]["mild"].as_str().unwrap_or("Mild"),
                quiet: &v["natures"]["quiet"].as_str().unwrap_or("Quiet"),
                bashful: &v["natures"]["bashful"].as_str().unwrap_or("Bashful"),
                rash: &v["natures"]["rash"].as_str().unwrap_or("Rash"),
                calm: &v["natures"]["calm"].as_str().unwrap_or("Calm"),
                gentle: &v["natures"]["gentle"].as_str().unwrap_or("Gentle"),
                sassy: &v["natures"]["sassy"].as_str().unwrap_or("Sassy"),
                careful: &v["natures"]["careful"].as_str().unwrap_or("Careful"),
                quirky: &v["natures"]["quirky"].as_str().unwrap_or("Quirky"),
            },

            abilities: AbilityTranslationData {
                damp: AbilityI18n {
                    name: &v["abilities"]["damp"]["name"].as_str().unwrap_or("Damp"),
                    desc: &v["abilities"]["damp"]["desc"].as_str().unwrap_or("Test"),
                },
                water_absorb: AbilityI18n {
                    name: &v["abilities"]["water_absorb"]["name"].as_str().unwrap_or("Water Absorb"),
                    desc: &v["abilities"]["water_absorb"]["desc"].as_str().unwrap_or("Test"),
                },
                unaware: AbilityI18n {
                    name: &v["abilities"]["unaware"]["name"].as_str().unwrap_or("Unaware"),
                    desc: &v["abilities"]["unaware"]["desc"].as_str().unwrap_or("Test"),
                },
                overgrow: AbilityI18n {
                    name: &v["abilities"]["overgrow"]["name"].as_str().unwrap_or("Overgrow"),
                    desc: &v["abilities"]["overgrow"]["desc"].as_str().unwrap_or("Test"),
                },
                chlorophyll: AbilityI18n {
                    name: &v["abilities"]["chlorophyll"]["name"].as_str().unwrap_or("Chlorophyll"),
                    desc: &v["abilities"]["chlorophyll"]["desc"].as_str().unwrap_or("Test"),
                },
            },

            gender: GenderTranslationData {
                male: &v["genders"]["male"].as_str().unwrap_or("Male"),
                female: &v["genders"]["female"].as_str().unwrap_or("Female"),
                unknown: &v["genders"]["unknown"].as_str().unwrap_or("Unknown"),
            },

            other_langs: OtherLanguageData {
                english: &v["other_langs"]["english"].as_str().unwrap_or("English"),
            },

            items: ItemTranslationData {
                lucky_egg: ItemI18n {
                    name: &v["items"]["lucky_egg"]["name"].as_str().unwrap_or("Lucky Egg"),
                    desc: &v["items"]["lucky_egg"]["desc"].as_str().unwrap_or("Test"),
                },
                connection_wire: ItemI18n {
                    name: &v["items"]["connection_wire"]["name"].as_str().unwrap_or("Connection Wire"),
                    desc: &v["items"]["connection_wire"]["desc"].as_str().unwrap_or("Test"),
                },
            },

            stats: StatTranslationData {
                health: &v["stats"]["health"].as_str().unwrap_or("Health"),
                speed: &v["stats"]["speed"].as_str().unwrap_or("Speed"),
                attack: &v["stats"]["attack"].as_str().unwrap_or("Attack"),
                defense: &v["stats"]["defense"].as_str().unwrap_or("Defense"),
                special_attack: &v["stats"]["special_attack"].as_str().unwrap_or("Sp. Attack"),
                special_defense: &v["stats"]["special_defense"].as_str().unwrap_or("Sp. Defense"),
            },

            types: TypesTranslationData {
                normal: &v["types"]["normal"].as_str().unwrap_or("Normal"),
                water: &v["types"]["water"].as_str().unwrap_or("Water"),
                fire: &v["types"]["fire"].as_str().unwrap_or("Fire"),
                grass: &v["types"]["grass"].as_str().unwrap_or("Grass"),
                psychic: &v["types"]["psychic"].as_str().unwrap_or("Psychic"),
                ground: &v["types"]["ground"].as_str().unwrap_or("Ground"),
                rock: &v["types"]["rock"].as_str().unwrap_or("Rock"),
                bug: &v["types"]["bug"].as_str().unwrap_or("Bug"),
                steel: &v["types"]["steel"].as_str().unwrap_or("Steel"),
                dark: &v["types"]["dark"].as_str().unwrap_or("Dark"),
                ice: &v["types"]["ice"].as_str().unwrap_or("Ice"),
                dragon: &v["types"]["dragon"].as_str().unwrap_or("Dragon"),
                fairy: &v["types"]["fairy"].as_str().unwrap_or("Fairy"),
                flying: &v["types"]["flying"].as_str().unwrap_or("Flying"),
                ghost: &v["types"]["ghost"].as_str().unwrap_or("Ghost"),
                fighting: &v["types"]["fighting"].as_str().unwrap_or("Fighting"),
                poison: &v["types"]["poison"].as_str().unwrap_or("Poison"),
                electric: &v["types"]["electric"].as_str().unwrap_or("Electric"),
            },
        };

        locales.push(locale)
    }
    Ok(locales)
}
