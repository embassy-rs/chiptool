use anyhow::{Context, Result};
use log::error;
use log::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};
use svd_parser as svd;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadSvd {
    pub file: String,
    pub path: String,
}

impl LoadSvd {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path = self.path.split("::").map(|x| x.to_string()).collect();

        let xml = &mut String::new();
        File::open(&self.file)
            .context("Cannot open the SVD file")?
            .read_to_string(xml)
            .context("Cannot read the SVD file")?;

        let device = svd::parse(xml)?;
        crate::svd2ir::convert(ir, &device, path)?;

        Ok(())
    }
}
