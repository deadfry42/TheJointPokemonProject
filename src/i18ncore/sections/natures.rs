use crate::i18ncore::localisation::{DataSection, SectionType};
use std::any::Any;

#[derive(Clone, Copy)]
pub struct NatureLocale {
    pub hardy: &'static str,
    pub lonely: &'static str,
    pub brave: &'static str,
    pub adamant: &'static str,
    pub naughty: &'static str,
    pub bold: &'static str,
    pub docile: &'static str,
    pub relaxed: &'static str,
    pub impish: &'static str,
    pub lax: &'static str,
    pub timid: &'static str,
    pub hasty: &'static str,
    pub serious: &'static str,
    pub jolly: &'static str,
    pub naive: &'static str,
    pub modest: &'static str,
    pub mild: &'static str,
    pub quiet: &'static str,
    pub bashful: &'static str,
    pub rash: &'static str,
    pub calm: &'static str,
    pub gentle: &'static str,
    pub sassy: &'static str,
    pub careful: &'static str,
    pub quirky: &'static str,
}

impl DataSection for NatureLocale {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_section_type(&self) -> crate::i18ncore::localisation::SectionType {
        SectionType::Data
    }

    fn run_data_index(&self, path: &'static str) -> Option<&'static str> {
        Some(match path {
            "hardy" => self.hardy,
            "lonely" => self.lonely,
            "brave" => self.brave,
            "adamant" => self.adamant,
            "naughty" => self.naughty,
            "bold" => self.bold,
            "docile" => self.docile,
            "relaxed" => self.relaxed,
            "impish" => self.impish,
            "lax" => self.lax,
            "timid" => self.timid,
            "hasty" => self.hasty,
            "serious" => self.serious,
            "jolly" => self.jolly,
            "naive" => self.naive,
            "modest" => self.modest,
            "mild" => self.mild,
            "quiet" => self.quiet,
            "bashful" => self.bashful,
            "rash" => self.rash,
            "calm" => self.calm,
            "gentle" => self.gentle,
            "sassy" => self.sassy,
            "careful" => self.careful,
            _ => self.quirky,
        })
    }

    fn run_container_index(&self, _: &'static str) -> Option<Box<&dyn DataSection>> {
        None
    }
}
