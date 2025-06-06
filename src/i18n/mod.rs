use crate::i18n::langs::json_generate::parse_json_files;

pub mod langs;
pub mod localisation;
pub mod sections;

pub fn load_localisation() {
    parse_json_files();
}

pub fn get_localisation() -> localisation::Localisation {
    // TODO: Support changing localisation
    langs::en_GB::LOCALISATION // base localisation
}
