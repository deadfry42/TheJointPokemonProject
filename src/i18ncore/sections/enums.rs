use std::any::Any;

use crate::i18ncore::localisation::{DataSection, SectionType};

pub struct GenderLocale {
    pub male: &'static str,
    pub female: &'static str,
    pub unknown: &'static str,
}

impl DataSection for GenderLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "male" => &self.male,
            "female" => &self.female,
            _ => &self.unknown,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}

pub struct StatLocale {
    pub health: &'static str,
    pub speed: &'static str,
    pub attack: &'static str,
    pub defense: &'static str,
    pub special_attack: &'static str,
    pub special_defense: &'static str,
}

impl DataSection for StatLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "health" => &self.health,
            "speed" => &self.speed,
            "attack" => &self.attack,
            "defense" => &self.defense,
            "special_attack" => &self.special_attack,
            _ => &self.special_defense,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}

pub struct OtherLanguageLocale {
    pub english: &'static str,
}

impl DataSection for OtherLanguageLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "english" => &self.english,
            _ => &self.english,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}

pub struct TypesLocale {
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

impl DataSection for TypesLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
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
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
