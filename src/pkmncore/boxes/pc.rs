use crate::pkmncore::{boxes::boxes::PCBox, pokemon::PokemonData};

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
