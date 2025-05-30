use super::typing::*;
use crate::pkmncore::moves::*;
use std::fmt::{self};

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Move {
    Tackle,
}

impl MoveType for Move {
    fn get_base(&self) -> MoveBase {
        match self {
            Move::Tackle => MoveBase {
                name: "Tackle",
                desc: "A tackle. Jesus christ",
                move_type: Type::Normal,
                move_power: 40,
                move_accuracy: 1_f32,
                move_pp: 35,
            },
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Tackle => write!(f, "Tackle"),
        }
    }
}
