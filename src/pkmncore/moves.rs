use super::constants::typing::*;

#[allow(dead_code)]
pub struct MoveBase {
    pub name: &'static str,
    pub desc: &'static str,
    pub move_type: Type,
    pub move_power: i8,
    pub move_accuracy: f32,
    pub move_pp: i8,
}

#[allow(dead_code)]
pub struct MoveData {
    pub base: MoveBase,
    pub pp: i8,
    pub pp_ups_used: i8,
}
