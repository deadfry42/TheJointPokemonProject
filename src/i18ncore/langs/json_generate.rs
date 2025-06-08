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
                    name: v["move"]["tackle"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.tackle.name),
                    desc: v["move"]["tackle"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.tackle.desc),
                },
                growl: MoveLocale {
                    name: v["move"]["growl"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.growl.name),
                    desc: v["move"]["growl"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.moves.growl.desc),
                },
            },

            nature: NatureLocale {
                hardy: v["nature"]["hardy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.hardy),
                lonely: v["nature"]["lonely"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.lonely),
                brave: v["nature"]["brave"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.brave),
                adamant: v["nature"]["adamant"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.adamant),
                naughty: v["nature"]["naughty"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.naughty),
                bold: v["nature"]["bold"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.bold),
                docile: v["nature"]["docile"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.docile),
                relaxed: v["nature"]["relaxed"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.relaxed),
                impish: v["nature"]["impish"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.impish),
                lax: v["nature"]["lax"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.lax),
                timid: v["nature"]["timid"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.timid),
                hasty: v["nature"]["hasty"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.hasty),
                serious: v["nature"]["serious"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.serious),
                jolly: v["nature"]["jolly"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.jolly),
                naive: v["nature"]["naive"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.naive),
                modest: v["nature"]["modest"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.modest),
                mild: v["nature"]["mild"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.mild),
                quiet: v["nature"]["quiet"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.quiet),
                bashful: v["nature"]["bashful"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.bashful),
                rash: v["nature"]["rash"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.rash),
                calm: v["nature"]["calm"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.calm),
                gentle: v["nature"]["gentle"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.gentle),
                sassy: v["nature"]["sassy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.sassy),
                careful: v["nature"]["careful"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.careful),
                quirky: v["nature"]["quirky"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.nature.quirky),
            },

            ability: AbilityLocaleContainer {
                damp: AbilityLocale {
                    name: v["ability"]["damp"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.damp.name),
                    desc: v["ability"]["damp"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.damp.desc),
                },
                water_absorb: AbilityLocale {
                    name: v["ability"]["water_absorb"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.water_absorb.name),
                    desc: v["ability"]["water_absorb"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.water_absorb.desc),
                },
                unaware: AbilityLocale {
                    name: v["ability"]["unaware"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.unaware.name),
                    desc: v["ability"]["unaware"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.unaware.name),
                },
                overgrow: AbilityLocale {
                    name: v["ability"]["overgrow"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.overgrow.name),
                    desc: v["ability"]["overgrow"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.overgrow.name),
                },
                chlorophyll: AbilityLocale {
                    name: v["ability"]["chlorophyll"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.chlorophyll.name),
                    desc: v["ability"]["chlorophyll"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.ability.chlorophyll.name),
                },
            },

            gender: GenderLocale {
                male: v["gender"]["male"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.male),
                female: v["gender"]["female"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.female),
                unknown: v["gender"]["unknown"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.gender.unknown),
            },

            other_langs: OtherLanguageLocale {
                english: v["other_langs"]["english"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.other_langs.english),
            },

            item: ItemLocaleContainer {
                lucky_egg: ItemLocale {
                    name: v["item"]["lucky_egg"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.item.lucky_egg.name),
                    desc: v["item"]["lucky_egg"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.item.lucky_egg.desc),
                },
                connection_wire: ItemLocale {
                    name: v["item"]["connection_wire"]["name"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.item.connection_wire.name),
                    desc: v["item"]["connection_wire"]["desc"]
                        .as_str()
                        .unwrap_or(en_GB::LOCALISATION.item.connection_wire.desc),
                },
            },

            stat: StatLocale {
                health: v["stat"]["health"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.health),
                speed: v["stat"]["speed"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.speed),
                attack: v["stat"]["attack"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.attack),
                defense: v["stat"]["defense"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.defense),
                special_attack: v["stat"]["special_attack"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.special_attack),
                special_defense: v["stat"]["special_defense"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.stat.special_defense),
            },

            types: TypesLocale {
                normal: v["type"]["normal"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.normal),
                water: v["type"]["water"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.water),
                fire: v["type"]["fire"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fire),
                grass: v["type"]["grass"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.grass),
                psychic: v["type"]["psychic"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.psychic),
                ground: v["type"]["ground"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ground),
                rock: v["type"]["rock"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.rock),
                bug: v["type"]["bug"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.bug),
                steel: v["type"]["steel"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.steel),
                dark: v["type"]["dark"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.dark),
                ice: v["type"]["ice"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ice),
                dragon: v["type"]["dragon"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.dragon),
                fairy: v["type"]["fairy"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fairy),
                flying: v["type"]["flying"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.flying),
                ghost: v["type"]["ghost"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.ghost),
                fighting: v["type"]["fighting"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.fighting),
                poison: v["type"]["poison"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.poison),
                electric: v["type"]["electric"]
                    .as_str()
                    .unwrap_or(en_GB::LOCALISATION.types.electric),
            },
        };

        locales.push(locale)
    }
    Ok(locales)
}
