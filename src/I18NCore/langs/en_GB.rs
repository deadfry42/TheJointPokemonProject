use crate::I18NCore::{
    localisation::*,
    sections::{abilities::*, enums::*, items::*, moves::*, natures::*, pokemon::*},
};

pub const LOCALISATION: Localisation = Localisation {
    code_name: SingleValued { value: "en_GB" },
    name: SingleValued {
        value: "English (UK, built-in)",
    },

    moves: MoveTranslationData {
        tackle: MoveI18n {
            name: "Tackle",
            desc: "A physical attack in which the user charges and slams into the target with its whole body.",
        },
        growl: MoveI18n {
            name: "Growl",
            desc: "The user growls in an endearing way, making opposing Pokémon less wary. This lowers their Attack stats.",
        },
    },

    pokemon: PokemonTranslationData {
        bulbasaur: PokemonI18n {
            name: "Bulbasaur",
            species: "Seed",
            dex: "Test",
        },
        ivysaur: PokemonI18n {
            name: "Ivysaur",
            species: "Seed",
            dex: "Test",
        },
        venusaur: PokemonI18n {
            name: "Venusaur",
            species: "Seed",
            dex: "Test",
        },
        wooper: PokemonI18n {
            name: "Wooper",
            species: "Water Fish",
            dex: "Test",
        },
    },

    nature: NatureTranslationData {
        hardy: "Hardy",
        lonely: "Lonely",
        brave: "Brave",
        adamant: "Adamant",
        naughty: "Naughty",
        bold: "Bold",
        docile: "Docile",
        relaxed: "Relaxed",
        impish: "Impish",
        lax: "Lax",
        timid: "Timid",
        hasty: "Hasty",
        serious: "Serious",
        jolly: "Jolly",
        naive: "Naive",
        modest: "Modest",
        mild: "Mild",
        quiet: "Quiet",
        bashful: "Bashful",
        rash: "Rash",
        calm: "Calm",
        gentle: "Gentle",
        sassy: "Sassy",
        careful: "Careful",
        quirky: "Quirky",
    },

    abilities: AbilityTranslationData {
        damp: AbilityI18n {
            name: "Damp",
            desc: "Test",
        },
        water_absorb: AbilityI18n {
            name: "WaterAbsorb",
            desc: "Test",
        },
        unaware: AbilityI18n {
            name: "Unaware",
            desc: "Test",
        },
        overgrow: AbilityI18n {
            name: "Overgrow",
            desc: "Test",
        },
        chlorophyll: AbilityI18n {
            name: "Chlorophyll",
            desc: "Test",
        },
    },

    gender: GenderTranslationData {
        male: "Male",
        female: "Female",
        unknown: "Unknown",
    },

    other_langs: OtherLanguageData { english: "English" },

    items: ItemTranslationData {
        lucky_egg: ItemI18n {
            name: "Lucky Egg",
            desc: "Test",
        },
        connection_wire: ItemI18n {
            name: "Connection Wire",
            desc: "Test",
        },
    },

    stats: StatTranslationData {
        health: "Health",
        speed: "Speed",
        attack: "Attack",
        defense: "Defense",
        special_attack: "Sp. Attack",
        special_defense: "Sp. Defense",
    },

    types: TypesTranslationData {
        normal: "Normal",
        water: "Water",
        fire: "Fire",
        grass: "Grass",
        psychic: "Psychic",
        ground: "Ground",
        rock: "Rock",
        bug: "Bug",
        steel: "Steel",
        dark: "Dark",
        ice: "Ice",
        dragon: "Dragon",
        fairy: "Fairy",
        flying: "Flying",
        ghost: "Ghost",
        fighting: "Fighting",
        poison: "Poison",
        electric: "Electric",
    },
};
