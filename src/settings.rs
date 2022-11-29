use std::{collections::HashMap};

#[derive(asr::Settings)]
pub struct SplitterSettings {
    #[default = true]
    auto_split: bool,
    #[default = true]
    auto_start: bool,
}

impl SplitterSettings {
    pub fn get_settings(&mut self) -> HashMap<&str, bool> {
        HashMap::from([
            ("auto_split", self.auto_split),
            ("auto_start", self.auto_start),
        ])
    }
}

