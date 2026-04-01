#![recursion_limit = "128"]

use anyhow::Result;
use chiptool::commands::check::{check, Check};
use chiptool::commands::extract_all::{extract_all, ExtractAll};
use chiptool::commands::extract_peripheral::{extract_peripheral, ExtractPeripheral};
use chiptool::commands::fmt::{fmt, Fmt};
use chiptool::commands::gen_block::{gen_block, GenBlock};
use chiptool::commands::gen_common::{gen_common, GenCommon};
use chiptool::commands::generate::{generate, Generate};
use chiptool::commands::transform::{transform, Transform};
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Dirbaio <dirbaio@dirbaio.net>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    Generate(Generate),
    ExtractAll(ExtractAll),
    ExtractPeripheral(ExtractPeripheral),
    Transform(Transform),
    Fmt(Fmt),
    Check(Check),
    GenBlock(GenBlock),
    GenCommon(GenCommon),
}

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::ExtractPeripheral(x) => extract_peripheral(x),
        Subcommand::ExtractAll(x) => extract_all(x),
        Subcommand::Generate(x) => generate(x),
        Subcommand::Transform(x) => transform(x),
        Subcommand::Fmt(x) => fmt(x),
        Subcommand::Check(x) => check(x),
        Subcommand::GenBlock(x) => gen_block(x),
        Subcommand::GenCommon(x) => gen_common(x),
    }
}
