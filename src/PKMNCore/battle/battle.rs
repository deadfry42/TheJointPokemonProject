use crate::PKMNCore::trainer::OTInformation;
use std::any::Any;

use super::pokemon::*;
use std::ops::Deref;

#[allow(dead_code)]
pub trait BattleUtils {
    fn get_turn_count(&self) -> &u32; // realistically a u8 would work here, but just in case some battles exceed 256 turns, im using u32
    fn is_wild_battle(&self) -> bool;
    fn process_turn(&self);
    fn get_player_side(&self) -> &BattleSide;
    fn get_opposing_side(&self) -> &BattleSide;
}

#[allow(dead_code)]
pub trait BattleTrainerType {
    fn is_wild(&self) -> bool;
    fn get_sent_out_index(&self) -> &usize;
    fn set_sent_out_index(&mut self, index: usize);
    fn get_available_indexes(&self) -> &Vec<usize>;
    fn as_any(&self) -> &dyn Any;
}

#[allow(dead_code)]
pub struct BattleSide {
    pub trainer_a: Box<dyn BattleTrainerType>,
    pub trainer_b: Option<Box<dyn BattleTrainerType>>,
    pub party: [Option<BattlePokemon>; 6],
}

#[allow(dead_code)]
impl BattleSide {
    fn trainer_b_exists(&self) -> bool {
        self.trainer_b.is_some()
    }

    // TODO
    // check if these functions are safe
    // probably not ;)
    fn get_trainer_a_pokemon(&self) -> Option<&BattlePokemon> {
        let index: &usize = self.trainer_a.deref().get_sent_out_index();

        if self.party[*index].is_none() {
            None
        } else {
            Some(self.party[*index].as_ref().unwrap())
        }
    }

    fn get_trainer_b_pokemon(&self) -> Option<&BattlePokemon> {
        if !self.trainer_b_exists() {
            return None;
        }

        let index: &usize = self.trainer_b.as_ref().unwrap().get_sent_out_index();

        if self.party[*index].is_none() {
            None
        } else {
            Some(self.party[*index].as_ref().unwrap())
        }
    }
}

pub struct BattleTrainer {
    pub currently_sent_out: usize,
    pub available_indexes: Vec<usize>,
    pub ot: OTInformation,
}

pub struct BattleWildPokemon {
    pub available_indexes: Vec<usize>, // make sure this variable is exclusively vec![0]
}

#[allow(unused_variables)]
impl BattleTrainerType for BattleWildPokemon {
    fn get_sent_out_index(&self) -> &usize {
        &0
    }

    fn set_sent_out_index(&mut self, index: usize) {
        // stub, shouldn't need to switch out
    }

    fn is_wild(&self) -> bool {
        true
    }

    fn get_available_indexes(&self) -> &Vec<usize> {
        &self.available_indexes
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BattleTrainerType for BattleTrainer {
    fn get_sent_out_index(&self) -> &usize {
        &self.currently_sent_out
    }

    fn set_sent_out_index(&mut self, index: usize) {
        self.currently_sent_out = index;
    }

    fn is_wild(&self) -> bool {
        false
    }

    fn get_available_indexes(&self) -> &Vec<usize> {
        &self.available_indexes
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code)]
impl BattleTrainer {
    fn get_ot(&self) -> &OTInformation {
        &self.ot
    }
}

pub struct Battle {
    pub turn_count: u32,
    pub player_side: BattleSide,
    pub opposing_side: BattleSide,
}

impl BattleUtils for Battle {
    fn get_turn_count(&self) -> &u32 {
        &self.turn_count
    }
    fn is_wild_battle(&self) -> bool {
        self.opposing_side.trainer_a.is_wild()
    }
    fn get_player_side(&self) -> &BattleSide {
        &self.player_side
    }
    fn get_opposing_side(&self) -> &BattleSide {
        &self.opposing_side
    }
    fn process_turn(&self) {
        //stub
    }
}
