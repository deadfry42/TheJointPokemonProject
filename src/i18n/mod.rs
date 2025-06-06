pub mod important;
pub mod langs;
pub mod sections;

pub fn get_localisation() -> important::Localisation {
    // TODO: Support changing localisation
    langs::en_GB::LOCALISATION // base localisation
}
