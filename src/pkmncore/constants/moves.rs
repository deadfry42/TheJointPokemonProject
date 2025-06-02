use super::{
    enums::{MoveCategory, MoveRange},
    pokemon::*,
    priority::*,
    typing::*,
};
use crate::pkmncore::battle::*;
use crate::pkmncore::moves::*;
use std::fmt::{self};

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
    fn use_move(&self, move_used: &mut MoveData, battle: &mut Battle, user: &mut BattlePokemon);
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
                move_priority: MovePriority::Neutral,
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
                move_priority: MovePriority::Neutral,
                move_pp: 40,
            },
        }
    }

    fn use_move(&self, move_used: &mut MoveData, battle: &mut Battle, user: &mut BattlePokemon) {
        move_used.pp -= 1;

        let mut targets: Vec<&mut BattlePokemon> = vec![];

        match move_used.base.get_base().move_range {
            MoveRange::Normal => {
                targets.push(user) // TODO: FIX THIS, TEST IMPL
            }
            _ => {
                // unimpl
            }
        }

        if targets.len() < 1 {
            return;
        } // TODO: make a move result enum or something
        // "But there was no target.."

        for target in targets.iter() {
            let effective_modifier: f64 =
                move_used
                    .base
                    .get_base()
                    .move_type
                    .get_type_multiplier(&target.base.get_base().types.type1)
                    * if target.base.get_base().types.type2.is_some() {
                        move_used.base.get_base().move_type.get_type_multiplier(
                            target.base.get_base().types.type2.as_ref().unwrap(),
                        )
                    } else {
                        1.0
                    };

            match self {
                Move::Tackle => {}
                Move::Growl => {}
            }
        }
    }
}
