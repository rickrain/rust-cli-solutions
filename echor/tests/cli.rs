use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello world").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "world"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1n() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.n.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello world", "-n"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello2n() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.n.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "world", "-n"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
