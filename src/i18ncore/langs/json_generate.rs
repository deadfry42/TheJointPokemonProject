use crate::{
    assetcore::assets::get_asset_folder,
    i18ncore::{
        langs::en_GB,
        localisation::*,
        sections::{
            abilities::{AbilityLocale, AbilityLocaleContainer},
            enums::{GenderLocale, OtherLanguageLocale, StatLocale, TypesLocale},
            items::{ItemLocale, ItemLocaleContainer},
            moves::{MoveLocale, MoveLocaleContainer},
            natures::NatureLocale,
            pokemon::{PokemonLocale, PokemonLocaleContainer},
        },
    },
    utils::logger::Logger,
};
use serde_json::Value;
use std::fs;
use std::io::Result;

pub fn parse_json_files() -> Result<Vec<Locale>> {
    let compatible_versions: Vec<i64> = vec![1];
    let recommended_version: i64 = 1;

    let paths = get_asset_folder("localisation")?;
    let mut locales: Vec<Locale> = vec![];
    for path in paths {
        let data: String = fs::read_to_string(path.as_ref().unwrap().path()).unwrap();
        let v: &'static mut Value = Box::leak(Box::new(serde_json::from_str::<Value>(&data)?));

        let code_name = v["code_name"].as_str();
        let name = v["name"].as_str();
        let file_ver = v["version"].as_i64();

        if code_name.is_none() || name.is_none() || file_ver.is_none() {
            // ignore locale files without "name" and/or "code_name" values
            Logger::warn(format!(
                "Locale file {} is invalid, and will not be loaded!",
                path.as_ref().unwrap().path().display()
            ));
            continue;
        }

        if compatible_versions.contains(&file_ver.unwrap()) {
            if file_ver.unwrap() != recommended_version {
                Logger::warn(format!(
                    "Locale file {}'s version (v{}) is compatible, but out of date! Consider refactoring it!",
                    path.as_ref().unwrap().path().display(),
                    file_ver.unwrap()
                ));
            }
        } else {
            Logger::warn(format!(
                "Locale file {}'s version (v{}) is incompatible with this version, and will not be loaded!",
                path.as_ref().unwrap().path().display(),
                file_ver.unwrap()
            ));
            continue;
        }

        let locale = Locale {
            code_name: SingleValuedData::new(code_name.unwrap()),
            name: SingleValuedData::new(name.unwrap()),

            pokemon: PokemonLocaleContainer {
                bulbasaur: PokemonLocale {
                    name: v["pokemon"]["bulbasaur"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.name),
                    dex: v["pokemon"]["bulbasaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.dex),
                    species: v["pokemon"]["bulbasaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.bulbasaur.species),
                },
                ivysaur: PokemonLocale {
                    name: v["pokemon"]["ivysaur"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.name),
                    dex: v["pokemon"]["ivysaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.dex),
                    species: v["pokemon"]["ivysaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.species),
                },
                venusaur: PokemonLocale {
                    name: v["pokemon"]["venusaur"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.name),
                    dex: v["pokemon"]["venusaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.dex),
                    species: v["pokemon"]["venusaur"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.ivysaur.species),
                },
                wooper: PokemonLocale {
                    name: v["pokemon"]["wooper"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.wooper.name),
                    dex: v["pokemon"]["wooper"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.wooper.dex),
                    species: v["pokemon"]["wooper"]["dex"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.pokemon.wooper.species),
                },
            },

            moves: MoveLocaleContainer {
                tackle: MoveLocale {
                    name: v["moves"]["tackle"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.tackle.name),
                    desc: v["moves"]["tackle"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.tackle.desc),
                },
                growl: MoveLocale {
                    name: v["moves"]["growl"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.growl.name),
                    desc: v["moves"]["growl"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.growl.desc),
                },
            },

            nature: NatureLocale {
                hardy: v["natures"]["hardy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.hardy),
                lonely: v["natures"]["lonely"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.lonely),
                brave: v["natures"]["brave"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.brave),
                adamant: v["natures"]["adamant"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.adamant),
                naughty: v["natures"]["naughty"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.naughty),
                bold: v["natures"]["bold"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.bold),
                docile: v["natures"]["docile"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.docile),
                relaxed: v["natures"]["relaxed"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.relaxed),
                impish: v["natures"]["impish"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.impish),
                lax: v["natures"]["lax"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.lax),
                timid: v["natures"]["timid"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.timid),
                hasty: v["natures"]["hasty"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.hasty),
                serious: v["natures"]["serious"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.serious),
                jolly: v["natures"]["jolly"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.jolly),
                naive: v["natures"]["naive"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.naive),
                modest: v["natures"]["modest"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.modest),
                mild: v["natures"]["mild"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.mild),
                quiet: v["natures"]["quiet"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.quiet),
                bashful: v["natures"]["bashful"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.bashful),
                rash: v["natures"]["rash"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.rash),
                calm: v["natures"]["calm"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.calm),
                gentle: v["natures"]["gentle"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.gentle),
                sassy: v["natures"]["sassy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.sassy),
                careful: v["natures"]["careful"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.careful),
                quirky: v["natures"]["quirky"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.quirky),
            },

            abilities: AbilityLocaleContainer {
                damp: AbilityLocale {
                    name: v["abilities"]["damp"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.damp.name),
                    desc: v["abilities"]["damp"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.damp.desc),
                },
                water_absorb: AbilityLocale {
                    name: v["abilities"]["water_absorb"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.water_absorb.name),
                    desc: v["abilities"]["water_absorb"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.water_absorb.desc),
                },
                unaware: AbilityLocale {
                    name: v["abilities"]["unaware"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.unaware.name),
                    desc: v["abilities"]["unaware"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.unaware.name),
                },
                overgrow: AbilityLocale {
                    name: v["abilities"]["overgrow"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.overgrow.name),
                    desc: v["abilities"]["overgrow"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.overgrow.name),
                },
                chlorophyll: AbilityLocale {
                    name: v["abilities"]["chlorophyll"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.chlorophyll.name),
                    desc: v["abilities"]["chlorophyll"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.abilities.chlorophyll.name),
                },
            },

            gender: GenderLocale {
                male: v["genders"]["male"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.male),
                female: v["genders"]["female"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.female),
                unknown: v["genders"]["unknown"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.unknown),
            },

            other_langs: OtherLanguageLocale {
                english: v["other_langs"]["english"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.other_langs.english),
            },

            items: ItemLocaleContainer {
                lucky_egg: ItemLocale {
                    name: v["items"]["lucky_egg"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.items.lucky_egg.name),
                    desc: v["items"]["lucky_egg"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.items.lucky_egg.desc),
                },
                connection_wire: ItemLocale {
                    name: v["items"]["connection_wire"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.items.connection_wire.name),
                    desc: v["items"]["connection_wire"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.items.connection_wire.desc),
                },
            },

            stats: StatLocale {
                health: v["stats"]["health"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.health),
                speed: v["stats"]["speed"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.speed),
                attack: v["stats"]["attack"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.attack),
                defense: v["stats"]["defense"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.defense),
                special_attack: v["stats"]["special_attack"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.special_attack),
                special_defense: v["stats"]["special_defense"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stats.special_defense),
            },

            types: TypesLocale {
                normal: v["types"]["normal"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.normal),
                water: v["types"]["water"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.water),
                fire: v["types"]["fire"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fire),
                grass: v["types"]["grass"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.grass),
                psychic: v["types"]["psychic"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.psychic),
                ground: v["types"]["ground"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ground),
                rock: v["types"]["rock"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.rock),
                bug: v["types"]["bug"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.bug),
                steel: v["types"]["steel"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.steel),
                dark: v["types"]["dark"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.dark),
                ice: v["types"]["ice"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ice),
                dragon: v["types"]["dragon"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.dragon),
                fairy: v["types"]["fairy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fairy),
                flying: v["types"]["flying"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.flying),
                ghost: v["types"]["ghost"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ghost),
                fighting: v["types"]["fighting"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fighting),
                poison: v["types"]["poison"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.poison),
                electric: v["types"]["electric"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.electric),
            },
        };

        locales.push(locale)
    }
    Ok(locales)
}
