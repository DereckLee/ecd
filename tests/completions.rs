use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("ecd").unwrap()
}

#[test]
fn complete_bash_generates() {
    bin()
        .args(["complete", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ecd"));
}

#[test]
fn complete_hidden_from_top_help() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete").not());
}

#[test]
fn complete_subcommand_has_help() {
    bin()
        .args(["complete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bash"));
}
