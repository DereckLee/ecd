use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::{NamedTempFile, tempdir};

fn bin() -> Command {
    Command::cargo_bin("ecd").unwrap()
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn convert_help_prints() {
    bin()
        .args(["convert", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--from"))
        .stdout(predicate::str::contains("--to"));
}

#[test]
fn convert_utf8_to_gbk_stdout() {
    let file = fixture("utf8.txt");
    let output = bin()
        .args([
            "convert",
            "-f",
            &file.to_string_lossy(),
            "--from",
            "utf-8",
            "--to",
            "gbk",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert!(!output.is_empty());
}

#[test]
fn convert_to_file() {
    let input = fixture("utf8.txt");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("output.gbk");

    bin()
        .args([
            "convert",
            "-f",
            &input.to_string_lossy(),
            "--from",
            "utf-8",
            "--to",
            "gbk",
            "-o",
            &out_path.to_string_lossy(),
        ])
        .assert()
        .success();

    let written = std::fs::read(out_path).unwrap();
    assert!(!written.is_empty());
}

#[test]
fn convert_rejects_planned_encoding() {
    let file = fixture("utf8.txt");
    bin()
        .args([
            "convert",
            "-f",
            &file.to_string_lossy(),
            "--from",
            "utf-8",
            "--to",
            "tis-620",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"))
        .stderr(predicate::str::contains("tis-620"));
}

#[test]
fn convert_same_path_without_force() {
    let file = fixture("utf8.txt");
    bin()
        .args([
            "convert",
            "-f",
            &file.to_string_lossy(),
            "--from",
            "utf-8",
            "--to",
            "gbk",
            "-o",
            &file.to_string_lossy(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("output path equals input path"));
}

#[test]
fn convert_binary_file_fails() {
    let file = NamedTempFile::new().unwrap();
    std::fs::write(file.path(), b"hello\x00world\xff\xfe").unwrap();
    bin()
        .args([
            "convert",
            "-f",
            &file.path().to_string_lossy(),
            "--from",
            "utf-8",
            "--to",
            "gbk",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("binary content"));
}
