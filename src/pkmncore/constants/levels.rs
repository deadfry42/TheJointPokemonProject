use super::{
    items::Item,
    pokemon::{Pokemon, PokemonType},
};
use crate::pkmncore::battle::battle::*;
use crate::pkmncore::battle::pokemon::*;
use std::fmt::{self};

pub trait LevellingCurveCalc {
    fn levels_to_min_exp(&self, levels: i8) -> u32;
    fn exp_to_levels(&self, exp: u32) -> i8;
    fn lut_function(&self, exp: u32) -> i8;
}

#[allow(dead_code)]
pub enum LevellingCurve {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

impl fmt::Display for LevellingCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LevellingCurve::Erratic => write!(f, "Erratic"),
            LevellingCurve::Fast => write!(f, "Fast"),
            LevellingCurve::MediumFast => write!(f, "Medium Fast"),
            LevellingCurve::MediumSlow => write!(f, "Medium Slow"),
            LevellingCurve::Slow => write!(f, "Slow"),
            LevellingCurve::Fluctuating => write!(f, "Fluctuating"),
        }
    }
}

impl LevellingCurveCalc for LevellingCurve {
    fn lut_function(&self, exp: u32) -> i8 {
        for lvl in 1..100 {
            let val = self.levels_to_min_exp(lvl);

            if (exp + 1) > val {
                continue;
            } else if (exp + 1) <= val {
                return lvl - 1;
            }
        }

        0
    }

    fn levels_to_min_exp(&self, levels: i8) -> u32 {
        let float_lvl = levels as f64;
        match self {
            LevellingCurve::Fast => ((float_lvl.powf(3.0_f64) * 4.0_f64) / 5.0_f64) as u32,
            LevellingCurve::Slow => ((float_lvl.powf(3.0_f64) * 5.0_f64) / 4.0_f64) as u32,
            LevellingCurve::MediumFast => float_lvl.powf(3.0_f64) as u32,
            LevellingCurve::MediumSlow => {
                ((6.0_f64 / 5.0_f64) * float_lvl.powf(3.0_f64)
                    - (15.0_f64 * float_lvl.powf(2.0_f64))
                    + (100.0_f64 * float_lvl)
                    - 140.0_f64) as u32
            }
            LevellingCurve::Erratic => match levels {
                i if i < 50 => {
                    ((float_lvl.powf(3.0_f64) * (100.0_f64 - float_lvl)) / 50.0_f64) as u32
                }
                i if 50 <= i && i < 68 => {
                    ((float_lvl.powf(3.0_f64) * (150.0_f64 - float_lvl)) / 100.0_f64) as u32
                }
                i if 68 <= i && levels < 98 => {
                    ((float_lvl.powf(3.0_f64)
                        * ((1911.0_f64 - (10.0_f64 * float_lvl)) / 3.0_f64).floor())
                        / 500.0_f64) as u32
                }
                i if 98 <= i && levels <= 100 => {
                    // bulbapedia is wrong here - it said n < 100 not n <= 100 lol
                    ((float_lvl.powf(3.0_f64) * (160.0_f64 - float_lvl)) / 100.0_f64) as u32
                }
                _ => 0,
            },
            LevellingCurve::Fluctuating => match levels {
                i if i < 15 => {
                    (float_lvl.powf(3.0_f64)
                        * (((float_lvl + 1.0_f64) / 3.0_f64).floor() + 24.0_f64)
                        / 50.0_f64) as u32
                }
                i if 15 <= i && i < 36 => {
                    (float_lvl.powf(3.0_f64) * (float_lvl + 14.0_f64) / 50.0_f64) as u32
                }
                i if 36 <= i && i <= 100 => {
                    (float_lvl.powf(3.0_f64) * ((float_lvl * 0.5_f64).floor() + 32.0_f64)
                        / 50.0_f64) as u32
                }
                _ => 0,
            },
        }
    }

    fn exp_to_levels(&self, exp: u32) -> i8 {
        match self {
            // formulaic ones (that have a direct conversion)
            LevellingCurve::Fast => {
                ((exp as f64 * 5.0_f64) / 4.0_f64).powf(1.0 / 3.0).floor() as i8
            }
            LevellingCurve::Slow => {
                ((exp as f64 * 4.0_f64) / 5.0_f64).powf(1.0 / 3.0).floor() as i8
            }
            LevellingCurve::MediumFast => (exp as f64).powf(1.0 / 3.0).floor() as i8,
            // LUT ones (they don't have a direct conversion)
            LevellingCurve::Fluctuating => self.lut_function(exp),
            LevellingCurve::Erratic => self.lut_function(exp),
            LevellingCurve::MediumSlow => self.lut_function(exp),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn calculate_battle_xp_gain(
    battle: &Battle,
    recipient_trainer: &dyn BattleTrainerType,
    recipient: &BattlePokemon,
    victim: &BattlePokemon,
) -> u32 {
    // gen 7 onwards xp formula
    let s: f64 = 1.0; // 1 when participated in battle, 2 if exp. share is enabled but didnt participate
    let trainer = recipient_trainer
        .as_any()
        .downcast_ref::<BattleTrainer>()
        .unwrap();
    let t: f64 = if trainer.ot.eq(recipient.ot.as_ref().unwrap()) {
        1.0_f64
    } else {
        1.5_f64
    };
    let e: f64 = if recipient.is_holding(Item::LuckyEgg) {
        1.5_f64
    } else {
        1.0_f64
    };
    let v: f64 = if Pokemon::get_evolution_level(&recipient.base.get_base().pkmn)
        .unwrap()
        .lt(&recipient.get_level())
    {
        4915_f64 / 4096_f64
    } else {
        1_f64
    };
    let f: f64 = 1.0; // 4915/4096 if atleast 2 hearts of affection
    let p: f64 = 1.0; // pass power equivalent, leave as 1 for now
    let victim_base_exp = victim.base.get_base().base_exp;
    let victim_level = victim
        .base
        .get_base()
        .levelling_curve
        .exp_to_levels(victim.exp) as u32;

    let recipient_level = recipient
        .base
        .get_base()
        .levelling_curve
        .exp_to_levels(recipient.exp) as u32;

    let basewinningxp: f64 = ((victim_base_exp * victim_level) / 5_u32) as f64
        * (1.0 / s)
        * ((2 * victim_level + 10) / victim_level + recipient_level + 10).pow(5 / 2) as f64
        + 1.0_f64;

    (basewinningxp * t * e * v * f * p) as u32
}
