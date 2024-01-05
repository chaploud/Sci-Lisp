use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn show_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "A Lisp for Scientific Computation written in Rust",
    ));
    Ok(())
}

#[test]
fn execute_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("tests/main.lisp");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Done."));
    Ok(())
}

#[test]
fn execute_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("lint").arg("tests/main.rs");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Done."));
    Ok(())
}
