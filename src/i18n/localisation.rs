use crate::i18n::sections::{
    abilities::AbilityTranslationData,
    enums::{GenderTranslationData, OtherLanguageData, StatTranslationData, TypesTranslationData},
    items::ItemTranslationData,
    moves::MoveTranslationData,
    natures::NatureTranslationData,
    pokemon::PokemonTranslationData,
};

pub struct Localisation {
    pub code_name: &'static str,
    pub name: &'static str,

    pub moves: MoveTranslationData,
    pub pokemon: PokemonTranslationData,
    pub nature: NatureTranslationData,
    pub abilities: AbilityTranslationData,
    pub gender: GenderTranslationData,
    pub items: ItemTranslationData,
    pub stats: StatTranslationData,
    pub types: TypesTranslationData,
    pub other_langs: OtherLanguageData,
}
