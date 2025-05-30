use super::trainer::*;
use super::{constants::levels::*, pokemon::PokemonData};

#[allow(dead_code)]
pub struct Battle {
    // "Teams" are used instead of trainers
    // for double battles
    pub team_a: BattleSide,
    pub team_b: BattleSide,
}

#[allow(dead_code)]
#[allow(unused_mut)]
impl Battle {
    fn is_single_battle(&self) -> bool {
        self.team_a.get_member_count() == 1 && self.team_b.get_member_count() == 1
    }

    fn knockout_pokemon(
        &self,
        recipient_trainer: BattleSideMember,
        mut recipient: BattlePokemon,
        mut victim: BattlePokemon,
    ) {
        recipient.data.award_xp(calculate_battle_xp_gain(
            self,
            &recipient_trainer,
            &recipient,
            &victim,
        ));
    }
}

#[allow(dead_code)]
pub struct BattlePokemon {
    pub data: PokemonData,
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
