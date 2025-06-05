use crate::pkmncore::constants::moves::*;
use crate::pkmncore::constants::priority::MovePriority;

use super::constants::abilities::Ability;
use super::constants::enums::{BattleConditions, StatusCondition};
use super::constants::items::*;
use super::constants::natures::Nature;
use super::constants::pokemon::{Pokemon, PokemonType};
use super::moves::*;
use super::trainer::*;
use super::{constants::levels::*, pokemon::*};

#[allow(dead_code)]
pub struct Battle<'a> {
    // "Teams" are used instead of trainers
    // for double battles
    pub team_a: BattleSide<'a>,
    pub team_b: BattleSide<'a>,
    pub is_wild: bool,
}

#[allow(dead_code)]
impl<'a> Battle<'a> {
    fn is_single_battle(&self) -> bool {
        self.team_a.get_member_count() == 1 && self.team_b.get_member_count() == 1
    }

    fn knockout_pokemon(
        &self,
        recipient_trainer: &BattleSideMember,
        recipient: &mut BattlePokemon,
        victim: &mut BattlePokemon,
    ) {
        recipient.award_xp(calculate_battle_xp_gain(
            self,
            recipient_trainer,
            recipient,
            victim,
        ));
    }

    fn play_turn(&self, turn: BattleTurn) {
        for event in turn.events {
            let priority: MovePriority = event.get_priority();
        }
    }
}

#[allow(dead_code)]
pub struct BattlePokemon {
    pub nickname: Option<&'static str>,
    pub ot: Option<OTInformation>,
    pub atk_stage: i8,
    pub def_stage: i8,
    pub spatk_stage: i8,
    pub spdef_stage: i8,
    pub speed_stage: i8,
    pub evasion_stage: i8,
    pub accuracy_modifier: f32,
    pub pid: u32,
    pub ability: Ability,
    pub shiny: bool,
    pub exp: u32,
    pub helditem: Option<Item>,
    pub evs: EVs,
    pub ivs: IVs,
    pub base: Pokemon,
    pub pokerus: bool,
    pub condition: Option<StatusCondition>,
    pub battle_condition: Vec<BattleConditions>,
    pub friendship: u8,
    pub nature: Nature,
    pub moves: [Option<MoveData>; 4],
}

#[allow(dead_code)]
impl BattlePokemon {
    // turn pokemon data into battle data
    pub fn from_data(data: &PokemonData) -> BattlePokemon {
        BattlePokemon {
            moves: data.moves,
            nickname: data.nickname,
            ot: Some(data.ot),
            atk_stage: 0,
            def_stage: 0,
            spatk_stage: 0,
            spdef_stage: 0,
            speed_stage: 0,
            evasion_stage: 0,
            accuracy_modifier: 1_f32,
            pid: data.pid,
            ability: data.ability,
            shiny: data.shiny,
            exp: data.exp,
            helditem: data.helditem,
            evs: data.evs,
            ivs: data.ivs,
            pokerus: data.pokerus,
            condition: data.condition,
            friendship: data.friendship,
            nature: data.nature,
            base: data.base.pkmn,
            battle_condition: vec![],
        }
    }

    // update pokemon data after battle
    pub fn update_data(&self, data: &mut PokemonData) {
        data.exp = self.exp;
        data.evs = self.evs;
        data.pokerus = self.pokerus;
        data.helditem = self.helditem;
        data.friendship = self.friendship;
        data.condition = self.condition;
        data.moves = self.moves;
    }

    pub fn award_xp(&mut self, xp: u32) {
        self.exp += xp;
    }

    pub fn get_level(&self) -> i8 {
        self.base.get_base().levelling_curve.exp_to_levels(self.exp)
    }

    pub fn is_holding(&self, i: Item) -> bool {
        self.helditem.as_ref().unwrap().eq(&i)
    }
}

#[allow(dead_code)]
pub struct BattleSide<'a> {
    pub trainers: Vec<BattleSideMember<'a>>,
}

#[allow(dead_code)]
impl<'a> BattleSide<'a> {
    fn get_member_count(&self) -> usize {
        self.trainers.len()
    }
}

#[allow(dead_code)]
pub struct BattleSideMember<'a> {
    pub team: [Option<BattlePokemon>; 6],
    pub trainer: Trainer,
    pub active_pokemon: Option<&'a BattlePokemon>,
}

#[allow(dead_code)]
pub struct BattleTurn {
    pub events: Vec<Box<dyn BattleEvent>>,
}

#[allow(dead_code)]
pub trait BattleEvent {
    fn get_priority(&self) -> MovePriority;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct StatChange {
    pub atk_change: i8,
    pub def_change: i8,
    pub spatk_change: i8,
    pub spdef_change: i8,
    pub speed_change: i8,
    pub evasion_change: i8,
    pub accuracy_modifier_change: f32,
}

#[allow(dead_code)]
pub struct UseMoveEvent<'a> {
    pub move_data: &'a MoveData,
    pub user_member: BattleSideMember<'a>,
    pub changes: Vec<Box<dyn MoveEffect<'a>>>,
}

impl<'a> BattleEvent for UseMoveEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        self.move_data.base.get_base().move_priority
    }
}

#[allow(dead_code)]
pub struct UseItemEvent {
    pub item: Item,
}

impl BattleEvent for UseItemEvent {
    fn get_priority(&self) -> MovePriority {
        MovePriority::Item
    }
}

pub struct SwitchPokemonEvent<'a> {
    pub member: &'a BattleSideMember<'a>,
    pub new_pokemon: &'a BattlePokemon,
}

impl<'a> BattleEvent for SwitchPokemonEvent<'a> {
    fn get_priority(&self) -> MovePriority {
        MovePriority::Switching
    }
}
