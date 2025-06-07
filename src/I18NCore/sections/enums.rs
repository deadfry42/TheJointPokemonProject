use crate::I18NCore::localisation::{I18NData, SectionData};
use std::any::Any;

pub struct GenderTranslationData {
    pub male: &'static str,
    pub female: &'static str,
    pub unknown: &'static str,
}

impl I18NData for GenderTranslationData {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "male" => &self.male,
            "female" => &self.female,
            _ => &self.unknown,
        }
    }
}
impl SectionData for GenderTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StatTranslationData {
    pub health: &'static str,
    pub speed: &'static str,
    pub attack: &'static str,
    pub defense: &'static str,
    pub special_attack: &'static str,
    pub special_defense: &'static str,
}

impl I18NData for StatTranslationData {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "health" => &self.health,
            "speed" => &self.speed,
            "attack" => &self.attack,
            "defense" => &self.defense,
            "special_attack" => &self.special_attack,
            _ => &self.special_defense,
        }
    }
}
impl SectionData for StatTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct OtherLanguageData {
    pub english: &'static str,
}

impl I18NData for OtherLanguageData {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "english" => &self.english,
            _ => &self.english,
        }
    }
}
impl SectionData for OtherLanguageData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TypesTranslationData {
    pub normal: &'static str,
    pub water: &'static str,
    pub fire: &'static str,
    pub grass: &'static str,
    pub psychic: &'static str,
    pub ground: &'static str,
    pub rock: &'static str,
    pub bug: &'static str,
    pub steel: &'static str,
    pub dark: &'static str,
    pub ice: &'static str,
    pub dragon: &'static str,
    pub fairy: &'static str,
    pub flying: &'static str,
    pub ghost: &'static str,
    pub fighting: &'static str,
    pub poison: &'static str,
    pub electric: &'static str,
}

impl I18NData for TypesTranslationData {
    fn index(&self, path: &'static str) -> &'static str {
        match path {
            "normal" => &self.normal,
            "water" => &self.water,
            "fire" => &self.fire,
            "grass" => &self.grass,
            "psychic" => &self.psychic,
            "ground" => &self.ground,
            "rock" => &self.rock,
            "bug" => &self.bug,
            "steel" => &self.steel,
            "dark" => &self.dark,
            "ice" => &self.ice,
            "dragon" => &self.dragon,
            "fairy" => &self.fairy,
            "flying" => &self.flying,
            "ghost" => &self.ghost,
            "fighting" => &self.fighting,
            "poison" => &self.poison,
            _ => &self.electric,
        }
    }
}
impl SectionData for TypesTranslationData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
