use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn cut_skips_missing_file() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    cmd.args(["-b", "1", "missing.txt"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn cut_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    let expected = fs::read_to_string("tests/inputs/cut_bytes_expected.txt")?;
    cmd.args(["-b", "17-28,29-31", "tests/inputs/cut_bytes.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cut_chars() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    let expected = fs::read_to_string("tests/inputs/cut_chars_expected.txt")?;
    cmd.args(["-c", "17-28", "tests/inputs/cut_bytes.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cut_fields_csv() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    let expected = fs::read_to_string("tests/inputs/cut_csv_expected.txt")?;
    cmd.args(["-f", "1", "-d", ",", "tests/inputs/cut_csv.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cut_fields_spaces() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    let expected = fs::read_to_string("tests/inputs/cut_logs_expected.txt")?;
    cmd.args(["-f", "2-4", "-d", " ", "tests/inputs/cut_logs.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cut_fields_tabs() -> TestResult {
    let mut cmd = Command::cargo_bin("cut")?;
    let expected = fs::read_to_string("tests/inputs/cut_tsv_expected.txt")?;
    cmd.args(["-f", "1", "tests/inputs/cut_tsv.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
