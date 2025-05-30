use crate::pkmncore::constants::enums::*;

use super::pokemon::PokemonData;

#[allow(dead_code)]
#[derive(PartialEq)]
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
    pub party: [Option<PokemonData>; 6],
    pub money: i64,
}

#[allow(dead_code)]
pub struct Trainer {
    pub info: OTInformation,
}
