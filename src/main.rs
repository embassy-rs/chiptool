#![recursion_limit = "128"]

use log::error;
use svd_parser as svd;
use transform::Transform;

mod generate;
mod ir;
mod svd2ir;
mod transform;
mod util;

use std::fs::File;
use std::io::Write;
use std::process;

use anyhow::{Context, Result};
use clap::{App, Arg};
use log::*;

use crate::util::Target;

fn run() -> Result<()> {
    use std::io::Read;

    let matches = App::new("svd2rust")
        .about("Generate a Rust API from SVD files")
        .arg(
            Arg::with_name("input")
                .help("Input SVD file")
                .short("i")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("config")
                .help("Config file")
                .short("c")
                .takes_value(true)
                .value_name("CONFIG"),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .help("Target architecture")
                .takes_value(true)
                .value_name("ARCH"),
        )
        .arg(
            Arg::with_name("log_level")
                .long("log")
                .short("l")
                .help(&format!(
                    "Choose which messages to log (overrides {})",
                    env_logger::DEFAULT_FILTER_ENV
                ))
                .takes_value(true)
                .possible_values(&["off", "error", "warn", "info", "debug", "trace"]),
        )
        .version(concat!(
            env!("CARGO_PKG_VERSION"),
            include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"))
        ))
        .get_matches();

    setup_logging(&matches);

    let target = matches
        .value_of("target")
        .map(|s| Target::parse(s))
        .unwrap_or(Ok(Target::CortexM))?;

    let config = match matches.value_of("config") {
        Some(file) => {
            let config = &mut String::new();
            File::open(file)
                .context("Cannot open the config file")?
                .read_to_string(config)
                .context("Cannot read the config file")?;
            serde_yaml::from_str(config).context("cannot deserialize config")?
        }
        None => Config::default(),
    };

    let xml = &mut String::new();
    match matches.value_of("input") {
        Some(file) => {
            File::open(file)
                .context("Cannot open the SVD file")?
                .read_to_string(xml)
                .context("Cannot read the SVD file")?;
        }
        None => {
            let stdin = std::io::stdin();
            stdin
                .lock()
                .read_to_string(xml)
                .context("Cannot read from stdin")?;
        }
    }

    let device = svd::parse(xml)?;
    let mut ir = svd2ir::convert(&device);
    transform::sanitize(&mut ir);
    transform::MergeIdenticalEnums {}.run(&mut ir)?;

    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
    }

    //transform::find_dup_enums(&mut ir);
    //transform::find_dup_fieldsets(&mut ir);

    let mut device_x = String::new();
    let items = generate::render(&ir, target, &mut device_x)?;
    let mut file = File::create("lib.rs").expect("Couldn't create lib.rs file");

    let data = items.to_string().replace("] ", "]\n");
    file.write_all(data.as_ref())
        .expect("Could not write code to lib.rs");

    if target == Target::CortexM || target == Target::Msp430 || target == Target::XtensaLX {
        writeln!(File::create("device.x")?, "{}", device_x)?;
        writeln!(File::create("build.rs")?, "{}", util::build_rs())?;
    }

    Ok(())
}

fn setup_logging(matches: &clap::ArgMatches) {
    // * Log at info by default.
    // * Allow users the option of setting complex logging filters using
    //   env_logger's `RUST_LOG` environment variable.
    // * Override both of those if the logging level is set via the `--log`
    //   command line argument.
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp(None);

    let log_lvl_from_env = std::env::var_os(env_logger::DEFAULT_FILTER_ENV).is_some();

    if log_lvl_from_env {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        let level = match matches.value_of("log_level") {
            Some(lvl) => lvl.parse().unwrap(),
            None => log::LevelFilter::Info,
        };
        log::set_max_level(level);
        builder.filter_level(level);
    }

    builder.init();
}

fn main() {
    if let Err(ref e) = run() {
        error!("{:?}", e);

        process::exit(1);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    transforms: Vec<Transform>,
}

impl Default for Config {
    fn default() -> Self {
        Self { transforms: vec![] }
    }
}
