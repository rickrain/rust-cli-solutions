use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
static BINARY_NAME: &str = "echor";

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run_expected(&["Hello world"], "tests/expected/hello1.txt")?;
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    run_expected(&["Hello", "world"], "tests/expected/hello2.txt")?;
    Ok(())
}

#[test]
fn hello1n() -> TestResult {
    run_expected(&["Hello world", "-n"], "tests/expected/hello1.n.txt")?;
    Ok(())
}

#[test]
fn hello2n() -> TestResult {
    run_expected(&["Hello", "world", "-n"], "tests/expected/hello2.n.txt")?;
    Ok(())
}

fn run_expected(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}