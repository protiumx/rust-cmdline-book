use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn head_args_conflict() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    cmd.args(["-n", "1", "-c", "11"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: the argument"));

    Ok(())
}

#[test]
fn head_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    cmd.args(["missing.txt"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn head_lines_default() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    let expected = fs::read_to_string("tests/inputs/head_default.txt")?;
    cmd.args(["tests/inputs/head_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_lines_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    let expected = fs::read_to_string("tests/inputs/head_lines_arg.txt")?;
    cmd.args(["tests/inputs/head_input.txt", "-n", "5"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_bytes_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    let expected = fs::read_to_string("tests/inputs/head_bytes_arg.txt")?;
    cmd.args(["tests/inputs/head_input.txt", "-c", "6"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_handles_errors() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    cmd.args(["-n", "4", "tests/inputs/head_input.txt", "missing"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn head_handles_multiple() -> TestResult {
    let mut cmd = Command::cargo_bin("head")?;
    let expected = fs::read_to_string("tests/inputs/head_multiple.txt")?;
    cmd.args([
        "-n",
        "2",
        "tests/inputs/head_input.txt",
        "tests/inputs/head_input.txt",
    ])
    .assert()
    .success()
    .stdout(expected);

    Ok(())
}
