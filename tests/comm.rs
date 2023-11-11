use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn comm_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("comm")?;
    cmd.args(["file1", "file2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn comm_default() -> TestResult {
    let mut cmd = Command::cargo_bin("comm")?;
    let expected = fs::read_to_string("tests/inputs/comm_default_expected.txt")?;
    cmd.args(["tests/inputs/comm_1.txt", "tests/inputs/comm_2.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn comm_delimiter() -> TestResult {
    let mut cmd = Command::cargo_bin("comm")?;
    let expected = fs::read_to_string("tests/inputs/comm_delimiter_expected.txt")?;
    cmd.args([
        "-d",
        ">>",
        "tests/inputs/comm_1.txt",
        "tests/inputs/comm_2.txt",
    ])
    .assert()
    .success()
    .stdout(expected);

    Ok(())
}

#[test]
fn comm_column1() -> TestResult {
    let mut cmd = Command::cargo_bin("comm")?;
    let expected = fs::read_to_string("tests/inputs/comm_col1_expected.txt")?;
    cmd.args(["-1", "tests/inputs/comm_1.txt", "tests/inputs/comm_2.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn comm_case_insensitive() -> TestResult {
    let mut cmd = Command::cargo_bin("comm")?;
    let expected = fs::read_to_string("tests/inputs/comm_case_insensitive_expected.txt")?;
    cmd.args(["-i", "tests/inputs/comm_1.txt", "tests/inputs/comm_2.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
