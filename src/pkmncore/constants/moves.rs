use super::{
    enums::{MoveCategory, MoveRange},
    typing::*,
};
use crate::pkmncore::battle::*;
use crate::pkmncore::moves::*;
use std::fmt::{self};

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
    fn use_move(
        &self,
        type_multiplier: f64,
        battle: &mut Battle,
        user: &mut BattlePokemon,
        targets: Vec<&mut BattlePokemon>,
    );
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
                name: "Tackle",
                desc: "A physical attack in which the user charges and slams into the target with its whole body.",
                move_category: MoveCategory::Physical,
                move_range: MoveRange::Normal,
                move_type: Type::Normal,
                move_power: Some(40),
                move_accuracy: Some(1_f32),
                move_pp: 35,
            },
            Move::Growl => MoveBase {
                name: "Growl",
                desc: "The user growls in an endearing way, making opposing Pokémon less wary. This lowers their Attack stats.",
                move_category: MoveCategory::Status,
                move_range: MoveRange::ManyOthers,
                move_type: Type::Normal,
                move_power: None,
                move_accuracy: Some(1_f32),
                move_pp: 40,
            },
        }
    }

    fn use_move(
        &self,
        type_multiplier: f64,
        battle: &mut Battle,
        user: &mut BattlePokemon,
        targets: Vec<&mut BattlePokemon>,
    ) {
        match self {
            Move::Tackle => {}
            Move::Growl => {}
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Tackle => write!(f, "Tackle"),
            Move::Growl => write!(f, "Growl"),
        }
    }
}
