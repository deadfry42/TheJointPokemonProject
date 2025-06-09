use crate::pkmncore::constants::pokemon::Pokemon;

const POKEDEX_SIZE: usize = 151;

pub struct Pokedex {
    pub pokedex: [Option<Pokemon>; POKEDEX_SIZE],
}

impl Pokedex {
    pub fn new() -> Pokedex {
        Pokedex {
            pokedex: [None; POKEDEX_SIZE],
        }
    }

    pub fn set(&mut self, index: usize, pkmn: Pokemon) {
        self.pokedex[index] = Some(pkmn);
    }

    pub fn create_regional_dex() -> Pokedex {
        let mut regional = Pokedex::new();
        regional.set(0, Pokemon::Foliwli);

        regional
    }
}
