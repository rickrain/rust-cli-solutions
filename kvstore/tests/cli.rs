use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "kvstore";

#[test]
fn no_args_will_show_usage() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE:\n"));
    Ok(())
}

#[test]
fn init_set_get() -> TestResult {
    let init_args = ["init"];
    let set_args = ["set", "foo", "bar"];
    let get_args = ["get", "foo"];

    // Create a new empty database
    Command::cargo_bin(PRG)?
        .args(init_args)
        .assert()
        .success();

    // Add key/value pair
    Command::cargo_bin(PRG)?
        .args(set_args)
        .assert()
        .success();

    // Retrieve key/value pair
    Command::cargo_bin(PRG)?
        .args(get_args)
        .assert()
        .success()
        .stdout(predicate::str::contains("bar"));

    Ok(())
}
