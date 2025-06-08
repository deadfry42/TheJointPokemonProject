use super::battle::*;
use super::priority::*;
use crate::pkmncore::constants::moves::*;
use crate::pkmncore::moves::*;

pub struct BattleTurn<'a> {
    pub events: Vec<&'a dyn BattleEvent>,
}

#[allow(dead_code)]
pub trait BattleEvent {
    fn get_priority(&self) -> MovePriority;
}

#[allow(dead_code)]
pub trait BattleEventTrainer<'b> {
    fn get_trainer(&self) -> &'b dyn BattleTrainerType;
}

pub struct BattleMoveEvent<'a> {
    pub move_used: &'a MoveData,
    pub trainer: &'a dyn BattleTrainerType,
}

impl<'a> BattleEvent for BattleMoveEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        self.move_used.base.get_base().move_priority
    }
}

impl<'a> BattleEventTrainer<'a> for BattleMoveEvent<'a> {
    fn get_trainer(&self) -> &'a dyn BattleTrainerType {
        self.trainer
    }
}

#[allow(dead_code)]
pub struct BattleSwitchEvent<'a> {
    pub new_index: usize,
    pub trainer: &'a dyn BattleTrainerType,
}

impl<'a> BattleEvent for BattleSwitchEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        MovePriority::Switching
    }
}

impl<'a> BattleEventTrainer<'a> for BattleSwitchEvent<'a> {
    fn get_trainer(&self) -> &'a dyn BattleTrainerType {
        self.trainer
    }
}

#[allow(dead_code)]
impl<'a> BattleSwitchEvent<'a> {
    fn get_new_index(&self) -> usize {
        self.new_index
    }
}
