use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

mod common;

use common::*;

#[test]
fn find_skips_missing_paths() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    cmd.args(["missing", "--", "expr"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn find_default_search() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    let expected = concat!(
        "tests/find/a\n",
        "tests/find/a/a.txt\n",
        "tests/find/a.txt\n",
        "tests/find/b/a\n",
        "tests/find/b/a/a.txt\n",
    );
    cmd.args(["tests/find", "--", "a"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn find_dir_search() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    let expected = concat!("tests/find/a\n", "tests/find/b/a\n",);
    cmd.args(["-t", "d", "tests/find", "--", "a"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn find_file_search() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    let expected = concat!(
        "tests/find/a/a.txt\n",
        "tests/find/a.txt\n",
        "tests/find/b/a/a.txt\n",
    );
    cmd.args(["-t", "f", "tests/find", "--", "a"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn find_no_results() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    cmd.args(["-t", "f", "tests/find", "--", "missing"])
        .assert()
        .success()
        .stdout("");

    Ok(())
}

#[test]
fn find_depth() -> TestResult {
    let mut cmd = Command::cargo_bin("find")?;
    let expected = concat!("tests/find/b\n");
    cmd.args(["-d", "1", "tests/find", "--", "b"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
