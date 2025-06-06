use super::constants::items::*;
use super::constants::pokemon::*;

use std::any::Any;

#[allow(dead_code)]
pub trait Evolution {
    fn get_evolution(&self) -> &Pokemon;
    fn as_any(&self) -> &dyn Any;
}

pub struct LevelUpEvolution {
    pub evolution: Pokemon,
    pub level: i8,
}

impl Evolution for LevelUpEvolution {
    fn get_evolution(&self) -> &Pokemon {
        &self.evolution
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LevelUpEvolution {
    pub fn get_level(&self) -> i8 {
        self.level
    }
}

pub struct TradeEvolution {
    pub evolution: Pokemon,
}

impl Evolution for TradeEvolution {
    fn get_evolution(&self) -> &Pokemon {
        &self.evolution
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TradeEvolutionHoldingItem {
    pub evolution: Pokemon,
    pub item: Item,
}

impl Evolution for TradeEvolutionHoldingItem {
    fn get_evolution(&self) -> &Pokemon {
        &self.evolution
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TradeEvolutionHoldingItem {
    pub fn get_item(&self) -> Item {
        self.item
    }
}

pub struct ItemEvolution {
    pub evolution: Pokemon,
    pub item: Item,
}

impl Evolution for ItemEvolution {
    fn get_evolution(&self) -> &Pokemon {
        &self.evolution
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ItemEvolution {
    pub fn get_item(&self) -> Item {
        self.item
    }
}
