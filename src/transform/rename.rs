use std::collections::HashMap;
use std::mem;

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub from: String,
    pub to: String,
}

impl Rename {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        do_rename(&re, &self.to, &mut ir.devices);
        do_rename(&re, &self.to, &mut ir.blocks);
        do_rename(&re, &self.to, &mut ir.fieldsets);
        do_rename(&re, &self.to, &mut ir.enums);

        Ok(())
    }
}

fn do_rename<T>(from: &Regex, to: &str, data: &mut HashMap<String, T>) {
    let old_data = mem::replace(data, HashMap::new());
    for (name, x) in old_data {
        if let Some(res) = match_expand(&name, from, to) {
            data.insert(res, x);
        } else {
            data.insert(name, x);
        }
    }
}
