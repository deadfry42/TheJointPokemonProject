use crate::i18n::{
    important::*,
    sections::{abilities::*, moves::*, natures::*, pokemon::*},
};

pub const LOCALISATION: Localisation = Localisation {
    code_name: "en_GB",
    name: "English (UK)",

    moves: MoveTranslationData {
        tackle: MoveI18n {
            name: "Tackle",
            desc: "Test",
        },
        growl: MoveI18n {
            name: "Growl",
            desc: "Test",
        },
    },

    pokemon: PokemonTranslationData {
        bulbasaur: PokemonI18n {
            name: "Bulbasaur",
            dex: "Test",
        },
        ivysaur: PokemonI18n {
            name: "Ivysaur",
            dex: "Test",
        },
        venusaur: PokemonI18n {
            name: "Venusaur",
            dex: "Test",
        },
        wooper: PokemonI18n {
            name: "Wooper",
            dex: "Test",
        },
    },

    nature: NatureTranslationData {
        hardy: NatureI18n { name: "Hardy" },
        lonely: NatureI18n { name: "Lonely" },
        brave: NatureI18n { name: "Brave" },
        adamant: NatureI18n { name: "Adamant" },
        naughty: NatureI18n { name: "Naughty" },
        bold: NatureI18n { name: "Bold" },
        docile: NatureI18n { name: "Docile" },
        relaxed: NatureI18n { name: "Relaxed" },
        impish: NatureI18n { name: "Impish" },
        lax: NatureI18n { name: "Lax" },
        timid: NatureI18n { name: "Timid" },
        hasty: NatureI18n { name: "Hasty" },
        serious: NatureI18n { name: "Serious" },
        jolly: NatureI18n { name: "Jolly" },
        naive: NatureI18n { name: "Naive" },
        modest: NatureI18n { name: "Modest" },
        mild: NatureI18n { name: "Mild" },
        quiet: NatureI18n { name: "Quiet" },
        bashful: NatureI18n { name: "Bashful" },
        rash: NatureI18n { name: "Rash" },
        calm: NatureI18n { name: "Calm" },
        gentle: NatureI18n { name: "Gentle" },
        sassy: NatureI18n { name: "Sassy" },
        careful: NatureI18n { name: "Careful" },
        quirky: NatureI18n { name: "Quirky" },
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
};
