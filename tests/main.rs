use assert_cmd::prelude::*;
use assert_cmd::Command as AssertCmd;
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
    cmd.assert().success().stdout(predicate::str::contains(
        "Hello from Sci-Lisp! [2024, 2024]",
    ));
    Ok(())
}

#[test]
fn execute_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("tests/notexist.rs");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("IO Error"));
    Ok(())
}

#[test]
fn execute_success_repl_00001() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (+ 1 1)
        "#,
    );
    let out = "2";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
