use crate::pkmncore::{
    constants::enums::{Stat, StatusCondition, Weather},
    moves::MoveData,
};

pub trait MoveChange {}

pub trait TargettedMoveChange {
    fn get_targets(&self) -> &Vec<usize>;
}

pub struct TargettedMoveDamageChange {
    pub targets: Vec<usize>,
    pub damage: u32,
}

impl MoveChange for TargettedMoveDamageChange {}
impl TargettedMoveChange for TargettedMoveDamageChange {
    fn get_targets(&self) -> &Vec<usize> {
        &self.targets
    }
}

pub struct MoveWeatherChange {
    pub weather: Weather,
}

impl MoveChange for MoveWeatherChange {}

pub struct TargettedMoveStatusChange {
    pub targets: Vec<usize>,
    pub status: StatusCondition,
}

impl MoveChange for TargettedMoveStatusChange {}
impl TargettedMoveChange for TargettedMoveStatusChange {
    fn get_targets(&self) -> &Vec<usize> {
        &self.targets
    }
}

pub struct TargettedMoveStatChange {
    pub targets: Vec<usize>,
    pub stat: Stat,
    pub stage_change: i8,
}

impl MoveChange for TargettedMoveStatChange {}
impl TargettedMoveChange for TargettedMoveStatChange {
    fn get_targets(&self) -> &Vec<usize> {
        &self.targets
    }
}

pub struct BattleMoveData<'a> {
    pub move_used: &'a MoveData,
    pub changes: Option<Vec<Box<dyn MoveChange>>>,
}
