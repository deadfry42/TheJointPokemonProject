use std::str::Split;

use crate::I18NCore::{
    localisation::{DataSection, Locale, SectionType, SingleValuedData},
    parsing::get_localisation,
};

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
        if self.path.len() == 0 {
            return "???";
        }

        let locale: &Locale = get_localisation();

        let mut count = -1;
        let mut truncated_path: Vec<&'static str> = self.path.clone();
        truncated_path.retain(|_| {
            // remove first element
            count += 1;
            count > 0
        });

        let mut current_section: Option<Box<&dyn DataSection>> = None;

        // first iteration, for locale, cuz it's different
        // value from locale base, has to be singlevalued
        if let Some(first_key) = self.path.get(0) {
            let basesection: Box<&dyn DataSection> = locale.index(first_key);

            if basesection.get_section_type() == SectionType::Container
                || basesection.get_section_type() == SectionType::Data
            {
                current_section = Some(basesection);
            } else if let Some(singlevalued) = basesection
                .as_ref()
                .as_any()
                .downcast_ref::<SingleValuedData>()
            {
                return singlevalued.value;
            }
        } else {
            return "???";
        }

        if count <= 0 || current_section.is_none() {
            return "???";
        } // count == truncated_path.len()

        for key in truncated_path.iter() {
            // like folder navigation but way more complicated than it should be

            if let Some(validsection) = &current_section {
                if validsection.get_section_type() == SectionType::Container {
                    current_section = validsection.run_container_index(key)
                } else if validsection.get_section_type() == SectionType::Data {
                    return validsection.run_data_index(key).unwrap_or("???");
                } else if let Some(singlevalued) = current_section
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .as_any()
                    .downcast_ref::<SingleValuedData>()
                {
                    return singlevalued.value;
                }
            }

            // else if let Some(i18n) = current_section
            //     .as_ref()
            //     .unwrap()
            //     .as_any()
            //     .downcast_ref::<Box<&dyn I18NData>>()
            // {
            //     return i18n.index(key);
            // } else if let Some(translation) = current_section
            //     .as_ref()
            //     .unwrap()
            //     .as_any()
            //     .downcast_ref::<Box<&dyn TranslationData>>()
            // {
            //     current_section = Some(translation.index(key))
            // }
        }

        "???"
    }
}
