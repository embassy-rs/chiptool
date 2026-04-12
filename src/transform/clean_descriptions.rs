use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::ir::IR;

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanDescriptions {}

impl CleanDescriptions {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let description_cleanups = [
            // Fix weird newline spam in descriptions.
            (Regex::new("[ \n]+").unwrap(), " "),
            // Fix weird tab and cr spam in descriptions.
            (Regex::new("[\r\t]+").unwrap(), " "),
            // Replace double-space (end of sentence) with period.
            (
                Regex::new(r"(?<first_sentence>.*?)[\s]{2}(?<next_sentence>.*)").unwrap(),
                "$first_sentence. $next_sentence",
            ),
            // Make sure every description ends with a period.
            (
                Regex::new(r"(?<full_description>.*)(?<last_character>[\s'[^\.\s']])$").unwrap(),
                "$full_description$last_character.",
            ),
            // Eliminate space characters between end of description and the closing period.
            (
                Regex::new(r"(?<full_description>.*)\s\.$").unwrap(),
                "$full_description.",
            ),
        ];

        for (re, rep) in description_cleanups.iter() {
            crate::transform::map_descriptions(ir, |d| re.replace_all(d, *rep).into_owned())?;
        }

        Ok(())
    }
}
