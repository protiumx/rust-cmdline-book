use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn grep_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("grep")?;
    cmd.args(["test", "missing.txt"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn grep_simple() -> TestResult {
    let mut cmd = Command::cargo_bin("grep")?;
    let expected = fs::read_to_string("tests/inputs/grep_simple_expected.txt")?;
    cmd.args(["no", "tests/inputs/grep/a.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn grep_inverse() -> TestResult {
    let mut cmd = Command::cargo_bin("grep")?;
    let expected = fs::read_to_string("tests/inputs/grep_inverse_expected.txt")?;
    cmd.args(["-v", "no", "tests/inputs/grep/a.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn grep_case_insensitive() -> TestResult {
    let mut cmd = Command::cargo_bin("grep")?;
    let expected = fs::read_to_string("tests/inputs/grep_insensitive_expected.txt")?;
    cmd.args(["-i", "error", "tests/inputs/grep/b.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn grep_recursive() -> TestResult {
    let mut cmd = Command::cargo_bin("grep")?;
    let expected = fs::read_to_string("tests/inputs/grep_recursive_expected.txt")?;
    cmd.args(["-r", "me", "tests/inputs/grep"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
