use super::background::*;
use crate::pkmncore::pokemon::PokemonData;

const PC_BOXES_COUNT: usize = 32;

#[allow(dead_code)]
pub struct PC {
    pub developer_revealed: bool,
    pub hand: Option<PokemonData>,
    pub boxes: [PCBox; PC_BOXES_COUNT],
}

impl PC {
    pub fn new_dummy() -> PC {
        PC {
            developer_revealed: false,
            hand: None,
            boxes: [
                // yay this is so pleasant to read
                // (32 dummy pc boxes)
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
                PCBox::new_dummy(),
            ],
        }
    }
}

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
