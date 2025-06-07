use crate::PKMNCore::constants::abilities::*;
use crate::PKMNCore::constants::enums::*;
use crate::PKMNCore::constants::items::*;
use crate::PKMNCore::constants::levels::*;
use crate::PKMNCore::constants::natures::*;
use crate::PKMNCore::constants::pokemon::*;
use crate::PKMNCore::moves::*;
use crate::PKMNCore::pokemon::*;
use crate::PKMNCore::trainer::*;

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

    pub fn calculate_stat(&self, stat: &Stat) -> i32 {
        if *stat == Stat::Health {
            // health has a different formula
            let base: i16 = self.base.get_base_stat(&stat);
            let iv: i8 = self.ivs.health;
            let ev: i16 = self.evs.health.unwrap_or(0);
            let level: i8 = self.get_level();

            ((((2.0_f64 * (base as f64) + (iv as f64) + ((ev as f64) / 4.0_f64)) * (level as f64))
                / 100.0_f64)
                .floor()
                + (level as f64)
                + 10.0_f64) as i32
        } else {
            // other stats have this formula
            let nature = self.nature.get_stat_multiplier(&stat);
            let level: i8 = self.get_level();

            let base: i16 = self.base.get_base_stat(&stat);
            let iv: i8 = self.get_iv(&stat);
            let ev: i16 = self.get_ev(&stat);

            ((((((2.0_f64 * base as f64 + iv as f64 + (ev as f64 / 4.0_f64)) * level as f64)
                / 100.0_f64)
                .floor()
                + 5.0_f64)
                * nature)
                .floor()) as i32
        }
    }

    pub fn get_iv(&self, stat: &Stat) -> i8 {
        match stat {
            Stat::Health => self.ivs.health,
            Stat::Attack => self.ivs.atk,
            Stat::SpecialAttack => self.ivs.spatk,
            Stat::Defense => self.ivs.def,
            Stat::SpecialDefence => self.ivs.spdef,
            Stat::Speed => self.ivs.speed,
        }
    }

    pub fn get_ev(&self, stat: &Stat) -> i16 {
        match stat {
            Stat::Health => self.evs.health.unwrap_or(0),
            Stat::Attack => self.evs.atk.unwrap_or(0),
            Stat::SpecialAttack => self.evs.spatk.unwrap_or(0),
            Stat::Defense => self.evs.def.unwrap_or(0),
            Stat::SpecialDefence => self.evs.spdef.unwrap_or(0),
            Stat::Speed => self.evs.speed.unwrap_or(0),
        }
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
