use super::typing::*;
use crate::pkmncore::battling::battle::Battle;
use crate::pkmncore::battling::movechanges::{
    MoveChange, TargettedMoveDamageChange, TargettedMoveStatChange,
};
use crate::pkmncore::battling::priority::*;
use crate::pkmncore::constants::enums::Stat;
use crate::pkmncore::moves::*;

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
    fn use_move(&self, battle: &Battle, user: usize) -> Option<Vec<Box<dyn MoveChange>>>; // temporary, replace with move events

    fn get_targets(&self, battle: &Battle, user: usize, range: &MoveRange) -> Vec<usize> {
        match range {
            MoveRange::Normal => {
                vec![user]
            } // TODO : ACTUALLY IMPLEMENT THIS PROPERLY
            MoveRange::ManyOthers => {
                vec![user]
            } // TODO : ACTUALLY IMPLEMENT THIS PROPERLY
        }
    }
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
                translation_path: MoveTranslation::new("move/tackle/"),
                move_category: MoveCategory::Physical,
                move_range: MoveRange::Normal,
                move_type: Type::Normal,
                move_power: Some(40),
                move_accuracy: Some(1_f32),
                move_priority: MovePriority::Neutral,
                move_pp: 35,
            },
            Move::Growl => MoveBase {
                translation_path: MoveTranslation::new("move/growl/"),
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

    fn use_move(&self, battle: &Battle, user: usize) -> Option<Vec<Box<dyn MoveChange>>> {
        let mut stack: Vec<Box<dyn MoveChange>> = vec![];

        match self {
            Move::Tackle => {
                let targets: Vec<usize> =
                    self.get_targets(battle, user, &self.get_base().move_range);

                stack.push(Box::new(TargettedMoveDamageChange {
                    targets: targets,
                    damage: 10,
                }))
            }
            Move::Growl => {
                let targets: Vec<usize> =
                    self.get_targets(battle, user, &self.get_base().move_range);

                stack.push(Box::new(TargettedMoveStatChange {
                    targets: targets,
                    stat: Stat::Attack,
                    stage_change: -1,
                }))
            }
        }

        Some(stack)
    }
}
