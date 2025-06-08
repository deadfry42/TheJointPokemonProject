use super::{enums::*, typing::*};
use crate::i18ncore::keys::TranslationKey;
use crate::pkmncore::battling::priority::*;
use crate::pkmncore::moves::*;

#[allow(dead_code)]
pub trait MoveType {
    fn get_base(&self) -> MoveBase;
    // fn use_move<'a>(
    //     &self,
    //     move_used: &mut MoveData,
    //     battle: &mut Battle,
    //     user: &mut BattleSideMember,
    // ) -> Vec<Box<dyn MoveEffect<'a>>>;
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Move {
    Tackle,
    Growl,
}

impl MoveType for Move {
    fn get_base(&self) -> MoveBase {
        match self {
            Move::Tackle => MoveBase {
                name: TranslationKey::new("moves/tackle/name"),
                desc: TranslationKey::new("moves/tackle/desc"),
                move_category: MoveCategory::Physical,
                move_range: MoveRange::Normal,
                move_type: Type::Normal,
                move_power: Some(40),
                move_accuracy: Some(1_f32),
                move_priority: MovePriority::Neutral,
                move_pp: 35,
            },
            Move::Growl => MoveBase {
                name: TranslationKey::new("moves/growl/name"),
                desc: TranslationKey::new("moves/growl/desc"),
                move_category: MoveCategory::Status,
                move_range: MoveRange::ManyOthers,
                move_type: Type::Normal,
                move_power: None,
                move_accuracy: Some(1_f32),
                move_priority: MovePriority::Neutral,
                move_pp: 40,
            },
        }
    }

    // fn use_move<'a>(
    //     &self,
    //     move_used: &mut MoveData,
    //     battle: &mut Battle,
    //     user: &mut BattleSideMember,
    // ) -> Vec<Box<dyn MoveEffect<'a>>> {
    //     move_used.pp -= 1;

    //     let mut targets: Vec<&mut BattlePokemon> = vec![];

    //     match move_used.base.get_base().move_range {
    //         MoveRange::Normal => {
    //             // if user.active_pokemon.is_some() {
    //             //     targets.push(user.active_pokemon.as_mut().unwrap()) // TODO: FIX THIS, TEST IMPL
    //             // }
    //         }
    //         _ => {
    //             // unimpl
    //         }
    //     }

    //     if targets.len() < 1 {
    //         return vec![];
    //     } // TODO: make a move result enum or something
    //     // "But there was no target.."

    //     let mut effects: Vec<Box<dyn MoveEffect<'a>>> = vec![];

    //     for target in targets.iter() {
    //         let effective_modifier: f64 =
    //             move_used
    //                 .base
    //                 .get_base()
    //                 .move_type
    //                 .get_type_multiplier(&target.base.get_base().types.type1)
    //                 * if target.base.get_base().types.type2.is_some() {
    //                     move_used.base.get_base().move_type.get_type_multiplier(
    //                         target.base.get_base().types.type2.as_ref().unwrap(),
    //                     )
    //                 } else {
    //                     1.0
    //                 };

    //         match self {
    //             Move::Tackle => effects.push(Box::new(MoveDamageEffect {
    //                 damage: 1,
    //                 target: user,
    //             })),
    //             Move::Growl => {}
    //         }
    //     }

    //     effects
    // }
}
