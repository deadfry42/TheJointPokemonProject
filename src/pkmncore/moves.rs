use super::constants::{enums::*, moves::*, typing::*};
use crate::pkmncore::battle::priority::*;

#[allow(dead_code)]
pub struct MoveBase {
    pub name: &'static str,
    pub desc: &'static str,
    pub move_type: Type,
    pub move_range: MoveRange,
    pub move_category: MoveCategory,
    pub move_power: Option<i8>,
    pub move_accuracy: Option<f32>,
    pub move_priority: MovePriority,
    pub move_pp: i8,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct MoveData {
    pub base: Move,
    pub pp: i8,
    pub pp_ups_used: i8,
}

// pub trait MoveEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a>;
// }

// pub struct MoveDamageEffect<'a> {
//     pub damage: u32,
//     pub target: &'a BattleSideMember<'a>,
// }

// impl<'a> MoveEffect<'a> for MoveDamageEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a> {
//         self.target
//     }
// }

// pub struct MoveStatChangeEffect<'a> {
//     pub stat: Stat,
//     pub stage_change: i8,
//     pub target: &'a BattleSideMember<'a>,
// }

// impl<'a> MoveEffect<'a> for MoveStatChangeEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a> {
//         self.target
//     }
// }

// pub struct MoveAccuraryChangeEffect<'a> {
//     pub modifier_change: i8,
//     pub target: &'a BattleSideMember<'a>,
// }

// impl<'a> MoveEffect<'a> for MoveAccuraryChangeEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a> {
//         self.target
//     }
// }

// pub struct MoveBattleConditionChangeEffect<'a> {
//     pub battle_condition: BattleConditions,
//     pub target: &'a BattleSideMember<'a>,
// }

// impl<'a> MoveEffect<'a> for MoveBattleConditionChangeEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a> {
//         self.target
//     }
// }

// pub struct MoveBattleStatusChangeEffect<'a> {
//     pub status: StatusCondition,
//     pub target: &'a BattleSideMember<'a>,
// }

// impl<'a> MoveEffect<'a> for MoveBattleStatusChangeEffect<'a> {
//     fn get_target(&self) -> &'a BattleSideMember<'a> {
//         self.target
//     }
// }
