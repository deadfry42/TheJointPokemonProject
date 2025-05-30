use rand::RngCore;

use crate::utils::hex::decimal_to_hex;

use super::trainer::OTInformation;

pub fn generate_personality() -> u32 {
    rand::rng().next_u32()
}

pub fn generate_trainer_id() -> u16 {
    rand::rng().next_u32() as u16
}

pub fn determine_shininess(personality: &u32, ot: &OTInformation) -> bool {
    // using bulbapedia impl. S=IDTrainer⊕IDSecret⊕p1⊕p2
    // TODO: verify this works

    let p1 = personality / 65536;
    let p2 = personality % 65536;

    let shinyval = ot.id ^ ot.sid ^ p1 as u16 ^ p2 as u16;

    shinyval < 8 // 1/8192 allegedly
    // in xy+ it's shinyval < 16, but i like the harder odds
}
