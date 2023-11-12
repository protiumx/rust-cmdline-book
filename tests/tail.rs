use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn tail_args_conflict() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    cmd.args(["-n", "1", "-c", "11"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: the argument"));

    Ok(())
}

#[test]
fn tail_handles_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    cmd.args(["missing.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn tail_default() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_default_expected.txt")?;
    cmd.args(["tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_lines_expected.txt")?;
    cmd.args(["-n", "5", "tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_lines_plus() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_lines_plus_expected.txt")?;
    cmd.args(["-n", "+2", "tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_lines_zero() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    cmd.args(["-n", "0", "tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn tail_lines_zero_plus() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_lines_zero_plus_expected.txt")?;
    cmd.args(["-n", "+0", "tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_mutilple_files() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_multiple_expected.txt")?;
    cmd.args([
        "-n",
        "5",
        "tests/inputs/tail_input.txt",
        "tests/inputs/tail_input.txt",
    ])
    .assert()
    .success()
    .stdout(expected);

    Ok(())
}
#[test]
fn tail_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_bytes_expected.txt")?;
    cmd.args(["tests/inputs/tail_input.txt", "-c", "10"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_bytes_plus() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_bytes_plus_expected.txt")?;
    cmd.args(["tests/inputs/tail_input.txt", "-c", "+5"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn tail_bytes_zero() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    cmd.args(["-c", "0", "tests/inputs/tail_input.txt"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn tail_bytes_zero_plus() -> TestResult {
    let mut cmd = Command::cargo_bin("tail")?;
    let expected = fs::read_to_string("tests/inputs/tail_bytes_zero_plus_expected.txt")?;
    cmd.args(["tests/inputs/tail_input.txt", "-c", "+0"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
