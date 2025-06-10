use super::{enums::*, typing::*};
use crate::i18ncore::keys::TranslationKey;
use crate::pkmncore::battling::priority::*;
use crate::pkmncore::moves::*;

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
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
#[derive(PartialEq, Clone, Copy)]
pub enum Move {
    Tackle,
    Growl,
}

impl MoveType for Move {
    fn get_base(&self) -> MoveBase {
        match self {
            Move::Tackle => MoveBase {
                name: TranslationKey::new("move/tackle/name"),
                desc: TranslationKey::new("move/tackle/desc"),
                move_category: MoveCategory::Physical,
                move_range: MoveRange::Normal,
                move_type: Type::Normal,
                move_power: Some(40),
                move_accuracy: Some(1_f32),
                move_priority: MovePriority::Neutral,
                move_pp: 35,
            },
            Move::Growl => MoveBase {
                name: TranslationKey::new("move/growl/name"),
                desc: TranslationKey::new("move/growl/desc"),
                move_category: MoveCategory::Status,
                move_range: MoveRange::ManyOthers,
                move_type: Type::Normal,
                move_power: None,
                move_accuracy: Some(1_f32),
                move_priority: MovePriority::Neutral,
                move_pp: 40,
            },
        }
    }
}
