use super::constants::{moves::*, typing::*};
use crate::utils::strings::*;
use crate::{i18ncore::keys::TranslationKey, pkmncore::battling::priority::*};

#[allow(dead_code)]
pub struct MoveBase {
    pub translation_path: MoveTranslation,
    pub move_type: Type,
    pub move_range: MoveRange,
    pub move_category: MoveCategory,
    pub move_power: Option<i8>,
    pub move_accuracy: Option<f32>,
    pub move_priority: MovePriority,
    pub move_pp: i8,
}

pub struct MoveTranslation {
    pub path: &'static str,
}

impl MoveTranslation {
    pub fn new(path: &'static str) -> MoveTranslation {
        MoveTranslation { path: path }
    }

    pub fn get_name(&self) -> TranslationKey {
        TranslationKey::new(concatenate_strings(self.path, "name"))
    }

    pub fn get_description(&self) -> TranslationKey {
        TranslationKey::new(concatenate_strings(self.path, "desc"))
    }
}

#[allow(dead_code)]
pub enum MoveRange {
    Normal,     //The move affects a selected adjacent target.
    ManyOthers, //Affects all adjacent opponents, but not allies
}

#[allow(dead_code)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct MoveData {
    pub base: Move,
    pub pp: i8,
    pub pp_ups_used: i8,
}

impl MoveData {
    pub fn get_max_pp(&self) -> i8 {
        (self.base.get_base().move_pp as f64 * (1.0_f64 + (self.pp_ups_used as f64 / 5.0_f64)))
            as i8
    }
}

pub trait LearntMove {
    fn get_move_type(&self) -> Move;
}

pub struct LevelUpMove {
    pub base: Move,
    pub level: i8,
}

pub struct EvolveMove {
    pub base: Move,
}

impl LearntMove for LevelUpMove {
    fn get_move_type(&self) -> Move {
        self.base
    }
}

impl LearntMove for EvolveMove {
    fn get_move_type(&self) -> Move {
        self.base
    }
}
