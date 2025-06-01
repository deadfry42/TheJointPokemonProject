use super::constants::abilities::Ability;
use super::constants::enums::{BattleConditions, MoveRange, StatusCondition};
use super::constants::items::*;
use super::constants::moves::MoveType;
use super::constants::natures::Nature;
use super::constants::pokemon::{Pokemon, PokemonType};
use super::constants::typing::Typing;
use super::moves::*;
use super::trainer::*;
use super::{constants::levels::*, pokemon::*};

#[allow(dead_code)]
pub struct Battle {
    // "Teams" are used instead of trainers
    // for double battles
    pub team_a: BattleSide,
    pub team_b: BattleSide,
    pub is_wild: bool,
}

#[allow(dead_code)]
impl Battle {
    fn is_single_battle(&self) -> bool {
        self.team_a.get_member_count() == 1 && self.team_b.get_member_count() == 1
    }

    fn use_move(&mut self, user: &mut BattlePokemon, move_used: &mut MoveData) {
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

            // move_used
            //     .base
            //     .use_move(effective_modifier, self, user, targets);
        }
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
pub struct BattleSide {
    pub trainers: Vec<BattleSideMember>,
}

#[allow(dead_code)]
impl BattleSide {
    fn get_member_count(&self) -> usize {
        self.trainers.len()
    }
}

#[allow(dead_code)]
pub struct BattleSideMember {
    pub team: [Option<BattlePokemon>; 6],
    pub trainer: Trainer,
}
