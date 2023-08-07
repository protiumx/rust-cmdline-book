use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn cat_checks_conflict_flags() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.args(["-n", "-b"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: the argument"));

    Ok(())
}

#[test]
fn cat_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.args(["missing.txt"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn cat_empty() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.args(["tests/inputs/cat_empty.txt"])
        .assert()
        .success()
        .stdout(predicate::eq(""));

    Ok(())
}

#[test]
fn cat_numbers() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    let expected = fs::read_to_string("tests/inputs/cat_numbers.txt")?;
    cmd.args(["tests/inputs/cat_input.txt", "-n"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_no_blanks() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    let expected = fs::read_to_string("tests/inputs/cat_no_blanks.txt")?;
    cmd.args(["tests/inputs/cat_input.txt", "-b"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_stdin() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.args(["-"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("test\n");

    Ok(())
}
