use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use chiptool::svd2ir::NamespaceMode;

const SVDS_REPO_URL: &str = "https://github.com/embassy-rs/chiptool-test-svds.git";
const SVDS_REPO_COMMIT: &str = "0e8e1c19b98b5e09d757a48f44c9ed1cfd1684db";

fn ensure_svds_repo() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("svds");

    if dir.join(".git").is_dir() {
        let head = Command::new("git")
            .args(["-C"])
            .arg(&dir)
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("git rev-parse failed");
        if head.status.success()
            && std::str::from_utf8(&head.stdout).unwrap().trim() == SVDS_REPO_COMMIT
        {
            return dir;
        }
    }

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("failed to remove stale svds dir");
    }

    let status = Command::new("git")
        .args(["clone", "--no-checkout", SVDS_REPO_URL])
        .arg(&dir)
        .status()
        .expect("git clone failed to spawn");
    assert!(status.success(), "git clone of {SVDS_REPO_URL} failed");

    let status = Command::new("git")
        .args(["-C"])
        .arg(&dir)
        .args(["checkout", "--detach", SVDS_REPO_COMMIT])
        .status()
        .expect("git checkout failed to spawn");
    assert!(status.success(), "git checkout {SVDS_REPO_COMMIT} failed");

    dir
}

fn process_svd(svd_path: &Path, out_dir: &Path) {
    eprintln!("Processing {}...", svd_path.display());
    let chip = svd_path.file_stem().unwrap().to_str().unwrap();
    let chip_out = out_dir.join(chip);
    if chip_out.exists() {
        fs::remove_dir_all(&chip_out).unwrap();
    }
    fs::create_dir_all(&chip_out).unwrap();

    let xml = fs::read_to_string(svd_path).unwrap();
    let config = svd_parser::Config::default()
        .expand_properties(true)
        .validate_level(svd_parser::ValidateLevel::Disabled);
    let device = svd_parser::parse_with_config(&xml, &config)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", svd_path.display()));

    for p in &device.peripherals {
        if p.derived_from.is_some() {
            continue;
        }
        let ir = chiptool::commands::extract_peripheral(p, NamespaceMode::None)
            .unwrap_or_else(|e| panic!("extract_peripheral failed for {}/{}: {e}", chip, p.name));

        let yaml = serde_yaml::to_string(&ir).unwrap();
        fs::write(chip_out.join(format!("{}.yaml", p.name)), yaml).unwrap();
    }
}

#[test]
fn svd2ir() {
    let svds_dir = ensure_svds_repo();
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("output");
    fs::create_dir_all(&out_dir).unwrap();

    let mut svds: Vec<_> = fs::read_dir(&svds_dir)
        .unwrap()
        .filter_map(|e| {
            let p = e.unwrap().path();
            (p.extension().and_then(|s| s.to_str()) == Some("svd")).then_some(p)
        })
        .collect();
    svds.sort();
    assert!(!svds.is_empty(), "no SVDs found in {}", svds_dir.display());

    for svd in &svds {
        process_svd(svd, &out_dir);
    }
}
