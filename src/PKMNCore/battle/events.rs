use super::battle::*;
use super::priority::*;
use crate::PKMNCore::constants::moves::*;
use crate::PKMNCore::moves::*;

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

// #[allow(dead_code)]
// pub trait BattleEvent {
//     fn get_priority(&self) -> MovePriority;
// }

// #[allow(dead_code)]
// #[derive(Clone, Copy)]
// pub struct StatChange {
//     pub atk_change: i8,
//     pub def_change: i8,
//     pub spatk_change: i8,
//     pub spdef_change: i8,
//     pub speed_change: i8,
//     pub evasion_change: i8,
//     pub accuracy_modifier_change: f32,
// }

// #[allow(dead_code)]
// pub struct UseMoveEvent<'a> {
//     pub move_data: &'a MoveData,
//     pub user_member: BattleSideMember<'a>,
//     pub changes: Vec<Box<dyn MoveEffect<'a>>>,
// }

// impl<'a> BattleEvent for UseMoveEvent<'a> {
//     fn get_priority(&self) -> MovePriority {
//         self.move_data.base.get_base().move_priority
//     }
// }

// #[allow(dead_code)]
// pub struct UseItemEvent {
//     pub item: Item,
// }

// impl BattleEvent for UseItemEvent {
//     fn get_priority(&self) -> MovePriority {
//         MovePriority::Item
//     }
// }

// pub struct SwitchPokemonEvent<'a> {
//     pub member: &'a BattleSideMember<'a>,
//     pub new_pokemon: &'a BattlePokemon,
// }

// impl<'a> BattleEvent for SwitchPokemonEvent<'a> {
//     fn get_priority(&self) -> MovePriority {
//         MovePriority::Switching
//     }
// }
