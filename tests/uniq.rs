use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn uniq_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    cmd.args(["missing.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn uniq_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    let expected = fs::read_to_string("tests/inputs/uniq_no_args.txt")?;
    cmd.args(["tests/inputs/uniq_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn uniq_repated() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    let expected = fs::read_to_string("tests/inputs/uniq_repeated.txt")?;
    cmd.args(["-d", "tests/inputs/uniq_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn uniq_unique() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    let expected = fs::read_to_string("tests/inputs/uniq_unique.txt")?;
    cmd.args(["-u", "tests/inputs/uniq_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn uniq_count() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    let expected = fs::read_to_string("tests/inputs/uniq_count.txt")?;
    cmd.args(["-c", "tests/inputs/uniq_input.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn uniq_stdin() -> TestResult {
    let mut cmd = Command::cargo_bin("uniq")?;
    let expected = "a\nb\n";
    cmd.args(["-"])
        .write_stdin("a\na\nb\nb\nb\n")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
