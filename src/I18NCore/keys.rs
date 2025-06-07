use std::str::Split;

pub struct TranslationKey {
    pub path: Vec<&'static str>,
}

impl TranslationKey {
    pub fn new(path: &'static str) -> TranslationKey {
        let splits: Split<'_, &'static str> = path.split("/");

        TranslationKey {
            path: splits.collect::<Vec<&'static str>>(),
        }
    }

    pub fn convert_to_string(&self) -> &'static str {
        "sigma"
    }
}
