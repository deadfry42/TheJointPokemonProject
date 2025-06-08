use crate::I18NCore::{
    localisation::*,
    sections::{abilities::*, enums::*, items::*, moves::*, natures::*, pokemon::*},
};

pub const LOCALISATION: Locale = Locale {
    code_name: SingleValuedData { value: "en_GB" },
    name: SingleValuedData {
        value: "English (UK, built-in)",
    },

    moves: MoveLocaleContainer {
        tackle: MoveLocale {
            name: "Tackle",
            desc: "A physical attack in which the user charges and slams into the target with its whole body.",
        },
        growl: MoveLocale {
            name: "Growl",
            desc: "The user growls in an endearing way, making opposing Pokémon less wary. This lowers their Attack stats.",
        },
    },

    pokemon: PokemonLocaleContainer {
        bulbasaur: PokemonLocale {
            name: "Bulbasaur",
            species: "Seed",
            dex: "Test",
        },
        ivysaur: PokemonLocale {
            name: "Ivysaur",
            species: "Seed",
            dex: "Test",
        },
        venusaur: PokemonLocale {
            name: "Venusaur",
            species: "Seed",
            dex: "Test",
        },
        wooper: PokemonLocale {
            name: "Wooper",
            species: "Water Fish",
            dex: "Test",
        },
    },

    nature: NatureLocale {
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

    abilities: AbilityLocaleContainer {
        damp: AbilityLocale {
            name: "Damp",
            desc: "Test",
        },
        water_absorb: AbilityLocale {
            name: "WaterAbsorb",
            desc: "Test",
        },
        unaware: AbilityLocale {
            name: "Unaware",
            desc: "Test",
        },
        overgrow: AbilityLocale {
            name: "Overgrow",
            desc: "Test",
        },
        chlorophyll: AbilityLocale {
            name: "Chlorophyll",
            desc: "Test",
        },
    },

    gender: GenderLocale {
        male: "Male",
        female: "Female",
        unknown: "Unknown",
    },

    other_langs: OtherLanguageLocale { english: "English" },

    items: ItemLocaleContainer {
        lucky_egg: ItemLocale {
            name: "Lucky Egg",
            desc: "Test",
        },
        connection_wire: ItemLocale {
            name: "Connection Wire",
            desc: "Test",
        },
    },

    stats: StatLocale {
        health: "Health",
        speed: "Speed",
        attack: "Attack",
        defense: "Defense",
        special_attack: "Sp. Attack",
        special_defense: "Sp. Defense",
    },

    types: TypesLocale {
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
