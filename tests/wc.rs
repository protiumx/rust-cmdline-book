use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn wc_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("wc")?;
    cmd.args(["missing.txt"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn wc_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("wc")?;
    let expected = fs::read_to_string("tests/inputs/wc_no_args.txt")?;
    cmd.args(["tests/inputs/wc_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_lines_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("wc")?;
    let expected = fs::read_to_string("tests/inputs/wc_lines_arg.txt")?;
    cmd.args(["tests/inputs/wc_input.txt", "-l"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_multiple_files() -> TestResult {
    let mut cmd = Command::cargo_bin("wc")?;
    let expected = fs::read_to_string("tests/inputs/wc_multiple.txt")?;
    cmd.args(["tests/inputs/wc_input.txt", "tests/inputs/wc_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
