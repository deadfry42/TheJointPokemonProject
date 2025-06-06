use crate::i18n::sections::{
    abilities::AbilityTranslationData, moves::MoveTranslationData, natures::NatureTranslationData,
    pokemon::PokemonTranslationData,
};

pub struct Localisation {
    pub code_name: &'static str,
    pub name: &'static str,

    pub moves: MoveTranslationData,
    pub pokemon: PokemonTranslationData,
    pub nature: NatureTranslationData,
    pub abilities: AbilityTranslationData,
}
