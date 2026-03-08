//! Basic command tests.

use anyhow::{bail, Context, Result};
use assert_cmd::assert::Assert;
use assert_cmd::cargo::cargo_bin_cmd;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

trait AssertExt {
    /// Print the command output to stdout and stderr.
    fn print(self) -> Self;
}
impl AssertExt for Assert {
    fn print(self) -> Self {
        let output = self.get_output();
        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        self
    }
}

/// Create a temporary dir.
fn create_temporary_dir() -> Result<TempDir> {
    TempDir::new().context("Failed to create temporary dir")
}

/// Copy svd file.
fn copy_svd_to(dir: &Path, filename: &str) -> Result<u64> {
    let from = "tests/ARM_Example.svd";
    let to = dir.join(filename);
    fs::copy(&from, &to).with_context(|| format!("Failed to copy {from:?} to {to:?}"))
}

/// Copy transform file.
fn copy_transform_to(dir: &Path, filename: &str) -> Result<u64> {
    let from = "tests/transform.yaml";
    let to = dir.join(filename);
    fs::copy(&from, &to).with_context(|| format!("Failed to copy {from:?} to {to:?}"))
}

/// Check that the files exit in the directory.
fn expect_files_in(dir: &Path, files: &[&str]) -> Result<()> {
    let found = fs::read_dir(dir)
        .with_context(|| format!("Failed to read dir {dir:?}"))?
        .filter_map(|result| {
            if let Ok(entry) = result {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        return Some(entry.file_name());
                    }
                } else {
                    eprintln!("TEST: bad entry {entry:?}");
                }
            } else {
                eprintln!("TEST: bad result {result:?}");
            }
            None
        })
        .collect::<Vec<_>>();
    'next_file: for file in files {
        for name in &found {
            if file == name {
                continue 'next_file;
            }
        }
        bail!("Failed to find file {file:?}, found {found:?}");
    }
    Ok(())
}

/// Command `generate` works.
#[test]
fn generate() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["generate", "--svd", "example.svd"]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["lib.rs", "device.x"])?;
    Ok(())
}

/// Command `extract-all` works.
#[test]
fn extract_all() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["extract-all", "--svd", "example.svd", "--output", "."]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["TIMER0.yaml"])?;
    if false {
        // FIXME this command does not support derivedFrom
        expect_files_in(tmp.path(), &["TIMER1.yaml", "TIMER2.yaml"])?;
    }
    Ok(())
}

/// Command `extract-peripheral` works.
#[test]
fn extract_peripheral() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    for peripheral in ["TIMER0", "TIMER1", "TIMER2"] {
        let mut cmd = cargo_bin_cmd!();
        cmd.current_dir(tmp.path());
        cmd.args([
            "extract-peripheral",
            "--svd",
            "example.svd",
            "--peripheral",
            peripheral,
        ]);
        cmd.assert()
            .append_context("cmd", format!("{cmd:?}"))
            .success()
            .print();
        // FIXME the block name of TIMER1 and TIMER2 say TIMER0
    }
    Ok(())
}

/// Command `transform` works.
#[test]
fn transform() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    copy_transform_to(tmp.path(), "transform.yaml")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["extract-all", "--svd", "example.svd", "--output", "."]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["TIMER0.yaml"])?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args([
        "transform",
        "--input",
        "TIMER0.yaml",
        "--output",
        "output.yaml",
        "--transform",
        "transform.yaml",
    ]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["output.yaml"])?;
    Ok(())
}

/// Command `fmt` works.
#[test]
fn fmt() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["extract-all", "--svd", "example.svd", "--output", "."]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["TIMER0.yaml"])?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["fmt", "TIMER0.yaml"]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    Ok(())
}

/// Command `check` works.
#[test]
fn check() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["extract-all", "--svd", "example.svd", "--output", "."]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["TIMER0.yaml"])?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["check", "TIMER0.yaml", "--allow-register-overlap"]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    Ok(())
}

/// Command `gen-block` works.
#[test]
fn gen_block() -> Result<()> {
    let tmp = create_temporary_dir()?;
    copy_svd_to(tmp.path(), "example.svd")?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args(["extract-all", "--svd", "example.svd", "--output", "."]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["TIMER0.yaml"])?;
    let mut cmd = cargo_bin_cmd!();
    cmd.current_dir(tmp.path());
    cmd.args([
        "gen-block",
        "--input",
        "TIMER0.yaml",
        "--output",
        "timer0.rs",
    ]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    expect_files_in(tmp.path(), &["timer0.rs"])?;
    Ok(())
}

/// Command `help` works.
#[test]
fn help() -> Result<()> {
    let mut cmd = cargo_bin_cmd!();
    cmd.args(["help"]);
    cmd.assert()
        .append_context("cmd", format!("{cmd:?}"))
        .success()
        .print();
    Ok(())
}
