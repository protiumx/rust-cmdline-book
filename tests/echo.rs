use assert_cmd::Command;
use predicates::prelude::*;

mod common;

use common::*;

#[test]
fn echoes() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("hello").assert().success().stdout("hello\n");

    Ok(())
}

#[test]
fn omits_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.args(["-n", "hello"])
        .assert()
        .success()
        .stdout(predicate::eq(b"hello" as &[u8]));

    Ok(())
}
