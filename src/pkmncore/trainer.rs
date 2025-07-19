use crate::pkmncore::boxes::pc::*;
use crate::pkmncore::constants::enums::*;

use super::pokemon::PokemonData;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub struct OTInformation {
    pub id: u16,
    pub sid: u16,
    pub lang: Language,
    pub gender: Gender,
    pub name: &'static str,
}

#[allow(dead_code)]
impl OTInformation {
    pub fn is_similar(&self, otinfo: OTInformation) -> bool {
        self.id == otinfo.id && self.sid == otinfo.sid
    }
}

#[allow(dead_code)]
pub struct Player {
    pub trainer: Trainer,
    pub pc: PC,
    pub party: [Option<PokemonData>; 6],
    pub money: i64,
}

impl Player {
    pub fn new(id: u16, sid: u16, lang: Language, gender: Gender, name: &'static str) -> Player {
        Player {
            trainer: Trainer {
                info: OTInformation {
                    id: id,
                    sid: sid,
                    lang: lang,
                    gender: gender,
                    name: name,
                },
            },
            pc: PC::new_dummy(),
            money: 0,
            party: [None, None, None, None, None, None],
        }
    }
}

#[allow(dead_code)]
pub struct Trainer {
    pub info: OTInformation,
}
