use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("ecd").unwrap()
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn version_prints() {
    let version = env!("CARGO_PKG_VERSION");
    bin()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(version));
}

#[test]
fn help_prints() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("ecd check -f man.txt"));
}

#[test]
fn check_help_prints() {
    bin()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--file"));
}

#[test]
fn single_file_outputs_encoding_only() {
    let file = fixture("utf8.txt");
    bin()
        .args(["check", "-f"])
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::is_match("^(utf-8|ascii)\n$").unwrap());
}

#[test]
fn single_gbk_file() {
    let file = fixture("gbk.txt");
    bin()
        .args(["check", "-f"])
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::is_match("^gbk\n$").unwrap());
}

#[test]
fn directory_scan_batch_format() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    bin()
        .args(["check", "-d"])
        .arg(&dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("[SKIP]"))
        .stdout(predicate::str::contains("utf8.txt").or(predicate::str::contains("UTF-8")));
}

#[test]
fn binary_file_is_skip_in_batch_mode() {
    let file = fixture("binary.dat");
    let other = fixture("ascii.txt");
    bin()
        .args([
            "check",
            "-f",
            &file.to_string_lossy(),
            "-f",
            &other.to_string_lossy(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("[SKIP]"))
        .stdout(predicate::str::contains("binary.dat"));
}

#[test]
fn ignore_encoding_filters_output() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    bin()
        .args(["check", "-d", &dir.to_string_lossy(), "-i", "ascii", "-q"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn no_input_errors() {
    bin()
        .arg("check")
        .assert()
        .failure()
        .stderr(predicate::str::contains("no input"));
}

#[test]
fn missing_file_errors() {
    bin()
        .args(["check", "-f", "/no/such/file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("path not found"));
}
