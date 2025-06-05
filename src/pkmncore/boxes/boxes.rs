use crate::pkmncore::pokemon::PokemonData;

use super::background::*;

#[allow(dead_code)]
pub struct PCBox {
    pub title: Option<&'static str>,
    pub background: Option<PCBackground>,
    pub pkmn: [Option<PokemonData>; 30],
}

impl PCBox {
    pub fn new_dummy() -> PCBox {
        PCBox {
            title: None,
            background: None,
            pkmn: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }
}
