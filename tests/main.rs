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

// REPL tests

// literal
#[test]
fn execute_success_repl_00001() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        "abc\n"
        "#,
    );
    let out = "\"abc\\n\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00002() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        #"[0-9]+"
        "##,
    );
    let out = "#\"[0-9]+\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00003() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        false
        "#,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00004() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        true
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00005() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        nil
        "#,
    );
    let out = "nil";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00006() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        -987_654_321
        "#,
    );
    let out = "-987654321";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00007() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        -3.14e15
        "#,
    );
    let out = "-3140000000000000";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00008() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        nan
        "#,
    );
    let out = "NaN";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00009() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        inf
        "#,
    );
    let out = "inf";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00010() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        -inf
        "#,
    );
    let out = "-inf";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00011() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        -0.0
        "#,
    );
    let out = "-0";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00012() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        :keyword
        "#,
    );
    let out = ":keyword";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00013() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        'symbol
        "#,
    );
    let out = "symbol";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

// truthy/falsy
#[test]
fn execute_success_repl_00014() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if "" true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00015() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if '() true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00016() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if [] true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00017() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if {} true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00018() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if #{} true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00019() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if 0 true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00020() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if nan true false)
        "#,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00021() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if false true false)
        "#,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00022() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (if nil true false)
        "#,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

// Collection
#[test]
fn execute_success_repl_00023() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        '(1, "a", :b)
        "#,
    );
    let out = "(1 \"a\" :b)";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00024() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        [1.01, 2.01, 3.01]
        "#,
    );
    let out = "[1.01, 2.01, 3.01]";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00025() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        {:a "a", :b "b", :c "c"}
        "#,
    );
    let out = "{:a \"a\", :b \"b\", :c \"c\"}";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_success_repl_00026() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        #{:a, :b, :c}
        "#,
    );
    let out = "#{:a, :b, :c}";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00027() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (type [1, 2, 3])
        "#,
    );
    let out = "\"vector\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00028() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (print [1, 2] "abc\n" 123)
        "#,
    );
    let out = "[1, 2] abc\n 123\nnil";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_success_repl_00029() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        (def a "abcde")
        a
        "#,
    );
    let out = "a\n\"abcde\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
