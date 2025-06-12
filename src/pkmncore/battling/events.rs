use super::battle::*;
use super::priority::*;
use crate::pkmncore::battling::movechanges::BattleMoveData;
use crate::pkmncore::constants::enums::Weather;
use crate::pkmncore::constants::moves::*;

pub struct BattleTurn<'a> {
    pub events: Vec<&'a dyn BattleEvent>,
}

#[allow(dead_code)]
pub trait BattleEvent {
    // Normal battle events
    fn get_priority(&self) -> MovePriority;
    fn run(&self, battle: &Battle);
}

#[allow(dead_code)]
pub trait BattleEventTrainer<'b> {
    // Events that trainers perform
    fn get_trainer(&self) -> &'b dyn BattleTrainerType;
    fn run_as_trainer(&self, battle: &Battle, trainer: &dyn BattleTrainerType);
}

pub struct BattleWeatherEvent {
    // Do weather effects
    pub weather_state: Weather,
}

impl BattleEvent for BattleWeatherEvent {
    fn get_priority(&self) -> MovePriority {
        MovePriority::Weather
    }

    fn run(&self, battle: &Battle) {}
}

pub struct BattleMoveEvent<'a> {
    // Move announcing, animation, then effects
    pub battle_move_data: BattleMoveData<'a>,
    pub trainer: &'a dyn BattleTrainerType,
}

impl<'a> BattleEvent for BattleMoveEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        self.battle_move_data
            .move_used
            .base
            .get_base()
            .move_priority
    }
    fn run(&self, battle: &Battle) {}
}

impl<'a> BattleEventTrainer<'a> for BattleMoveEvent<'a> {
    fn get_trainer(&self) -> &'a dyn BattleTrainerType {
        self.trainer
    }
    fn run_as_trainer(&self, battle: &Battle, trainer: &dyn BattleTrainerType) {}
}

#[allow(dead_code)]
pub struct BattleSwitchEvent<'a> {
    // Switching in a new pokemon
    pub new_index: usize,
    pub trainer: &'a dyn BattleTrainerType,
}

impl<'a> BattleEvent for BattleSwitchEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        MovePriority::Switching
    }
    fn run(&self, battle: &Battle) {}
}

impl<'a> BattleEventTrainer<'a> for BattleSwitchEvent<'a> {
    fn get_trainer(&self) -> &'a dyn BattleTrainerType {
        self.trainer
    }
    fn run_as_trainer(&self, battle: &Battle, trainer: &dyn BattleTrainerType) {}
}

#[allow(dead_code)]
impl<'a> BattleSwitchEvent<'a> {
    fn get_new_index(&self) -> usize {
        self.new_index
    }
}
