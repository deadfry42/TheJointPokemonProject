use crate::i18n::{
    localisation::*,
    sections::{abilities::*, enums::*, items::*, moves::*, natures::*, pokemon::*},
};
use serde_json::Value;
use std::fs;

pub fn parse_json_files() -> std::io::Result<Vec<Localisation>> {
    let paths = fs::read_dir("./assets/localisation/")?;
    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());
        let data = fs::read_to_string(&path.unwrap().path()).unwrap();
        let v: Value = serde_json::from_str(&data)?;

        println!("Test {}", v)
    }
    Ok(vec![])
}
